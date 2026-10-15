use oxml_py_support::{ContentPath, PathSeg};
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList, PySlice, PyTuple};

use crate::dml::{FillTarget, PyFillFormat, PyLineFormat};
use crate::normalize_index;
use crate::presentation::PyPresentation;
use crate::shape::{length, shape_mut_at, shape_ref_at};
use crate::text::{PyTextFrame, anchor_from_value, anchor_object};
use crate::{rpptx_to_pyerr, validate_path};

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyTable>()?;
    module.add_class::<PyColumn>()?;
    module.add_class::<PyColumnCollection>()?;
    module.add_class::<PyRow>()?;
    module.add_class::<PyRowCollection>()?;
    module.add_class::<PyCell>()?;
    Ok(())
}

fn row_index(path: &ContentPath) -> Option<usize> {
    path.segs.iter().find_map(|segment| match segment {
        PathSeg::Row(index) => Some(*index),
        _ => None,
    })
}

fn cell_index(path: &ContentPath) -> Option<usize> {
    path.segs.iter().rev().find_map(|segment| match segment {
        PathSeg::Cell(index) => Some(*index),
        _ => None,
    })
}

/// Resolves the index a new row or column gets, as `list.insert` does, where
/// `None` or the length appends, but raises for an index outside `-len..=len`.
fn insertion_index(index: Option<isize>, len: usize, kind: &str) -> PyResult<usize> {
    let Some(index) = index else {
        return Ok(len);
    };
    let resolved = if index < 0 {
        len as isize + index
    } else {
        index
    };
    if resolved < 0 || resolved > len as isize {
        return Err(PyIndexError::new_err(format!("{kind} index out of range")));
    }
    Ok(resolved as usize)
}

/// Applies one row or column edit to the table at `path` and advances the
/// revision, because every row, column, and cell handle names an index.
fn restructure(
    py: Python<'_>,
    presentation: &Py<PyPresentation>,
    path: &ContentPath,
    operation: &'static str,
    edit: impl FnOnce(&mut rpptx::TableMut<'_>) -> rpptx::Result<()>,
) -> PyResult<(
    rpptx::Presentation,
    crate::presentation::PreparedOwners,
    ContentPath,
)> {
    let presentation = presentation.borrow(py);
    let current = presentation
        .current_path(path)
        .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?;
    let mut candidate = presentation.inner.clone();
    {
        let mut table = crate::shape::native_shape_mut_at(&mut candidate, &current)
            .and_then(rpptx::ShapeMut::into_table_mut)
            .ok_or_else(|| PyValueError::new_err("shape has no table"))?;
        edit(&mut table).map_err(|error| rpptx_to_pyerr(py, error))?;
    }
    let survivors = presentation.grid_changed_owners(path)?;
    let prepared =
        presentation.prepare_owners(&candidate, operation, &survivors, false, false, false)?;
    Ok((candidate, prepared, current))
}

/// Returns the index a row or column handle names when it belongs to the
/// table at `table_path`.
fn member_index(
    presentation: &PyPresentation,
    table_path: &ContentPath,
    member_path: &ContentPath,
    index: fn(&PathSeg) -> Option<usize>,
) -> Option<usize> {
    if table_path.owner_generation() != member_path.owner_generation() {
        return None;
    }
    let current_table = presentation.current_path(table_path)?;
    let current_member = presentation.current_path(member_path)?;
    let (last, table) = current_member.segs.split_last()?;
    (table == current_table.segs.as_slice()).then(|| index(last))?
}

/// Returns the path segments that name a cell's table shape.
fn table_segments(path: &ContentPath) -> impl Iterator<Item = &PathSeg> {
    path.segs
        .iter()
        .filter(|segment| !matches!(segment, PathSeg::Row(_) | PathSeg::Cell(_)))
}

pub(crate) fn cell_ref_at<'a>(
    presentation: &'a PyPresentation,
    path: &ContentPath,
) -> Option<rpptx::TableCellRef<'a>> {
    shape_ref_at(presentation, path)?
        .table()?
        .cell(row_index(path)?, cell_index(path)?)
}

