//! Fill, line, shadow, and colour formats, mirroring python-pptx `pptx.dml`.
//!
//! Shape fills, line fills, slide backgrounds, table cell fills, and cell
//! border fills share one fill model, so the formats read and write through a
//! single target.

use oxml_py_support::ContentPath;
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use rpptx::{GradientFill, GradientGeometry, PathGradientKind};

use crate::presentation::PyPresentation;
use crate::shape::{
    ANGLE_UNITS_PER_DEGREE, ANGLE_UNITS_PER_TURN, MAX_COORDINATE, image_bytes, length,
    shape_mut_at, shape_owner, shape_ref_at,
};
use crate::table::{cell_mut_at, cell_ref_at};
use crate::{normalize_index, rpptx_to_pyerr, validate_path};

const MAX_LINE_WIDTH_EMU: i64 = 20_116_800;

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyFillFormat>()?;
    module.add_class::<PyGradientStops>()?;
    module.add_class::<PyGradientStop>()?;
    module.add_class::<PyLineFormat>()?;
    module.add_class::<PyLineEndFormat>()?;
    module.add_class::<PyColorFormat>()?;
    module.add_class::<PyShadowFormat>()?;
    Ok(())
}

/// The DrawingML fill one format object reads and writes.
///
/// `Line` and `CellBorder` also name the line a `LineFormat` reads and writes.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum FillTarget {
    Shape,
    Line,
    Background,
    TableCell,
    CellBorder(rpptx::CellBorder),
}

impl FillTarget {
    fn suffix(self) -> &'static str {
        match self {
            Self::Shape | Self::TableCell => ".fill",
            Self::Line => ".line.fill",
            Self::Background => ".background.fill",
            Self::CellBorder(rpptx::CellBorder::Left) => ".border_left.fill",
            Self::CellBorder(rpptx::CellBorder::Right) => ".border_right.fill",
            Self::CellBorder(rpptx::CellBorder::Top) => ".border_top.fill",
            Self::CellBorder(rpptx::CellBorder::Bottom) => ".border_bottom.fill",
        }
    }

    fn line_suffix(self) -> &'static str {
        self.suffix()
            .strip_suffix(".fill")
            .expect("every fill suffix ends in .fill")
    }
}

/// Reads the shape line or cell border a line target names.
fn current_line(
    presentation: &PyPresentation,
    path: &ContentPath,
    target: FillTarget,
) -> PyResult<Option<rpptx::CT_LineProperties>> {
    Ok(match target {
        FillTarget::CellBorder(edge) => cell_ref_at(presentation, path)
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?
            .border(edge)
            .cloned(),
        _ => shape_ref_at(presentation, path)
            .ok_or_else(|| PyIndexError::new_err("shape index out of range"))?
            .line()
            .cloned(),
    })
}

/// Replaces the shape line or cell border a line target names.
fn write_line(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    target: FillTarget,
    line: rpptx::CT_LineProperties,
) -> PyResult<()> {
    match target {
        FillTarget::CellBorder(edge) => {
            cell_mut_at(presentation, path)
                .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?
                .set_border(edge, Some(line));
            Ok(())
        }
        _ => shape_mut_at(presentation, path)
            .ok_or_else(|| PyIndexError::new_err("shape index out of range"))?
            .set_line(line)
            .map_err(|error| rpptx_to_pyerr(py, error)),
    }
}

fn current_fill(
    presentation: &PyPresentation,
    path: &ContentPath,
    target: FillTarget,
) -> PyResult<Option<rpptx::Fill>> {
    let missing = || PyIndexError::new_err("shape index out of range");
    Ok(match target {
        FillTarget::Shape => shape_ref_at(presentation, path)
            .ok_or_else(missing)?
            .fill()
            .cloned(),
        FillTarget::Line | FillTarget::CellBorder(_) => {
            current_line(presentation, path, target)?.and_then(|line| line.fill)
        }
        FillTarget::Background => presentation
            .inner
            .background_fill_at(shape_owner(
                &presentation
                    .current_path(path)
                    .ok_or_else(|| PyIndexError::new_err("background owner is absent"))?,
            )?)
            .map_err(|error| PyValueError::new_err(error.to_string()))?
            .cloned(),
        FillTarget::TableCell => cell_ref_at(presentation, path)
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?
            .fill()
            .cloned(),
    })
}

fn fill_owner(
    presentation: &PyPresentation,
    path: &ContentPath,
    target: FillTarget,
) -> PyResult<rpptx::FillOwnerLocation> {
    let current = presentation
        .current_path(path)
        .ok_or_else(|| PyIndexError::new_err("fill owner is absent"))?;
    let owner = shape_owner(&current)?;
    let shape_path = current
        .segs
        .iter()
        .filter_map(|segment| match segment {
            oxml_py_support::PathSeg::Shape(index) => Some(*index),
            _ => None,
        })
        .collect();
    Ok(match target {
        FillTarget::Shape => rpptx::FillOwnerLocation::OwnedShape { owner, shape_path },
        FillTarget::Background => rpptx::FillOwnerLocation::OwnedBackground { owner },
        FillTarget::TableCell => {
            let row = current
                .segs
                .iter()
                .find_map(|segment| match segment {
                    oxml_py_support::PathSeg::Row(index) => Some(*index),
                    _ => None,
                })
                .ok_or_else(|| PyValueError::new_err("cell row is absent"))?;
            let column = current
                .segs
                .iter()
                .find_map(|segment| match segment {
                    oxml_py_support::PathSeg::Cell(index) => Some(*index),
                    _ => None,
                })
                .ok_or_else(|| PyValueError::new_err("cell column is absent"))?;
            rpptx::FillOwnerLocation::OwnedTableCell {
                owner,
                shape_path,
                row,
                column,
            }
        }
        _ => unreachable!("line refused before allocation"),
    })
}

fn write_fill(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    target: FillTarget,
    fill: rpptx::Fill,
) -> PyResult<()> {
    if current_fill(presentation, path, target)?.as_ref() == Some(&fill) {
        return Ok(());
    }
    if matches!(target, FillTarget::Line | FillTarget::CellBorder(_)) {
        let mut line = current_line(presentation, path, target)?.unwrap_or_default();
        line.fill = Some(fill);
        return write_line(py, presentation, path, target, line);
    }
    let owner = fill_owner(presentation, path, target)?;
    let mut candidate = presentation.inner.clone();
    candidate
        .set_fill_at(&owner, fill)
        .map_err(|error| rpptx_to_pyerr(py, error))?;
    let proofs = presentation.unchanged_owners()?;
    let prepared =
        presentation.prepare_owners(&candidate, "FillFormat.fill", &proofs, false, false, false)?;
    presentation.publish_owners(candidate, prepared)
}

/// Changes the direct line of a shape or cell border. Without one, `a:ln` is created only
/// when `create` is set, so a removal never adds an empty line.
fn update_line(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    target: FillTarget,
    create: bool,
    change: impl FnOnce(&mut rpptx::CT_LineProperties),
) -> PyResult<()> {
    let line = current_line(presentation, path, target)?;
    let mut line = match line {
        Some(line) => line,
        None if create => rpptx::CT_LineProperties::default(),
        None => return Ok(()),
    };
    change(&mut line);
    write_line(py, presentation, path, target, line)
}

fn dml_enum(py: Python<'_>, name: &str, value: i32) -> PyResult<Py<PyAny>> {
    py.import("rpptx.enum.dml")?
        .getattr(name)?
        .call1((value,))
        .map(Bound::unbind)
}

/// `MSO_LINE_DASH_STYLE` values, with the python-pptx mapping for the nine
/// members it shares.
fn dash_value(dash: rpptx::ST_PresetLineDashVal) -> i32 {
    use rpptx::ST_PresetLineDashVal as Dash;
    match dash {
        Dash::Solid => 1,
        Dash::SystemDash => 2,
        Dash::SystemDot => 3,
        Dash::Dash => 4,
        Dash::DashDot => 5,
        Dash::LargeDashDotDot => 6,
        Dash::LargeDash => 7,
        Dash::LargeDashDot => 8,
        Dash::SystemDashDot => 12,
        Dash::Dot => 13,
        Dash::SystemDashDotDot => 14,
    }
}

fn dash_from_value(value: i32) -> PyResult<rpptx::ST_PresetLineDashVal> {
    rpptx::ST_PresetLineDashVal::ALL
        .into_iter()
        .find(|dash| dash_value(*dash) == value)
        .ok_or_else(|| {
            PyValueError::new_err(
                "dash style must be an MSO_LINE_DASH_STYLE member other than DASH_STYLE_MIXED",
            )
        })
}

