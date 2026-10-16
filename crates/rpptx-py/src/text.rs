use oxml_py_support::{ContentPath, PathSeg};
use pyo3::PyClass;
use pyo3::exceptions::{PyIndexError, PyNotImplementedError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBool, PyFloat, PyList, PySlice, PyString};
use rpptx::{
    AutofitMode, CT_TextCharacterProperties, CT_TextParagraphProperties, ColorChoice, Emu, Fill,
    RgbColor, SolidFill, TextAlignment, TextAnchor, TextAutoNumber, TextAutoNumberScheme,
    TextBullet, TextBulletCharacter, TextBulletChoice, TextBulletColor, TextBulletSize,
    TextBulletSizeValue, TextFont, TextNoBullet, TextPointValue, TextSpacing, TextStrike,
    TextUnderline,
};

use crate::normalize_index;
use crate::presentation::PyPresentation;
use crate::replacement_count_to_pyerr;
use crate::rpptx_to_pyerr;
use crate::shape::{shape_ref_at, slide_index};
use crate::slide::PySlide;
use crate::table::cell_ref_at;
use crate::validate_path;

const EMU_PER_CENTIPOINT: i64 = 127;
const MAX_SPACING_EMU: i64 = 158_400 * EMU_PER_CENTIPOINT;
const MAX_TEXT_MARGIN: i64 = 51_206_400;

/// `ST_TextFontSize` in hundredths of a point, 1 to 4000 points.
const MIN_FONT_SIZE: i64 = 100;
const MAX_FONT_SIZE: i64 = 400_000;

/// `a:lnSpc`, `a:spcBef`, and `a:spcAft` accept at most 132 lines.
const MAX_SPACING_LINES: f64 = 132.0;

fn length_object(py: Python<'_>, emu: i64) -> PyResult<Py<PyAny>> {
    py.import("rpptx")?
        .getattr("Length")?
        .call1((emu,))
        .map(Bound::unbind)
}

pub(crate) fn anchor_object(py: Python<'_>, anchor: TextAnchor) -> PyResult<Py<PyAny>> {
    let value = match anchor {
        TextAnchor::Top => 1,
        TextAnchor::Center => 3,
        TextAnchor::Bottom => 4,
        TextAnchor::Justified => 6,
        TextAnchor::Distributed => 7,
    };
    text_enum(py, "MSO_VERTICAL_ANCHOR", value)
}

pub(crate) fn anchor_from_value(value: i32) -> PyResult<TextAnchor> {
    match value {
        1 => Ok(TextAnchor::Top),
        3 => Ok(TextAnchor::Center),
        4 => Ok(TextAnchor::Bottom),
        6 => Ok(TextAnchor::Justified),
        7 => Ok(TextAnchor::Distributed),
        _ => Err(PyValueError::new_err(
            "vertical anchor must be an MSO_ANCHOR member",
        )),
    }
}

fn text_enum(py: Python<'_>, name: &str, value: i32) -> PyResult<Py<PyAny>> {
    py.import("rpptx.enum.text")?
        .getattr(name)?
        .call1((value,))
        .map(Bound::unbind)
}

fn spacing_object(py: Python<'_>, spacing: Option<TextSpacing>) -> PyResult<Option<Py<PyAny>>> {
    match spacing {
        None => Ok(None),
        Some(TextSpacing::Points(centipoints)) => {
            length_object(py, i64::from(centipoints) * EMU_PER_CENTIPOINT).map(Some)
        }
        Some(TextSpacing::Percent(value)) => {
            let lines = match value.strip_suffix('%') {
                Some(percent) => percent.parse::<f64>().map(|percent| percent / 100.0),
                None => value.parse::<f64>().map(|value| value / 100_000.0),
            };
            let lines = lines.map_err(|_| PyValueError::new_err("spacing is not a number"))?;
            Ok(Some(PyFloat::new(py, lines).into_any().unbind()))
        }
    }
}

fn points_spacing(emu: i64) -> PyResult<TextSpacing> {
    if !(0..=MAX_SPACING_EMU).contains(&emu) {
        return Err(PyValueError::new_err(
            "spacing must be from 0 to 1584 points",
        ));
    }
    let centipoints = emu / EMU_PER_CENTIPOINT;
    if emu != 0 && centipoints == 0 {
        return Err(PyValueError::new_err(
            "spacing bare int is EMU; storage is 0.01 point; use rpptx.util.Pt(6) or an explicit float line multiple",
        ));
    }
    Ok(TextSpacing::Points(centipoints as i32))
}

fn lines_spacing(lines: f64) -> PyResult<TextSpacing> {
    if !(0.0..=MAX_SPACING_LINES).contains(&lines) {
        return Err(PyValueError::new_err("spacing must be from 0 to 132 lines"));
    }
    Ok(TextSpacing::Percent(
        ((lines * 100_000.0).round() as i64).to_string(),
    ))
}

fn margin_value(value: Option<i64>, min: i64, name: &str) -> PyResult<Option<i32>> {
    value
        .map(|value| {
            if (min..=MAX_TEXT_MARGIN).contains(&value) {
                Ok(value as i32)
            } else {
                Err(PyValueError::new_err(format!(
                    "{name} must be from {min} to {MAX_TEXT_MARGIN} EMU"
                )))
            }
        })
        .transpose()
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyTextFrame>()?;
    module.add_class::<PyParagraph>()?;
    module.add_class::<PyParagraphCollection>()?;
    module.add_class::<PyRun>()?;
    module.add_class::<PyRunCollection>()?;
    module.add_class::<PyHyperlink>()?;
    module.add_class::<PyFont>()?;
    module.add_class::<PyMasterTextStyles>()?;
    module.add_class::<PyTextStyleList>()?;
    Ok(())
}

fn paragraph_index(path: &ContentPath) -> Option<usize> {
    path.segs.iter().find_map(|segment| match segment {
        PathSeg::Para(index) => Some(*index),
        _ => None,
    })
}

fn run_index(path: &ContentPath) -> Option<usize> {
    path.segs.iter().find_map(|segment| match segment {
        PathSeg::Run(index) => Some(*index),
        _ => None,
    })
}

fn names_cell(path: &ContentPath) -> bool {
    path.segs
        .iter()
        .any(|segment| matches!(segment, PathSeg::Row(_)))
        && path
            .segs
            .iter()
            .any(|segment| matches!(segment, PathSeg::Cell(_)))
}

fn text_frame_ref_at<'a>(
    presentation: &'a PyPresentation,
    path: &ContentPath,
) -> Option<rpptx::TextFrameRef<'a>> {
    if names_cell(path) {
        cell_ref_at(presentation, path)?.text_frame()
    } else {
        shape_ref_at(presentation, path)?.text_frame()
    }
}

fn text_frame_mut_at<'a>(
    presentation: &'a mut PyPresentation,
    path: &ContentPath,
) -> Option<rpptx::TextFrame<'a>> {
    let current = presentation.current_path(path)?;
    native_text_frame_mut_at(&mut presentation.inner, &current)
}

fn native_text_frame_mut_at<'a>(
    presentation: &'a mut rpptx::Presentation,
    path: &ContentPath,
) -> Option<rpptx::TextFrame<'a>> {
    if names_cell(path) {
        crate::table::native_cell_mut_at(presentation, path)
            .map(rpptx::TableCellMut::into_text_frame)
    } else {
        crate::shape::native_shape_mut_at(presentation, path)?.into_text_frame()
    }
}

fn style_address(
    presentation: &PyPresentation,
    path: &ContentPath,
) -> Option<(usize, &'static str, usize)> {
    let current = presentation.current_path(path)?;
    let master = current.segs.iter().find_map(|segment| {
        if let PathSeg::Master(index) = segment {
            Some(*index)
        } else {
            None
        }
    })?;
    let style = current.segs.iter().find_map(|segment| {
        if let PathSeg::TextStyle(index) = segment {
            ["title", "body", "other"].get(*index).copied()
        } else {
            None
        }
    })?;
    Some((master, style, paragraph_index(&current)? + 1))
}

fn update_style_level(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    update: impl FnOnce(&mut CT_TextParagraphProperties) -> PyResult<()>,
) -> PyResult<()> {
    let (master, style, level) = style_address(presentation, path)
        .ok_or_else(|| PyIndexError::new_err("master style owner is absent"))?;
    let mut properties = presentation
        .inner
        .master_text_style(master, style, level)
        .map_err(|error| crate::rpptx_to_pyerr(py, error))?
        .cloned()
        .unwrap_or_default();
    let before = properties.clone();
    update(&mut properties)?;
    if properties == before {
        return Ok(());
    }
    let mut candidate = presentation.inner.clone();
    candidate
        .set_master_text_style(master, style, level, properties)
        .map_err(|error| crate::rpptx_to_pyerr(py, error))?;
    let proofs = presentation.unchanged_owners()?;
    let prepared = presentation.prepare_owners(
        &candidate,
        "MasterTextStyles.level",
        &proofs,
        false,
        false,
        false,
    )?;
    presentation.publish_owners(candidate, prepared)
}

fn font_properties(
    presentation: &PyPresentation,
    path: &ContentPath,
) -> Option<rpptx::CT_TextCharacterProperties> {
    if let Some((master, style, level)) = style_address(presentation, path) {
        return presentation
            .inner
            .master_text_style(master, style, level)
            .ok()??
            .default_run_properties
            .clone();
    }
    let paragraph = paragraph_index(path)?;
    let paragraph =
        text_frame_ref_at(presentation, path).and_then(|frame| frame.paragraph(paragraph))?;
    match run_index(path) {
        Some(run) => paragraph.run(run)?.properties().cloned(),
        None => paragraph.default_run_properties().cloned(),
    }
}