pub(crate) fn cell_mut_at<'a>(
    presentation: &'a mut PyPresentation,
    path: &ContentPath,
) -> Option<rpptx::TableCellMut<'a>> {
    let current = presentation.current_path(path)?;
    native_cell_mut_at(&mut presentation.inner, &current)
}

pub(crate) fn native_cell_mut_at<'a>(
    presentation: &'a mut rpptx::Presentation,
    path: &ContentPath,
) -> Option<rpptx::TableCellMut<'a>> {
    let (row, column) = (row_index(path)?, cell_index(path)?);
    crate::shape::native_shape_mut_at(presentation, path)?
        .into_table_mut()?
        .into_cell_mut(row, column)
}

/// Left, right, top, and bottom cell margins, as the facade reports them.
type Margins = (
    Option<rpptx::Emu>,
    Option<rpptx::Emu>,
    Option<rpptx::Emu>,
    Option<rpptx::Emu>,
);

#[pyclass(name = "Table")]
pub struct PyTable {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyTable {
    pub(crate) fn new(presentation: Py<PyPresentation>, path: ContentPath) -> Self {
        Self { presentation, path }
    }

    fn dimensions(&self, py: Python<'_>) -> PyResult<(usize, usize)> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "table",
            ".table",
        )?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .map(|table| (table.row_count(), table.column_count()))
            .ok_or_else(|| PyValueError::new_err("shape has no table"))
    }
}

impl PyTable {
    fn flag(&self, py: Python<'_>, read: fn(&rpptx::TableRef<'_>) -> bool) -> PyResult<bool> {
        self.dimensions(py)?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .map(|table| read(&table))
            .ok_or_else(|| PyValueError::new_err("shape has no table"))
    }

    fn edit_properties(
        &self,
        py: Python<'_>,
        edit: impl FnOnce(&mut rpptx::TableMut<'_>) -> rpptx::Result<()>,
    ) -> PyResult<()> {
        self.dimensions(py)?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        let mut table = crate::shape::native_shape_mut_at(&mut candidate, &current)
            .and_then(rpptx::ShapeMut::into_table_mut)
            .ok_or_else(|| PyValueError::new_err("shape has no table"))?;
        edit(&mut table).map_err(|error| rpptx_to_pyerr(py, error))?;
        drop(table);
        let proofs = presentation.unchanged_owners()?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Table.formatting",
            &proofs,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)
    }
}

#[pymethods]
impl PyTable {
    #[getter]
    fn element(&self, py: Python<'_>) -> PyResult<Py<crate::presentation::PyPptElement>> {
        self.dimensions(py)?;
        let presentation = self.presentation.borrow(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyValueError::new_err("Element owner is absent"))?;
        let path = presentation.capture_xml_children_path(current.segs)?;
        drop(presentation);
        crate::presentation::PyPptElement::new_checked(
            py,
            self.presentation.clone_ref(py),
            path,
            crate::presentation::PptElementKind::Table,
        )
    }