/// `MSO_ARROWHEAD_STYLE` values, as `MsoArrowheadStyle` numbers them.
fn end_type_value(kind: rpptx::LineEndType) -> i32 {
    match kind {
        rpptx::LineEndType::None => 1,
        rpptx::LineEndType::Triangle => 2,
        rpptx::LineEndType::Arrow => 3,
        rpptx::LineEndType::Stealth => 4,
        rpptx::LineEndType::Diamond => 5,
        rpptx::LineEndType::Oval => 6,
    }
}

fn end_type_from_value(value: i32) -> PyResult<rpptx::LineEndType> {
    Ok(match value {
        1 => rpptx::LineEndType::None,
        2 => rpptx::LineEndType::Triangle,
        3 => rpptx::LineEndType::Arrow,
        4 => rpptx::LineEndType::Stealth,
        5 => rpptx::LineEndType::Diamond,
        6 => rpptx::LineEndType::Oval,
        _ => {
            return Err(PyValueError::new_err(
                "line end type must be an MSO_ARROWHEAD_STYLE member",
            ));
        }
    })
}

/// `MSO_ARROWHEAD_WIDTH` and `MSO_ARROWHEAD_LENGTH` share these values.
fn end_size_value(size: rpptx::LineEndSize) -> i32 {
    match size {
        rpptx::LineEndSize::Small => 1,
        rpptx::LineEndSize::Medium => 2,
        rpptx::LineEndSize::Large => 3,
    }
}

fn end_size_from_value(value: i32, name: &str) -> PyResult<rpptx::LineEndSize> {
    Ok(match value {
        1 => rpptx::LineEndSize::Small,
        2 => rpptx::LineEndSize::Medium,
        3 => rpptx::LineEndSize::Large,
        _ => {
            return Err(PyValueError::new_err(format!(
                "line end size must be an {name} member"
            )));
        }
    })
}

fn fill_type_name(fill: Option<&rpptx::Fill>) -> &'static str {
    match fill {
        None => "_NoneFill",
        Some(rpptx::Fill::NoFill(_)) => "_NoFill",
        Some(rpptx::Fill::Solid(_)) => "_SolidFill",
        Some(rpptx::Fill::Gradient(_)) => "_GradFill",
        Some(rpptx::Fill::Pattern(_)) => "_PattFill",
        Some(rpptx::Fill::Blip(_)) => "_BlipFill",
    }
}

fn no_foreground(fill: Option<&rpptx::Fill>) -> PyErr {
    PyTypeError::new_err(format!(
        "fill type {} has no foreground color, call .solid() or .patterned() first",
        fill_type_name(fill)
    ))
}

pub(crate) fn with_rgb(
    color: Option<rpptx::ColorChoice>,
    rgb: rpptx::RgbColor,
) -> rpptx::ColorChoice {
    match color {
        Some(rpptx::ColorChoice::Srgb {
            transforms,
            raw_children,
            ..
        }) => rpptx::ColorChoice::Srgb {
            value: rgb,
            transforms,
            raw_children,
        },
        _ => rpptx::ColorChoice::srgb(rgb),
    }
}

/// The python-pptx default gradient: accent 1 shaded from top to bottom.
const DEFAULT_GRADIENT: &str = r#"<a:gradFill xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="accent1"><a:tint val="100000"/><a:shade val="100000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="accent1"><a:tint val="50000"/><a:shade val="100000"/><a:satMod val="350000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="0"/></a:gradFill>"#;

/// A new pattern fill: five percent black dots on white.
const DEFAULT_PATTERN: &str = r#"<a:pattFill xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" prst="pct5"><a:fgClr><a:srgbClr val="000000"/></a:fgClr><a:bgClr><a:srgbClr val="FFFFFF"/></a:bgClr></a:pattFill>"#;

/// `MSO_PATTERN_TYPE` values and their `ST_PresetPatternVal` tokens.
const PATTERNS: [(i32, &str); 54] = [
    (1, "pct5"),
    (2, "pct10"),
    (3, "pct20"),
    (4, "pct25"),
    (5, "pct30"),
    (6, "pct40"),
    (7, "pct50"),
    (8, "pct60"),
    (9, "pct70"),
    (10, "pct75"),
    (11, "pct80"),
    (12, "pct90"),
    (13, "dkHorz"),
    (14, "dkVert"),
    (15, "dkDnDiag"),
    (16, "dkUpDiag"),
    (17, "smCheck"),
    (18, "trellis"),
    (19, "ltHorz"),
    (20, "ltVert"),
    (21, "ltDnDiag"),
    (22, "ltUpDiag"),
    (23, "smGrid"),
    (24, "dotDmnd"),
    (25, "wdDnDiag"),
    (26, "wdUpDiag"),
    (27, "dashUpDiag"),
    (28, "dashDnDiag"),
    (29, "narVert"),
    (30, "narHorz"),
    (31, "dashVert"),
    (32, "dashHorz"),
    (33, "lgConfetti"),
    (34, "lgGrid"),
    (35, "horzBrick"),
    (36, "lgCheck"),
    (37, "smConfetti"),
    (38, "zigZag"),
    (39, "solidDmnd"),
    (40, "diagBrick"),
    (41, "openDmnd"),
    (42, "plaid"),
    (43, "sphere"),
    (44, "weave"),
    (45, "dotGrid"),
    (46, "divot"),
    (47, "shingle"),
    (48, "wave"),
    (49, "horz"),
    (50, "vert"),
    (51, "cross"),
    (52, "dnDiag"),
    (53, "upDiag"),
    (54, "diagCross"),
];

fn parse_fill(xml: &str) -> rpptx::Fill {
    rpptx::Fill::from_xml(xml.as_bytes()).expect("built-in fill XML is valid")
}

fn not_a_gradient(fill: Option<&rpptx::Fill>) -> PyErr {
    PyTypeError::new_err(format!(
        "fill type {} is not a gradient, call .gradient() first",
        fill_type_name(fill)
    ))
}

fn not_a_pattern(fill: Option<&rpptx::Fill>) -> PyErr {
    PyTypeError::new_err(format!(
        "fill type {} has no pattern, call .patterned() first",
        fill_type_name(fill)
    ))
}

/// A live view of one shape fill, line fill, or slide background fill.
#[pyclass(name = "FillFormat")]
pub struct PyFillFormat {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    target: FillTarget,
}

impl PyFillFormat {
    pub(crate) fn new(
        presentation: Py<PyPresentation>,
        path: ContentPath,
        target: FillTarget,
    ) -> Self {
        Self {
            presentation,
            path,
            target,
        }
    }

    fn fill(&self, py: Python<'_>) -> PyResult<Option<rpptx::Fill>> {
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "fill", self.target.suffix())?;
        current_fill(&presentation, &self.path, self.target)
    }

    fn set(&self, py: Python<'_>, fill: rpptx::Fill) -> PyResult<()> {
        let mut presentation = self.presentation.borrow_mut(py);
        write_fill(py, &mut presentation, &self.path, self.target, fill)?;
        let owner = self.path.owner_generation();
        presentation
            .gradient_stops
            .retain(|state| Some(state.owner) != owner || state.target != self.target);
        Ok(())
    }

    fn gradient_fill(&self, py: Python<'_>) -> PyResult<GradientFill> {
        match self.fill(py)? {
            Some(rpptx::Fill::Gradient(fill)) => Ok(fill),
            other => Err(not_a_gradient(other.as_ref())),
        }
    }

    fn edit_gradient(
        &self,
        py: Python<'_>,
        edit: impl FnOnce(&mut GradientFill) -> PyResult<()>,
    ) -> PyResult<()> {
        let mut fill = self.gradient_fill(py)?;
        edit(&mut fill)?;
        let mut presentation = self.presentation.borrow_mut(py);
        write_fill(
            py,
            &mut presentation,
            &self.path,
            self.target,
            rpptx::Fill::Gradient(fill),
        )
    }

    fn stop_tokens(&self, py: Python<'_>) -> PyResult<Vec<u64>> {
        let len = self.gradient_fill(py)?.stops.len();
        let owner = self
            .path
            .owner_generation()
            .ok_or_else(|| PyValueError::new_err("gradient owner has no checked generation"))?;
        let mut presentation = self.presentation.borrow_mut(py);
        if let Some(state) = presentation
            .gradient_stops
            .iter()
            .find(|state| state.owner == owner && state.target == self.target)
        {
            if state.tokens.len() != len {
                return Err(PyValueError::new_err(
                    "gradient stop ownership does not match source",
                ));
            }
            return Ok(state.tokens.clone());
        }
        let end = presentation
            .next_gradient_stop
            .checked_add(len as u64)
            .ok_or_else(|| {
                pyo3::exceptions::PyOverflowError::new_err("gradient stop generation exhausted")
            })?;
        let tokens: Vec<_> = (presentation.next_gradient_stop..end).collect();
        presentation.gradient_stops.try_reserve(1).map_err(|_| {
            pyo3::exceptions::PyMemoryError::new_err("gradient ownership allocation failed")
        })?;
        presentation.gradient_stops.push(GradientStopTokens {
            owner,
            target: self.target,
            tokens: tokens.clone(),
        });
        presentation.next_gradient_stop = end;
        Ok(tokens)
    }

    fn stop_index(&self, py: Python<'_>, token: u64) -> PyResult<usize> {
        self.gradient_fill(py)?;
        let presentation = self.presentation.borrow(py);
        presentation
            .gradient_stops
            .iter()
            .find(|state| {
                Some(state.owner) == self.path.owner_generation() && state.target == self.target
            })
            .and_then(|state| state.tokens.iter().position(|current| *current == token))
            .ok_or_else(|| {
                PyValueError::new_err("gradient stop is stale after deletion or fill replacement")
            })
    }
}