#[pyclass(name = "TextFrame")]
pub struct PyTextFrame {
    pub(crate) presentation: Py<PyPresentation>,
    pub(crate) path: ContentPath,
}

fn fit_point_size(value: &Bound<'_, PyAny>) -> PyResult<u16> {
    if value.is_instance_of::<PyBool>() || !value.is_instance_of::<pyo3::types::PyInt>() {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "max_size must be a positive integer",
        ));
    }
    let size = value
        .extract::<u16>()
        .map_err(|_| PyValueError::new_err("max_size must be from 1 through 4000"))?;
    if size == 0 || size > 4000 {
        return Err(PyValueError::new_err(
            "max_size must be from 1 through 4000",
        ));
    }
    Ok(size)
}

impl PyTextFrame {
    pub(crate) fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "text frame",
            ".text_frame",
        )
    }

    fn read<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(rpptx::TextFrameRef<'_>) -> T,
    ) -> PyResult<T> {
        self.validate(py)?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .map(read)
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))
    }

    fn update<T>(
        &self,
        py: Python<'_>,
        update: impl FnOnce(&mut rpptx::TextFrame<'_>) -> T,
    ) -> PyResult<T> {
        self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        text_frame_mut_at(&mut presentation, &self.path)
            .map(|mut frame| update(&mut frame))
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))
    }

    fn margin(
        &self,
        py: Python<'_>,
        side: fn(&mut Insets) -> &mut Option<Emu>,
    ) -> PyResult<Option<Py<PyAny>>> {
        let mut insets = self.read(py, |frame| frame.insets())?;
        side(&mut insets)
            .map(|inset| length_object(py, inset.0))
            .transpose()
    }

    fn set_margin(
        &self,
        py: Python<'_>,
        side: fn(&mut Insets) -> &mut Option<Emu>,
        value: Option<i64>,
    ) -> PyResult<()> {
        if value.is_some_and(|value| i32::try_from(value).is_err()) {
            return Err(PyValueError::new_err(
                "text frame margin must fit a 32-bit EMU coordinate",
            ));
        }
        let mut insets = self.read(py, |frame| frame.insets())?;
        *side(&mut insets) = value.map(Emu);
        let (left, right, top, bottom) = insets;
        self.update(py, |frame| frame.set_insets(left, right, top, bottom))?
            .map_err(|error| rpptx_to_pyerr(py, error))
    }
}

/// Left, right, top, and bottom text insets, as the facade reports them.
type Insets = (Option<Emu>, Option<Emu>, Option<Emu>, Option<Emu>);

#[pymethods]
impl PyTextFrame {
    #[getter(_txBody)]
    fn tx_body(&self, py: Python<'_>) -> PyResult<()> {
        self.validate(py)?;
        Err(PyNotImplementedError::new_err(
            "TextFrame._txBody does not expose lxml; use text_frame.element.xml to inspect XML or text_frame.element.replace_xml(...) for checked replacement",
        ))
    }

    #[getter]
    fn element(&self, py: Python<'_>) -> PyResult<Py<crate::presentation::PyPptElement>> {
        self.validate(py)?;
        let presentation = self.presentation.borrow(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyValueError::new_err("Element owner is absent"))?;
        let path = presentation.capture_text_body_path(current.segs, None)?;
        drop(presentation);
        crate::presentation::PyPptElement::new_checked(
            py,
            self.presentation.clone_ref(py),
            path,
            crate::presentation::PptElementKind::TextBody,
        )
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> PyResult<String> {
        self.validate(py)?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .map(|frame| frame.text())
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))
    }

    #[setter]
    fn set_text(&self, py: Python<'_>, value: &str) -> PyResult<()> {
        self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        native_text_frame_mut_at(&mut candidate, &current)
            .map(|mut frame| frame.set_text(value))
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))?;
        let survivors = presentation.text_changed_owners(&self.path)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "TextFrame.text",
            &survivors,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)?;
        Ok(())
    }

    /// Formats one checked owner without changing text topology.
    #[pyo3(signature=(font_family=None,max_size=18,*,fonts=None,font_dir=None,font_dirs=None))]
    fn fit_text<'py>(
        &self,
        py: Python<'py>,
        font_family: Option<String>,
        #[pyo3(from_py_with=fit_point_size)] max_size: u16,
        fonts: Option<Vec<(String, Bound<'py, pyo3::types::PyBytes>)>>,
        font_dir: Option<std::path::PathBuf>,
        font_dirs: Option<Bound<'py, PyAny>>,
    ) -> PyResult<crate::layout::PyAutofitUpdate> {
        self.validate(py)?;
        let size = max_size;
        let files = crate::presentation::collect_caller_fonts(py, fonts, font_dir, font_dirs)?
            .unwrap_or_default();
        let refs = files
            .iter()
            .map(|f| (f.family.as_str(), f.data.as_slice()))
            .collect::<Vec<_>>();
        let mut presentation = self.presentation.borrow_mut(py);
        let slide = slide_index(&self.path)?;
        let children = self
            .path
            .segs
            .iter()
            .filter_map(|s| {
                if let PathSeg::Shape(i) = s {
                    Some(*i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let row = self.path.segs.iter().find_map(|s| {
            if let PathSeg::Row(i) = s {
                Some(*i)
            } else {
                None
            }
        });
        let column = self.path.segs.iter().find_map(|s| {
            if let PathSeg::Cell(i) = s {
                Some(*i)
            } else {
                None
            }
        });
        let location = presentation
            .inner
            .text_frame_location(slide, &children, row.zip(column))
            .map_err(|e| rpptx_to_pyerr(py, e))?;
        let inner = &mut presentation.inner;
        let result = py
            .detach(|| inner.fit_text_at(&location, font_family.as_deref(), size, &refs))
            .map_err(|e| rpptx_to_pyerr(py, e))?;
        Ok(crate::layout::PyAutofitUpdate::from(&result))
    }

    #[getter]
    fn paragraphs(&self, py: Python<'_>) -> PyResult<Py<PyParagraphCollection>> {
        self.validate(py)?;
        Py::new(
            py,
            PyParagraphCollection::new(self.presentation.clone_ref(py), {
                let presentation = self.presentation.borrow(py);
                let current = presentation
                    .current_path(&self.path)
                    .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
                if current
                    .segs
                    .iter()
                    .any(|seg| matches!(seg, PathSeg::Shape(_)))
                    && text_frame_ref_at(&presentation, &current).is_some()
                {
                    presentation.capture_text_body_path(
                        current.segs,
                        Some(oxml_py_support::RevisionScope::Text),
                    )?
                } else {
                    presentation
                        .capture_path(current.segs, Some(oxml_py_support::RevisionScope::Text))?
                }
            }),
        )
    }

    /// Replaces literal text in this text frame only and returns the count.
    ///
    /// The contract is `Presentation.try_replace_text` restricted to this
    /// frame: with `expect`, the replacement runs on a copy of the text body,
    /// a count that differs raises and leaves the presentation and its
    /// revision as they were, and the revision advances once only when
    /// something was replaced.
    #[pyo3(signature = (placeholder, replacement, *, expect = None))]
    fn try_replace_text(
        &self,
        py: Python<'_>,
        placeholder: &str,
        replacement: &str,
        expect: Option<usize>,
    ) -> PyResult<usize> {
        self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        if names_cell(&self.path) && text_frame_ref_at(&presentation, &self.path).is_none() {
            cell_ref_at(&presentation, &self.path)
                .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?;
            if placeholder.is_empty() {
                return Err(rpptx_to_pyerr(
                    py,
                    rpptx::Error::InvalidSlideMutation {
                        operation: "replace text",
                        message: "placeholder must not be empty".to_owned(),
                    },
                ));
            }
            if let Some(expected) = expect.filter(|&expected| expected != 0) {
                return Err(replacement_count_to_pyerr(py, placeholder, expected, 0));
            }
            return Ok(0);
        }
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        let count = native_text_frame_mut_at(&mut candidate, &current)
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))?
            .try_replace_text(placeholder, replacement, expect)
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        if let Some(expected) = expect.filter(|&expected| expected != count) {
            return Err(replacement_count_to_pyerr(py, placeholder, expected, count));
        }
        if count > 0 {
            let survivors = presentation.text_changed_owners(&self.path)?;
            let prepared = presentation.prepare_owners(
                &candidate,
                "TextFrame.try_replace_text",
                &survivors,
                false,
                false,
                false,
            )?;
            presentation.publish_owners(candidate, prepared)?;
        }
        Ok(count)
    }

    fn add_paragraph(&self, py: Python<'_>) -> PyResult<Py<PyParagraph>> {
        self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        let index = {
            let mut frame = native_text_frame_mut_at(&mut candidate, &current)
                .ok_or_else(|| PyValueError::new_err("shape has no text frame"))?;
            let index = frame.paragraph_count();
            frame.add_paragraph();
            index
        };
        let survivors = presentation.appended_text_owners(&self.path)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "TextFrame.add_paragraph",
            &survivors,
            false,
            false,
            false,
        )?;
        let mut segments = current.segs;
        segments.push(PathSeg::Para(index));
        let path = presentation.capture_prepared_path(&prepared, segments, None)?;
        let result = Py::new(py, PyParagraph::new(self.presentation.clone_ref(py), path))?;
        presentation.publish_owners(candidate, prepared)?;
        Ok(result)
    }

    #[getter]
    fn autofit(&self, py: Python<'_>) -> PyResult<Option<&'static str>> {
        self.validate(py)?;
        Ok(text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|frame| frame.autofit_mode())
            .map(|mode| match mode {
                rpptx::AutofitMode::None => "none",
                rpptx::AutofitMode::Normal => "normal",
                rpptx::AutofitMode::Shape => "shape",
            }))
    }

    #[getter]
    fn auto_size(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let mode = self.read(py, |frame| frame.autofit_mode())?;
        mode.map(|mode| {
            let value = match mode {
                AutofitMode::None => 0,
                AutofitMode::Shape => 1,
                AutofitMode::Normal => 2,
            };
            text_enum(py, "MSO_AUTO_SIZE", value)
        })
        .transpose()
    }

    #[setter]
    fn set_auto_size(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let mode = value
            .map(|value| match value {
                0 => Ok(AutofitMode::None),
                1 => Ok(AutofitMode::Shape),
                2 => Ok(AutofitMode::Normal),
                _ => Err(PyValueError::new_err(
                    "auto size must be an MSO_AUTO_SIZE member",
                )),
            })
            .transpose()?;
        self.update(py, |frame| frame.set_autofit_mode(mode))
    }

    #[getter]
    fn margin_left(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |insets| &mut insets.0)
    }

    #[setter]
    fn set_margin_left(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |insets| &mut insets.0, value)
    }

    #[getter]
    fn margin_right(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |insets| &mut insets.1)
    }

    #[setter]
    fn set_margin_right(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |insets| &mut insets.1, value)
    }

    #[getter]
    fn margin_top(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |insets| &mut insets.2)
    }

    #[setter]
    fn set_margin_top(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |insets| &mut insets.2, value)
    }

    #[getter]
    fn margin_bottom(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |insets| &mut insets.3)
    }

    #[setter]
    fn set_margin_bottom(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |insets| &mut insets.3, value)
    }

    #[getter]
    fn vertical_anchor(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.read(py, |frame| frame.vertical_anchor())?
            .map(|anchor| anchor_object(py, anchor))
            .transpose()
    }

    #[setter]
    fn set_vertical_anchor(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let anchor = value.map(anchor_from_value).transpose()?;
        self.update(py, |frame| frame.set_vertical_anchor(anchor))
    }

    #[getter]
    fn word_wrap(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |frame| frame.word_wrap())
    }

    #[setter]
    fn set_word_wrap(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |frame| frame.set_word_wrap(value))
    }
}