    #[getter]
    fn first_row(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.first_row())
    }
    #[setter]
    fn set_first_row(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.first_row(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_first_row(value);
            Ok(())
        })
    }

    #[getter]
    fn last_row(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.last_row())
    }
    #[setter]
    fn set_last_row(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.last_row(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_last_row(value);
            Ok(())
        })
    }

    #[getter]
    fn first_col(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.first_column())
    }
    #[setter]
    fn set_first_col(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.first_col(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_first_column(value);
            Ok(())
        })
    }

    #[getter]
    fn last_col(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.last_column())
    }
    #[setter]
    fn set_last_col(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.last_col(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_last_column(value);
            Ok(())
        })
    }

    #[getter]
    fn horz_banding(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.horizontal_banding())
    }
    #[setter]
    fn set_horz_banding(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.horz_banding(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_horizontal_banding(value);
            Ok(())
        })
    }

    #[getter]
    fn vert_banding(&self, py: Python<'_>) -> PyResult<bool> {
        self.flag(py, |table| table.vertical_banding())
    }
    #[setter]
    fn set_vert_banding(&self, py: Python<'_>, value: bool) -> PyResult<()> {
        if self.vert_banding(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| {
            table.set_vertical_banding(value);
            Ok(())
        })
    }

    #[getter]
    fn style_id(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.dimensions(py)?;
        Ok(shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .and_then(|table| table.style_id().map(str::to_owned)))
    }
    #[setter]
    fn set_style_id(&self, py: Python<'_>, value: Option<String>) -> PyResult<()> {
        if self.style_id(py)? == value {
            return Ok(());
        }
        self.edit_properties(py, |table| table.set_style_id(value))
    }

    #[getter]
    fn columns(&self, py: Python<'_>) -> PyResult<Py<PyColumnCollection>> {
        self.dimensions(py)?;
        Py::new(
            py,
            PyColumnCollection {
                presentation: self.presentation.clone_ref(py),
                path: self.presentation.borrow(py).capture_path(
                    self.presentation
                        .borrow(py)
                        .current_path(&self.path)
                        .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
                        .segs,
                    None,
                )?,
            },
        )
    }

    #[getter]
    fn rows(&self, py: Python<'_>) -> PyResult<Py<PyRowCollection>> {
        self.dimensions(py)?;
        Py::new(
            py,
            PyRowCollection {
                presentation: self.presentation.clone_ref(py),
                path: self.presentation.borrow(py).capture_path(
                    self.presentation
                        .borrow(py)
                        .current_path(&self.path)
                        .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
                        .segs,
                    None,
                )?,
            },
        )
    }

    fn cell(&self, py: Python<'_>, row: isize, col: isize) -> PyResult<Py<PyCell>> {
        let (rows, columns) = self.dimensions(py)?;
        let row = normalize_index(row, rows, "row")?;
        let column = normalize_index(col, columns, "cell")?;
        let mut segments = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?
            .segs;
        segments.push(PathSeg::Row(row));
        segments.push(PathSeg::Cell(column));
        let path = self.presentation.borrow(py).capture_path(segments, None)?;
        Py::new(
            py,
            PyCell {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }
}

#[pyclass(name = "ColumnCollection")]
pub struct PyColumnCollection {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyColumnCollection {
    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "column collection",
            ".table.columns",
        )?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .map(|table| table.column_count())
            .ok_or_else(|| PyValueError::new_err("shape has no table"))
    }

    fn item(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyColumn>> {
        let mut segments = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?
            .segs;
        segments.push(PathSeg::Cell(index));
        let path = self
            .presentation
            .borrow(py)
            .capture_path(segments, Some(oxml_py_support::RevisionScope::Grid))?;
        Py::new(
            py,
            PyColumn {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }
}

#[pymethods]
impl PyColumnCollection {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        self.len(py)
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let len = self.len(py)?;
        if let Ok(index) = key.extract::<isize>() {
            return Ok(self
                .item(py, normalize_index(index, len, "column")?)?
                .into_any());
        }
        if key.is_instance_of::<PySlice>() {
            let (start, stop, step): (isize, isize, isize) =
                key.call_method1("indices", (len,))?.extract()?;
            let items = PyList::empty(py);
            let mut index = start;
            while if step > 0 { index < stop } else { index > stop } {
                items.append(self.item(py, index as usize)?)?;
                index += step;
            }
            return Ok(items.into_any().unbind());
        }
        Err(PyTypeError::new_err(
            "column indices must be integers or slices",
        ))
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyColumnIterator>> {
        self.len(py)?;
        Py::new(
            py,
            PyColumnIterator {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                index: 0,
            },
        )
    }

    /// Inserts a grid column before `index`, or appends one, and returns it.
    ///
    /// The column copies the width and cell formatting, without the text, of
    /// the column left of it, or of the first column when it becomes the
    /// first, and the table frame grows by its width. The revision advances
    /// once.
    #[pyo3(signature = (index=None))]
    fn add_column(&self, py: Python<'_>, index: Option<isize>) -> PyResult<Py<PyColumn>> {
        let index = insertion_index(index, self.len(py)?, "column")?;
        let (candidate, prepared, mut current) = restructure(
            py,
            &self.presentation,
            &self.path,
            "ColumnCollection.add_column",
            |table| table.insert_column(index),
        )?;
        current.segs.push(PathSeg::Cell(index));
        let path = self.presentation.borrow(py).capture_prepared_path(
            &prepared,
            current.segs,
            Some(oxml_py_support::RevisionScope::Grid),
        )?;
        let result = Py::new(
            py,
            PyColumn {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )?;
        self.presentation
            .borrow_mut(py)
            .publish_owners(candidate, prepared)?;
        Ok(result)
    }

    /// Removes one column of this table and shrinks the table frame by its
    /// width. The revision advances once.
    fn remove(&self, py: Python<'_>, column: &Bound<'_, PyAny>) -> PyResult<()> {
        self.len(py)?;
        let column = column.extract::<PyRef<'_, PyColumn>>()?;
        if !column.presentation.is(&self.presentation) {
            return Err(PyValueError::new_err("column is not in this collection"));
        }
        validate_path(
            py,
            &self.presentation.borrow(py),
            &column.path,
            "column",
            "",
        )?;
        let index = member_index(
            &self.presentation.borrow(py),
            &self.path,
            &column.path,
            |segment| match segment {
                PathSeg::Cell(index) => Some(*index),
                _ => None,
            },
        )
        .ok_or_else(|| PyValueError::new_err("column is not in this collection"))?;
        let (candidate, prepared, _) = restructure(
            py,
            &self.presentation,
            &self.path,
            "ColumnCollection.remove",
            |table| table.remove_column(index),
        )?;
        self.presentation
            .borrow_mut(py)
            .publish_owners(candidate, prepared)
    }
}

#[pyclass]
struct PyColumnIterator {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    index: usize,
}

#[pymethods]
impl PyColumnIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyColumn>>> {
        let collection = PyColumnCollection {
            presentation: self.presentation.clone_ref(py),
            path: self.path.clone(),
        };
        if self.index >= collection.len(py)? {
            return Ok(None);
        }
        let index = self.index;
        self.index += 1;
        collection.item(py, index).map(Some)
    }
}

#[pyclass(name = "Column")]
pub struct PyColumn {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

#[pymethods]
impl PyColumn {
    #[getter]
    fn width(&self, py: Python<'_>) -> PyResult<i64> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "column", "")?;
        let column = cell_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("column index is missing"))?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .and_then(|table| table.column_width(column))
            .map(|width| width.0)
            .ok_or_else(|| PyIndexError::new_err("column index out of range"))
    }

    #[setter]
    fn set_width(&self, py: Python<'_>, width: i64) -> PyResult<()> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "column", "")?;
        let column = cell_index(&self.path)
            .ok_or_else(|| PyIndexError::new_err("column index is missing"))?;
        shape_mut_at(&mut self.presentation.borrow_mut(py), &self.path)
            .and_then(rpptx::ShapeMut::into_table_mut)
            .ok_or_else(|| PyValueError::new_err("shape has no table"))?
            .set_column_width(column, rpptx::Emu(width))
            .map_err(|error| crate::rpptx_to_pyerr(py, error))
    }
}

#[pyclass(name = "RowCollection")]
pub struct PyRowCollection {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyRowCollection {
    fn len(&self, py: Python<'_>) -> PyResult<usize> {
        validate_path(
            py,
            &self.presentation.borrow(py),
            &self.path,
            "row collection",
            ".table.rows",
        )?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .map(|table| table.row_count())
            .ok_or_else(|| PyValueError::new_err("shape has no table"))
    }

    fn item(&self, py: Python<'_>, index: usize) -> PyResult<Py<PyRow>> {
        let mut segments = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("owner is absent"))?
            .segs;
        segments.push(PathSeg::Row(index));
        let path = self.presentation.borrow(py).capture_path(segments, None)?;
        Py::new(
            py,
            PyRow {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )
    }
}

#[pymethods]
impl PyRowCollection {
    fn __len__(&self, py: Python<'_>) -> PyResult<usize> {
        self.len(py)
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let len = self.len(py)?;
        if let Ok(index) = key.extract::<isize>() {
            return Ok(self
                .item(py, normalize_index(index, len, "row")?)?
                .into_any());
        }
        if key.is_instance_of::<PySlice>() {
            let (start, stop, step): (isize, isize, isize) =
                key.call_method1("indices", (len,))?.extract()?;
            let items = PyList::empty(py);
            let mut index = start;
            while if step > 0 { index < stop } else { index > stop } {
                items.append(self.item(py, index as usize)?)?;
                index += step;
            }
            return Ok(items.into_any().unbind());
        }
        Err(PyTypeError::new_err(
            "row indices must be integers or slices",
        ))
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyRowIterator>> {
        self.len(py)?;
        Py::new(
            py,
            PyRowIterator {
                presentation: self.presentation.clone_ref(py),
                path: self.path.clone(),
                index: 0,
            },
        )
    }

    /// Inserts a row before `index`, or appends one, and returns it.
    ///
    /// The row copies the height and cell formatting, without the text, of
    /// the row above it, or of the first row when it becomes the first, and
    /// the table frame grows by its height. The revision advances once.
    #[pyo3(signature = (index=None))]
    fn add_row(&self, py: Python<'_>, index: Option<isize>) -> PyResult<Py<PyRow>> {
        let index = insertion_index(index, self.len(py)?, "row")?;
        let (candidate, prepared, mut current) = restructure(
            py,
            &self.presentation,
            &self.path,
            "RowCollection.add_row",
            |table| table.insert_row(index),
        )?;
        current.segs.push(PathSeg::Row(index));
        let path = self.presentation.borrow(py).capture_prepared_path(
            &prepared,
            current.segs,
            Some(oxml_py_support::RevisionScope::Grid),
        )?;
        let result = Py::new(
            py,
            PyRow {
                presentation: self.presentation.clone_ref(py),
                path,
            },
        )?;
        self.presentation
            .borrow_mut(py)
            .publish_owners(candidate, prepared)?;
        Ok(result)
    }

    /// Removes one row of this table and shrinks the table frame by its
    /// height. The revision advances once.
    fn remove(&self, py: Python<'_>, row: &Bound<'_, PyAny>) -> PyResult<()> {
        self.len(py)?;
        let row = row.extract::<PyRef<'_, PyRow>>()?;
        if !row.presentation.is(&self.presentation) {
            return Err(PyValueError::new_err("row is not in this collection"));
        }
        validate_path(py, &self.presentation.borrow(py), &row.path, "row", "")?;
        let index = member_index(
            &self.presentation.borrow(py),
            &self.path,
            &row.path,
            |segment| match segment {
                PathSeg::Row(index) => Some(*index),
                _ => None,
            },
        )
        .ok_or_else(|| PyValueError::new_err("row is not in this collection"))?;
        let (candidate, prepared, _) = restructure(
            py,
            &self.presentation,
            &self.path,
            "RowCollection.remove",
            |table| table.remove_row(index),
        )?;
        self.presentation
            .borrow_mut(py)
            .publish_owners(candidate, prepared)
    }
}

#[pyclass]
struct PyRowIterator {
    presentation: Py<PyPresentation>,
    path: ContentPath,
    index: usize,
}

#[pymethods]
impl PyRowIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyRow>>> {
        let collection = PyRowCollection {
            presentation: self.presentation.clone_ref(py),
            path: self.path.clone(),
        };
        if self.index >= collection.len(py)? {
            return Ok(None);
        }
        let index = self.index;
        self.index += 1;
        collection.item(py, index).map(Some)
    }
}

#[pyclass(name = "Row")]
pub struct PyRow {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

#[pymethods]
impl PyRow {
    #[getter]
    fn cells<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "row", "")?;
        let row =
            row_index(&self.path).ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
        let columns = shape_ref_at(&presentation, &self.path)
            .and_then(|shape| shape.table())
            .filter(|table| row < table.row_count())
            .map(|table| table.column_count())
            .ok_or_else(|| PyIndexError::new_err("row index out of range"))?;
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("row owner is absent"))?;
        let mut cells = Vec::with_capacity(columns);
        for column in 0..columns {
            let mut segments = current.segs.clone();
            segments.push(PathSeg::Cell(column));
            let path = presentation.capture_path(segments, None)?;
            cells.push(Py::new(
                py,
                PyCell {
                    presentation: self.presentation.clone_ref(py),
                    path,
                },
            )?);
        }
        PyTuple::new(py, cells)
    }

    /// The stored row height, a minimum that PowerPoint grows to fit text.
    #[getter]
    fn height(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "row", "")?;
        let row =
            row_index(&self.path).ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
        let height = shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .and_then(|table| table.row_height(row))
            .ok_or_else(|| PyIndexError::new_err("row index out of range"))?;
        length(py, Some(height))
    }

    /// Sets the row height and keeps the table frame height equal to the sum
    /// of its rows.
    #[setter]
    fn set_height(&self, py: Python<'_>, height: i64) -> PyResult<()> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "row", "")?;
        let row =
            row_index(&self.path).ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
        shape_mut_at(&mut self.presentation.borrow_mut(py), &self.path)
            .and_then(rpptx::ShapeMut::into_table_mut)
            .ok_or_else(|| PyValueError::new_err("shape has no table"))?
            .set_row_height(row, rpptx::Emu(height))
            .map_err(|error| rpptx_to_pyerr(py, error))
    }
}

#[pyclass(name = "Cell")]
pub struct PyCell {
    presentation: Py<PyPresentation>,
    path: ContentPath,
}

impl PyCell {
    fn read<T>(
        &self,
        py: Python<'_>,
        read: impl FnOnce(rpptx::TableCellRef<'_>) -> T,
    ) -> PyResult<T> {
        let presentation = self.presentation.borrow(py);
        validate_path(py, &presentation, &self.path, "cell", "")?;
        cell_ref_at(&presentation, &self.path)
            .map(read)
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))
    }

    fn edit<T>(
        &self,
        py: Python<'_>,
        edit: impl FnOnce(&mut rpptx::TableCellMut<'_>) -> T,
    ) -> PyResult<T> {
        let mut presentation = self.presentation.borrow_mut(py);
        validate_path(py, &presentation, &self.path, "cell", "")?;
        cell_mut_at(&mut presentation, &self.path)
            .map(|mut cell| edit(&mut cell))
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))
    }

    fn border(&self, py: Python<'_>, edge: rpptx::CellBorder) -> PyResult<Py<PyLineFormat>> {
        self.read(py, |_| ())?;
        Py::new(
            py,
            PyLineFormat::new(
                self.presentation.clone_ref(py),
                self.path.clone(),
                FillTarget::CellBorder(edge),
            ),
        )
    }

    fn margin(
        &self,
        py: Python<'_>,
        side: fn(&mut Margins) -> &mut Option<rpptx::Emu>,
    ) -> PyResult<Option<Py<PyAny>>> {
        let mut margins = self.read(py, |cell| cell.margins())?;
        length(py, *side(&mut margins))
    }

    /// Writes one margin and keeps the other three, leaving the package
    /// unchanged when the value equals the stored one.
    fn set_margin(
        &self,
        py: Python<'_>,
        side: fn(&mut Margins) -> &mut Option<rpptx::Emu>,
        value: Option<i64>,
    ) -> PyResult<()> {
        if value.is_some_and(|value| i32::try_from(value).is_err()) {
            return Err(PyValueError::new_err(
                "cell margin must fit a 32-bit EMU coordinate",
            ));
        }
        let current = self.read(py, |cell| cell.margins())?;
        let mut margins = current;
        *side(&mut margins) = value.map(rpptx::Emu);
        if margins == current {
            return Ok(());
        }
        let (left, right, top, bottom) = margins;
        self.edit(py, |cell| cell.set_margins(left, right, top, bottom))
    }
}