#[pymethods]
impl PyFillFormat {
    #[getter(r#type)]
    fn fill_type(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let value = match self.fill(py)? {
            None => return Ok(None),
            Some(rpptx::Fill::Solid(_)) => 1,
            Some(rpptx::Fill::Pattern(_)) => 2,
            Some(rpptx::Fill::Gradient(_)) => 3,
            Some(rpptx::Fill::NoFill(_)) => 5,
            Some(rpptx::Fill::Blip(_)) => 6,
        };
        dml_enum(py, "MSO_FILL_TYPE", value).map(Some)
    }

    /// Sets a solid fill, keeping an existing solid fill and its colour.
    fn solid(&self, py: Python<'_>) -> PyResult<()> {
        if matches!(self.fill(py)?, Some(rpptx::Fill::Solid(_))) {
            return Ok(());
        }
        self.set(py, rpptx::Fill::Solid(rpptx::SolidFill::default()))
    }

    /// Sets an explicit no-fill, so what lies behind shows through.
    fn background(&self, py: Python<'_>) -> PyResult<()> {
        self.fill(py)?;
        self.set(py, rpptx::Fill::NoFill(rpptx::NoFill::default()))
    }

    /// Sets python-pptx's default linear gradient, accent 1 shaded from top
    /// to bottom, keeping an existing gradient as it is.
    fn gradient(&self, py: Python<'_>) -> PyResult<()> {
        if matches!(self.fill(py)?, Some(rpptx::Fill::Gradient(_))) {
            return Ok(());
        }
        self.set(py, parse_fill(DEFAULT_GRADIENT))
    }

    /// Sets a pattern fill, five percent black dots on white, keeping an
    /// existing pattern fill as it is. PowerPoint, LibreOffice and rpptx draw
    /// the supported pattern presets. Google Slides degrades them.
    fn patterned(&self, py: Python<'_>) -> PyResult<()> {
        if matches!(self.fill(py)?, Some(rpptx::Fill::Pattern(_))) {
            return Ok(());
        }
        self.set(py, parse_fill(DEFAULT_PATTERN))
    }

    /// Uses a stretched image owned by this shape, cell or slide background.
    fn picture(&self, py: Python<'_>, image_file: &Bound<'_, PyAny>) -> PyResult<()> {
        if matches!(self.target, FillTarget::Line | FillTarget::CellBorder(_)) {
            return Err(PyTypeError::new_err("a line cannot take a picture fill"));
        }
        self.fill(py)?;
        let (bytes, filename) = image_bytes(image_file)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let owner = fill_owner(&presentation, &self.path, self.target)?;
        let mut candidate = presentation.inner.clone();
        candidate
            .set_picture_fill(&owner, &bytes, &filename)
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        let proofs = presentation.unchanged_owners()?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "FillFormat.picture",
            &proofs,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)?;
        presentation.gradient_stops.retain(|state| {
            Some(state.owner) != self.path.owner_generation() || state.target != self.target
        });
        Ok(())
    }

    /// The transparent fraction of a solid fill, from zero to one.
    #[getter]
    fn transparency(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        let fill = self.fill(py)?;
        let Some(rpptx::Fill::Solid(fill)) = fill else {
            return Ok(None);
        };
        let mut opacity = 1.0;
        if let Some(color) = &fill.color {
            for transform in color.transforms() {
                match transform {
                    rpptx::ColorTransform::Alpha(value) => opacity = f64::from(value.0) / 100_000.0,
                    rpptx::ColorTransform::AlphaOffset(value) => {
                        opacity += f64::from(value.0) / 100_000.0
                    }
                    rpptx::ColorTransform::AlphaModulation(value) => {
                        opacity *= f64::from(value.0) / 100_000.0
                    }
                    _ => {}
                }
            }
        }
        Ok(Some(1.0 - opacity.clamp(0.0, 1.0)))
    }
    #[setter]
    fn set_transparency(&self, py: Python<'_>, value: f64) -> PyResult<()> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(PyValueError::new_err(
                "transparency must be finite and from zero to one",
            ));
        }
        let fill = self.fill(py)?;
        let Some(rpptx::Fill::Solid(mut fill)) = fill else {
            return Err(PyTypeError::new_err("transparency requires a solid fill"));
        };
        let color = fill
            .color
            .get_or_insert_with(|| rpptx::ColorChoice::srgb(rpptx::RgbColor::new(0, 0, 0)));
        let transforms = color_transforms_mut(color);
        transforms.retain(|transform| {
            !matches!(
                transform,
                rpptx::ColorTransform::Alpha(_)
                    | rpptx::ColorTransform::AlphaOffset(_)
                    | rpptx::ColorTransform::AlphaModulation(_)
            )
        });
        let opacity = ((1.0 - value) * 100_000.0).trunc() as i32;
        if opacity != 100_000 {
            transforms.push(rpptx::ColorTransform::Alpha(rpptx::Percent1000(opacity)));
        }
        let mut presentation = self.presentation.borrow_mut(py);
        write_fill(
            py,
            &mut presentation,
            &self.path,
            self.target,
            rpptx::Fill::Solid(fill),
        )
    }

    #[getter]
    fn fore_color(&self, py: Python<'_>) -> PyResult<Py<PyColorFormat>> {
        let fill = self.fill(py)?;
        if !matches!(fill, Some(rpptx::Fill::Solid(_) | rpptx::Fill::Pattern(_))) {
            return Err(no_foreground(fill.as_ref()));
        }
        Py::new(
            py,
            PyColorFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                source: ColorSource::Fill {
                    target: self.target,
                    solidify: false,
                },
            },
        )
    }
    /// The background colour of a pattern fill.
    #[getter]
    fn back_color(&self, py: Python<'_>) -> PyResult<Py<PyColorFormat>> {
        let fill = self.fill(py)?;
        if !matches!(fill, Some(rpptx::Fill::Pattern(_))) {
            return Err(not_a_pattern(fill.as_ref()));
        }
        Py::new(
            py,
            PyColorFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                source: ColorSource::PatternBackground {
                    target: self.target,
                },
            },
        )
    }

    /// The preset of a pattern fill as an `MSO_PATTERN_TYPE` member, or
    /// `None` when the fill names none.
    #[getter]
    fn pattern(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let fill = self.fill(py)?;
        let Some(rpptx::Fill::Pattern(pattern)) = &fill else {
            return Err(not_a_pattern(fill.as_ref()));
        };
        pattern
            .preset
            .as_deref()
            .and_then(|token| PATTERNS.iter().find(|(_, name)| *name == token))
            .map(|(value, _)| dml_enum(py, "MSO_PATTERN_TYPE", *value))
            .transpose()
    }

    #[setter]
    fn set_pattern(&self, py: Python<'_>, value: i32) -> PyResult<()> {
        let token = PATTERNS
            .iter()
            .find(|(member, _)| *member == value)
            .map(|(_, token)| *token)
            .ok_or_else(|| {
                PyValueError::new_err("pattern must be an MSO_PATTERN_TYPE member other than MIXED")
            })?;
        let fill = self.fill(py)?;
        let Some(rpptx::Fill::Pattern(mut pattern)) = fill else {
            return Err(not_a_pattern(fill.as_ref()));
        };
        pattern.preset = Some(token.to_owned());
        self.set(py, rpptx::Fill::Pattern(pattern))
    }

    /// The angle of a linear gradient in degrees, counter-clockwise from the
    /// positive x axis as python-pptx reports it, in `0.0 <= angle < 360.0`.
    #[getter]
    fn gradient_angle(&self, py: Python<'_>) -> PyResult<f64> {
        match self.gradient_fill(py)?.geometry {
            Some(GradientGeometry::Linear(linear)) => {
                let clockwise = f64::from(linear.angle.0) / ANGLE_UNITS_PER_DEGREE;
                Ok(if clockwise == 0.0 {
                    0.0
                } else {
                    360.0 - clockwise
                })
            }
            _ => Err(PyValueError::new_err("not a linear gradient")),
        }
    }

    #[setter]
    fn set_gradient_angle(&self, py: Python<'_>, value: f64) -> PyResult<()> {
        if !value.is_finite() {
            return Err(PyValueError::new_err(
                "gradient angle must be a finite number of degrees",
            ));
        }
        let units = ((-value.rem_euclid(360.0) * ANGLE_UNITS_PER_DEGREE).round_ties_even() as i64)
            .rem_euclid(ANGLE_UNITS_PER_TURN);
        let angle = rpptx::Angle(i32::try_from(units).expect("a normalized angle fits in i32"));
        self.edit_gradient(py, |gradient| match gradient.geometry.as_mut() {
            Some(GradientGeometry::Linear(linear)) => {
                linear.angle = angle;
                Ok(())
            }
            _ => Err(PyValueError::new_err("not a linear gradient")),
        })
    }

    /// The shape of a path gradient, `circle`, `rect`, or `shape`, or `None`
    /// for a linear gradient. Assigning a shape makes the gradient radiate
    /// from the centre, and `None` makes it linear, left to right.
    /// PowerPoint and LibreOffice draw a path gradient, Google Slides
    /// degrades it, and rpptx's own renderer draws supported
    /// path gradients.
    #[getter]
    fn gradient_path(&self, py: Python<'_>) -> PyResult<Option<&'static str>> {
        Ok(match self.gradient_fill(py)?.geometry {
            Some(GradientGeometry::Path(path)) => Some(match path.kind {
                PathGradientKind::Circle => "circle",
                PathGradientKind::Rectangle => "rect",
                PathGradientKind::Shape => "shape",
            }),
            _ => None,
        })
    }

    #[setter]
    fn set_gradient_path(&self, py: Python<'_>, value: Option<&str>) -> PyResult<()> {
        let geometry = match value {
            None => r#"<a:lin ang="0" scaled="0"/>"#,
            Some("circle") => {
                r#"<a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>"#
            }
            Some("rect") => {
                r#"<a:path path="rect"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>"#
            }
            Some("shape") => {
                r#"<a:path path="shape"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>"#
            }
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "gradient path must be circle, rect, shape, or None, got {other:?}"
                )));
            }
        };
        let probe = format!(
            r#"<a:gradFill xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:gsLst><a:gs pos="0"/><a:gs pos="100000"/></a:gsLst>{geometry}</a:gradFill>"#
        );
        let rpptx::Fill::Gradient(probe) = parse_fill(&probe) else {
            unreachable!("a gradFill parses as a gradient");
        };
        self.edit_gradient(py, |gradient| {
            gradient.geometry = probe.geometry;
            Ok(())
        })
    }

    /// The stops of a gradient fill, in document order.
    #[getter]
    fn gradient_stops(&self, py: Python<'_>) -> PyResult<Py<PyGradientStops>> {
        self.stop_tokens(py)?;
        Py::new(
            py,
            PyGradientStops {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                target: self.target,
            },
        )
    }
}