#[pyclass(name = "Paragraph")]
pub struct PyParagraph {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyParagraph {
    fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn validate(&self, py: Python<'_>) -> PyResult<usize> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "paragraph",
            "",
        )?;
        paragraph_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))
    }

    fn read<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(Option<&CT_TextParagraphProperties>) -> T,
    ) -> PyResult<T> {
        let index = self.validate(py)?;
        let presentation = self.presentation.borrow(py);
        if let Some((master, style, level)) = style_address(&presentation, &self.path) {
            let properties = presentation
                .inner
                .master_text_style(master, style, level)
                .map_err(|error| crate::rpptx_to_pyerr(py, error))?;
            return Ok(read(properties));
        }
        text_frame_ref_at(&presentation, &self.path)
            .and_then(|frame| frame.paragraph(index))
            .map(|paragraph| read(paragraph.properties()))
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))
    }

    /// Applies `update` to the direct paragraph properties, writing them only
    /// when they change, so clearing an absent value inserts no `a:pPr`.
    fn update(
        &self,
        py: Python<'_>,
        update: impl FnOnce(&mut CT_TextParagraphProperties),
    ) -> PyResult<()> {
        let index = self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        if style_address(&presentation, &self.path).is_some() {
            return update_style_level(py, &mut presentation, &self.path, |properties| {
                update(properties);
                Ok(())
            });
        }
        let mut paragraph = text_frame_mut_at(&mut presentation, &self.path)
            .and_then(|frame| frame.into_paragraph_mut(index))
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
        let current = paragraph.properties().cloned().unwrap_or_default();
        let mut properties = current.clone();
        update(&mut properties);
        if properties != current {
            paragraph.set_properties(properties);
        }
        Ok(())
    }

    fn spacing(
        &self,
        py: Python<'_>,
        spacing: fn(&CT_TextParagraphProperties) -> Option<&TextSpacing>,
    ) -> PyResult<Option<Py<PyAny>>> {
        let spacing = self.read(py, |properties| properties.and_then(spacing).cloned())?;
        spacing_object(py, spacing)
    }

    fn margin(
        &self,
        py: Python<'_>,
        margin: fn(&CT_TextParagraphProperties) -> Option<i32>,
    ) -> PyResult<Option<Py<PyAny>>> {
        self.read(py, |properties| properties.and_then(margin))?
            .map(|emu| length_object(py, i64::from(emu)))
            .transpose()
    }
}

/// Reads a space-before or space-after value: a `Length` is exact points,
/// and a float is a number of lines.
fn paragraph_spacing(value: Option<&Bound<'_, PyAny>>) -> PyResult<Option<TextSpacing>> {
    match value {
        None => Ok(None),
        Some(value) if value.is_none() => Ok(None),
        Some(value) if value.is_instance_of::<PyBool>() => Err(PyTypeError::new_err(
            "spacing rejects bool as EMU; use rpptx.util.Pt(6) or an explicit float line multiple",
        )),
        Some(value) if value.is_instance_of::<PyFloat>() => {
            lines_spacing(value.extract()?).map(Some)
        }
        Some(value) => points_spacing(value.extract()?).map(Some),
    }
}

fn alignment_value(alignment: TextAlignment) -> i32 {
    match alignment {
        TextAlignment::Left => 1,
        TextAlignment::Center => 2,
        TextAlignment::Right => 3,
        TextAlignment::Justified => 4,
        TextAlignment::Distributed => 5,
        TextAlignment::ThaiDistributed => 6,
        TextAlignment::JustifiedLow => 7,
    }
}

fn alignment_from_value(value: i32) -> PyResult<TextAlignment> {
    match value {
        1 => Ok(TextAlignment::Left),
        2 => Ok(TextAlignment::Center),
        3 => Ok(TextAlignment::Right),
        4 => Ok(TextAlignment::Justified),
        5 => Ok(TextAlignment::Distributed),
        6 => Ok(TextAlignment::ThaiDistributed),
        7 => Ok(TextAlignment::JustifiedLow),
        _ => Err(PyValueError::new_err(
            "paragraph alignment must be a PP_ALIGN member",
        )),
    }
}