#[pymethods]
impl PyCell {
    #[getter]
    fn element(&self, py: Python<'_>) -> PyResult<Py<crate::presentation::PyPptElement>> {
        self.read(py, |_| ())?;
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
            crate::presentation::PptElementKind::Cell,
        )
    }

    /// Direct a:tcPr anchor, independent of TextFrame a:bodyPr.
    #[getter]
    fn vertical_anchor(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.read(py, |cell| cell.vertical_anchor())?
            .map(|anchor| anchor_object(py, anchor))
            .transpose()
    }

    #[setter]
    fn set_vertical_anchor(&self, py: Python<'_>, value: Option<i32>) -> PyResult<()> {
        let anchor = value.map(anchor_from_value).transpose()?;
        self.edit(py, |cell| cell.set_vertical_anchor(anchor))
    }

    #[getter]
    fn text(&self, py: Python<'_>) -> PyResult<String> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "cell", "")?;
        let row =
            row_index(&self.path).ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
        let cell =
            cell_index(&self.path).ok_or_else(|| PyIndexError::new_err("cell index is missing"))?;
        shape_ref_at(&self.presentation.borrow(py), &self.path)
            .and_then(|shape| shape.table())
            .and_then(|table| table.cell(row, cell))
            .map(|cell| cell.text())
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))
    }

    #[getter]
    fn text_frame(&self, py: Python<'_>) -> PyResult<Py<PyTextFrame>> {
        self.read(py, |_| ())?;
        let has_body = self.read(py, |cell| cell.text_frame().is_some())?;
        let current = self
            .presentation
            .borrow(py)
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("cell owner is absent"))?;
        let path = if has_body {
            self.presentation
                .borrow(py)
                .capture_text_body_path(current.segs, None)?
        } else {
            // The established absent-cell-body facade stays virtual. A real
            // checked cell replacement retires it through this child token.
            self.presentation
                .borrow(py)
                .capture_xml_children_path(current.segs)?
        };
        Py::new(py, PyTextFrame::new(self.presentation.clone_ref(py), path))
    }

    #[setter]
    fn set_text(&self, py: Python<'_>, value: &str) -> PyResult<()> {
        validate_path(py, &self.presentation.borrow(py), &self.path, "cell", "")?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("cell owner is absent"))?;
        let mut candidate = presentation.inner.clone();
        native_cell_mut_at(&mut candidate, &current)
            .map(|mut cell| cell.set_text(value))
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?;
        let survivors = presentation.text_changed_owners(&self.path)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Cell.text",
            &survivors,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)?;
        Ok(())
    }

    /// Merges the rectangle between this cell and `other_cell`, moving the
    /// text of the spanned cells into the top-left origin, as python-pptx does.
    fn merge(&self, py: Python<'_>, other_cell: &Bound<'_, PyAny>) -> PyResult<()> {
        let other = other_cell.extract::<PyRef<'_, PyCell>>()?;
        if !other.presentation.is(&self.presentation) {
            return Err(PyValueError::new_err("other_cell from different table"));
        }
        self.read(py, |_| ())?;
        other.read(py, |_| ())?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("cell owner is absent"))?;
        let other_current = presentation
            .current_path(&other.path)
            .ok_or_else(|| PyIndexError::new_err("cell owner is absent"))?;
        if !table_segments(&current).eq(table_segments(&other_current)) {
            return Err(PyValueError::new_err("other_cell from different table"));
        }
        let (row, column) = (
            row_index(&other_current)
                .ok_or_else(|| PyIndexError::new_err("row index is missing"))?,
            cell_index(&other_current)
                .ok_or_else(|| PyIndexError::new_err("cell index is missing"))?,
        );
        let (own_row, own_column) = (
            row_index(&current).ok_or_else(|| PyIndexError::new_err("row index is missing"))?,
            cell_index(&current).ok_or_else(|| PyIndexError::new_err("cell index is missing"))?,
        );
        let mut candidate = presentation.inner.clone();
        native_cell_mut_at(&mut candidate, &current)
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?
            .merge_to(row, column)
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        let before = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
            .xml()
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        let after = crate::shape::native_shape_ref_at(&candidate, &current)
            .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
            .xml()
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        if before == after {
            return Ok(());
        }
        let survivors = presentation.cell_text_owners(
            &current,
            own_row.min(row),
            own_row.max(row),
            own_column.min(column),
            own_column.max(column),
        )?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Cell.merge",
            &survivors,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)
    }

    /// Splits this merge origin back into its grid cells.
    fn split(&self, py: Python<'_>) -> PyResult<()> {
        self.read(py, |_| ())?;
        let mut presentation = self.presentation.borrow_mut(py);
        let current = presentation
            .current_path(&self.path)
            .ok_or_else(|| PyIndexError::new_err("cell owner is absent"))?;
        let row =
            row_index(&current).ok_or_else(|| PyIndexError::new_err("row index is missing"))?;
        let column =
            cell_index(&current).ok_or_else(|| PyIndexError::new_err("cell index is missing"))?;
        let (height, width) = cell_ref_at(&presentation, &current)
            .map(|cell| (cell.span_height(), cell.span_width()))
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?;
        let mut candidate = presentation.inner.clone();
        native_cell_mut_at(&mut candidate, &current)
            .ok_or_else(|| PyIndexError::new_err("cell index out of range"))?
            .split()
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        let before = crate::shape::native_shape_ref_at(&presentation.inner, &current)
            .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
            .xml()
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        let after = crate::shape::native_shape_ref_at(&candidate, &current)
            .ok_or_else(|| PyIndexError::new_err("table owner is absent"))?
            .xml()
            .map_err(|error| rpptx_to_pyerr(py, error))?;
        if before == after {
            return Ok(());
        }
        let last_row = row
            .checked_add(height.saturating_sub(1) as usize)
            .ok_or_else(|| PyValueError::new_err("cell span overflow"))?;
        let last_column = column
            .checked_add(width.saturating_sub(1) as usize)
            .ok_or_else(|| PyValueError::new_err("cell span overflow"))?;
        let survivors =
            presentation.cell_text_owners(&current, row, last_row, column, last_column)?;
        let prepared = presentation.prepare_owners(
            &candidate,
            "Cell.split",
            &survivors,
            false,
            false,
            false,
        )?;
        presentation.publish_owners(candidate, prepared)
    }

    #[getter]
    fn is_merge_origin(&self, py: Python<'_>) -> PyResult<bool> {
        self.read(py, |cell| cell.is_merge_origin())
    }

    #[getter]
    fn is_spanned(&self, py: Python<'_>) -> PyResult<bool> {
        self.read(py, |cell| cell.is_spanned())
    }

    #[getter]
    fn span_height(&self, py: Python<'_>) -> PyResult<u32> {
        self.read(py, |cell| cell.span_height())
    }

    #[getter]
    fn span_width(&self, py: Python<'_>) -> PyResult<u32> {
        self.read(py, |cell| cell.span_width())
    }

    #[getter]
    fn fill(&self, py: Python<'_>) -> PyResult<Py<PyFillFormat>> {
        self.read(py, |_| ())?;
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
            PyFillFormat::new(self.presentation.clone_ref(py), path, FillTarget::TableCell),
        )
    }

    #[getter]
    fn margin_left(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |margins| &mut margins.0)
    }

    #[setter]
    fn set_margin_left(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |margins| &mut margins.0, value)
    }

    #[getter]
    fn margin_right(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |margins| &mut margins.1)
    }

    #[setter]
    fn set_margin_right(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |margins| &mut margins.1, value)
    }

    #[getter]
    fn margin_top(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |margins| &mut margins.2)
    }

    #[setter]
    fn set_margin_top(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |margins| &mut margins.2, value)
    }

    #[getter]
    fn margin_bottom(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.margin(py, |margins| &mut margins.3)
    }

    #[setter]
    fn set_margin_bottom(&self, py: Python<'_>, value: Option<i64>) -> PyResult<()> {
        self.set_margin(py, |margins| &mut margins.3, value)
    }

    /// The `a:lnL` border as a live `LineFormat`.
    #[getter]
    fn border_left(&self, py: Python<'_>) -> PyResult<Py<PyLineFormat>> {
        self.border(py, rpptx::CellBorder::Left)
    }

    /// The `a:lnR` border as a live `LineFormat`.
    #[getter]
    fn border_right(&self, py: Python<'_>) -> PyResult<Py<PyLineFormat>> {
        self.border(py, rpptx::CellBorder::Right)
    }

    /// The `a:lnT` border as a live `LineFormat`.
    #[getter]
    fn border_top(&self, py: Python<'_>) -> PyResult<Py<PyLineFormat>> {
        self.border(py, rpptx::CellBorder::Top)
    }

    /// The `a:lnB` border as a live `LineFormat`.
    #[getter]
    fn border_bottom(&self, py: Python<'_>) -> PyResult<Py<PyLineFormat>> {
        self.border(py, rpptx::CellBorder::Bottom)
    }
}