/// Runtime stop identities parallel the current native stop vector.
/// They never enter XML, and identical stop values remain distinct owners.
pub(crate) struct GradientStopTokens {
    pub(crate) owner: u64,
    target: FillTarget,
    tokens: Vec<u64>,
}

#[pyclass(name = "GradientStops")]
pub struct PyGradientStops {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    target: FillTarget,
}

impl PyGradientStops {
    fn format(&self, py: Python<'_>) -> PyFillFormat {
        PyFillFormat::new(
            self.presentation.clone_ref(py),
            self.path.clone(),
            self.target,
        )
    }
    fn stop(&self, py: Python<'_>, token: u64) -> PyResult<Py<PyGradientStop>> {
        Py::new(
            py,
            PyGradientStop {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                target: self.target,
                token,
            },
        )
    }
}

#[pymethods]
impl PyGradientStops {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        Ok(self.format(py).stop_tokens(py)?.len())
    }
    fn __getitem__(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyGradientStop>> {
        let tokens = self.format(py).stop_tokens(py)?;
        self.stop(
            py,
            tokens[normalize_index(index, tokens.len(), "gradient stop")?],
        )
    }
    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let stops = self
            .format(py)
            .stop_tokens(py)?
            .into_iter()
            .map(|token| self.stop(py, token))
            .collect::<PyResult<Vec<_>>>()?;
        PyList::new(py, stops)?
            .call_method0("__iter__")
            .map(Bound::unbind)
    }
    fn __delitem__(&self, py: Python<'_>, index: isize) -> PyResult<()> {
        let format = self.format(py);
        let mut tokens = format.stop_tokens(py)?;
        let index = normalize_index(index, tokens.len(), "gradient stop")?;
        if tokens.len() <= 2 {
            return Err(PyValueError::new_err("a gradient keeps at least two stops"));
        }
        format.edit_gradient(py, |fill| {
            fill.stops.remove(index);
            Ok(())
        })?;
        tokens.remove(index);
        let mut presentation = self.presentation.borrow_mut(py);
        let state = presentation
            .gradient_stops
            .iter_mut()
            .find(|state| {
                Some(state.owner) == self.path.owner_generation() && state.target == self.target
            })
            .expect("checked gradient tokens");
        state.tokens = tokens;
        Ok(())
    }
    fn append(&self, py: Python<'_>, position: f64) -> PyResult<Py<PyGradientStop>> {
        let position = stop_position(position)?;
        let format = self.format(py);
        let mut tokens = format.stop_tokens(py)?;
        let token = self.presentation.borrow(py).next_gradient_stop;
        let next = token.checked_add(1).ok_or_else(|| {
            pyo3::exceptions::PyOverflowError::new_err("gradient stop generation exhausted")
        })?;
        tokens.try_reserve(1).map_err(|_| {
            pyo3::exceptions::PyMemoryError::new_err("gradient stop allocation failed")
        })?;
        let result = self.stop(py, token)?;
        let mut inserted = 0;
        format.edit_gradient(py, |fill| {
            if fill.stops.len() < 2 {
                return Err(PyValueError::new_err(
                    "gradient source must contain at least two stops",
                ));
            }
            inserted = fill
                .stops
                .iter()
                .position(|stop| stop.position.0 > position)
                .unwrap_or(fill.stops.len());
            let mut stop = fill.stops[inserted.saturating_sub(1)].clone();
            stop.position = rpptx::Percent1000(position);
            fill.stops.try_reserve(1).map_err(|_| {
                pyo3::exceptions::PyMemoryError::new_err("gradient stop allocation failed")
            })?;
            fill.stops.insert(inserted, stop);
            Ok(())
        })?;
        tokens.insert(inserted, token);
        let mut presentation = self.presentation.borrow_mut(py);
        let state = presentation
            .gradient_stops
            .iter_mut()
            .find(|state| {
                Some(state.owner) == self.path.owner_generation() && state.target == self.target
            })
            .expect("checked gradient tokens");
        state.tokens = tokens;
        presentation.next_gradient_stop = next;
        drop(presentation);
        Ok(result)
    }
}

fn stop_position(position: f64) -> PyResult<i32> {
    if !position.is_finite() || !(0.0..=1.0).contains(&position) {
        return Err(PyValueError::new_err(
            "gradient stop position must be finite and from 0.0 to 1.0",
        ));
    }
    Ok((position * 100_000.0).trunc() as i32)
}

#[pyclass(name = "GradientStop")]
pub struct PyGradientStop {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    target: FillTarget,
    token: u64,
}

impl PyGradientStop {
    fn format(&self, py: Python<'_>) -> PyFillFormat {
        PyFillFormat::new(
            self.presentation.clone_ref(py),
            self.path.clone(),
            self.target,
        )
    }
}
#[pymethods]
impl PyGradientStop {
    #[getter]
    fn color(&self, py: Python<'_>) -> PyResult<Py<PyColorFormat>> {
        self.format(py).stop_index(py, self.token)?;
        Py::new(
            py,
            PyColorFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                source: ColorSource::GradientStop {
                    target: self.target,
                    token: self.token,
                },
            },
        )
    }
    #[getter]
    fn position(&self, py: Python<'_>) -> PyResult<f64> {
        let format = self.format(py);
        let index = format.stop_index(py, self.token)?;
        Ok(f64::from(format.gradient_fill(py)?.stops[index].position.0) / 100_000.0)
    }
    #[setter]
    fn set_position(&self, py: Python<'_>, value: f64) -> PyResult<()> {
        let position = stop_position(value)?;
        let format = self.format(py);
        let index = format.stop_index(py, self.token)?;
        format.edit_gradient(py, |fill| {
            fill.stops[index].position = rpptx::Percent1000(position);
            Ok(())
        })
    }
}