#[pymethods]
impl PyParagraph {
    #[getter]
    fn element(&self, py: Python<'_>) -> PyResult<Py<crate::presentation::PyPptElement>> {
        self.validate(py)?;
        let presentation = self.presentation.borrow(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyValueError::new_err("Element owner is absent"))?;
        let path = presentation.capture_path(current.segs, None)?;
        drop(presentation);
        crate::presentation::PyPptElement::new_checked(
            py,
            self.presentation.clone_ref(py),
            path,
            crate::presentation::PptElementKind::Paragraph,
        )
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> PyResult<String> {
        let index = self.validate(py)?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|frame| frame.paragraph(index))
            .map(|paragraph| paragraph.text())
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))
    }

    #[setter]
    fn set_text(&self, py: Python<'_>, value: &str) -> PyResult<()> {
        let index = self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        native_text_frame_mut_at(&mut candidate, &current)
            .and_then(|frame| frame.into_paragraph_mut(index))
            .map(|mut paragraph| paragraph.set_text(value))
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
        let survivors = presentation.text_changed_owners(&self.path)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Paragraph.text",
            &survivors,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)?;
        Ok(())
    }

    #[getter]
    fn level(&self, py: Python<'_>) -> PyResult<u8> {
        let index = self.validate(py)?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|frame| frame.paragraph(index))
            .map(|paragraph| paragraph.level())
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))
    }

    #[setter]
    fn set_level(&self, py: Python<'_>, value: u8) -> PyResult<()> {
        let index = self.validate(py)?;
        let changed = text_frame_mut_at(&mut self.presentation.borrow_mut(py), &self.path)
            .and_then(|frame| frame.into_paragraph_mut(index))
            .is_some_and(|mut paragraph| paragraph.set_level(value));
        if changed {
            Ok(())
        } else if value > 8 {
            Err(PyValueError::new_err("paragraph level must be from 0 to 8"))
        } else {
            Err(PyIndexError::new_err("paragraph index out of range"))
        }
    }

    #[getter]
    fn font(&self, py: Python<'_>) -> PyResult<Py<PyFont>> {
        self.validate(py)?;
        let current = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let path = self
            .presentation
            .borrow(py)
            .capture_xml_children_path(current.segs)?;
        Py::new(
            py,
            PyFont {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }

    #[getter]
    fn runs(&self, py: Python<'_>) -> PyResult<Py<PyRunCollection>> {
        self.validate(py)?;
        let current = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph owner is absent"))?;
        let path = self
            .presentation
            .borrow(py)
            .capture_path(current.segs, Some(oxml_py_support::RevisionScope::Text))?;
        Py::new(
            py,
            PyRunCollection::new(self.presentation.clone_ref(py), path),
        )
    }

    #[pyo3(signature = (field_type, text=None))]
    fn add_field(&self, py: Python<'_>, field_type: &str, text: Option<String>) -> PyResult<()> {
        let paragraph = self.validate(py)?;
        let mut p = self.presentation.borrow_mut(py);
        let path = p
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("text owner is absent"))?;
        let slide = slide_index(&path)?;
        let children = path
            .segs
            .iter()
            .filter_map(|seg| {
                if let PathSeg::Shape(index) = seg {
                    Some(*index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let cell = path.segs.iter().find_map(|seg| {
            if let PathSeg::Cell(index) = seg {
                Some(*index)
            } else {
                None
            }
        });
        let row = path.segs.iter().find_map(|seg| {
            if let PathSeg::Row(index) = seg {
                Some(*index)
            } else {
                None
            }
        });
        if row.is_some() != cell.is_some() {
            return Err(PyIndexError::new_err("cell path is incomplete"));
        }
        let cell = row.zip(cell);
        let mut candidate = p.inner.clone();
        candidate
            .add_text_field(
                slide,
                &children,
                cell,
                paragraph,
                field_type,
                text.as_deref(),
            )
            .map_err(|e| rpptx_to_pyerr(py, e))?;
        let proofs = p.appended_text_owners(&self.path)?;
        let prepared = p.prepare_owners(
            &candidate,
            "Paragraph.add_field",
            &proofs,
            false,
            false,
            false,
        )?;
        p.publish_owners(candidate, prepared)
    }

    #[pyo3(signature = (text = ""))]
    fn add_run(&self, py: Python<'_>, text: &str) -> PyResult<Py<PyRun>> {
        let index = self.validate(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let run = text_frame_ref_at(&presentation, &self.path)
            .and_then(|frame| frame.paragraph(index))
            .map(|paragraph| paragraph.run_count())
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
        let mut candidate = presentation.inner.clone();
        {
            let mut paragraph = native_text_frame_mut_at(&mut candidate, &current)
                .and_then(|frame| frame.into_paragraph_mut(index))
                .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
            paragraph.add_run(text);
        }
        let survivors = presentation.appended_text_owners(&self.path)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Paragraph.add_run",
            &survivors,
            false,
            false,
            false,
        )?;
        let mut segments = current.segs;
        segments.push(PathSeg::Run(run));
        let path = presentation.capture_prepared_path(&prepared, segments, None)?;
        let result = Py::new(py, PyRun::new(self.presentation.clone_ref(py), path))?;
        presentation.publish_owners(candidate, prepared)?;
        Ok(result)
    }

    #[getter]
    fn alignment(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.alignment)
        })?
        .map(|alignment| text_enum(py, "PP_PARAGRAPH_ALIGNMENT", alignment_value(alignment)))
        .transpose()
    }

    #[setter]
    fn set_alignment(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let alignment = value.map(alignment_from_value).transpose()?;
        self.update(py, |properties| properties.alignment = alignment)
    }

    #[getter]
    fn line_spacing(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.spacing(py, |properties| properties.line_spacing.as_ref())
    }

    #[setter]
    fn set_line_spacing(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let spacing = match value {
            None => None,
            Some(value) if value.is_none() => None,
            Some(value) if value.is_instance_of::<PyBool>() => {
                return Err(PyTypeError::new_err(
                    "line_spacing rejects bool; use a positive line multiple such as 1.5 or rpptx.util.Pt(18)",
                ));
            }
            Some(value) if value.is_instance(&py.import("rpptx")?.getattr("Length")?)? => {
                Some(points_spacing(value.extract()?)?)
            }
            Some(value) => Some(lines_spacing(value.extract()?)?),
        };
        if spacing.as_ref().is_some_and(|spacing| match spacing {
            TextSpacing::Points(value) => *value <= 0,
            TextSpacing::Percent(value) => value == "0",
        }) {
            return Err(PyValueError::new_err(
                "line_spacing must store a positive value; use a positive line multiple such as 1.5 or rpptx.util.Pt(18)",
            ));
        }
        self.update(py, |properties| properties.line_spacing = spacing)
    }

    #[getter]
    fn space_before(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.spacing(py, |properties| properties.space_before.as_ref())
    }

    #[setter]
    fn set_space_before(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let spacing = paragraph_spacing(value)?;
        self.update(py, |properties| properties.space_before = spacing)
    }

    #[getter]
    fn space_after(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.spacing(py, |properties| properties.space_after.as_ref())
    }

    #[setter]
    fn set_space_after(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let spacing = paragraph_spacing(value)?;
        self.update(py, |properties| properties.space_after = spacing)
    }

    #[getter]
    fn left_indent(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |properties| properties.left_margin)
    }

    #[setter]
    fn set_left_indent(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        let margin = margin_value(value, 0, "left indent")?;
        self.update(py, |properties| properties.left_margin = margin)
    }

    #[getter]
    fn right_indent(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |properties| properties.right_margin)
    }

    #[setter]
    fn set_right_indent(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        let margin = margin_value(value, 0, "right indent")?;
        self.update(py, |properties| properties.right_margin = margin)
    }

    #[getter]
    fn first_line_indent(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |properties| properties.indent)
    }

    #[setter]
    fn set_first_line_indent(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        let indent = margin_value(value, -MAX_TEXT_MARGIN, "first line indent")?;
        self.update(py, |properties| properties.indent = indent)
    }

    #[getter]
    fn bullet(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let bullet = self.read(py, |properties| {
            let properties = properties?;
            let choice = properties
                .bullet
                .as_ref()
                .and_then(|bullet| bullet.choice.as_ref());
            match choice {
                Some(TextBulletChoice::Character(character)) => {
                    Some(Ok(character.character.clone()))
                }
                Some(TextBulletChoice::None(_)) => Some(Err(false)),
                Some(TextBulletChoice::AutoNumber(_)) => Some(Err(true)),
                None => properties.has_picture_bullet().then_some(Err(true)),
            }
        })?;
        Ok(bullet.map(|bullet| match bullet {
            Ok(character) => PyString::new(py, &character).into_any().unbind(),
            Err(shown) => PyBool::new(py, shown).to_owned().into_any().unbind(),
        }))
    }

    #[setter]
    fn set_bullet(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let choice = match value {
            None => None,
            Some(value) if value.is_none() => None,
            Some(value) if value.is_instance_of::<PyBool>() => {
                if value.extract::<bool>()? {
                    return Err(PyValueError::new_err(
                        "assign a bullet character, False for no bullet, or None",
                    ));
                }
                Some(TextBulletChoice::None(TextNoBullet::default()))
            }
            Some(value) => {
                let character = value.extract::<String>()?;
                let character = TextBulletCharacter::new(character)
                    .map_err(|_| PyValueError::new_err("bullet character must not be empty"))?;
                Some(TextBulletChoice::Character(character))
            }
        };
        self.update(py, |properties| {
            let Some(choice) = choice else {
                properties.set_bullet(None);
                return;
            };
            let mut bullet = properties.bullet.clone().unwrap_or_default();
            match (&mut bullet.choice, choice) {
                (
                    Some(TextBulletChoice::Character(current)),
                    TextBulletChoice::Character(character),
                ) => current.character = character.character,
                (current, choice) => *current = Some(choice),
            }
            properties.set_bullet(Some(bullet));
        })
    }
    /// Compatibility spelling for the same checked bullet choice.
    #[getter]
    fn bullet_char(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.bullet(py)
    }

    #[setter]
    fn set_bullet_char(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        self.set_bullet(py, value)
    }

    /// The auto-numbering scheme of a numbered paragraph, such as
    /// `arabicPeriod` (1. 2. 3.), `alphaLcParenR` (a) b) c)), or
    /// `romanUcPeriod` (I. II. III.), or `None` when the paragraph is not
    /// numbered. Assigning a scheme numbers the paragraph, keeping a start
    /// value already set, and `None` removes the numbering.
    #[getter]
    fn auto_number(&self, py: Python<'_>) -> PyResult<Option<&'static str>> {
        self.read_bullet(py, |bullet| match &bullet.choice {
            Some(TextBulletChoice::AutoNumber(number)) => Some(number.scheme.as_str()),
            _ => None,
        })
    }

    #[setter]
    fn set_auto_number(&self, py: Python<'_>, value: Option<&str>) -> PyResult<()> {
        let scheme = value
            .map(|value| {
                TextAutoNumberScheme::parse(value).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "auto number scheme must be an ST_TextAutonumberScheme token such as \
                         \"arabicPeriod\", got {value:?}"
                    ))
                })
            })
            .transpose()?;
        self.edit_bullet(py, |bullet| match (scheme, &mut bullet.choice) {
            (Some(scheme), Some(TextBulletChoice::AutoNumber(number))) => number.scheme = scheme,
            (Some(scheme), choice) => {
                *choice = Some(TextBulletChoice::AutoNumber(TextAutoNumber::new(scheme)));
            }
            (None, choice @ Some(TextBulletChoice::AutoNumber(_))) => *choice = None,
            (None, _) => {}
        })
    }

    /// The number of the first paragraph of an auto-numbered list, from 1
    /// to 32767, or `None` to start at 1.
    #[getter]
    fn auto_number_start(&self, py: Python<'_>) -> PyResult<Option<u16>> {
        self.read_bullet(py, |bullet| match &bullet.choice {
            Some(TextBulletChoice::AutoNumber(number)) => number.start_at,
            _ => None,
        })
    }

    #[setter]
    fn set_auto_number_start(&self, py: Python<'_>, value: Option<u16>) -> PyResult<()> {
        if value.is_some_and(|value| !(1..=32_767).contains(&value)) {
            return Err(PyValueError::new_err(
                "auto number start must be from 1 to 32767",
            ));
        }
        if value.is_some() && self.auto_number(py)?.is_none() {
            return Err(PyValueError::new_err(
                "the paragraph is not numbered, set auto_number first",
            ));
        }
        self.edit_bullet(py, |bullet| {
            if let Some(TextBulletChoice::AutoNumber(number)) = &mut bullet.choice {
                number.start_at = value;
            }
        })
    }

    /// The sRGB colour of the bullet or number as an `RGBColor`, or `None`
    /// when it follows the text or uses another kind of colour.
    #[getter]
    fn bullet_color(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let color = self.read_bullet(py, |bullet| match bullet.color.as_ref()?.color {
            ColorChoice::Srgb { value, .. } => Some(value),
            _ => None,
        })?;
        color
            .map(|color| {
                let [red, green, blue] = color.components();
                py.import("rpptx.dml.color")?
                    .getattr("RGBColor")?
                    .call1((red, green, blue))
                    .map(Bound::unbind)
            })
            .transpose()
    }

    #[setter]
    fn set_bullet_color(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let color = value
            .filter(|value| !value.is_none())
            .map(|value| {
                oxml_py_support::rgb_channels(value, "bullet_color")
                    .map(|(r, g, b)| RgbColor::new(r, g, b))
            })
            .transpose()?;
        self.edit_bullet(py, |bullet| match color {
            Some(rgb) => {
                if let Some(current) = &mut bullet.color {
                    current.color = crate::dml::with_rgb(Some(current.color.clone()), rgb);
                } else {
                    bullet.color = Some(TextBulletColor::new(ColorChoice::srgb(rgb)));
                }
            }
            None => bullet.color = None,
        })
    }

    /// The bullet or number size as a fraction of the text size, from 0.25
    /// to 4.0. `None` means no percentage value, including retained point-size
    /// values. Assigning `None` removes only the explicit size owner.
    #[getter]
    fn bullet_size(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        let size = self.read_bullet(py, |bullet| match &bullet.size.as_ref()?.value {
            TextBulletSizeValue::Percent(value) => Some(value.clone()),
            TextBulletSizeValue::Points(_) => None,
        })?;
        size.map(|value| {
            match value.strip_suffix('%') {
                Some(percent) => percent.parse::<f64>().map(|percent| percent / 100.0),
                None => value.parse::<f64>().map(|value| value / 100_000.0),
            }
            .map_err(|_| PyValueError::new_err(format!("bullet size {value:?} is not a number")))
        })
        .transpose()
    }

    #[setter]
    fn set_bullet_size(&self, py: Python<'_>, value: Option<f64>) -> PyResult<()> {
        let size = value
            .map(|value| {
                if !value.is_finite() || !(0.25..=4.0).contains(&value) {
                    return Err(PyValueError::new_err(
                        "bullet size must be from 0.25 to 4.0",
                    ));
                }
                TextBulletSize::percent(((value * 100_000.0).trunc() as i64).to_string()).map_err(
                    |_| {
                        PyValueError::new_err(format!(
                            "bullet size must be from 0.25 to 4.0, got {value}"
                        ))
                    },
                )
            })
            .transpose()?;
        self.edit_bullet(py, |bullet| bullet.size = size)
    }

    /// The typeface of a bullet character, such as `Wingdings`, or `None`
    /// when it follows the text.
    #[getter]
    fn bullet_font(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.read_bullet(py, |bullet| {
            bullet.font.as_ref().map(|font| font.typeface.clone())
        })
    }

    #[setter]
    fn set_bullet_font(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        let font = typeface(value)?;
        self.edit_bullet(py, |bullet| set_typeface(&mut bullet.font, font))
    }
}

