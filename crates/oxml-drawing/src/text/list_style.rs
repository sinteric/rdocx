use std::io::Write;

use oxml_core::OxmlError;
use oxml_core::raw_xml::{capture_element, capture_empty_element};
use oxml_core::xml::{local_name, matches_local_name};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer, XmlVersion};

use super::paragraph::{
    TextNamespaceScope, text_close_generated_child, text_root_spellings_scoped, text_validate_root,
};
use crate::order::OrderedRawChildren;

use super::body::{Result, TextError, missing_end};
use super::paragraph::{
    CT_TextParagraphProperties, text_capture_opaque_event, text_fragment_scope, text_owned_name,
    text_root_spellings, text_source_start,
};

/// DrawingML list defaults for each of the nine paragraph levels.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]
pub struct CT_TextListStyle {
    pub default_paragraph_properties: Option<CT_TextParagraphProperties>,
    pub level1: Option<CT_TextParagraphProperties>,
    pub level2: Option<CT_TextParagraphProperties>,
    pub level3: Option<CT_TextParagraphProperties>,
    pub level4: Option<CT_TextParagraphProperties>,
    pub level5: Option<CT_TextParagraphProperties>,
    pub level6: Option<CT_TextParagraphProperties>,
    pub level7: Option<CT_TextParagraphProperties>,
    pub level8: Option<CT_TextParagraphProperties>,
    pub level9: Option<CT_TextParagraphProperties>,
    raw_attributes: Vec<(String, String)>,
    source_root: Vec<String>,
    source_tag: String,
    source_namespaces: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
}
impl PartialEq for CT_TextListStyle {
    fn eq(&self, other: &Self) -> bool {
        self.default_paragraph_properties == other.default_paragraph_properties
            && self.level1 == other.level1
            && self.level2 == other.level2
            && self.level3 == other.level3
            && self.level4 == other.level4
            && self.level5 == other.level5
            && self.level6 == other.level6
            && self.level7 == other.level7
            && self.level8 == other.level8
            && self.level9 == other.level9
            && self
                .raw_attributes
                .iter()
                .filter(|(key, _)| key != "xmlns" && !key.starts_with("xmlns:"))
                .eq(other
                    .raw_attributes
                    .iter()
                    .filter(|(key, _)| key != "xmlns" && !key.starts_with("xmlns:")))
            && matches!(
                (
                    super::paragraph::text_namespace_scope(
                        &text_source_start(&self.source_tag, &self.source_root),
                        &[],
                    ),
                    super::paragraph::text_namespace_scope(
                        &text_source_start(&other.source_tag, &other.source_root),
                        &[],
                    ),
                ),
                (Ok(left), Ok(right)) if left.iter().filter(|(key, value)| key != "r" || value != "http://schemas.openxmlformats.org/officeDocument/2006/relationships").map(|(key, value)| (key, value)).collect::<std::collections::BTreeMap<_, _>>()
                    == right.iter().filter(|(key, value)| key != "r" || value != "http://schemas.openxmlformats.org/officeDocument/2006/relationships").map(|(key, value)| (key, value)).collect::<std::collections::BTreeMap<_, _>>()
            )
            && self.raw_children == other.raw_children
    }
}
impl Eq for CT_TextListStyle {}