/// The colour one colour format reads and writes.
#[derive(Clone, Copy)]
enum ColorSource {
    /// The foreground of a fill. `solidify` makes a set colour a solid fill.
    Fill {
        target: FillTarget,
        solidify: bool,
    },
    /// The colour of the outer shadow.
    Shadow,
    PatternBackground {
        target: FillTarget,
    },
    GradientStop {
        target: FillTarget,
        token: u64,
    },
    /// Direct run or paragraph-default font colour, including table cells.
    Font,
    /// Direct run or paragraph-default highlight colour.
    FontHighlight,
}

impl ColorSource {
    fn suffix(self) -> &'static str {
        match self {
            Self::Fill { target, .. } => target.suffix(),
            Self::Shadow => ".shadow.color",
            Self::PatternBackground { .. } => ".fill.back_color",
            Self::GradientStop { .. } => ".fill.gradient_stops",
            Self::Font => ".font.color",
            Self::FontHighlight => ".font.highlight",
        }
    }
}

/// A live view of the foreground colour of one fill, or of a shadow colour.
#[pyclass(name = "ColorFormat")]
pub struct PyColorFormat {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    source: ColorSource,
}

impl PyColorFormat {
    pub(crate) fn font_highlight(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self {
            presentation,
            path,
            source: ColorSource::FontHighlight,
        }
    }

    pub(crate) fn font(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self {
            presentation,
            path,
            source: ColorSource::Font,
        }
    }

    fn color(&self, py: Python<'_>) -> PyResult<Option<rpptx::ColorChoice>> {
        if matches!(self.source, ColorSource::Font) {
            return crate::text::font_color_at(py, &self.presentation, &self.path);
        }
        if matches!(self.source, ColorSource::FontHighlight) {
            return crate::text::font_highlight_at(py, &self.presentation, &self.path);
        }
        let stop_index = if let ColorSource::GradientStop { target, token } = self.source {
            Some(
                PyFillFormat::new(self.presentation.clone_ref(py), self.path.clone(), target)
                    .stop_index(py, token)?,
            )
        } else {
            None
        };
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "color", self.source.suffix())?;
        Ok(match self.source {
            ColorSource::Fill { target, .. } => {
                match current_fill(&presentation, &self.path, target)? {
                    Some(rpptx::Fill::Solid(fill)) => fill.color,
                    Some(rpptx::Fill::Pattern(fill)) => fill.foreground,
                    _ => None,
                }
            }
            ColorSource::Shadow => {
                current_shadow(&presentation, &self.path)?.and_then(|shadow| shadow.color)
            }
            ColorSource::PatternBackground { target } => {
                match current_fill(&presentation, &self.path, target)? {
                    Some(rpptx::Fill::Pattern(fill)) => fill.background,
                    _ => return Err(not_a_pattern(None)),
                }
            }
            ColorSource::GradientStop { target, .. } => {
                match current_fill(&presentation, &self.path, target)? {
                    Some(rpptx::Fill::Gradient(fill)) => {
                        fill.stops[stop_index.expect("checked stop")].color.clone()
                    }
                    _ => return Err(not_a_gradient(None)),
                }
            }
            ColorSource::Font | ColorSource::FontHighlight => {
                unreachable!("font owner was resolved above")
            }
        })
    }

    fn authoring_color(&self, py: Python<'_>) -> PyResult<Option<rpptx::ColorChoice>> {
        if !matches!(self.source, ColorSource::Shadow) {
            return self.color(py);
        }
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "color", self.source.suffix())?;
        Ok(current_shadow(&presentation, &self.path)?
            .map_or_else(|| default_outer_shadow().color, |shadow| shadow.color))
    }

    fn write_color(&self, py: Python<'_>, color: Option<rpptx::ColorChoice>) -> PyResult<()> {
        if matches!(self.source, ColorSource::Font) {
            return crate::text::write_font_color(py, &self.presentation, &self.path, color);
        }
        if matches!(self.source, ColorSource::FontHighlight) {
            return crate::text::write_font_highlight(py, &self.presentation, &self.path, color);
        }
        let color = color.ok_or_else(|| {
            PyTypeError::new_err(
                "ColorFormat clearing with None is supported only for font colours",
            )
        })?;
        match self.source {
            ColorSource::GradientStop { target, token } => {
                let format =
                    PyFillFormat::new(self.presentation.clone_ref(py), self.path.clone(), target);
                let index = format.stop_index(py, token)?;
                return format.edit_gradient(py, |fill| {
                    fill.stops[index].color = Some(color);
                    Ok(())
                });
            }
            ColorSource::PatternBackground { target } => {
                let format =
                    PyFillFormat::new(self.presentation.clone_ref(py), self.path.clone(), target);
                let mut fill = match format.fill(py)? {
                    Some(rpptx::Fill::Pattern(fill)) => fill,
                    other => return Err(not_a_pattern(other.as_ref())),
                };
                fill.background = Some(color);
                let mut presentation = self.presentation.borrow_mut(py);
                return write_fill(
                    py,
                    &mut presentation,
                    &self.path,
                    target,
                    rpptx::Fill::Pattern(fill),
                );
            }
            _ => {}
        }
        let mut presentation = self.presentation.borrow_mut(py);
        validate_path(py, &presentation, &self.path, "color", self.source.suffix())?;
        let (target, solidify) = match self.source {
            ColorSource::Fill { target, solidify } => (target, solidify),
            ColorSource::Shadow => {
                return edit_shadow(py, &mut presentation, &self.path, |shadow| {
                    shadow.replace_color(color)
                });
            }
            ColorSource::Font
            | ColorSource::FontHighlight
            | ColorSource::PatternBackground { .. }
            | ColorSource::GradientStop { .. } => {
                unreachable!("colour owner was resolved above")
            }
        };
        let fill = match current_fill(&presentation, &self.path, target)? {
            Some(rpptx::Fill::Solid(mut fill)) => {
                fill.color = Some(color);
                rpptx::Fill::Solid(fill)
            }
            Some(rpptx::Fill::Pattern(mut fill)) => {
                fill.foreground = Some(color);
                rpptx::Fill::Pattern(fill)
            }
            _ if solidify => {
                let mut fill = rpptx::SolidFill::default();
                fill.color = Some(color);
                rpptx::Fill::Solid(fill)
            }
            other => return Err(no_foreground(other.as_ref())),
        };
        write_fill(py, &mut presentation, &self.path, target, fill)
    }

    fn parameter(&self, property: &str) -> String {
        let owner = match self.source {
            ColorSource::Font => "Font.color",
            ColorSource::FontHighlight => "Font.highlight",
            ColorSource::Shadow => "ShadowFormat.color",
            ColorSource::PatternBackground { .. } => "FillFormat.back_color",
            ColorSource::GradientStop { .. } => "GradientStop.color",
            ColorSource::Fill {
                target: FillTarget::Line | FillTarget::CellBorder(_),
                ..
            } => "LineFormat.color",
            ColorSource::Fill { .. } => "FillFormat.fore_color",
        };
        format!("{owner}.{property}")
    }
}

const THEME_COLORS: [&str; 16] = [
    "dk1", "lt1", "dk2", "lt2", "accent1", "accent2", "accent3", "accent4", "accent5", "accent6",
    "hlink", "folHlink", "tx1", "bg1", "tx2", "bg2",
];

#[pymethods]
impl PyColorFormat {
    #[getter]
    fn rgb(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let Some(rpptx::ColorChoice::Srgb { value, .. }) = self.color(py)? else {
            return Ok(None);
        };
        let [red, green, blue] = value.components();
        py.import("rpptx.dml.color")?
            .getattr("RGBColor")?
            .call1((red, green, blue))
            .map(|color| Some(color.unbind()))
    }