impl PyParagraph {
    /// Reads one part of the direct bullet.
    fn read_bullet<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(&TextBullet) -> Option<T>,
    ) -> PyResult<Option<T>> {
        self.read(py, |properties| {
            properties
                .and_then(|properties| properties.bullet.as_ref())
                .and_then(read)
        })
    }

    /// Changes the direct bullet, removing it when nothing is left.
    fn edit_bullet(&self, py: Python<'_>, edit: impl FnOnce(&mut TextBullet)) -> PyResult<()> {
        self.update(py, |properties| {
            let mut bullet = properties.bullet.clone().unwrap_or_default();
            edit(&mut bullet);
            properties.set_bullet((bullet != TextBullet::default()).then_some(bullet));
        })
    }
}

#[pyclass(name = "ParagraphCollection")]
pub struct PyParagraphCollection {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyParagraphCollection {
    fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        let presentation = self.presentation.borrow(py);
        validate_path(
            py,
            &presentation,
            &self.path,
            "paragraph collection",
            ".text_frame.paragraphs",
        )?;
        drop(presentation);
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .map(|frame| frame.paragraph_count())
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))
    }

    fn item(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyParagraph>> {
        let mut segments = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?
            .segs;
        segments.push(PathSeg::Para(index));
        let path = self.presentation.borrow(py).capture_path(segments, None)?;
        Py::new(py, PyParagraph::new(self.presentation.clone_ref(py), path))
    }
}

#[pymethods]
impl PyParagraphCollection {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        self.len(py)
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        sequence_item(py, key, self.len(py)?, "paragraph", |index| {
            self.item(py, index)
        })
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyParagraphIterator>> {
        self.len(py)?;
        Py::new(
            py,
            PyParagraphIterator {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                index: 0,
            },
        )
    }
}

#[pyclass]
struct PyParagraphIterator {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    index: usize,
}

#[pymethods]
impl PyParagraphIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyParagraph>>> {
        let collection =
            PyParagraphCollection::new(self.presentation.clone_ref(py), self.path.clone());
        if self.index >= collection.len(py)? {
            return Ok(None);
        }
        let index = self.index;
        self.index += 1;
        collection.item(py, index).map(Some)
    }
}

#[pyclass(name = "Run")]
pub struct PyRun {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyRun {
    fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }
}