impl CT_TextListStyle {
    /// Parses a complete list-style element with any prefix, including the
    /// PresentationML wrappers that share this DrawingML content model.
    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        Self::from_xml_with_namespaces(xml, &TextNamespaceScope::default())
    }

    /// Parses a list-style fragment using its actual enclosing namespace bindings.
    /// The original root attributes and opaque children remain retained source facts.
    pub fn from_xml_with_inherited_namespaces(
        xml: &[u8],
        inherited: &[(String, String)],
    ) -> Result<Self> {
        Self::from_xml_with_namespaces(xml, &TextNamespaceScope::inherited(inherited))
    }

    pub(crate) fn from_xml_with_namespaces(
        xml: &[u8],
        inherited: &TextNamespaceScope,
    ) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element) if is_list_style_root(element.name().as_ref()) => {
                    validate_list_style_root(&element)?;
                    return Self::from_element(&mut reader, &element, inherited);
                }
                Event::Empty(element) if is_list_style_root(element.name().as_ref()) => {
                    validate_list_style_root(&element)?;
                    let mut style = Self::from_start(&element)?;
                    style.source_namespaces = text_fragment_scope(&element, inherited)?.physical;
                    style.source_root = text_root_spellings_scoped(
                        &element,
                        inherited,
                        &capture_empty_element(&element)?,
                    )?;
                    return Ok(style);
                }
                Event::Start(element) | Event::Empty(element) => {
                    return Err(unexpected(&element));
                }
                Event::Eof => {
                    return Err(TextError::Xml(OxmlError::MissingElement(
                        "DrawingML list style".to_owned(),
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
            source_root: text_root_spellings(start)?,
            source_tag: std::str::from_utf8(start.name().as_ref())
                .map_err(OxmlError::from)?
                .to_owned(),
            ..Self::default()
        })
    }

    fn from_element(
        reader: &mut Reader<&[u8]>,
        start: &BytesStart<'_>,
        inherited: &TextNamespaceScope,
    ) -> Result<Self> {
        let mut namespaces = text_fragment_scope(start, inherited)?;
        namespaces.fixed_list_style_children = true;
        let mut style = Self::from_start(start)?;
        style.source_namespaces = namespaces.physical.clone();
        style.source_root = text_root_spellings_scoped(
            start,
            inherited,
            &capture_element(&mut reader.clone(), start)?,
        )?;
        let root_name = local_name(start.name().as_ref()).to_vec();
        let mut boundary = 0;
        let mut buffer = Vec::new();
        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element) => {
                    let name = text_owned_name(&element, &namespaces)?;
                    if name == b"defPPr" || list_level(&name)?.is_some() {
                        validate_list_style_root(&element)?;
                    }
                    let raw = capture_element(reader, &element)?;
                    style.capture_child(&name, raw, &mut boundary, &namespaces)?;
                }
                Event::Empty(element) => {
                    let name = text_owned_name(&element, &namespaces)?;
                    if name == b"defPPr" || list_level(&name)?.is_some() {
                        validate_list_style_root(&element)?;
                    }
                    let raw = capture_empty_element(&element)?;
                    style.capture_child(&name, raw, &mut boundary, &namespaces)?;
                }
                Event::End(element)
                    if local_name(element.name().as_ref()) == root_name.as_slice() =>
                {
                    return Ok(style);
                }
                Event::Eof => {
                    return Err(missing_end(
                        std::str::from_utf8(&root_name).unwrap_or("list style"),
                    ));
                }
                event @ (Event::Comment(_) | Event::PI(_) | Event::Text(_) | Event::CData(_)) => {
                    style
                        .raw_children
                        .push(boundary, text_capture_opaque_event(event)?);
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    fn capture_child(
        &mut self,
        name: &[u8],
        raw: Vec<u8>,
        boundary: &mut usize,
        namespaces: &TextNamespaceScope,
    ) -> Result<()> {
        if name == b"defPPr" {
            if self.default_paragraph_properties.is_some() {
                return Err(TextError::DuplicateElement("defPPr".to_owned()));
            }
            self.default_paragraph_properties = Some(
                CT_TextParagraphProperties::from_xml_with_namespaces(&raw, namespaces)?,
            );
            *boundary = (*boundary).max(1);
            return Ok(());
        }
        if let Some(level) = list_level(name)? {
            if self.level(level).is_some() {
                return Err(TextError::DuplicateElement(
                    String::from_utf8_lossy(name).into_owned(),
                ));
            }
            self.set_level(
                level,
                CT_TextParagraphProperties::from_xml_with_namespaces(&raw, namespaces)?,
            );
            *boundary = (*boundary).max(level + 1);
            return Ok(());
        }

        let schema_boundary = match name {
            b"extLst" => 10,
            _ => *boundary,
        };
        self.raw_children
            .push((*boundary).max(schema_boundary), raw);
        *boundary = (*boundary).max(schema_boundary);
        Ok(())
    }

    /// Returns local list defaults over the inherited default and each level.
    pub fn with_inherited_defaults(&self, defaults: &Self) -> Result<Self> {
        let mut local = Writer::new(Vec::new());
        self.write_xml(&mut local)?;
        let mut inherited = Writer::new(Vec::new());
        defaults.write_xml(&mut inherited)?;
        let xml = super::paragraph::text_inherit_property_xml(
            &local.into_inner(),
            &self.source_namespaces,
            &inherited.into_inner(),
            &defaults.source_namespaces,
        )?;
        let mut result = Self::from_xml_with_inherited_namespaces(&xml, &self.source_namespaces)?;
        for level in 1..=9 {
            if defaults.default_paragraph_properties.is_none()
                && defaults.level(level).is_none()
                && self.default_paragraph_properties.is_none()
                && self.level(level).is_none()
            {
                continue;
            }
            let mut properties = defaults
                .default_paragraph_properties
                .clone()
                .unwrap_or_default();
            for overlay in [
                defaults.level(level),
                self.default_paragraph_properties.as_ref(),
                self.level(level),
            ]
            .into_iter()
            .flatten()
            {
                properties = overlay.with_inherited_defaults(&properties)?;
            }
            result.set_level(level, properties);
        }
        Ok(result)
    }

    /// Writes the list style with fixed prefixes and ascending level order.
    pub fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.write_xml_as(writer, "a:lstStyle")
    }

    /// Writes the list style using a caller-selected OOXML wrapper tag.
    pub fn write_xml_as<W: Write>(&self, writer: &mut Writer<W>, tag: &str) -> Result<()> {
        let mut start =
            super::paragraph::text_wrapper_start(tag, &self.source_tag, &self.source_root)?;
        let owned_tag = std::str::from_utf8(start.name().as_ref())
            .map_err(OxmlError::from)?
            .to_owned();
        let tag = owned_tag.as_str();
        if self.source_root.is_empty() {
            push_raw_attributes(&mut start, &self.raw_attributes);
        }
        let has_modelled = self.default_paragraph_properties.is_some()
            || (1..=9).any(|level| self.level(level).is_some());
        if !has_modelled && self.raw_children.is_empty() {
            return write_empty(writer, start);
        }

        write_start(writer, start)?;
        emit_raw(writer, self.raw_children.at(0))?;
        if let Some(properties) = &self.default_paragraph_properties {
            {
                let mut child = Writer::new(Vec::new());
                properties.write_xml(&mut child, "a:defPPr")?;
                writer
                    .get_mut()
                    .write_all(&text_close_generated_child(
                        &child.into_inner(),
                        &super::paragraph::text_namespace_scope(
                            &text_source_start(tag, &self.source_root),
                            &[],
                        )?,
                    )?)
                    .map_err(OxmlError::from)?;
            }
        }
        for level in 1..=9 {
            emit_raw(writer, self.raw_children.at(level))?;
            if let Some(properties) = self.level(level) {
                let tag = level_tag(level)
                    .ok_or_else(|| TextError::UnexpectedElement(format!("list level {level}")))?;
                {
                    let mut child = Writer::new(Vec::new());
                    properties.write_xml(&mut child, tag)?;
                    writer
                        .get_mut()
                        .write_all(&text_close_generated_child(
                            &child.into_inner(),
                            &super::paragraph::text_namespace_scope(
                                &text_source_start(tag, &self.source_root),
                                &[],
                            )?,
                        )?)
                        .map_err(OxmlError::from)?;
                }
            }
        }
        emit_raw(writer, self.raw_children.at(10))?;
        write_end(writer, tag)
    }

    /// Serialises a complete list-style fragment.
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(Vec::new());
        self.write_xml(&mut writer)?;
        Ok(writer.into_inner())
    }

    /// Returns one level by its one-based DrawingML number.
    pub fn level(&self, level: usize) -> Option<&CT_TextParagraphProperties> {
        match level {
            1 => self.level1.as_ref(),
            2 => self.level2.as_ref(),
            3 => self.level3.as_ref(),
            4 => self.level4.as_ref(),
            5 => self.level5.as_ref(),
            6 => self.level6.as_ref(),
            7 => self.level7.as_ref(),
            8 => self.level8.as_ref(),
            9 => self.level9.as_ref(),
            _ => None,
        }
    }

    pub fn raw_children(&self) -> &OrderedRawChildren {
        &self.raw_children
    }

    fn set_level(&mut self, level: usize, properties: CT_TextParagraphProperties) {
        let slot = match level {
            1 => &mut self.level1,
            2 => &mut self.level2,
            3 => &mut self.level3,
            4 => &mut self.level4,
            5 => &mut self.level5,
            6 => &mut self.level6,
            7 => &mut self.level7,
            8 => &mut self.level8,
            9 => &mut self.level9,
            _ => return,
        };
        *slot = Some(properties);
    }
}