    #[setter]
    fn set_rgb(&self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<()> {
        if value.is_none() && matches!(self.source, ColorSource::Font | ColorSource::FontHighlight)
        {
            return self.write_color(py, None);
        }
        let (red, green, blue) = oxml_py_support::rgb_channels(value, &self.parameter("rgb"))?;
        let rgb = rpptx::RgbColor::new(red, green, blue);
        let current = self.authoring_color(py)?;
        let color = if matches!(self.source, ColorSource::Shadow) {
            shadow_rgb(current, rgb)
        } else {
            with_rgb(current, rgb)
        };
        self.write_color(py, Some(color))
    }

    #[getter(r#type)]
    fn color_type(&self, py: Python<'_>) -> PyResult<Option<&'static str>> {
        Ok(self.color(py)?.map(|color| match color {
            rpptx::ColorChoice::Srgb { .. } => "rgb",
            rpptx::ColorChoice::Scheme { .. } => "theme",
            rpptx::ColorChoice::System { .. } => "system",
            rpptx::ColorChoice::Preset { .. } => "preset",
        }))
    }

    #[getter]
    fn theme_color(&self, py: Python<'_>) -> PyResult<Option<String>> {
        Ok(match self.color(py)? {
            Some(rpptx::ColorChoice::Scheme { value, .. }) => Some(value),
            _ => None,
        })
    }

    #[setter]
    fn set_theme_color(&self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<()> {
        if value.is_none() && matches!(self.source, ColorSource::Font | ColorSource::FontHighlight)
        {
            return self.write_color(py, None);
        }
        let parameter = self.parameter("theme_color");
        let token = if let Ok(token) = value.extract::<String>() {
            token
        } else {
            let code = if value.is_instance_of::<pyo3::types::PyBool>() {
                None
            } else {
                value.extract::<i32>().ok()
            };
            let code = code.ok_or_else(|| {
                PyTypeError::new_err(format!(
                    "{parameter} accepts a supported theme string or MSO_THEME_COLOR, got {}",
                    value
                        .get_type()
                        .name()
                        .map(|name| name.to_string())
                        .unwrap_or_else(|_| "unknown type".to_owned())
                ))
            })?;
            THEME_COLORS
                .get(
                    usize::try_from(code)
                        .ok()
                        .and_then(|code| code.checked_sub(1))
                        .unwrap_or(usize::MAX),
                )
                .ok_or_else(|| {
                    PyValueError::new_err(format!("{parameter} is not a supported MSO_THEME_COLOR"))
                })?
                .to_string()
        };
        if token != "phClr" && !THEME_COLORS.contains(&token.as_str()) {
            return Err(PyValueError::new_err(format!(
                "{parameter} is not a supported theme token"
            )));
        }
        let color = match self.authoring_color(py)? {
            Some(rpptx::ColorChoice::Scheme {
                transforms,
                raw_children,
                ..
            }) => rpptx::ColorChoice::Scheme {
                value: token,
                transforms,
                raw_children,
            },
            current => rpptx::ColorChoice::Scheme {
                value: token,
                transforms: if matches!(self.source, ColorSource::Shadow) {
                    current
                        .iter()
                        .flat_map(rpptx::ColorChoice::transforms)
                        .filter(|value| matches!(value, rpptx::ColorTransform::Alpha(_)))
                        .copied()
                        .collect()
                } else {
                    Vec::new()
                },
                raw_children: Default::default(),
            },
        };
        self.write_color(py, Some(color))
    }

    #[getter]
    fn brightness(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        let Some(color) = self.color(py)? else {
            return Ok(None);
        };
        let transforms = color.transforms();
        if let Some(value) = transforms.iter().find_map(|value| match value {
            rpptx::ColorTransform::LuminanceOffset(value) => Some(value.to_fraction()),
            _ => None,
        }) {
            return Ok(Some(value));
        }
        Ok(Some(
            transforms
                .iter()
                .find_map(|value| match value {
                    rpptx::ColorTransform::LuminanceModulation(value) => {
                        Some(value.to_fraction() - 1.0)
                    }
                    _ => None,
                })
                .unwrap_or(0.0),
        ))
    }

    #[setter]
    fn set_brightness(&self, py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let parameter = self.parameter("brightness");
        let value = if value.is_instance_of::<pyo3::types::PyBool>() {
            None
        } else {
            value.extract::<f64>().ok()
        }
        .ok_or_else(|| {
            PyTypeError::new_err(format!(
                "{parameter} accepts a finite number from -1 to 1, got {}",
                value
                    .get_type()
                    .name()
                    .map(|name| name.to_string())
                    .unwrap_or_else(|_| "unknown type".to_owned())
            ))
        })?;
        if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
            return Err(PyValueError::new_err(format!(
                "{parameter} must be finite and from -1 to 1"
            )));
        }
        let mut color = self.color(py)?.ok_or_else(|| {
            PyValueError::new_err(format!("{parameter} requires a modeled explicit colour"))
        })?;
        let transforms = color_transforms_mut(&mut color);
        transforms.retain(|value| {
            !matches!(
                value,
                rpptx::ColorTransform::LuminanceModulation(_)
                    | rpptx::ColorTransform::LuminanceOffset(_)
            )
        });
        if value != 0.0 {
            let modulation = ((1.0 - value.abs()) * 100_000.0).round_ties_even() as i32;
            transforms.push(rpptx::ColorTransform::LuminanceModulation(
                rpptx::Percent1000(modulation),
            ));
            if value > 0.0 {
                transforms.push(rpptx::ColorTransform::LuminanceOffset(rpptx::Percent1000(
                    (value * 100_000.0).round_ties_even() as i32,
                )));
            }
        }
        self.write_color(py, Some(color))
    }
}

/// A live view of the outline of one shape, picture, or connector, or of one
/// table cell border.
#[pyclass(name = "LineFormat")]
pub struct PyLineFormat {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    target: FillTarget,
}

impl PyLineFormat {
    /// Creates a view of a shape line, with `FillTarget::Line`, or of a cell
    /// border, with `FillTarget::CellBorder`.
    pub(crate) fn new(
        presentation: Py<PyPresentation>,
        path: ContentPath,
        target: FillTarget,
    ) -> Self {
        Self {
            presentation,
            path,
            target,
        }
    }

    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "line",
            self.target.line_suffix(),
        )
    }
}

#[pymethods]
impl PyLineFormat {
    /// Returns the line colour. Setting its `rgb` makes the line fill solid.
    #[getter]
    fn color(&self, py: Python<'_>) -> PyResult<Py<PyColorFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            PyColorFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                source: ColorSource::Fill {
                    target: self.target,
                    solidify: true,
                },
            },
        )
    }

    #[getter]
    fn fill(&self, py: Python<'_>) -> PyResult<Py<PyFillFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            PyFillFormat::new(
                self.presentation.clone_ref(py),
                self.path.clone(),
                self.target,
            ),
        )
    }

    #[getter]
    fn width(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.validate(py)?;
        let width = current_line(&self.presentation.borrow(py), &self.path, self.target)?
            .and_then(|line| line.width)
            .unwrap_or(0);
        length(py, Some(rpptx::Emu(i64::from(width))))
    }

    #[setter]
    fn set_width(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.validate(py)?;
        let width = value.unwrap_or(0);
        if !(0..=MAX_LINE_WIDTH_EMU).contains(&width) {
            return Err(PyValueError::new_err(format!(
                "line width must be between 0 and {MAX_LINE_WIDTH_EMU} EMU, got {width}"
            )));
        }
        let mut presentation = self.presentation.borrow_mut(py);
        let mut line = current_line(&presentation, &self.path, self.target)?.unwrap_or_default();
        line.width = Some(width as u32);
        write_line(py, &mut presentation, &self.path, self.target, line)
    }

    /// Returns the preset dash, or `None` without one or with a custom dash.
    #[getter]
    fn dash_style(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.validate(py)?;
        let dash = current_line(&self.presentation.borrow(py), &self.path, self.target)?
            .as_ref()
            .and_then(|line| match &line.dash {
                Some(rpptx::LineDash::Preset(dash)) => Some(dash.value),
                _ => None,
            });
        dash.map(|dash| dml_enum(py, "MSO_LINE_DASH_STYLE", dash_value(dash)))
            .transpose()
    }

    /// Writes `a:prstDash`. `None` removes a preset or custom dash.
    #[setter]
    fn set_dash_style(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        self.validate(py)?;
        let dash = value.map(dash_from_value).transpose()?;
        let mut presentation = self.presentation.borrow_mut(py);
        update_line(
            py,
            &mut presentation,
            &self.path,
            self.target,
            dash.is_some(),
            |line| {
                line.dash = match (dash, line.dash.take()) {
                    (None, _) => None,
                    (Some(value), Some(rpptx::LineDash::Preset(mut preset))) => {
                        preset.value = value;
                        Some(rpptx::LineDash::Preset(preset))
                    }
                    (Some(value), _) => {
                        Some(rpptx::LineDash::Preset(rpptx::PresetDash::new(value)))
                    }
                };
            },
        )
    }

    /// Returns the `a:headEnd`, the end at the first point of the line.
    #[getter]
    fn head_end(&self, py: Python<'_>) -> PyResult<Py<PyLineEndFormat>> {
        self.line_end(py, false)
    }

    /// Returns the `a:tailEnd`, the end at the last point of the line.
    #[getter]
    fn tail_end(&self, py: Python<'_>) -> PyResult<Py<PyLineEndFormat>> {
        self.line_end(py, true)
    }
}