#[pymethods]
impl PyRun {
    #[getter]
    fn element(&self, py: Python<'_>) -> PyResult<Py<crate::presentation::PyPptElement>> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "run", "")?;
        let presentation = self.presentation.borrow(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyValueError::new_err("Element owner is absent"))?;
        let path = presentation.capture_path(current.segs, None)?;
        drop(presentation);
        crate::presentation::PyPptElement::new_checked(
            py,
            self.presentation.clone_ref(py),
            path,
            crate::presentation::PptElementKind::Run,
        )
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> PyResult<String> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "run", "")?;
        let paragraph = paragraph_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&self.path).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|frame| frame.paragraph(paragraph))
            .and_then(|paragraph| paragraph.run(run))
            .map(|run| run.text().to_owned())
            .ok_or_else(|| PyIndexError::new_err("run index out of range"))
    }

    /// Replaces this run's text in place. No run, break, or paragraph is
    /// added or removed for any value, so the revision does not advance.
    #[setter]
    fn set_text(&self, py: Python<'_>, value: &str) -> PyResult<()> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "run", "")?;
        let paragraph = paragraph_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&self.path).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        let mut presentation = self.presentation.borrow_mut(py);
        let frame = text_frame_mut_at(&mut presentation, &self.path)
            .ok_or_else(|| PyValueError::new_err("shape has no text frame"))?;
        let mut paragraph = frame
            .into_paragraph_mut(paragraph)
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
        let mut run = paragraph
            .run_mut(run)
            .ok_or_else(|| PyIndexError::new_err("run index out of range"))?;
        run.set_text(value);
        Ok(())
    }

    #[getter]
    fn font(&self, py: Python<'_>) -> PyResult<Py<PyFont>> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "run", "")?;
        let current = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let path = self
            .presentation
            .borrow(py)
            .capture_xml_children_path(current.segs)?;
        Py::new(
            py,
            PyFont {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }

    #[getter]
    fn hyperlink(&self, py: Python<'_>) -> PyResult<Py<PyHyperlink>> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "run", "")?;
        let current = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?;
        let path = self
            .presentation
            .borrow(py)
            .capture_xml_children_path(current.segs)?;
        Py::new(
            py,
            PyHyperlink {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }
}

/// The click hyperlink of one run, like python-pptx `_Hyperlink`.
#[pyclass(name = "Hyperlink")]
pub struct PyHyperlink {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyHyperlink {
    /// Returns the current physical owner path and direct character properties,
    /// raising when the run no longer exists.
    fn run_properties(
        &self,
        py: Python<'_>,
        presentation: &PyPresentation,
    ) -> PyResult<(ContentPath, Option<CT_TextCharacterProperties>)> {
        validate_path(py, presentation, &self.path, "hyperlink", ".hyperlink")?;
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("run owner is absent"))?;
        let paragraph = paragraph_index(&current)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&current).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        let shape = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .ok_or_else(|| PyIndexError::new_err("shape owner is absent"))?;
        let frame = if names_cell(&current) {
            let row = current
                .segs
                .iter()
                .find_map(|segment| match segment {
                    PathSeg::Row(index) => Some(*index),
                    _ => None,
                })
                .ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
            let column = current
                .segs
                .iter()
                .rev()
                .find_map(|segment| match segment {
                    PathSeg::Cell(index) => Some(*index),
                    _ => None,
                })
                .ok_or_else(|| PyIndexError::new_err("column index is missing"))?;
            shape
                .table()
                .and_then(|table| table.cell(row, column))
                .and_then(|cell| cell.text_frame())
        } else {
            shape.text_frame()
        };
        let properties = frame
            .and_then(|frame| frame.paragraph(paragraph))
            .and_then(|paragraph| paragraph.run(run))
            .map(|run| run.properties().cloned())
            .ok_or_else(|| PyIndexError::new_err("run index out of range"))?;
        Ok((current, properties))
    }
}

#[pymethods]
impl PyHyperlink {
    /// The address the run's click hyperlink opens, or `None` without one.
    #[getter]
    fn address(&self, py: Python<'_>) -> PyResult<Option<String>> {
        let presentation = self.presentation.borrow(py);
        let (current, properties) = self.run_properties(py, &presentation)?;
        let Some(relationship_id) = properties
            .as_ref()
            .and_then(|properties| properties.hyperlink_click.as_ref())
            .and_then(|hyperlink| hyperlink.relationship_id.as_deref())
            .filter(|id| !id.is_empty())
        else {
            return Ok(None);
        };
        Ok(presentation
            .inner
            .hyperlink_address(slide_index(&current)?, relationship_id)
            .map(str::to_owned))
    }

    /// The checked named slide-jump target, or None for another click action.
    #[getter]
    fn target_slide(&self, py: Python<'_>) -> PyResult<Option<Py<PySlide>>> {
        let presentation = self.presentation.borrow(py);
        let (current, _) = self.run_properties(py, &presentation)?;
        if names_cell(&current) {
            return Err(PyValueError::new_err(
                "cell hyperlink assignment is not supported",
            ));
        }
        let paragraph = paragraph_index(&current)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&current).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        let shape_id = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .and_then(|shape| shape.non_visual_id())
            .ok_or_else(|| PyValueError::new_err("shape has no id"))?;
        let Some(target) = presentation
            .inner
            .run_target_slide(slide_index(&current)?, shape_id, paragraph, run)
            .map_err(|error| rpptx_to_pyerr(py, error))?
        else {
            return Ok(None);
        };
        let path = presentation.capture_path(smallvec::smallvec![PathSeg::Slide(target)], None)?;
        Py::new(py, PySlide::new(self.presentation.clone_ref(py), path)).map(Some)
    }

    /// Replaces the click target with an owned slide, or clears it for None.
    #[setter]
    fn set_target_slide(&self, py: Python<'_>, slide: Option<PyRef<'_, PySlide>>) -> PyResult<()> {
        let target = match slide {
            Some(slide) if !slide.presentation.is(&self.presentation) => {
                return Err(PyValueError::new_err("slide is not in this presentation"));
            }
            Some(slide) => Some(slide.validate(py)?),
            None => None,
        };
        let mut presentation = self.presentation.borrow_mut(py);
        let (current, _) = self.run_properties(py, &presentation)?;
        if names_cell(&current) {
            return Err(PyValueError::new_err(
                "cell hyperlink assignment is not supported",
            ));
        }
        let paragraph = paragraph_index(&current)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&current).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        let shape_id = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .and_then(|shape| shape.non_visual_id())
            .ok_or_else(|| PyValueError::new_err("shape has no id"))?;
        presentation
            .inner
            .set_run_target_slide(slide_index(&current)?, shape_id, paragraph, run, target)
            .map_err(|error| rpptx_to_pyerr(py, error))
    }

    /// Points the run's click hyperlink at `value`, or removes it for `None`
    /// or an empty string, as python-pptx does. The relationship the old
    /// hyperlink used goes when nothing else on the slide uses it.
    #[setter]
    fn set_address(&self, py: Python<'_>, value: Option<&str>) -> PyResult<()> {
        let mut presentation = self.presentation.borrow_mut(py);
        let (current, _) = self.run_properties(py, &presentation)?;
        let paragraph = paragraph_index(&current)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let run =
            run_index(&current).ok_or_else(|| PyIndexError::new_err("run index is missing"))?;
        if names_cell(&current) {
            return Err(PyValueError::new_err(
                "cell hyperlink assignment is not supported",
            ));
        }
        let shape_id = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .and_then(|shape| shape.non_visual_id())
            .ok_or_else(|| PyValueError::new_err("shape has no id"))?;
        presentation
            .inner
            .set_run_hyperlink(
                slide_index(&current)?,
                shape_id,
                paragraph,
                run,
                value.filter(|value| !value.is_empty()),
            )
            .map_err(|error| rpptx_to_pyerr(py, error))
    }
}

#[pyclass(name = "RunCollection")]
pub struct PyRunCollection {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyRunCollection {
    fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "run collection",
            ".runs",
        )?;
        let paragraph = paragraph_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        text_frame_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|frame| frame.paragraph(paragraph))
            .map(|paragraph| paragraph.run_count())
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))
    }

    fn item(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyRun>> {
        let mut segments = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?
            .segs;
        segments.push(PathSeg::Run(index));
        let path = self.presentation.borrow(py).capture_path(segments, None)?;
        Py::new(py, PyRun::new(self.presentation.clone_ref(py), path))
    }
}

#[pymethods]
impl PyRunCollection {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        self.len(py)
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        sequence_item(py, key, self.len(py)?, "run", |index| self.item(py, index))
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyRunIterator>> {
        self.len(py)?;
        Py::new(
            py,
            PyRunIterator {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                index: 0,
            },
        )
    }
}

#[pyclass]
struct PyRunIterator {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    index: usize,
}

#[pymethods]
impl PyRunIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyRun>>> {
        let collection = PyRunCollection::new(self.presentation.clone_ref(py), self.path.clone());
        if self.index >= collection.len(py)? {
            return Ok(None);
        }
        let index = self.index;
        self.index += 1;
        collection.item(py, index).map(Some)
    }
}

fn typeface(value: Option<String>) -> PyResult<Option<TextFont>> {
    value
        .map(TextFont::new)
        .transpose()
        .map_err(|_| PyValueError::new_err("font name must not be empty"))
}

/// Replaces or removes one typeface. An existing font element keeps its
/// panose and charset attributes.
fn set_typeface(slot: &mut Option<TextFont>, font: Option<TextFont>) {
    match (slot.as_mut(), font) {
        (_, None) => *slot = None,
        (Some(current), Some(font)) => current.typeface = font.typeface,
        (None, font) => *slot = font,
    }
}

#[pyclass(name = "Font")]
pub struct PyFont {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyFont {
    pub(crate) fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "font",
            ".font",
        )
    }

    fn read<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(Option<&CT_TextCharacterProperties>) -> T,
    ) -> PyResult<T> {
        self.validate(py)?;
        let properties = font_properties(&self.presentation.borrow(py), &self.path);
        Ok(read(properties.as_ref()))
    }

    /// Applies `update` to the direct run or paragraph default properties,
    /// writing them only when they change, so clearing an absent value
    /// inserts no `a:rPr` or `a:defRPr`.
    fn update(
        &self,
        py: Python<'_>,
        update: impl FnOnce(&mut CT_TextCharacterProperties),
    ) -> PyResult<()> {
        self.update_checked(py, |properties| {
            update(properties);
            Ok(())
        })
    }

    fn update_checked(
        &self,
        py: Python<'_>,
        update: impl FnOnce(&mut CT_TextCharacterProperties) -> PyResult<()>,
    ) -> PyResult<()> {
        self.validate(py)?;
        let paragraph = paragraph_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("paragraph index is missing"))?;
        let mut presentation = self.presentation.borrow_mut(py);
        if style_address(&presentation, &self.path).is_some() {
            return update_style_level(py, &mut presentation, &self.path, |level| {
                let before = level.default_run_properties.clone().unwrap_or_default();
                let mut properties = before.clone();
                update(&mut properties)?;
                if properties != before {
                    level.default_run_properties = Some(properties);
                }
                Ok(())
            });
        }
        let mut paragraph = text_frame_mut_at(&mut presentation, &self.path)
            .and_then(|frame| frame.into_paragraph_mut(paragraph))
            .ok_or_else(|| PyIndexError::new_err("paragraph index out of range"))?;
        if let Some(run) = run_index(&self.path) {
            let mut run = paragraph
                .run_mut(run)
                .ok_or_else(|| PyIndexError::new_err("run index out of range"))?;
            let current = run.properties().cloned().unwrap_or_default();
            let mut properties = current.clone();
            update(&mut properties)?;
            if properties != current {
                run.set_properties(properties);
            }
        } else {
            let current = paragraph
                .properties()
                .and_then(|properties| properties.default_run_properties.clone())
                .unwrap_or_default();
            let mut properties = current.clone();
            update(&mut properties)?;
            if properties != current {
                *paragraph.default_run_properties_mut() = properties;
            }
        }
        Ok(())
    }
}

/// Read direct run or paragraph-default colour through the validated font owner.
pub(crate) fn font_color_at(
    py: Python<'_>,
    presentation: &Py<PyPresentation>,
    path: &ContentPath,
) -> PyResult<Option<ColorChoice>> {
    PyFont::new(presentation.clone_ref(py), path.clone()).read(py, |properties| {
        match properties.and_then(|properties| properties.fill.as_ref()) {
            Some(Fill::Solid(fill)) => fill.color.clone(),
            _ => None,
        }
    })
}

