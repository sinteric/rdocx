use std::fmt;
use std::io::Write;

use oxml_core::OxmlError;
use oxml_core::raw_xml::{capture_element, capture_empty_element};
use oxml_core::xml::{local_name, matches_local_name};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer, XmlVersion};

use crate::effect::{
    CT_EffectList, EffectError, raw_contains_placeholder_color, raw_is_effect_dag,
};
use crate::fill::{Fill, FillError};
use crate::geometry::{CT_CustomGeometry2D, CT_PresetGeometry2D, GeometryError};
use crate::line::{CT_LineProperties, LineError};
use crate::namespace::{
    property_is_drawing, property_namespace_scope, property_reject_writer_conflicts,
};
use crate::order::OrderedRawChildren;
use crate::xfrm::{CT_Transform2D, TransformError};

/// Errors produced while parsing or writing DrawingML shape properties.
#[derive(Debug)]
pub enum ShapePropertiesError {
    Xml(OxmlError),
    Transform(TransformError),
    Geometry(GeometryError),
    Fill(FillError),
    Line(LineError),
    Effect(EffectError),
    UnexpectedElement(String),
}

impl fmt::Display for ShapePropertiesError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Xml(error) => error.fmt(formatter),
            Self::Transform(error) => error.fmt(formatter),
            Self::Geometry(error) => error.fmt(formatter),
            Self::Fill(error) => error.fmt(formatter),
            Self::Line(error) => error.fmt(formatter),
            Self::Effect(error) => error.fmt(formatter),
            Self::UnexpectedElement(element) => {
                write!(
                    formatter,
                    "unexpected DrawingML shape-properties element: {element}"
                )
            }
        }
    }
}

impl std::error::Error for ShapePropertiesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Xml(error) => Some(error),
            Self::Transform(error) => Some(error),
            Self::Geometry(error) => Some(error),
            Self::Fill(error) => Some(error),
            Self::Line(error) => Some(error),
            Self::Effect(error) => Some(error),
            Self::UnexpectedElement(_) => None,
        }
    }
}

impl From<OxmlError> for ShapePropertiesError {
    fn from(error: OxmlError) -> Self {
        Self::Xml(error)
    }
}

impl From<TransformError> for ShapePropertiesError {
    fn from(error: TransformError) -> Self {
        Self::Transform(error)
    }
}

impl From<GeometryError> for ShapePropertiesError {
    fn from(error: GeometryError) -> Self {
        Self::Geometry(error)
    }
}

impl From<FillError> for ShapePropertiesError {
    fn from(error: FillError) -> Self {
        Self::Fill(error)
    }
}

impl From<LineError> for ShapePropertiesError {
    fn from(error: LineError) -> Self {
        Self::Line(error)
    }
}

impl From<EffectError> for ShapePropertiesError {
    fn from(error: EffectError) -> Self {
        Self::Effect(error)
    }
}

pub type Result<T> = std::result::Result<T, ShapePropertiesError>;

/// The modelled children of one DrawingML `a:spPr` element.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]
pub struct CT_ShapeProperties {
    pub transform: Option<CT_Transform2D>,
    pub custom_geometry: Option<CT_CustomGeometry2D>,
    pub preset_geometry: Option<CT_PresetGeometry2D>,
    pub fill: Option<Fill>,
    pub line: Option<CT_LineProperties>,
    pub effects: Option<CT_EffectList>,
    raw_attributes: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
    source_namespaces: Vec<(String, String)>,
    source_xml: Option<Vec<u8>>,
    source_projection: Vec<u8>,
}

impl PartialEq for CT_ShapeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.custom_geometry == other.custom_geometry
            && self.preset_geometry == other.preset_geometry
            && self.fill == other.fill
            && self.line == other.line
            && self.effects == other.effects
            && self.raw_attributes == other.raw_attributes
            && self.raw_children == other.raw_children
    }
}

impl CT_ShapeProperties {
    /// Returns staged local properties with absent source-master defaults filled.
    pub fn with_inherited_defaults(&self, defaults: &Self) -> Result<Self> {
        let scoped_defaults = defaults.scoped_inheritance_defaults()?;
        let defaults = &scoped_defaults;
        let mut result = self.scoped_inheritance_defaults()?;
        for (name, value) in &defaults.raw_attributes {
            if name == "xmlns" || name.starts_with("xmlns:") {
                continue;
            }
            let identity = shape_property_attribute_identity(name, &defaults.source_namespaces)?;
            let mut existing = None;
            for (key, local) in &self.raw_attributes {
                if key == "xmlns" || key.starts_with("xmlns:") {
                    continue;
                }
                if shape_property_attribute_identity(key, &self.source_namespaces)? == identity {
                    existing = Some(local);
                    break;
                }
            }
            if let Some(local) = existing {
                if name != "bwMode"
                    && (local != value
                        || !shape_property_context_equal(
                            &self.source_namespaces,
                            &defaults.source_namespaces,
                        ))
                {
                    return Err(OxmlError::InvalidValue(
                        "conflicting inherited opaque shape-property attribute".into(),
                    )
                    .into());
                }
            } else {
                if name != "bwMode"
                    && !shape_property_context_equal(
                        &self.source_namespaces,
                        &defaults.source_namespaces,
                    )
                {
                    return Err(OxmlError::InvalidValue(
                        "unrepresentable inherited shape-property namespace authority".into(),
                    )
                    .into());
                }
                result.raw_attributes.push((name.clone(), value.clone()));
            }
        }

        match (&mut result.transform, &defaults.transform) {
            (Some(local), Some(parent)) => {
                if local.offset.is_none() {
                    local.offset = parent.offset;
                }
                if local.extent.is_none() {
                    local.extent = parent.extent;
                }
                if local.child_offset.is_none() {
                    local.child_offset = parent.child_offset;
                }
                if local.child_extent.is_none() {
                    local.child_extent = parent.child_extent;
                }
            }
            (local @ None, Some(parent)) => *local = Some(parent.clone()),
            _ => {}
        }
        if result.custom_geometry.is_none() && result.preset_geometry.is_none() {
            result.custom_geometry = defaults.custom_geometry.clone();
            result.preset_geometry = defaults.preset_geometry.clone();
        }
        if result.line.is_none() {
            result.line = defaults.line.clone();
        }
        let mut occupied = Vec::new();
        for boundary in 0..=8 {
            for raw in self.raw_children.at(boundary) {
                if let Some(slot) = inherited_shape_property_slot(raw, &self.source_namespaces)? {
                    occupied.push(slot);
                }
            }
        }
        if result.fill.is_none() && !occupied.contains(&2) {
            result.fill = defaults.fill.clone();
        }
        if result.effects.is_none() && !occupied.contains(&4) {
            result.effects = defaults.effects.clone();
        }
        if self.transform.is_some() {
            occupied.push(0);
        }
        if self.custom_geometry.is_some() || self.preset_geometry.is_some() {
            occupied.push(1);
        }
        if self.fill.is_some() {
            occupied.push(2);
        }
        if self.line.is_some() {
            occupied.push(3);
        }
        if self.effects.is_some() {
            occupied.push(4);
        }
        for boundary in 0..=8 {
            for raw in defaults.raw_children.at(boundary) {
                if let Some(slot) = inherited_shape_property_slot(raw, &defaults.source_namespaces)?
                {
                    if occupied.contains(&slot) {
                        continue;
                    }
                    occupied.push(slot);
                } else if shape_property_context_equal(
                    &self.source_namespaces,
                    &defaults.source_namespaces,
                ) && (0..=8).any(|at| self.raw_children.at(at).any(|local| local == raw))
                {
                    continue;
                }
                result.raw_children.push(
                    boundary,
                    close_inherited_shape_property(raw, &defaults.source_namespaces)?,
                );
            }
        }
        let xml = result.to_xml()?;
        let reparsed = Self::from_xml_with_inherited_namespaces(&xml, &self.source_namespaces)?;
        if reparsed.transform != result.transform
            || reparsed.fill != result.fill
            || reparsed.line != result.line
        {
            return Err(OxmlError::InvalidValue(
                "inherited shape properties changed during reparse".into(),
            )
            .into());
        }
        // The staged serialization is the new physical owner. Retain its fresh
        // projection so a subsequent inheritance operation does not treat our
        // own completed merge as an ambiguous edit of the predecessor scope.
        Ok(reparsed)
    }