impl PyLineFormat {
    fn line_end(&self, py: Python<'_>, tail: bool) -> PyResult<Py<PyLineEndFormat>> {
        self.validate(py)?;
        Py::new(
            py,
            PyLineEndFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                target: self.target,
                tail,
            },
        )
    }
}

/// A live view of one end of a line, its `a:headEnd` or `a:tailEnd`.
///
/// Each property is `None` when its attribute is absent. Assigning `None`
/// removes the attribute, and an end left with no attribute is removed. An
/// `a:ln` left empty is kept, as python-pptx keeps it.
#[pyclass(name = "LineEndFormat")]
pub struct PyLineEndFormat {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    target: FillTarget,
    tail: bool,
}

impl PyLineEndFormat {
    fn end(&self, py: Python<'_>) -> PyResult<Option<rpptx::LineEnd>> {
        let presentation = self.presentation.borrow(py);
        let suffix = if self.tail {
            ".line.tail_end"
        } else {
            ".line.head_end"
        };
        validate_path(py, &presentation, &self.path, "line end", suffix)?;
        let line = current_line(&presentation, &self.path, self.target)?;
        Ok(line.as_ref().and_then(|line| {
            if self.tail {
                line.tail_end.clone()
            } else {
                line.head_end.clone()
            }
        }))
    }

    fn update(
        &self,
        py: Python<'_>,
        create: bool,
        change: impl FnOnce(&mut rpptx::LineEnd),
    ) -> PyResult<()> {
        self.end(py)?;
        let tail = self.tail;
        let mut presentation = self.presentation.borrow_mut(py);
        update_line(
            py,
            &mut presentation,
            &self.path,
            self.target,
            create,
            |line| {
                let slot = if tail {
                    &mut line.tail_end
                } else {
                    &mut line.head_end
                };
                if slot.is_none() && !create {
                    return;
                }
                let mut end = slot.take().unwrap_or_default();
                change(&mut end);
                *slot = (end != rpptx::LineEnd::default()).then_some(end);
            },
        )
    }
}

#[pymethods]
impl PyLineEndFormat {
    #[getter(r#type)]
    fn end_type(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.end(py)?
            .and_then(|end| end.kind)
            .map(|kind| dml_enum(py, "MSO_ARROWHEAD_STYLE", end_type_value(kind)))
            .transpose()
    }

    /// Writes `type`. `NONE` writes `type="none"`, as PowerPoint scripting does.
    #[setter(r#type)]
    fn set_end_type(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let kind = value.map(end_type_from_value).transpose()?;
        self.update(py, kind.is_some(), |end| end.kind = kind)
    }

    #[getter]
    fn width(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.end(py)?
            .and_then(|end| end.width)
            .map(|size| dml_enum(py, "MSO_ARROWHEAD_WIDTH", end_size_value(size)))
            .transpose()
    }

    #[setter]
    fn set_width(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let size = value
            .map(|value| end_size_from_value(value, "MSO_ARROWHEAD_WIDTH"))
            .transpose()?;
        self.update(py, size.is_some(), |end| end.width = size)
    }

    #[getter]
    fn length(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.end(py)?
            .and_then(|end| end.length)
            .map(|size| dml_enum(py, "MSO_ARROWHEAD_LENGTH", end_size_value(size)))
            .transpose()
    }

    #[setter]
    fn set_length(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let size = value
            .map(|value| end_size_from_value(value, "MSO_ARROWHEAD_LENGTH"))
            .transpose()?;
        self.update(py, size.is_some(), |end| end.length = size)
    }
}

const DEFAULT_SHADOW_BLUR_RADIUS: i64 = 50_800;
const DEFAULT_SHADOW_DISTANCE: i64 = 38_100;
const DEFAULT_SHADOW_DIRECTION: i32 = 2_700_000;
const DEFAULT_SHADOW_ALPHA: i32 = 40_000;
const OPAQUE_ALPHA: f64 = 100_000.0;
const RECT_ALIGNMENTS: &str = "tl, t, tr, l, ctr, r, bl, b, br";

fn current_effects(
    presentation: &PyPresentation,
    path: &ContentPath,
) -> PyResult<Option<rpptx::CT_EffectList>> {
    Ok(shape_ref_at(presentation, path)
        .ok_or_else(|| PyIndexError::new_err("shape index out of range"))?
        .effects()
        .cloned())
}

fn current_shadow(
    presentation: &PyPresentation,
    path: &ContentPath,
) -> PyResult<Option<rpptx::CT_OuterShadowEffect>> {
    Ok(current_effects(presentation, path)?.and_then(|effects| effects.outer_shadow))
}

fn write_effects(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    effects: Option<rpptx::CT_EffectList>,
) -> PyResult<()> {
    shape_mut_at(presentation, path)
        .ok_or_else(|| PyIndexError::new_err("shape index out of range"))?
        .set_effects(effects)
        .map_err(|error| rpptx_to_pyerr(py, error))
}

/// The outer shadow PowerPoint for Mac writes for its Offset Diagonal
/// Bottom Right preset, `msoShadow21`: preset black at 40% opacity, with a
/// 4 pt blur, 3 pt away at 45 degrees.
fn default_outer_shadow() -> rpptx::CT_OuterShadowEffect {
    let mut shadow = rpptx::CT_OuterShadowEffect::default();
    shadow.blur_radius = Some(DEFAULT_SHADOW_BLUR_RADIUS);
    shadow.distance = Some(DEFAULT_SHADOW_DISTANCE);
    shadow.direction = Some(rpptx::Angle(DEFAULT_SHADOW_DIRECTION));
    shadow.alignment = Some(rpptx::RectAlignment::TopLeft);
    shadow.rotate_with_shape = Some(false);
    shadow.color = Some(rpptx::ColorChoice::Preset {
        value: "black".to_owned(),
        transforms: vec![rpptx::ColorTransform::Alpha(rpptx::Percent1000(
            DEFAULT_SHADOW_ALPHA,
        ))],
        raw_children: Default::default(),
    });
    shadow
}

/// Changes the outer shadow, first adding PowerPoint's default one when the
/// shape has none. Sibling effects in the list are kept.
fn edit_shadow(
    py: Python<'_>,
    presentation: &mut PyPresentation,
    path: &ContentPath,
    change: impl FnOnce(&mut rpptx::CT_OuterShadowEffect),
) -> PyResult<()> {
    let mut effects = current_effects(presentation, path)?.unwrap_or_default();
    change(
        effects
            .outer_shadow
            .get_or_insert_with(default_outer_shadow),
    );
    write_effects(py, presentation, path, Some(effects))
}

/// Gives a shadow colour a new sRGB value. The opacity survives a change of
/// colour kind, which a theme or preset colour's other transforms do not.
fn shadow_rgb(color: Option<rpptx::ColorChoice>, rgb: rpptx::RgbColor) -> rpptx::ColorChoice {
    if let Some(color @ rpptx::ColorChoice::Srgb { .. }) = color {
        return with_rgb(Some(color), rgb);
    }
    rpptx::ColorChoice::Srgb {
        value: rgb,
        transforms: color
            .iter()
            .flat_map(rpptx::ColorChoice::transforms)
            .filter(|transform| matches!(transform, rpptx::ColorTransform::Alpha(_)))
            .copied()
            .collect(),
        raw_children: Default::default(),
    }
}

fn color_transforms_mut(color: &mut rpptx::ColorChoice) -> &mut Vec<rpptx::ColorTransform> {
    match color {
        rpptx::ColorChoice::Srgb { transforms, .. }
        | rpptx::ColorChoice::Scheme { transforms, .. }
        | rpptx::ColorChoice::System { transforms, .. }
        | rpptx::ColorChoice::Preset { transforms, .. } => transforms,
    }
}

fn check_shadow_length(name: &str, value: i64) -> PyResult<i64> {
    if (0..=MAX_COORDINATE).contains(&value) {
        Ok(value)
    } else {
        Err(PyValueError::new_err(format!(
            "shadow {name} must be between 0 and {MAX_COORDINATE} EMU, got {value}"
        )))
    }
}