/// Replace direct font colour, preserving existing solid-fill source metadata.
pub(crate) fn write_font_color(
    py: Python<'_>,
    presentation: &Py<PyPresentation>,
    path: &ContentPath,
    color: Option<ColorChoice>,
) -> PyResult<()> {
    PyFont::new(presentation.clone_ref(py), path.clone()).update(py, |properties| {
        match (properties.fill.as_mut(), color) {
            (_, None) => properties.fill = None,
            (Some(Fill::Solid(fill)), Some(color)) => fill.color = Some(color),
            (_, Some(color)) => {
                let mut fill = SolidFill::default();
                fill.color = Some(color);
                properties.fill = Some(Fill::Solid(fill));
            }
        }
    })
}

/// Reads highlight through the same validated run or paragraph owner as Font.color.
pub(crate) fn font_highlight_at(
    py: Python<'_>,
    presentation: &Py<PyPresentation>,
    path: &ContentPath,
) -> PyResult<Option<ColorChoice>> {
    PyFont::new(presentation.clone_ref(py), path.clone()).read(py, |properties| {
        properties.and_then(|properties| properties.highlight.clone())
    })
}

/// Refuses unsupported source edits before publishing changed properties.
pub(crate) fn write_font_highlight(
    py: Python<'_>,
    presentation: &Py<PyPresentation>,
    path: &ContentPath,
    color: Option<ColorChoice>,
) -> PyResult<()> {
    PyFont::new(presentation.clone_ref(py), path.clone()).update_checked(py, |properties| {
        properties
            .set_highlight(color)
            .map_err(|error| PyValueError::new_err(format!("Font.highlight: {error}")))
    })
}

fn underline_from_value(value: i32) -> PyResult<TextUnderline> {
    Ok(match value {
        0 => TextUnderline::None,
        1 => TextUnderline::Words,
        2 => TextUnderline::Single,
        3 => TextUnderline::Double,
        4 => TextUnderline::Heavy,
        5 => TextUnderline::Dotted,
        6 => TextUnderline::DottedHeavy,
        7 => TextUnderline::Dash,
        8 => TextUnderline::DashHeavy,
        9 => TextUnderline::DashLong,
        10 => TextUnderline::DashLongHeavy,
        11 => TextUnderline::DotDash,
        12 => TextUnderline::DotDashHeavy,
        13 => TextUnderline::DotDotDash,
        14 => TextUnderline::DotDotDashHeavy,
        15 => TextUnderline::Wavy,
        16 => TextUnderline::WavyHeavy,
        17 => TextUnderline::WavyDouble,
        _ => {
            return Err(PyValueError::new_err(
                "underline must be True, False, None, or an MSO_UNDERLINE member",
            ));
        }
    })
}

fn underline_value(underline: TextUnderline) -> i32 {
    match underline {
        TextUnderline::None => 0,
        TextUnderline::Words => 1,
        TextUnderline::Single => 2,
        TextUnderline::Double => 3,
        TextUnderline::Heavy => 4,
        TextUnderline::Dotted => 5,
        TextUnderline::DottedHeavy => 6,
        TextUnderline::Dash => 7,
        TextUnderline::DashHeavy => 8,
        TextUnderline::DashLong => 9,
        TextUnderline::DashLongHeavy => 10,
        TextUnderline::DotDash => 11,
        TextUnderline::DotDashHeavy => 12,
        TextUnderline::DotDotDash => 13,
        TextUnderline::DotDotDashHeavy => 14,
        TextUnderline::Wavy => 15,
        TextUnderline::WavyHeavy => 16,
        TextUnderline::WavyDouble => 17,
    }
}

#[pymethods]
impl PyFont {
    #[getter]
    fn bold(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.bold)
        })
    }

    #[setter]
    fn set_bold(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |properties| properties.bold = value)
    }

    #[getter]
    fn italic(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.italic)
        })
    }

    #[setter]
    fn set_italic(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |properties| properties.italic = value)
    }

    #[getter]
    fn underline(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let underline = self.read(py, |properties| {
            properties.and_then(|properties| properties.underline)
        })?;
        underline
            .map(|underline| match underline {
                TextUnderline::None => Ok(PyBool::new(py, false).to_owned().into_any().unbind()),
                TextUnderline::Single => Ok(PyBool::new(py, true).to_owned().into_any().unbind()),
                underline => text_enum(py, "MSO_TEXT_UNDERLINE_TYPE", underline_value(underline)),
            })
            .transpose()
    }

    #[setter]
    fn set_underline(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let underline = match value {
            None => None,
            Some(value) if value.is_none() => None,
            Some(value) if value.is_instance_of::<PyBool>() => Some(if value.extract::<bool>()? {
                TextUnderline::Single
            } else {
                TextUnderline::None
            }),
            Some(value) => Some(underline_from_value(value.extract()?)?),
        };
        self.update(py, |properties| properties.underline = underline)
    }

    #[getter]
    fn strike(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |properties| {
            properties
                .and_then(|properties| properties.strike)
                .map(|strike| strike != TextStrike::None)
        })
    }

    #[setter]
    fn set_strike(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |properties| {
            properties.strike = match value {
                None => None,
                Some(false) => Some(TextStrike::None),
                // A double strike already struck through stays double.
                Some(true) if properties.strike == Some(TextStrike::Double) => {
                    Some(TextStrike::Double)
                }
                Some(true) => Some(TextStrike::Single),
            };
        })
    }

    #[getter]
    fn all_caps(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.all_caps)
        })
    }

    #[setter]
    fn set_all_caps(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |properties| properties.set_all_caps(value))
    }

    #[getter]
    fn small_caps(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.small_caps)
        })
    }

    #[setter]
    fn set_small_caps(&self, py: Python<'_>, value: Option<bool>) -> PyResult<()> {
        self.update(py, |properties| properties.set_small_caps(value))
    }

    #[getter]
    fn name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.read(py, |properties| {
            properties
                .and_then(|properties| properties.latin.as_ref())
                .map(|font| font.typeface.clone())
        })
    }

    #[setter]
    fn set_name(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        let font = value
            .map(TextFont::new)
            .transpose()
            .map_err(|_| PyValueError::new_err("font name must not be empty"))?;
        self.update(py, |properties| match (properties.latin.as_mut(), font) {
            (_, None) => properties.latin = None,
            // An existing Latin font keeps its panose and charset attributes.
            (Some(current), Some(font)) => current.typeface = font.typeface,
            (None, font) => properties.latin = font,
        })
    }

    #[getter]
    fn color(&self, py: Python<'_>) -> PyResult<Py<crate::dml::PyColorFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            crate::dml::PyColorFormat::font(self.presentation.clone_ref(py), self.path.clone()),
        )
    }

    #[setter]
    fn set_color(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let color = value
            .filter(|value| !value.is_none())
            .map(|value| {
                let (red, green, blue) = oxml_py_support::rgb_channels(value, "Font.color")?;
                Ok::<_, PyErr>(RgbColor::new(red, green, blue))
            })
            .transpose()?;
        let current = font_color_at(py, &self.presentation, &self.path)?;
        let color = color.map(|rgb| crate::dml::with_rgb(current, rgb));
        write_font_color(py, &self.presentation, &self.path, color)
    }

    #[getter]
    fn highlight(&self, py: Python<'_>) -> PyResult<Py<crate::dml::PyColorFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            crate::dml::PyColorFormat::font_highlight(
                self.presentation.clone_ref(py),
                self.path.clone(),
            ),
        )
    }

    #[setter]
    fn set_highlight(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let color = value
            .filter(|value| !value.is_none())
            .map(|value| {
                let (red, green, blue) = oxml_py_support::rgb_channels(value, "Font.highlight")?;
                Ok::<_, PyErr>(RgbColor::new(red, green, blue))
            })
            .transpose()?;
        let current = font_highlight_at(py, &self.presentation, &self.path)?;
        let color = color.map(|rgb| crate::dml::with_rgb(current, rgb));
        write_font_highlight(py, &self.presentation, &self.path, color)
    }

    #[getter]
    fn highlight_color(&self, py: Python<'_>) -> PyResult<Py<crate::dml::PyColorFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            crate::dml::PyColorFormat::font_highlight(
                self.presentation.clone_ref(py),
                self.path.clone(),
            ),
        )
    }

    #[setter]
    fn set_highlight_color(
        &self,
        py: Python<'_>,
        value: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        let color = value
            .filter(|value| !value.is_none())
            .map(|value| {
                let (red, green, blue) =
                    oxml_py_support::rgb_channels(value, "Font.highlight_color")?;
                Ok::<_, PyErr>(RgbColor::new(red, green, blue))
            })
            .transpose()?;
        let current = font_highlight_at(py, &self.presentation, &self.path)?;
        let color = color.map(|rgb| crate::dml::with_rgb(current, rgb));
        write_font_highlight(py, &self.presentation, &self.path, color)
    }

    /// The vertical offset as a fraction of the font size: positive raises
    /// the text (0.3 is PowerPoint's superscript), negative lowers it (-0.25
    /// is its subscript), and `None` inherits.
    #[getter]
    fn baseline(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        let baseline = self.read(py, |properties| {
            properties.and_then(|properties| properties.baseline.clone())
        })?;
        baseline
            .map(|value| {
                match value.strip_suffix('%') {
                    Some(percent) => percent.parse::<f64>().map(|percent| percent / 100.0),
                    None => value.parse::<f64>().map(|value| value / 100_000.0),
                }
                .map_err(|_| PyValueError::new_err(format!("baseline {value:?} is not a number")))
            })
            .transpose()
    }

    #[setter]
    fn set_baseline(&self, py: Python<'_>, value: Option<f64>) -> PyResult<()> {
        let baseline = value
            .map(|value| {
                if (-1.0..=1.0).contains(&value) {
                    Ok(((value * 100_000.0).trunc() as i32).to_string())
                } else {
                    Err(PyValueError::new_err(format!(
                        "baseline must be from -1.0 to 1.0, got {value}"
                    )))
                }
            })
            .transpose()?;
        self.update(py, |properties| properties.baseline = baseline)
    }

    /// The character spacing as a `Length`, positive to expand and negative
    /// to condense, or `None` to inherit.
    #[getter]
    fn spacing(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let spacing = self.read(py, |properties| {
            properties.and_then(|properties| properties.spacing.clone())
        })?;
        match spacing {
            Some(TextPointValue::Centipoints(centipoints)) => {
                length_object(py, i64::from(centipoints) * EMU_PER_CENTIPOINT).map(Some)
            }
            Some(TextPointValue::UniversalMeasure(value)) => {
                let unit_start = value
                    .find(|character: char| character.is_ascii_alphabetic())
                    .unwrap_or(value.len());
                let (number, unit) = value.split_at(unit_start);
                let multiplier = match unit {
                    "mm" => 36_000.0,
                    "cm" => 360_000.0,
                    "in" => 914_400.0,
                    "pt" => 12_700.0,
                    "pc" | "pi" => 152_400.0,
                    _ => {
                        return Err(PyValueError::new_err(format!(
                            "unsupported character spacing unit {unit:?}"
                        )));
                    }
                };
                let emu = number
                    .parse::<f64>()
                    .map_err(|_| PyValueError::new_err("invalid character spacing measure"))?
                    * multiplier;
                if !emu.is_finite() || emu < i64::MIN as f64 || emu >= i64::MAX as f64 {
                    return Err(PyValueError::new_err(
                        "character spacing measure is outside the Length range",
                    ));
                }
                length_object(py, emu.trunc() as i64).map(Some)
            }
            None => Ok(None),
        }
    }

    #[setter]
    fn set_spacing(&self, py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
        let spacing = value
            .map(|value| -> PyResult<TextPointValue> {
                if value.is_instance_of::<PyBool>() {
                    return Err(PyTypeError::new_err("character spacing rejects bool as EMU; use rpptx.util.Pt(2)"));
                }
                let emu = value.extract::<i64>()?;
                let centipoints = emu / EMU_PER_CENTIPOINT;
                if emu != 0 && centipoints == 0 {
                    return Err(PyValueError::new_err("character spacing bare int is EMU; storage is 0.01 point; use rpptx.util.Pt(2), with signed Length values supported"));
                }
                if (-MAX_FONT_SIZE..=MAX_FONT_SIZE).contains(&centipoints) {
                    Ok(TextPointValue::Centipoints(centipoints as i32))
                } else {
                    Err(PyValueError::new_err(
                        "character spacing must be from -4000 to 4000 points",
                    ))
                }
            })
            .transpose()?;
        self.update(py, |properties| properties.spacing = spacing)
    }

    #[getter]
    fn language_id(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let tag = self.language(py)?;
        py.import("rpptx.enum.lang")?
            .getattr("_from_tag")?
            .call1((tag,))
            .map(Bound::unbind)
    }

    #[setter]
    fn set_language_id(&self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<()> {
        self.validate(py)?;
        let tag = py
            .import("rpptx.enum.lang")?
            .getattr("_to_tag")?
            .call1((value,))?
            .extract::<Option<String>>()?;
        self.update(py, |properties| properties.set_language(tag.as_deref()))
    }

    /// The language tag of the text, such as `en-US` or `fr-FR`, which
    /// proofing and hyphenation use, or `None` to inherit.
    #[getter]
    fn language(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.read(py, |properties| {
            properties
                .and_then(CT_TextCharacterProperties::language)
                .map(str::to_owned)
        })
    }

    #[setter]
    fn set_language(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        if let Some(value) = value.as_deref()
            && (value.is_empty()
                || !value
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'))
        {
            return Err(PyValueError::new_err(format!(
                "language must be a tag such as \"en-US\", got {value:?}"
            )));
        }
        self.update(py, |properties| properties.set_language(value.as_deref()))
    }

    /// The East Asian typeface (`a:ea`), or `None` to inherit.
    #[getter]
    fn east_asian_name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.read(py, |properties| {
            properties
                .and_then(|properties| properties.east_asian.as_ref())
                .map(|font| font.typeface.clone())
        })
    }

    #[setter]
    fn set_east_asian_name(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        let font = typeface(value)?;
        self.update(py, |properties| {
            set_typeface(&mut properties.east_asian, font);
        })
    }

    /// The complex-script typeface (`a:cs`), used for Arabic, Hebrew, Thai
    /// and other complex scripts, or `None` to inherit.
    #[getter]
    fn complex_script_name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.read(py, |properties| {
            properties
                .and_then(|properties| properties.complex_script.as_ref())
                .map(|font| font.typeface.clone())
        })
    }

    #[setter]
    fn set_complex_script_name(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        let font = typeface(value)?;
        self.update(py, |properties| {
            set_typeface(&mut properties.complex_script, font);
        })
    }

    #[getter]
    fn size(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.read(py, |properties| {
            properties.and_then(|properties| properties.font_size)
        })?
        .map(|centipoints| length_object(py, i64::from(centipoints) * EMU_PER_CENTIPOINT))
        .transpose()
    }

    #[setter]
    fn set_size(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        let centipoints = value
            .map(|emu| {
                let centipoints = emu.div_euclid(EMU_PER_CENTIPOINT);
                if (MIN_FONT_SIZE..=MAX_FONT_SIZE).contains(&centipoints) {
                    Ok(centipoints as i32)
                } else {
                    Err(PyValueError::new_err(
                        "font size must be from 1 to 4000 points",
                    ))
                }
            })
            .transpose()?;
        self.update(py, |properties| properties.font_size = centipoints)
    }
}