    // Only inheritance transfers typed descendants out of their physical owner.
    // Ordinary serialization and equality keep their existing model semantics.
    fn scoped_inheritance_defaults(&self) -> Result<Self> {
        let projection = self.to_xml()?;
        let original = self.source_xml.as_deref().unwrap_or(&projection);
        let unchanged = projection == self.source_projection;
        if !unchanged && self.source_xml.is_some() {
            let mut reader = Reader::from_reader(original);
            let mut depth = 0;
            loop {
                match reader.read_event().map_err(OxmlError::from)? {
                    event @ (Event::Start(_) | Event::Empty(_)) => {
                        let empty = matches!(&event, Event::Empty(_));
                        let start = match &event {
                            Event::Start(start) | Event::Empty(start) => start,
                            _ => unreachable!(),
                        };
                        if depth > 0
                            && capture_raw_attributes(start)?
                                .iter()
                                .any(|(name, _)| name == "xmlns" || name.starts_with("xmlns:"))
                        {
                            return Err(OxmlError::InvalidValue(
                                "edited inherited shape properties have nested namespace authority"
                                    .into(),
                            )
                            .into());
                        }
                        if !empty {
                            depth += 1;
                        }
                    }
                    Event::End(_) => depth -= 1,
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        let input = if unchanged { original } else { &projection };
        let closed = close_inherited_typed_shape_groups(input, &self.source_namespaces)?;
        let represented =
            Self::from_xml_with_inherited_namespaces(&closed, &self.source_namespaces)?;
        let mut writer_scope = self.source_namespaces.clone();
        if !writer_scope.iter().any(|(prefix, _)| prefix == "a") {
            writer_scope.push((
                "a".into(),
                "http://schemas.openxmlformats.org/drawingml/2006/main".into(),
            ));
        }
        let source_metadata = inherited_shape_metadata(original, &self.source_namespaces)?;
        let represented_metadata = inherited_shape_metadata(&represented.to_xml()?, &writer_scope)?;
        if let Some((key, _)) = source_metadata
            .iter()
            .find(|(key, count)| represented_metadata.get(*key).copied().unwrap_or(0) < **count)
        {
            return Err(OxmlError::InvalidValue(format!(
                "unrepresentable opaque metadata in inherited shape properties at {key}"
            ))
            .into());
        }
        Ok(represented)
    }

    /// Parses one complete `a:spPr` element with any namespace prefix.
    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        Self::from_xml_with_inherited_namespaces(xml, &[])
    }

    /// Parses a property fragment in its actual enclosing namespace scope.
    pub fn from_xml_with_inherited_namespaces(
        xml: &[u8],
        inherited: &[(String, String)],
    ) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element) if matches_local_name(element.name().as_ref(), b"spPr") => {
                    let scope = property_namespace_scope(&element, inherited)?;
                    property_reject_writer_conflicts(&scope)?;
                    let mut properties = Self::from_element(&mut reader, &element, &scope)?;
                    properties.source_xml = Some(xml.to_vec());
                    properties.source_projection = properties.to_xml()?;
                    return Ok(properties);
                }
                Event::Empty(element) if matches_local_name(element.name().as_ref(), b"spPr") => {
                    property_reject_writer_conflicts(&property_namespace_scope(
                        &element, inherited,
                    )?)?;
                    let mut properties = Self::from_start(&element)?;
                    properties.source_namespaces = property_namespace_scope(&element, inherited)?;
                    properties.source_xml = Some(xml.to_vec());
                    properties.source_projection = properties.to_xml()?;
                    return Ok(properties);
                }
                Event::Start(element) | Event::Empty(element) => {
                    return Err(ShapePropertiesError::UnexpectedElement(element_name(
                        &element,
                    )));
                }
                Event::Eof => {
                    return Err(ShapePropertiesError::Xml(OxmlError::MissingElement(
                        "DrawingML shape properties".to_owned(),
                    )));
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    fn from_start(start: &BytesStart<'_>) -> Result<Self> {
        Ok(Self {
            raw_attributes: capture_raw_attributes(start)?,
            ..Self::default()
        })
    }

    fn from_element(
        reader: &mut Reader<&[u8]>,
        start: &BytesStart<'_>,
        namespaces: &[(String, String)],
    ) -> Result<Self> {
        let mut properties = Self::from_start(start)?;
        properties.source_namespaces = namespaces.to_vec();
        let mut boundary = 0;
        let mut buffer = Vec::new();

        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element) => {
                    let name = local_name(element.name().as_ref()).to_vec();
                    let owned = property_is_drawing(&element, namespaces)?;
                    let child_scope = property_namespace_scope(&element, namespaces)?;
                    let raw = capture_element(reader, &element)?;
                    if owned && properties.parse_modelled(&name, &raw, &child_scope)? {
                        boundary = boundary.max(modelled_boundary(&name));
                    } else {
                        properties.raw_children.push(
                            if owned {
                                boundary.max(raw_boundary_before(&name))
                            } else {
                                boundary
                            },
                            raw,
                        );
                        if owned {
                            boundary = boundary.max(raw_boundary_after(&name));
                        }
                    }
                }
                Event::Empty(element) => {
                    let name = local_name(element.name().as_ref()).to_vec();
                    let owned = property_is_drawing(&element, namespaces)?;
                    let child_scope = property_namespace_scope(&element, namespaces)?;
                    let raw = capture_empty_element(&element)?;
                    if owned && properties.parse_modelled(&name, &raw, &child_scope)? {
                        boundary = boundary.max(modelled_boundary(&name));
                    } else {
                        properties.raw_children.push(
                            if owned {
                                boundary.max(raw_boundary_before(&name))
                            } else {
                                boundary
                            },
                            raw,
                        );
                        if owned {
                            boundary = boundary.max(raw_boundary_after(&name));
                        }
                    }
                }
                Event::End(element) if matches_local_name(element.name().as_ref(), b"spPr") => {
                    break;
                }
                Event::Eof => return Err(missing_end()),
                _ => {}
            }
            buffer.clear();
        }
        Ok(properties)
    }

    fn parse_modelled(
        &mut self,
        name: &[u8],
        raw: &[u8],
        namespaces: &[(String, String)],
    ) -> Result<bool> {
        if matches!(
            name,
            b"xfrm" | b"custGeom" | b"prstGeom" | b"ln" | b"effectLst"
        ) || is_fill(name)
        {
            property_reject_writer_conflicts(namespaces)?;
        }
        match name {
            b"xfrm" if self.transform.is_none() => {
                self.transform = Some(CT_Transform2D::from_xml(raw)?);
            }
            b"custGeom" if self.custom_geometry.is_none() => {
                self.custom_geometry = Some(CT_CustomGeometry2D::from_xml(raw)?);
            }
            b"prstGeom" if self.preset_geometry.is_none() => {
                self.preset_geometry = Some(CT_PresetGeometry2D::from_xml(raw)?);
            }
            name if is_fill(name) && self.fill.is_none() => {
                self.fill = Some(Fill::from_xml(raw)?);
            }
            b"ln" if self.line.is_none() => {
                self.line = Some(CT_LineProperties::from_xml_with_inherited_namespaces(
                    raw, namespaces,
                )?);
            }
            b"effectLst" if self.effects.is_none() => {
                self.effects = Some(CT_EffectList::from_xml(raw)?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// Writes shape properties with the fixed `a:` prefix and schema order.
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(Vec::new());
        self.write_xml(&mut writer)?;
        Ok(writer.into_inner())
    }

    /// Writes shape properties into an existing XML writer.
    pub fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.write_xml_as(writer, "a:spPr")
    }

    /// Writes shape properties under the caller's required root name.
    pub fn write_xml_as<W: Write>(&self, writer: &mut Writer<W>, name: &str) -> Result<()> {
        let mut start = BytesStart::new(name);
        push_raw_attributes(&mut start, &self.raw_attributes);
        if self.transform.is_none()
            && self.custom_geometry.is_none()
            && self.preset_geometry.is_none()
            && self.fill.is_none()
            && self.line.is_none()
            && self.effects.is_none()
            && self.raw_children.is_empty()
        {
            writer
                .write_event(Event::Empty(start))
                .map_err(OxmlError::from)?;
            return Ok(());
        }

        writer
            .write_event(Event::Start(start))
            .map_err(OxmlError::from)?;
        emit_raw(writer, self.raw_children.at(0))?;
        if let Some(transform) = &self.transform {
            transform.write_xml(writer)?;
        }
        emit_raw(writer, self.raw_children.at(1))?;
        if let Some(geometry) = &self.custom_geometry {
            writer
                .get_mut()
                .write_all(&geometry.to_xml()?)
                .map_err(OxmlError::from)?;
        } else if let Some(geometry) = &self.preset_geometry {
            writer
                .get_mut()
                .write_all(&geometry.to_xml()?)
                .map_err(OxmlError::from)?;
        }
        emit_raw(writer, self.raw_children.at(2))?;
        if let Some(fill) = &self.fill {
            fill.write_xml(writer)?;
        }
        emit_raw(writer, self.raw_children.at(3))?;
        if let Some(line) = &self.line {
            line.write_xml(writer)?;
        }
        emit_raw(writer, self.raw_children.at(4))?;
        if let Some(effects) = &self.effects {
            effects.write_xml(writer)?;
        }
        for boundary in 5..=8 {
            emit_raw(writer, self.raw_children.at(boundary))?;
        }
        writer
            .write_event(Event::End(BytesEnd::new(name)))
            .map_err(OxmlError::from)?;
        Ok(())
    }

    pub fn raw_children(&self) -> &OrderedRawChildren {
        &self.raw_children
    }

    /// Returns whether an opaque effect DAG is present instead of a typed list.
    pub fn has_unmodelled_effect(&self) -> bool {
        (0..=8).any(|boundary| self.raw_children.at(boundary).any(raw_is_effect_dag))
    }

    /// Reports a placeholder colour inside an opaque effect DAG.
    pub fn has_unmodelled_effect_placeholder_color(&self) -> bool {
        (0..=8).any(|boundary| {
            self.raw_children
                .at(boundary)
                .any(|xml| raw_is_effect_dag(xml) && raw_contains_placeholder_color(xml))
        })
    }
}

fn is_fill(name: &[u8]) -> bool {
    matches!(
        name,
        b"noFill" | b"solidFill" | b"gradFill" | b"pattFill" | b"blipFill"
    )
}

fn modelled_boundary(name: &[u8]) -> usize {
    match name {
        b"xfrm" => 1,
        b"custGeom" | b"prstGeom" => 2,
        name if is_fill(name) => 3,
        b"ln" => 4,
        b"effectLst" => 5,
        _ => 0,
    }
}

fn raw_boundary_after(name: &[u8]) -> usize {
    match name {
        b"xfrm" => 1,
        b"custGeom" | b"prstGeom" => 2,
        b"noFill" | b"solidFill" | b"gradFill" | b"blipFill" | b"pattFill" | b"grpFill" => 3,
        b"ln" => 4,
        b"effectLst" | b"effectDag" => 5,
        b"scene3d" => 6,
        b"sp3d" => 7,
        b"extLst" => 8,
        _ => 0,
    }
}

/// The earliest boundary an unmodelled child may be written at. A child such
/// as `a:scene3d` read where no `a:effectLst` stood stays behind the effect
/// slot, so an effect list added later is written before it.
fn raw_boundary_before(name: &[u8]) -> usize {
    raw_boundary_after(name).saturating_sub(1)
}

fn shape_property_attribute_identity(
    name: &str,
    scope: &[(String, String)],
) -> Result<(String, String)> {
    if let Some((prefix, local)) = name.split_once(':') {
        let uri = if prefix == "xml" {
            "http://www.w3.org/XML/1998/namespace"
        } else {
            scope
                .iter()
                .rev()
                .find(|(key, _)| key == prefix)
                .map(|(_, uri)| uri.as_str())
                .ok_or_else(|| {
                    OxmlError::InvalidValue(
                        "unbound inherited shape-property attribute prefix".into(),
                    )
                })?
        };
        Ok((uri.to_owned(), local.to_owned()))
    } else {
        Ok((String::new(), name.to_owned()))
    }
}

fn shape_property_context_equal(left: &[(String, String)], right: &[(String, String)]) -> bool {
    let mut left: std::collections::BTreeMap<_, _> = left
        .iter()
        .map(|(prefix, uri)| (prefix.as_str(), uri.as_str()))
        .collect();
    let mut right: std::collections::BTreeMap<_, _> = right
        .iter()
        .map(|(prefix, uri)| (prefix.as_str(), uri.as_str()))
        .collect();
    left.entry("").or_insert("");
    right.entry("").or_insert("");
    left == right
}

fn inherited_shape_property_slot(
    raw: &[u8],
    namespaces: &[(String, String)],
) -> Result<Option<usize>> {
    let mut reader = Reader::from_reader(raw);
    match reader.read_event().map_err(OxmlError::from)? {
        Event::Start(start) | Event::Empty(start) if property_is_drawing(&start, namespaces)? => {
            let name = local_name(start.name().as_ref()).to_vec();
            Ok(match name.as_slice() {
                b"xfrm" => Some(0),
                b"custGeom" | b"prstGeom" => Some(1),
                b"grpFill" => Some(2),
                name if is_fill(name) => Some(2),
                b"ln" => Some(3),
                b"effectLst" | b"effectDag" => Some(4),
                b"scene3d" => Some(5),
                b"sp3d" => Some(6),
                b"extLst" => Some(7),
                _ => None,
            })
        }
        _ => Ok(None),
    }
}

// Admission checks metadata occupancy at the same expanded physical path.
// Values of modeled attributes can legitimately change through public fields.
// Namespace declarations are authority, not opaque attribute payload, and were
// already closed independently before the owning projection was reparsed.
fn inherited_shape_metadata(
    xml: &[u8],
    namespaces: &[(String, String)],
) -> Result<std::collections::BTreeMap<String, usize>> {
    let mut reader = Reader::from_reader(xml);
    let mut scopes = vec![namespaces.to_vec()];
    let mut paths = vec![String::new()];
    let mut metadata = std::collections::BTreeMap::new();
    loop {
        match reader.read_event().map_err(OxmlError::from)? {
            event @ (Event::Start(_) | Event::Empty(_)) => {
                let empty = matches!(&event, Event::Empty(_));
                let start = match &event {
                    Event::Start(start) | Event::Empty(start) => start,
                    _ => unreachable!(),
                };
                let scope = property_namespace_scope(start, scopes.last().unwrap())?;
                let name = start.name();
                let spelling = std::str::from_utf8(name.as_ref()).map_err(OxmlError::from)?;
                let (prefix, local) = spelling.split_once(':').unwrap_or(("", spelling));
                let uri = scope
                    .iter()
                    .find(|(key, _)| key == prefix)
                    .map_or("", |(_, uri)| uri.as_str());
                if !prefix.is_empty() && uri.is_empty() {
                    return Err(OxmlError::InvalidValue(
                        "unbound shape-property metadata owner".into(),
                    )
                    .into());
                }
                let path = if paths.len() == 1 {
                    String::new()
                } else {
                    format!(
                        "{}/{}:{uri}{}:{local}",
                        paths.last().unwrap(),
                        uri.len(),
                        local.len()
                    )
                };
                for (name, _) in capture_raw_attributes(start)? {
                    if name == "xmlns" || name.starts_with("xmlns:") {
                        continue;
                    }
                    let (attribute_uri, attribute_local) =
                        shape_property_attribute_identity(&name, &scope)?;
                    if uri == "http://schemas.openxmlformats.org/drawingml/2006/main"
                        && local == "xfrm"
                        && attribute_uri.is_empty()
                        && matches!(attribute_local.as_str(), "rot" | "flipH" | "flipV")
                    {
                        // The existing transform writer canonicalizes schema-default
                        // rotation and flip values. They are modeled scalars, not
                        // unsupported opaque metadata.
                        continue;
                    }
                    let key = format!(
                        "{path}@{}:{attribute_uri}{}:{attribute_local}",
                        attribute_uri.len(),
                        attribute_local.len()
                    );
                    *metadata.entry(key).or_insert(0) += 1;
                }
                if !empty {
                    scopes.push(scope);
                    paths.push(path);
                }
            }
            Event::End(_) => {
                scopes.pop();
                paths.pop();
            }
            Event::Comment(text) => {
                let key = format!("{}!comment:{:?}", paths.last().unwrap(), &*text);
                *metadata.entry(key).or_insert(0) += 1;
            }
            Event::PI(text) => {
                let key = format!("{}!pi:{:?}", paths.last().unwrap(), &*text);
                *metadata.entry(key).or_insert(0) += 1;
            }
            Event::Text(text) if text.iter().any(|byte| !byte.is_ascii_whitespace()) => {
                let key = format!("{}!text:{:?}", paths.last().unwrap(), &*text);
                *metadata.entry(key).or_insert(0) += 1;
            }
            Event::CData(text) => {
                let key = format!("{}!cdata:{:?}", paths.last().unwrap(), &*text);
                *metadata.entry(key).or_insert(0) += 1;
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(metadata)
}

// Close every physical descendant of a transferred typed property. Lower-level
// typed writers may omit their root declarations, so opaque descendant scopes
// must already be self-contained when those writers receive the parsed model.
fn close_inherited_typed_shape_groups(
    xml: &[u8],
    namespaces: &[(String, String)],
) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(xml);
    let mut writer = Writer::new(Vec::new());
    let mut scopes = vec![namespaces.to_vec()];
    let mut transferring = Vec::new();
    loop {
        match reader.read_event().map_err(OxmlError::from)? {
            event @ (Event::Start(_) | Event::Empty(_)) => {
                let empty = matches!(&event, Event::Empty(_));
                let start = match &event {
                    Event::Start(start) | Event::Empty(start) => start,
                    _ => unreachable!(),
                };
                let scope = property_namespace_scope(start, scopes.last().unwrap())?;
                let transfer = transferring.last().copied().unwrap_or(false)
                    || (scopes.len() == 2
                        && property_is_drawing(start, &scope)?
                        && modelled_boundary(local_name(start.name().as_ref())) > 0);
                if transfer {
                    let mut content = std::str::from_utf8(start.as_ref())
                        .map_err(OxmlError::from)?
                        .to_owned();
                    let attributes = capture_raw_attributes(start)?;
                    let mut closed_scope = scope.clone();
                    if !closed_scope.iter().any(|(prefix, _)| prefix.is_empty()) {
                        closed_scope.push((String::new(), String::new()));
                    }
                    for (prefix, uri) in closed_scope {
                        let name = if prefix.is_empty() {
                            "xmlns".to_owned()
                        } else {
                            format!("xmlns:{prefix}")
                        };
                        if !attributes.iter().any(|(key, _)| key == &name) {
                            content.push_str(&format!(
                                " {name}=\"{}\"",
                                quick_xml::escape::escape(&uri)
                            ));
                        }
                    }
                    let closed = BytesStart::from_content(content, start.name().as_ref().len());
                    writer
                        .write_event(if empty {
                            Event::Empty(closed)
                        } else {
                            Event::Start(closed)
                        })
                        .map_err(OxmlError::from)?;
                } else {
                    writer.write_event(event).map_err(OxmlError::from)?;
                }
                if !empty {
                    scopes.push(scope);
                    transferring.push(transfer);
                }
            }
            Event::End(end) => {
                writer
                    .write_event(Event::End(end))
                    .map_err(OxmlError::from)?;
                scopes.pop();
                transferring.pop();
            }
            Event::Eof => break,
            event => writer.write_event(event).map_err(OxmlError::from)?,
        }
    }
    Ok(writer.into_inner())
}

fn close_inherited_shape_property(raw: &[u8], namespaces: &[(String, String)]) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(raw);
    let (start, empty) = match reader.read_event().map_err(OxmlError::from)? {
        Event::Start(start) => (start, false),
        Event::Empty(start) => (start, true),
        _ => return Ok(raw.to_vec()),
    };
    let mut content = std::str::from_utf8(start.as_ref())
        .map_err(OxmlError::from)?
        .to_owned();
    let attributes = capture_raw_attributes(&start)?;
    let mut scope = namespaces.to_vec();
    if !scope.iter().any(|(prefix, _)| prefix.is_empty()) {
        scope.push((String::new(), String::new()));
    }
    for (prefix, uri) in &scope {
        let name = if prefix.is_empty() {
            "xmlns".to_owned()
        } else {
            format!("xmlns:{prefix}")
        };
        if attributes.iter().any(|(key, _)| key == &name) {
            continue;
        }
        content.push_str(&format!(" {name}=\"{}\"", quick_xml::escape::escape(uri)));
    }
    let name_len = start.name().as_ref().len();
    let mut writer = Writer::new(Vec::new());
    let closed = BytesStart::from_content(content, name_len);
    if empty {
        writer
            .write_event(Event::Empty(closed))
            .map_err(OxmlError::from)?;
    } else {
        writer
            .write_event(Event::Start(closed))
            .map_err(OxmlError::from)?;
        writer
            .get_mut()
            .extend_from_slice(&raw[reader.buffer_position() as usize..]);
    }
    Ok(writer.into_inner())
}

fn emit_raw<'a, W: Write>(
    writer: &mut Writer<W>,
    children: impl Iterator<Item = &'a [u8]>,
) -> Result<()> {
    for child in children {
        writer.get_mut().write_all(child).map_err(OxmlError::from)?;
    }
    Ok(())
}

fn capture_raw_attributes(start: &BytesStart<'_>) -> Result<Vec<(String, String)>> {
    let mut raw = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute.map_err(OxmlError::from)?;
        let name = std::str::from_utf8(attribute.key.as_ref())
            .map_err(OxmlError::from)?
            .to_owned();
        let value = attribute
            .decoded_and_normalized_value(XmlVersion::Implicit1_0, start.decoder())
            .map_err(OxmlError::from)?
            .into_owned();
        raw.push((name, value));
    }
    Ok(raw)
}