fn validate_list_style_root(start: &BytesStart<'_>) -> Result<()> {
    let namespaces = super::paragraph::text_namespace_scope(start, &[])?;
    let name = start.name();
    let raw = std::str::from_utf8(name.as_ref()).map_err(OxmlError::from)?;
    let (prefix, local) = raw.split_once(':').unwrap_or(("", raw));
    if local != "lstStyle"
        && namespaces.iter().any(|(key, uri)| {
            key == prefix && uri == "http://schemas.openxmlformats.org/presentationml/2006/main"
        })
    {
        return Ok(());
    }
    text_validate_root(start)
}

fn is_list_style_root(name: &[u8]) -> bool {
    matches_local_name(name, b"lstStyle")
        || matches_local_name(name, b"defaultTextStyle")
        || matches_local_name(name, b"titleStyle")
        || matches_local_name(name, b"bodyStyle")
        || matches_local_name(name, b"otherStyle")
        || matches_local_name(name, b"notesStyle")
}

fn list_level(name: &[u8]) -> Result<Option<usize>> {
    let Some(number) = name
        .strip_prefix(b"lvl")
        .and_then(|name| name.strip_suffix(b"pPr"))
    else {
        return Ok(None);
    };
    if !number.is_empty() && number.iter().all(u8::is_ascii_digit) {
        if number.len() == 1 && (b'1'..=b'9').contains(&number[0]) {
            return Ok(Some(usize::from(number[0] - b'0')));
        }
        return Err(TextError::UnexpectedElement(
            String::from_utf8_lossy(name).into_owned(),
        ));
    }
    Ok(None)
}