fn sequence_item<T, F>(
    py: Python<'_>,
    key: &Bound<'_, PyAny>,
    len: usize,
    kind: &str,
    mut item: F,
) -> PyResult<Py<PyAny>>
where
    T: PyClass,
    F: FnMut(usize) -> PyResult<Py<T>>,
{
    if let Ok(index) = key.extract::<isize>() {
        return Ok(item(normalize_index(index, len, kind)?)?.into_any());
    }
    if key.is_instance_of::<PySlice>() {
        let (start, stop, step): (isize, isize, isize) =
            key.call_method1("indices", (len,))?.extract()?;
        let items = PyList::empty(py);
        let mut index = start;
        while if step > 0 { index < stop } else { index > stop } {
            items.append(item(index as usize)?)?;
            index += step;
        }
        return Ok(items.into_any().unbind());
    }
    Err(PyTypeError::new_err(format!(
        "{kind} indices must be integers or slices"
    )))
}

/// Master title, body and other styles, bound to the existing physical owner.
#[pyclass(name = "MasterTextStyles")]
pub struct PyMasterTextStyles {
    pub(crate) presentation: Py<PyPresentation>,
    pub(crate) path: ContentPath,
}
impl PyMasterTextStyles {
    fn list(&self, py: Python<'_>, style: usize) -> PyResult<Py<PyTextStyleList>> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "master text styles",
            ".text_styles",
        )?;
        let mut path = self.path.clone();
        path.segs.push(PathSeg::TextStyle(style));
        Py::new(
            py,
            PyTextStyleList {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }
}
#[pymethods]
impl PyMasterTextStyles {
    #[getter]
    fn title(&self, py: Python<'_>) -> PyResult<Py<PyTextStyleList>> {
        self.list(py, 0)
    }
    #[getter]
    fn body(&self, py: Python<'_>) -> PyResult<Py<PyTextStyleList>> {
        self.list(py, 1)
    }
    #[getter]
    fn other(&self, py: Python<'_>) -> PyResult<Py<PyTextStyleList>> {
        self.list(py, 2)
    }
}
/// Nine master levels with the existing paragraph-format editing interface.
#[pyclass(name = "TextStyleList")]
pub struct PyTextStyleList {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}
impl PyTextStyleList {
    fn item(&self, py: Python<'_>, level: usize) -> PyResult<Py<PyParagraph>> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "master text style",
            ".text_styles",
        )?;
        if level >= 9 {
            return Err(PyIndexError::new_err("text style level is out of range"));
        }
        let mut path = self.path.clone();
        path.segs.push(PathSeg::Para(level));
        Py::new(py, PyParagraph::new(self.presentation.clone_ref(py), path))
    }
}
#[pymethods]
impl PyTextStyleList {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "master text style",
            ".text_styles",
        )?;
        Ok(9)
    }
    fn __getitem__(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyParagraph>> {
        self.item(py, normalize_index(index, 9, "text style level")?)
    }
    /// One-based selection matching the native DrawingML level number.
    fn level(&self, py: Python<'_>, level: usize) -> PyResult<Py<PyParagraph>> {
        if !(1..=9).contains(&level) {
            return Err(PyValueError::new_err("level must be from 1 through 9"));
        }
        self.item(py, level - 1)
    }
    /// The first level's direct font, without materializing absent defaults.
    #[getter]
    fn font(&self, py: Python<'_>) -> PyResult<Py<PyFont>> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "master text style",
            ".text_styles",
        )?;
        let mut path = self.path.clone();
        path.segs.push(PathSeg::Para(0));
        Py::new(py, PyFont::new(self.presentation.clone_ref(py), path))
    }
}