fn push_raw_attributes(start: &mut BytesStart<'_>, attributes: &[(String, String)]) {
    for (name, value) in attributes {
        start.push_attribute((name.as_str(), value.as_str()));
    }
}

fn element_name(element: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(element.name().as_ref()).into_owned()
}

fn missing_end() -> ShapePropertiesError {
    ShapePropertiesError::Xml(OxmlError::MissingElement(
        "closing DrawingML shape properties".to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::CT_ShapeProperties;

    #[test]
    fn shape_properties_round_trip_in_schema_order() {
        let xml = br#"<z:spPr><x:before/><z:xfrm rot="60000"><z:off x="1" y="2"/></z:xfrm><x:afterXfrm/><z:custGeom><z:pathLst><z:path/></z:pathLst></z:custGeom><x:afterGeom/><z:solidFill><z:srgbClr val="112233"/></z:solidFill><x:afterFill/><z:ln w="12700"><z:noFill/></z:ln><x:afterLine/><z:effectLst><z:glow rad="40000"><z:srgbClr val="445566"/></z:glow></z:effectLst><x:afterEffects/></z:spPr>"#;
        let properties = CT_ShapeProperties::from_xml(xml).unwrap();
        assert!(properties.transform.is_some());
        assert!(properties.custom_geometry.is_some());
        assert!(properties.fill.is_some());
        assert!(properties.line.is_some());
        assert!(properties.effects.is_some());

        let written = properties.to_xml().unwrap();
        assert_eq!(written, br#"<a:spPr><x:before/><a:xfrm rot="60000"><a:off x="1" y="2"/></a:xfrm><x:afterXfrm/><a:custGeom><a:pathLst><a:path/></a:pathLst></a:custGeom><x:afterGeom/><a:solidFill><a:srgbClr val="112233"/></a:solidFill><x:afterFill/><a:ln w="12700"><a:noFill/></a:ln><x:afterLine/><a:effectLst><z:glow rad="40000"><z:srgbClr val="445566"/></z:glow></a:effectLst><x:afterEffects/></a:spPr>"#);
        assert_eq!(CT_ShapeProperties::from_xml(&written).unwrap(), properties);
    }

    #[test]
    fn modelled_children_added_later_precede_unmodelled_3d_and_extensions() {
        let xml = br#"<a:spPr><a:prstGeom prst="rect"/><a:scene3d/><x:after/><a:sp3d/><a:extLst/></a:spPr>"#;
        let mut properties = CT_ShapeProperties::from_xml(xml).unwrap();
        assert_eq!(properties.to_xml().unwrap(), xml);

        properties.fill = Some(crate::fill::Fill::from_xml(br#"<a:noFill/>"#).unwrap());
        properties.line = Some(crate::line::CT_LineProperties::default());
        properties.effects = Some(crate::effect::CT_EffectList::default());
        assert_eq!(
            properties.to_xml().unwrap(),
            br#"<a:spPr><a:prstGeom prst="rect"/><a:noFill/><a:ln/><a:effectLst/><a:scene3d/><x:after/><a:sp3d/><a:extLst/></a:spPr>"#
        );
    }

    #[test]
    fn shape_property_root_attributes_round_trip_without_loss() {
        let properties =
            CT_ShapeProperties::from_xml(br#"<q:spPr bwMode="gray" x:future="keep &amp; stay"/>"#)
                .unwrap();
        assert_eq!(
            properties.to_xml().unwrap(),
            br#"<a:spPr bwMode="gray" x:future="keep &amp; stay"/>"#
        );
    }

    #[test]
    fn preset_geometry_round_trips_with_unknown_children_verbatim() {
        let xml = br#"<p:spPr><u:before/><q:prstGeom xmlns:q="http://schemas.openxmlformats.org/drawingml/2006/main" prst="trapezoid"><u:first id="1"><u:nested/></u:first><q:avLst><u:inside/><q:gd name="adj" fmla="val 50000"/></q:avLst><u:last>keep &amp; stay</u:last></q:prstGeom><u:after/></p:spPr>"#;
        for declaration in ["", " xmlns:q=\"urn:foreign\""] {
            let raw = format!("<q:prstGeom{declaration} prst=\"trapezoid\"/>");
            let source = format!("<p:spPr>{raw}</p:spPr>");
            let opaque = CT_ShapeProperties::from_xml(source.as_bytes()).unwrap();
            assert!(opaque.preset_geometry.is_none());
            assert!(
                String::from_utf8(opaque.to_xml().unwrap())
                    .unwrap()
                    .contains(&raw)
            );
        }
        let properties = CT_ShapeProperties::from_xml(xml).unwrap();
        let preset = properties.preset_geometry.as_ref().unwrap();
        assert_eq!(preset.preset, "trapezoid");
        assert_eq!(preset.adjust_values().len(), 1);

        let written = String::from_utf8(properties.to_xml().unwrap()).unwrap();
        assert!(written.starts_with("<a:spPr>"));
        assert!(written.contains("<a:prstGeom prst=\"trapezoid\">"));
        for raw in [
            "<u:before/>",
            "<u:first id=\"1\"><u:nested/></u:first>",
            "<u:inside/>",
            "<u:last>keep &amp; stay</u:last>",
            "<u:after/>",
        ] {
            assert!(written.contains(raw), "missing raw subtree {raw}");
        }
        let before = written.find("<u:before/>").unwrap();
        let preset_start = written.find("<a:prstGeom").unwrap();
        let first = written.find("<u:first").unwrap();
        let adjustments = written.find("<a:avLst").unwrap();
        let inside = written.find("<u:inside/>").unwrap();
        let guide = written.find("<a:gd ").unwrap();
        let last = written.find("<u:last>").unwrap();
        let preset_end = written.find("</a:prstGeom>").unwrap();
        let after = written.find("<u:after/>").unwrap();
        assert!(before < preset_start);
        assert!(preset_start < first && first < adjustments);
        assert!(adjustments < inside && inside < guide);
        assert!(guide < last && last < preset_end && preset_end < after);
        assert_eq!(
            CT_ShapeProperties::from_xml(written.as_bytes()).unwrap(),
            properties
        );
    }
}

#[cfg(test)]
#[test]
fn fx216_local_group_fill_blocks_inherited_fill_choice() {
    let group = CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:grpFill/></a:spPr>"#).unwrap();
    let solid = CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:solidFill><a:srgbClr val="FF0000"/></a:solidFill></a:spPr>"#).unwrap();
    let before_group = group.to_xml().unwrap();
    let before_solid = solid.to_xml().unwrap();
    let result = group.with_inherited_defaults(&solid).unwrap();
    assert!(
        result.fill.is_none(),
        "raw local group fill must remain the only fill choice"
    );
    let xml = result.to_xml().unwrap();
    assert!(String::from_utf8_lossy(&xml).contains("<a:grpFill/>"));
    assert!(!String::from_utf8_lossy(&xml).contains("solidFill"));
    assert_eq!(
        CT_ShapeProperties::from_xml(&xml)
            .unwrap()
            .to_xml()
            .unwrap(),
        xml
    );
    let converse = solid
        .with_inherited_defaults(&group)
        .unwrap()
        .to_xml()
        .unwrap();
    assert!(!String::from_utf8_lossy(&converse).contains("grpFill"));
    assert!(String::from_utf8_lossy(&converse).contains("solidFill"));
    let foreign=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:f="urn:foreign"><f:grpFill keep='&#65;'/></a:spPr>"#).unwrap();
    let result = foreign.with_inherited_defaults(&solid).unwrap();
    assert!(result.fill.is_some());
    assert!(
        String::from_utf8_lossy(&result.to_xml().unwrap()).contains("<f:grpFill keep='&#65;'/>")
    );
    assert_eq!(group.to_xml().unwrap(), before_group);
    assert_eq!(solid.to_xml().unwrap(), before_solid);
}

#[cfg(test)]
#[test]
fn fx216_opaque_attribute_aliases_reuse_or_refuse_expanded_identity() {
    for inherited_value in ["shared", "different"] {
        let local=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:producer" xmlns:y="urn:producer" y:keep="shared"/>"#).unwrap();
        let parent_xml = format!(
            r#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:producer" xmlns:y="urn:producer" x:keep="{inherited_value}"/>"#
        );
        let defaults = CT_ShapeProperties::from_xml(parent_xml.as_bytes()).unwrap();
        let before_local = local.to_xml().unwrap();
        let before_defaults = defaults.to_xml().unwrap();
        if inherited_value == "different" {
            assert!(local.with_inherited_defaults(&defaults).is_err());
        } else {
            let xml = local
                .with_inherited_defaults(&defaults)
                .unwrap()
                .to_xml()
                .unwrap();
            let mut reader = quick_xml::NsReader::from_reader(xml.as_slice());
            let mut count = 0;
            if let Event::Empty(element) = reader.read_event().unwrap() {
                for attribute in element.attributes() {
                    let attribute = attribute.unwrap();
                    let (namespace, name) = reader.resolver().resolve_attribute(attribute.key);
                    if name.as_ref() == b"keep"
                        && matches!(namespace,quick_xml::name::ResolveResult::Bound(uri) if uri.as_ref()==b"urn:producer")
                    {
                        count += 1;
                    }
                }
            } else {
                panic!("expected unchanged empty property owner");
            }
            assert_eq!(
                count, 1,
                "one expanded opaque attribute must not become two alias spellings"
            );
            assert_eq!(
                CT_ShapeProperties::from_xml(&xml)
                    .unwrap()
                    .to_xml()
                    .unwrap(),
                xml
            );
        }
        assert_eq!(local.to_xml().unwrap(), before_local);
        assert_eq!(defaults.to_xml().unwrap(), before_defaults);
    }
}

#[cfg(test)]
#[test]
fn fx216_opaque_children_with_equal_bytes_keep_distinct_namespace_authority() {
    let local=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:local"><z:payload meaning="z:item"/></a:spPr>"#).unwrap();
    let defaults=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source"><z:payload meaning="z:item"/></a:spPr>"#).unwrap();
    let before_local = local.to_xml().unwrap();
    let before_defaults = defaults.to_xml().unwrap();
    let xml = local
        .with_inherited_defaults(&defaults)
        .unwrap()
        .to_xml()
        .unwrap();
    let mut reader = quick_xml::NsReader::from_reader(xml.as_slice());
    let mut seen = std::collections::HashSet::new();
    loop {
        let (namespace, event) = reader.read_resolved_event().unwrap();
        match event {
            Event::Empty(element) if element.local_name().as_ref() == b"payload" => {
                if let quick_xml::name::ResolveResult::Bound(namespace) = namespace {
                    seen.insert(String::from_utf8_lossy(namespace.as_ref()).into_owned());
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(
        seen,
        std::collections::HashSet::from(["urn:local".to_owned(), "urn:source".to_owned()])
    );
    assert_eq!(local.to_xml().unwrap(), before_local);
    assert_eq!(defaults.to_xml().unwrap(), before_defaults);
    let same = local
        .with_inherited_defaults(&local)
        .unwrap()
        .to_xml()
        .unwrap();
    assert_eq!(same, before_local);
    assert_eq!(
        CT_ShapeProperties::from_xml(&xml)
            .unwrap()
            .to_xml()
            .unwrap(),
        xml
    );
}

#[cfg(test)]
#[test]
fn fx216_typed_inherited_fill_preserves_source_opaque_namespace() {
    let local=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:local"/>"#).unwrap();
    let defaults=CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source"><a:solidFill><a:srgbClr val="FF0000"/><z:payload meaning="z:item"/></a:solidFill></a:spPr>"#).unwrap();
    let before_local = local.to_xml().unwrap();
    let before_defaults = defaults.to_xml().unwrap();
    let result = local.with_inherited_defaults(&defaults).unwrap();
    let xml = result.to_xml().unwrap();
    let mut reader = quick_xml::NsReader::from_reader(xml.as_slice());
    let mut seen = false;
    loop {
        let (namespace, event) = reader.read_resolved_event().unwrap();
        match event {
            Event::Empty(element) if element.local_name().as_ref() == b"payload" => {
                assert!(
                    matches!(namespace,quick_xml::name::ResolveResult::Bound(uri) if uri.as_ref()==b"urn:source")
                );
                seen = true;
            }
            Event::Eof => break,
            _ => {}
        }
    }
    assert!(seen);
    assert_eq!(local.to_xml().unwrap(), before_local);
    assert_eq!(defaults.to_xml().unwrap(), before_defaults);
}

#[cfg(test)]
#[test]
fn fx216_typed_groups_close_actual_ancestor_scopes_and_preserve_edits() {
    let local = CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:local" xmlns="urn:local"><opaque/></a:spPr>"#).unwrap();
    let groups = [
        r#"<a:xfrm><a:off x="10" y="20"/><z:payload meaning="z:item"/></a:xfrm>"#,
        r#"<a:prstGeom prst="rect"><a:avLst/><z:payload meaning="z:item"/></a:prstGeom>"#,
        r#"<a:custGeom><a:avLst/><a:gdLst/><a:ahLst/><a:cxnLst/><a:pathLst><z:payload meaning="z:item"/></a:pathLst></a:custGeom>"#,
        r#"<a:solidFill><a:srgbClr val="FF0000"/><z:payload meaning="z:item"/></a:solidFill>"#,
        r#"<a:solidFill><a:srgbClr val="FF0000"/><payload meaning="item"/></a:solidFill>"#,
        r#"<a:ln w="10" z:keep="z:item"><z:payload meaning="z:item"/></a:ln>"#,
        r#"<a:effectLst><z:payload meaning="z:item"/></a:effectLst>"#,
        r#"<a:solidFill><a:srgbClr val="FF0000"/><z:payload xmlns:z="urn:source" meaning="z:item"/></a:solidFill>"#,
    ];
    for (index, group) in groups.iter().enumerate() {
        // The namespace authority comes from the physical shape ancestor, not
        // from either the property fragment or the destination owner.
        let outer = format!(
            r#"<p:sp xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source" xmlns="urn:source"><p:spPr>{group}</p:spPr></p:sp>"#
        );
        let mut reader = Reader::from_reader(outer.as_bytes());
        let Event::Start(owner) = reader.read_event().unwrap() else {
            panic!("shape owner")
        };
        let scope = property_namespace_scope(&owner, &[]).unwrap();
        let Event::Start(properties) = reader.read_event().unwrap() else {
            panic!("property owner")
        };
        let raw = capture_element(&mut reader, &properties).unwrap();
        let mut defaults =
            CT_ShapeProperties::from_xml_with_inherited_namespaces(&raw, &scope).unwrap();
        if index == 0 {
            defaults
                .transform
                .as_mut()
                .unwrap()
                .offset
                .as_mut()
                .unwrap()
                .x = oxml_core::units::Emu(123);
        }
        if index == 1 {
            defaults.preset_geometry.as_mut().unwrap().preset = "ellipse".into();
        }
        if index == 5 {
            defaults.line.as_mut().unwrap().width = Some(456);
        }
        let before = defaults.to_xml().unwrap();
        let result = local.with_inherited_defaults(&defaults).unwrap();
        let xml = result.to_xml().unwrap();
        let mut scan = quick_xml::NsReader::from_reader(xml.as_slice());
        let mut payloads = 0;
        let mut locals = 0;
        loop {
            let (namespace, event) = scan.read_resolved_event().unwrap();
            match event {
                Event::Empty(start) if start.local_name().as_ref() == b"payload" => {
                    assert!(
                        matches!(namespace, quick_xml::name::ResolveResult::Bound(uri) if uri.as_ref() == b"urn:source"),
                        "{index}: {}",
                        String::from_utf8_lossy(&xml)
                    );
                    payloads += 1;
                }
                Event::Empty(start) if start.local_name().as_ref() == b"opaque" => {
                    assert!(
                        matches!(namespace, quick_xml::name::ResolveResult::Bound(uri) if uri.as_ref() == b"urn:local")
                    );
                    locals += 1;
                }
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(payloads, 1, "group {index}");
        assert_eq!(locals, 1);
        let reparsed = CT_ShapeProperties::from_xml(&xml).unwrap();
        if index == 0 {
            assert_eq!(reparsed.transform.unwrap().offset.unwrap().x.0, 123);
        }
        if index == 1 {
            assert_eq!(reparsed.preset_geometry.unwrap().preset, "ellipse");
        }
        if index == 5 {
            assert_eq!(reparsed.line.unwrap().width, Some(456));
        }
        assert_eq!(defaults.to_xml().unwrap(), before);
    }
    let mut nested = CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:ln w="10" xmlns:z="urn:nested"><z:payload/></a:ln></a:spPr>"#).unwrap();
    nested.line.as_mut().unwrap().width = Some(99);
    let before = nested.to_xml().unwrap();
    assert!(local.with_inherited_defaults(&nested).is_err());
    assert_eq!(nested.to_xml().unwrap(), before);
}

#[cfg(test)]
#[test]
fn fx216_unrepresentable_typed_metadata_refuses_local_and_parent_without_mutation() {
    let empty = CT_ShapeProperties::from_xml(br#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source"/>"#).unwrap();
    for group in [
        r#"<a:solidFill z:meaning="z:item"><a:srgbClr val="FF0000"/></a:solidFill>"#,
        r#"<a:ln><a:solidFill z:meaning="z:item"><a:srgbClr val="FF0000"/></a:solidFill></a:ln>"#,
        r#"<a:prstGeom prst="rect"><a:avLst><a:gd name="adjust" fmla="val 100" z:meaning="z:item"/></a:avLst></a:prstGeom>"#,
    ] {
        let xml = format!(
            r#"<a:spPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:z="urn:source">{group}</a:spPr>"#
        );
        let properties = CT_ShapeProperties::from_xml(xml.as_bytes()).unwrap();
        let before = properties.to_xml().unwrap();
        if !String::from_utf8_lossy(&before).contains("z:meaning") {
            assert!(
                empty.with_inherited_defaults(&properties).is_err(),
                "unrepresentable inherited {group}"
            );
            assert!(
                properties.with_inherited_defaults(&empty).is_err(),
                "unrepresentable local {group}"
            );
        } else {
            let inherited = empty
                .with_inherited_defaults(&properties)
                .unwrap()
                .to_xml()
                .unwrap();
            assert!(String::from_utf8_lossy(&inherited).contains("z:meaning"));
        }
        assert_eq!(properties.to_xml().unwrap(), before);
    }
}