fn level_tag(level: usize) -> Option<&'static str> {
    match level {
        1 => Some("a:lvl1pPr"),
        2 => Some("a:lvl2pPr"),
        3 => Some("a:lvl3pPr"),
        4 => Some("a:lvl4pPr"),
        5 => Some("a:lvl5pPr"),
        6 => Some("a:lvl6pPr"),
        7 => Some("a:lvl7pPr"),
        8 => Some("a:lvl8pPr"),
        9 => Some("a:lvl9pPr"),
        _ => None,
    }
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

fn emit_raw<'a, W: Write>(
    writer: &mut Writer<W>,
    children: impl Iterator<Item = &'a [u8]>,
) -> Result<()> {
    for child in children {
        writer.get_mut().write_all(child).map_err(OxmlError::from)?;
    }
    Ok(())
}

fn write_start<W: Write>(writer: &mut Writer<W>, start: BytesStart<'_>) -> Result<()> {
    writer
        .write_event(Event::Start(start))
        .map_err(OxmlError::from)?;
    Ok(())
}

fn write_empty<W: Write>(writer: &mut Writer<W>, start: BytesStart<'_>) -> Result<()> {
    writer
        .write_event(Event::Empty(start))
        .map_err(OxmlError::from)?;
    Ok(())
}

fn write_end<W: Write>(writer: &mut Writer<W>, tag: &str) -> Result<()> {
    writer
        .write_event(Event::End(BytesEnd::new(tag)))
        .map_err(OxmlError::from)?;
    Ok(())
}

fn unexpected(element: &BytesStart<'_>) -> TextError {
    TextError::UnexpectedElement(String::from_utf8_lossy(element.name().as_ref()).into_owned())
}