/// A live view of the shadow of one shape, picture, connector, or group.
///
/// `inherit` is python-pptx's: whether the shape has no `a:effectLst` of
/// its own and so takes its theme effect. The other properties read and
/// write the shape's own `a:outerShdw`. They read `None` when the shape has
/// none, and a property the element omits reads as its schema default.
/// Setting one on a shape without an outer shadow first adds PowerPoint's
/// Offset Diagonal Bottom Right shadow, preset black at 40% opacity with a
/// 4 pt blur, 3 pt away at 45 degrees.
#[pyclass(name = "ShadowFormat")]
pub struct PyShadowFormat {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyShadowFormat {
    pub(crate) fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn read<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(Option<rpptx::CT_EffectList>) -> T,
    ) -> PyResult<T> {
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "shadow", ".shadow")?;
        Ok(read(current_effects(&presentation, &self.path)?))
    }

    fn read_shadow<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(&rpptx::CT_OuterShadowEffect) -> T,
    ) -> PyResult<Option<T>> {
        self.read(py, |effects| {
            effects.and_then(|effects| effects.outer_shadow.as_ref().map(read))
        })
    }

    fn write(&self, py: Python<'_>, effects: Option<rpptx::CT_EffectList>) -> PyResult<()> {
        let mut presentation = self.presentation.borrow_mut(py);
        validate_path(py, &presentation, &self.path, "shadow", ".shadow")?;
        write_effects(py, &mut presentation, &self.path, effects)
    }

    fn edit(
        &self,
        py: Python<'_>,
        change: impl FnOnce(&mut rpptx::CT_OuterShadowEffect),
    ) -> PyResult<()> {
        let mut presentation = self.presentation.borrow_mut(py);
        validate_path(py, &presentation, &self.path, "shadow", ".shadow")?;
        edit_shadow(py, &mut presentation, &self.path, change)
    }
}

#[pymethods]
impl PyShadowFormat {
    /// Whether the shape takes its effects from the theme.
    ///
    /// Setting `True` removes the shape's `a:effectLst`, and with it every
    /// effect of its own. Setting `False` adds an empty one when absent, so
    /// no effect shows, as python-pptx does.
    #[getter]
    fn inherit(&self, py: Python<'_>) -> PyResult<bool> {
        self.read(py, |effects| effects.is_none())
    }

    #[setter]
    fn set_inherit(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        let effects = self.read(py, |effects| effects)?;
        match (value, effects) {
            (true, None) | (false, Some(_)) => Ok(()),
            (true, Some(_)) => self.write(py, None),
            (false, None) => self.write(py, Some(rpptx::CT_EffectList::default())),
        }
    }

    /// Whether the shape has an outer shadow of its own.
    ///
    /// A theme shadow reached through `inherit` is not reported. Setting
    /// `False` removes the outer shadow and keeps the effect list, so the
    /// theme shadow does not return either. On a shape without an outer
    /// shadow of its own it changes nothing.
    #[getter]
    fn visible(&self, py: Python<'_>) -> PyResult<bool> {
        Ok(self.read_shadow(py, |_| ())?.is_some())
    }

    #[setter]
    fn set_visible(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if value {
            return self.edit(py, |_| {});
        }
        let Some(mut effects) = self.read(py, |effects| effects)? else {
            return Ok(());
        };
        if effects.outer_shadow.take().is_none() {
            return Ok(());
        }
        self.write(py, Some(effects))
    }

    #[getter]
    fn color(&self, py: Python<'_>) -> PyResult<Py<PyColorFormat>> {
        self.read(py, |_| ())?;
        Py::new(
            py,
            PyColorFormat {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                source: ColorSource::Shadow,
            },
        )
    }

    /// The shadow opacity from 0.0, transparent, to 1.0, opaque.
    ///
    /// A colour rpptx does not model, `a:scrgbClr` or `a:hslClr`, reads
    /// `None` and refuses a new opacity until `color.rgb` replaces it.
    #[getter]
    fn alpha(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        let alpha = self.read_shadow(py, |shadow| {
            if shadow.has_unmodelled_color() {
                return None;
            }
            Some(
                shadow
                    .color
                    .iter()
                    .flat_map(rpptx::ColorChoice::transforms)
                    .rev()
                    .find_map(|transform| match transform {
                        rpptx::ColorTransform::Alpha(value) => {
                            Some(f64::from(value.0) / OPAQUE_ALPHA)
                        }
                        _ => None,
                    })
                    .unwrap_or(1.0),
            )
        })?;
        Ok(alpha.flatten())
    }

    #[setter]
    fn set_alpha(&self, py: Python<'_>, value: f64) -> PyResult<()> {
        if !(0.0..=1.0).contains(&value) {
            return Err(PyValueError::new_err(format!(
                "shadow alpha must be between 0.0 and 1.0, got {value}"
            )));
        }
        if self.read_shadow(py, rpptx::CT_OuterShadowEffect::has_unmodelled_color)? == Some(true) {
            return Err(PyValueError::new_err(
                "shadow colour is an a:scrgbClr or a:hslClr, which rpptx does not model, \
                 set color.rgb first",
            ));
        }
        let alpha = (value * OPAQUE_ALPHA).round_ties_even() as i32;
        self.edit(py, |shadow| {
            let color = shadow
                .color
                .get_or_insert_with(|| rpptx::ColorChoice::srgb(rpptx::RgbColor::new(0, 0, 0)));
            let transforms = color_transforms_mut(color);
            transforms.retain(|transform| !matches!(transform, rpptx::ColorTransform::Alpha(_)));
            if alpha < OPAQUE_ALPHA as i32 {
                transforms.push(rpptx::ColorTransform::Alpha(rpptx::Percent1000(alpha)));
            }
        })
    }

    #[getter]
    fn blur_radius(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let blur = self.read_shadow(py, |shadow| shadow.blur_radius.unwrap_or(0))?;
        length(py, blur.map(rpptx::Emu))
    }

    #[setter]
    fn set_blur_radius(&self, py: Python<'_>, value: i64) -> PyResult<()> {
        let value = check_shadow_length("blur_radius", value)?;
        self.edit(py, |shadow| shadow.blur_radius = Some(value))
    }

    #[getter]
    fn distance(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let distance = self.read_shadow(py, |shadow| shadow.distance.unwrap_or(0))?;
        length(py, distance.map(rpptx::Emu))
    }

    #[setter]
    fn set_distance(&self, py: Python<'_>, value: i64) -> PyResult<()> {
        let value = check_shadow_length("distance", value)?;
        self.edit(py, |shadow| shadow.distance = Some(value))
    }

    /// The direction the shadow is cast in, in degrees clockwise from the
    /// positive x axis, normalized to `0.0 <= direction < 360.0`.
    #[getter]
    fn direction(&self, py: Python<'_>) -> PyResult<Option<f64>> {
        self.read_shadow(py, |shadow| {
            f64::from(shadow.direction.unwrap_or_default().0) / ANGLE_UNITS_PER_DEGREE
        })
    }

    #[setter]
    fn set_direction(&self, py: Python<'_>, value: f64) -> PyResult<()> {
        if !value.is_finite() {
            return Err(PyValueError::new_err(
                "shadow direction must be a finite number of degrees",
            ));
        }
        let units = ((value * ANGLE_UNITS_PER_DEGREE).round_ties_even() as i64)
            .rem_euclid(ANGLE_UNITS_PER_TURN);
        let angle = rpptx::Angle(i32::try_from(units).expect("a normalized angle fits in i32"));
        self.edit(py, |shadow| shadow.direction = Some(angle))
    }

    /// The `ST_RectAlignment` token the shadow is anchored at: `tl`, `t`,
    /// `tr`, `l`, `ctr`, `r`, `bl`, `b`, or `br`.
    #[getter]
    fn align(&self, py: Python<'_>) -> PyResult<Option<&'static str>> {
        self.read_shadow(py, |shadow| {
            shadow
                .alignment
                .unwrap_or(rpptx::RectAlignment::Bottom)
                .as_str()
        })
    }

    #[setter]
    fn set_align(&self, py: Python<'_>, value: &str) -> PyResult<()> {
        let alignment = rpptx::RectAlignment::parse(value).ok_or_else(|| {
            PyValueError::new_err(format!(
                "shadow align must be one of {RECT_ALIGNMENTS}, got {value:?}"
            ))
        })?;
        self.edit(py, |shadow| shadow.alignment = Some(alignment))
    }

    #[getter]
    fn rotate_with_shape(&self, py: Python<'_>) -> PyResult<Option<bool>> {
        self.read_shadow(py, |shadow| shadow.rotate_with_shape.unwrap_or(true))
    }

    #[setter]
    fn set_rotate_with_shape(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        self.edit(py, |shadow| shadow.rotate_with_shape = Some(value))
    }
}
