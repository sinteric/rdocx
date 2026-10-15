use std::io::Write;

use oxml_core::OxmlError;
use oxml_core::raw_xml::{capture_element, capture_empty_element};
use oxml_core::units::Emu;
use oxml_core::xml::reject_non_xml_characters;
use oxml_drawing::namespace::A_NS;
use oxml_drawing::order::OrderedRawChildren;
use oxml_drawing::text::CT_TextListStyle;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer, XmlVersion};

use crate::namespace::{
    FIXED_MODEL_PREFIXES, NamespaceBindings, P_NS, R_NS, all_attributes, root_attributes,
};

const MIN_SLIDE_ID: u32 = 256;
const MAX_SLIDE_ID: u32 = 2_147_483_647;
const P14_NS: &str = "http://schemas.microsoft.com/office/powerpoint/2010/main";
const SECTION_EXTENSION_URI: &str = "{521415D9-36F7-43E2-AB2F-B90AF26B5E84}";

pub type Result<T> = std::result::Result<T, OxmlError>;
type RawAttributes = Vec<(String, String)>;
type ParsedSlideList = (Vec<CT_SlideId>, RawAttributes, OrderedRawChildren);
type ParsedMasterList = (Vec<CT_SlideMasterId>, RawAttributes, OrderedRawChildren);
type ParsedIdentifier = (Option<u32>, String, RawAttributes, OrderedRawChildren);
type ParsedSectionSlideIds = (
    Vec<u32>,
    Vec<u8>,
    OrderedRawChildren,
    Vec<SectionSlideIdSidecar>,
);
type ParsedSectionList = (Vec<Section>, Option<(usize, usize)>, Vec<(String, String)>);

/// The typed size of a presentation slide or notes page in EMUs.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CT_SlideSize {
    pub cx: Emu,
    pub cy: Emu,
    pub kind: Option<String>,
    raw_attributes: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
}

impl CT_SlideSize {
    pub fn new(cx: Emu, cy: Emu) -> Result<Self> {
        validate_dimension("size", "cx", cx)?;
        validate_dimension("size", "cy", cy)?;
        Ok(Self {
            cx,
            cy,
            kind: None,
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }
}

/// One ordered slide identifier and its unresolved OPC relationship id.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CT_SlideId {
    pub id: u32,
    pub relationship_id: String,
    raw_attributes: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
}

impl CT_SlideId {
    pub fn new(id: u32, relationship_id: impl Into<String>) -> Result<Self> {
        validate_slide_id(id)?;
        Ok(Self {
            id,
            relationship_id: relationship_id.into(),
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }
}

/// One slide-master identifier and its unresolved OPC relationship id.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CT_SlideMasterId {
    /// The producer-assigned master id, when present in the source part.
    pub id: Option<u32>,
    pub relationship_id: String,
    raw_attributes: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
}

/// One producer section and the stable slide ids assigned to it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    pub id: Option<String>,
    pub name: Option<String>,
    pub slide_ids: Vec<u32>,
    original_slide_ids: Vec<u32>,
    raw_attributes: Vec<u8>,
    slide_id_list_attributes: Vec<u8>,
    slide_id_list_raw_children: OrderedRawChildren,
    slide_id_sidecars: Vec<SectionSlideIdSidecar>,
    raw_children: OrderedRawChildren,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SectionSlideIdSidecar {
    id: u32,
    raw_attributes: Vec<u8>,
    raw_children: Vec<u8>,
}

impl Section {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        slide_ids: Vec<u32>,
    ) -> Result<Self> {
        let id = id.into();
        validate_guid("section", &id)?;
        let name = name.into();
        reject_non_xml_characters("section name", &name)?;
        Ok(Self {
            id: Some(id),
            name: Some(name),
            original_slide_ids: slide_ids.clone(),
            slide_ids,
            raw_attributes: Vec::new(),
            slide_id_list_attributes: Vec::new(),
            slide_id_list_raw_children: OrderedRawChildren::default(),
            slide_id_sidecars: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }

    /// Maps copied slide identities while retaining each source slide's XML sidecar.
    /// Unselected source slots remain absent from reconciliation of the copied list.
    pub fn remap_slide_ids(&mut self, ids: &std::collections::HashMap<u32, u32>) -> Result<()> {
        let mapped = self
            .slide_ids
            .iter()
            .filter_map(|id| ids.get(id).copied())
            .collect::<Vec<_>>();
        let mut used = std::collections::HashSet::new();
        for &id in &mapped {
            validate_slide_id(id)?;
            if !used.insert(id) {
                return Err(invalid_value("duplicate copied section slide id".into()));
            }
        }
        for id in &mut self.original_slide_ids {
            // Zero is outside the valid slide-id range and cannot match a copied slot.
            *id = ids.get(id).copied().unwrap_or(0);
        }
        self.slide_id_sidecars.retain_mut(|sidecar| {
            if let Some(&id) = ids.get(&sidecar.id) {
                sidecar.id = id;
                true
            } else {
                false
            }
        });
        self.slide_ids = mapped;
        Ok(())
    }
}

impl CT_SlideMasterId {
    pub fn new(id: u32, relationship_id: impl Into<String>) -> Self {
        Self {
            id: Some(id),
            relationship_id: relationship_id.into(),
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        }
    }
}

/// The four PresentationML embedded face slots, in schema order.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EmbeddedFontKind {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl EmbeddedFontKind {
    pub const ALL: [Self; 4] = [Self::Regular, Self::Bold, Self::Italic, Self::BoldItalic];

    pub fn element_name(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::BoldItalic => "boldItalic",
        }
    }

    fn from_name(name: &[u8]) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.element_name().as_bytes() == name)
    }
}

/// One supported embedded-font description. Untouched source stays lexical XML.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CT_EmbeddedFontListEntry {
    typeface: String,
    panose: Option<String>,
    pitch_family: Option<i8>,
    charset: Option<i8>,
    faces: [Option<String>; 4],
    raw: Vec<u8>,
    namespaces: Vec<(String, String)>,
}

impl CT_EmbeddedFontListEntry {
    pub fn new(typeface: impl Into<String>) -> Result<Self> {
        let typeface = typeface.into();
        reject_non_xml_characters("embedded font typeface", &typeface)?;
        if typeface.trim().is_empty() {
            return Err(invalid_value("embedded font typeface is empty".into()));
        }
        let escaped = quick_xml::escape::escape(&typeface)
            .replace('\r', "&#13;")
            .replace('\n', "&#10;")
            .replace('\t', "&#9;");
        let raw = format!(
            r#"<p:embeddedFont xmlns:p="{P_NS}"><p:font typeface="{escaped}"/></p:embeddedFont>"#
        )
        .into_bytes();
        parse_font_entry(&raw, &NamespaceBindings::default())
    }

    pub fn typeface(&self) -> &str {
        &self.typeface
    }
    pub fn panose(&self) -> Option<&str> {
        self.panose.as_deref()
    }
    pub fn pitch_family(&self) -> Option<i8> {
        self.pitch_family
    }
    pub fn charset(&self) -> Option<i8> {
        self.charset
    }
    pub fn face(&self, kind: EmbeddedFontKind) -> Option<&str> {
        self.faces[kind as usize].as_deref()
    }
    pub fn faces(&self) -> impl Iterator<Item = (EmbeddedFontKind, &str)> {
        EmbeddedFontKind::ALL
            .into_iter()
            .filter_map(|kind| self.face(kind).map(|id| (kind, id)))
    }

    /// Replaces only the owned relationship attribute, preserving its aliases and payload.
    pub fn set_face(
        &mut self,
        kind: EmbeddedFontKind,
        relationship_id: impl Into<String>,
    ) -> Result<Option<String>> {
        let relationship_id = relationship_id.into();
        reject_non_xml_characters("embedded font relationship id", &relationship_id)?;
        validate_font_relationship_id(&relationship_id)?;
        let previous = self.face(kind).map(str::to_owned);
        if previous.as_deref() == Some(&relationship_id) {
            return Ok(previous);
        }
        let scope = NamespaceBindings::from_entries(&self.namespaces);
        let mut parts = font_element_parts(&self.raw)?;
        let mut existing = None;
        let mut insertion = parts.children.len();
        for (index, child) in parts.children.iter().enumerate() {
            if let Some((name, _)) = font_child_start(child, &scope)?
                && let Some(current) = EmbeddedFontKind::from_name(&name)
            {
                if current == kind {
                    existing = Some(index);
                }
                if (current as usize) > kind as usize {
                    insertion = insertion.min(index);
                }
            }
        }
        if let Some(index) = existing {
            let mut reader = Reader::from_reader(parts.children[index].as_slice());
            let start = match reader.read_event()? {
                Event::Start(start) | Event::Empty(start) => start,
                _ => unreachable!(),
            };
            let child_scope = scope.with_start(&start)?;
            let owned = all_attributes(&start)?
                .into_iter()
                .find(|(key, _)| {
                    local_name(key.as_bytes()) == b"id"
                        && child_scope.attribute_uri(key.as_bytes()) == Some(R_NS)
                })
                .ok_or_else(|| missing_attribute("embedded font face", "r:id"))?
                .0;
            crate::namespace::set_non_visual_drawing_text(
                &mut parts.children[index],
                &owned,
                Some(&relationship_id),
            )?;
        } else {
            let escaped = quick_xml::escape::escape(&relationship_id);
            let child = format!(
                r#"<p:{} xmlns:p="{P_NS}" xmlns:r="{R_NS}" r:id="{escaped}"/>"#,
                kind.element_name()
            )
            .into_bytes();
            parts.children.insert(insertion, child);
        }
        let candidate = parse_font_entry(&parts.join(), &scope)?;
        *self = candidate;
        Ok(previous)
    }

    /// Refuses to discard producer metadata or payload attached to an owned face.
    pub fn remove_face(&mut self, kind: EmbeddedFontKind) -> Result<Option<String>> {
        let previous = self.face(kind).map(str::to_owned);
        if previous.is_none() {
            return Ok(None);
        }
        let scope = NamespaceBindings::from_entries(&self.namespaces);
        let mut parts = font_element_parts(&self.raw)?;
        for index in 0..parts.children.len() {
            if let Some((name, child_scope)) = font_child_start(&parts.children[index], &scope)?
                && name == kind.element_name().as_bytes()
            {
                let face_parts = font_element_parts(&parts.children[index])?;
                let mut reader = Reader::from_reader(parts.children[index].as_slice());
                let start = match reader.read_event()? {
                    Event::Start(start) | Event::Empty(start) => start,
                    _ => unreachable!(),
                };
                let opaque_attributes = all_attributes(&start)?.iter().any(|(key, _)| {
                    !(key == "xmlns" || key.starts_with("xmlns:"))
                        && !(local_name(key.as_bytes()) == b"id"
                            && child_scope.attribute_uri(key.as_bytes()) == Some(R_NS))
                });
                if opaque_attributes
                    || face_parts
                        .children
                        .iter()
                        .any(|child| !child.iter().all(u8::is_ascii_whitespace))
                {
                    return Err(invalid_value(
                        "removing embedded font face would discard opaque source".into(),
                    ));
                }
                parts.children.remove(index);
                *self = parse_font_entry(&parts.join(), &scope)?;
                return Ok(previous);
            }
        }
        Err(invalid_value(
            "embedded font face projection is inconsistent".into(),
        ))
    }

    /// Carries one missing imported face, retaining its source payload and namespaces.
    /// The caller has already admitted equal typeface descriptions and font bytes.
    pub fn import_face(
        &mut self,
        source: &Self,
        kind: EmbeddedFontKind,
        relationship_id: &str,
    ) -> Result<()> {
        if source.face(kind).is_none()
            || self.typeface != source.typeface
            || !self.import_description_matches(source)?
        {
            return Err(invalid_value(
                "embedded font description conflicts with destination".into(),
            ));
        }
        let previous = self.face(kind).map(str::to_owned);
        if previous.as_deref().is_some_and(|id| id != relationship_id) {
            return Err(invalid_value(
                "embedded font face import would overwrite destination".into(),
            ));
        }
        let destination = NamespaceBindings::from_entries(&self.namespaces);
        let mut source = source.clone().into_namespace_context(&destination)?;
        source.set_face(kind, relationship_id)?;
        let source_scope = NamespaceBindings::from_entries(&source.namespaces);
        let source_parts = font_element_parts(&source.raw)?;
        let mut child = source_parts
            .children
            .into_iter()
            .find_map(|child| match font_child_start(&child, &source_scope) {
                Ok(Some((name, _))) if name == kind.element_name().as_bytes() => Some(Ok(child)),
                Err(error) => Some(Err(error)),
                _ => None,
            })
            .transpose()?
            .ok_or_else(|| invalid_value("source embedded font face is absent".into()))?;
        let original_names = font_expanded_names(&child, &source_scope)?;
        let mut reader = Reader::from_reader(child.as_slice());
        let start = match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => start,
            _ => unreachable!(),
        };
        let local = all_attributes(&start)?;
        let mut bindings = source.namespaces.clone();
        if !bindings.iter().any(|(prefix, _)| prefix.is_empty()) {
            bindings.push((String::new(), String::new()));
        }
        for (prefix, uri) in bindings {
            let key = if prefix.is_empty() {
                "xmlns".to_owned()
            } else {
                format!("xmlns:{prefix}")
            };
            if !local.iter().any(|(name, _)| name == &key)
                && !destination
                    .entries()
                    .iter()
                    .any(|(name, value)| name == &prefix && value == &uri)
            {
                crate::namespace::set_non_visual_drawing_text(&mut child, &key, Some(&uri))?;
            }
        }
        if font_expanded_names(&child, &destination)? != original_names {
            return Err(invalid_value(
                "imported embedded face would change namespace ownership".into(),
            ));
        }
        if previous.is_some() {
            let mut left_plain = self.clone();
            let mut right_plain = source.clone();
            if left_plain.remove_face(kind).is_ok() && right_plain.remove_face(kind).is_ok() {
                return Ok(());
            }
            let parts = font_element_parts(&self.raw)?;
            let existing = parts
                .children
                .into_iter()
                .find_map(|child| match font_child_start(&child, &destination) {
                    Ok(Some((name, _))) if name == kind.element_name().as_bytes() => {
                        Some(Ok(child))
                    }
                    Err(error) => Some(Err(error)),
                    _ => None,
                })
                .transpose()?
                .ok_or_else(|| invalid_value("destination embedded face is absent".into()))?;
            let strip_declarations = |mut xml: Vec<u8>| -> Result<Vec<u8>> {
                let mut reader = Reader::from_reader(xml.as_slice());
                let start = match reader.read_event()? {
                    Event::Start(start) | Event::Empty(start) => start,
                    _ => unreachable!(),
                };
                let attributes = all_attributes(&start)?;
                for (key, _) in attributes
                    .into_iter()
                    .filter(|(key, _)| key == "xmlns" || key.starts_with("xmlns:"))
                {
                    crate::namespace::set_non_visual_drawing_text(&mut xml, &key, None)?;
                }
                Ok(xml)
            };
            if font_expanded_names(&existing, &destination)? != original_names
                || font_opaque_namespace_context(&existing, &destination)?
                    != font_opaque_namespace_context(&child, &destination)?
                || strip_declarations(existing)? != strip_declarations(child)?
            {
                return Err(invalid_value(
                    "embedded font face opaque payload conflicts with destination".into(),
                ));
            }
            return Ok(());
        }
        let mut parts = font_element_parts(&self.raw)?;
        let mut insertion = parts.children.len();
        for (index, old) in parts.children.iter().enumerate() {
            if let Some((name, _)) = font_child_start(old, &destination)?
                && let Some(current) = EmbeddedFontKind::from_name(&name)
                && (current as usize) > kind as usize
            {
                insertion = insertion.min(index);
            }
        }
        parts.children.insert(insertion, child);
        *self = parse_font_entry(&parts.join(), &destination)?;
        Ok(())
    }

    /// Checks complete description and non-face metadata before merging face graphs.
    fn import_description_matches(&self, source: &Self) -> Result<bool> {
        let description = |entry: &Self| -> Result<Vec<u8>> {
            let scope = NamespaceBindings::from_entries(&entry.namespaces);
            let mut parts = font_element_parts(&entry.raw)?;
            let mut retained = Vec::new();
            for child in parts.children {
                if font_child_start(&child, &scope)?
                    .is_some_and(|(name, _)| EmbeddedFontKind::from_name(&name).is_some())
                {
                    continue;
                }
                retained.push(child);
            }
            parts.children = retained;
            Ok(parts.join())
        };
        let left = description(self)?;
        let right = description(source)?;
        let left_scope = NamespaceBindings::from_entries(&self.namespaces);
        let right_scope = NamespaceBindings::from_entries(&source.namespaces);
        let left_model = parse_font_entry(&left, &left_scope)?;
        let right_model = parse_font_entry(&right, &right_scope)?;
        if !left_model.has_opaque_source()? && !right_model.has_opaque_source()? {
            return Ok(self.typeface == source.typeface
                && self.panose == source.panose
                && self.pitch_family == source.pitch_family
                && self.charset == source.charset);
        }
        if font_expanded_names(&left, &left_scope)? != font_expanded_names(&right, &right_scope)?
            || font_opaque_namespace_context(&left, &left_scope)?
                != font_opaque_namespace_context(&right, &right_scope)?
        {
            return Ok(false);
        }
        let strip = |mut xml: Vec<u8>| -> Result<Vec<u8>> {
            let mut reader = Reader::from_reader(xml.as_slice());
            let start = match reader.read_event()? {
                Event::Start(start) | Event::Empty(start) => start,
                _ => unreachable!(),
            };
            for (key, _) in all_attributes(&start)?
                .into_iter()
                .filter(|(key, _)| key == "xmlns" || key.starts_with("xmlns:"))
            {
                crate::namespace::set_non_visual_drawing_text(&mut xml, &key, None)?;
            }
            Ok(xml)
        };
        Ok(strip(left)? == strip(right)?)
    }

    fn into_namespace_context(mut self, destination: &NamespaceBindings) -> Result<Self> {
        let source = NamespaceBindings::from_entries(&self.namespaces);
        let original_names = font_expanded_names(&self.raw, &source)?;
        let mut reader = Reader::from_reader(self.raw.as_slice());
        let start = match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => start,
            _ => unreachable!(),
        };
        let local = all_attributes(&start)?;
        let destination_entries = destination.entries();
        let mut bindings = self.namespaces.clone();
        if !bindings.iter().any(|(prefix, _)| prefix.is_empty()) {
            bindings.push((String::new(), String::new()));
        }
        for (prefix, uri) in bindings {
            let key = if prefix.is_empty() {
                "xmlns".to_owned()
            } else {
                format!("xmlns:{prefix}")
            };
            if local.iter().any(|(name, _)| name == &key) {
                continue;
            }
            let inherited = destination_entries
                .iter()
                .find(|(name, _)| name == &prefix)
                .map(|(_, value)| value.as_str());
            if inherited == Some(uri.as_str())
                || (prefix.is_empty() && inherited.is_none() && uri.is_empty())
            {
                continue;
            }
            crate::namespace::set_non_visual_drawing_text(&mut self.raw, &key, Some(&uri))?;
        }
        if font_expanded_names(&self.raw, destination)? != original_names {
            return Err(invalid_value(
                "embedded font transfer would change opaque namespace ownership".into(),
            ));
        }
        parse_font_entry(&self.raw, destination)
    }

    fn has_opaque_source(&self) -> Result<bool> {
        let scope = NamespaceBindings::from_entries(&self.namespaces);
        let parts = font_element_parts(&self.raw)?;
        let mut reader = Reader::from_reader(self.raw.as_slice());
        let start = match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => start,
            _ => unreachable!(),
        };
        if all_attributes(&start)?
            .iter()
            .any(|(key, _)| !(key == "xmlns" || key.starts_with("xmlns:")))
        {
            return Ok(true);
        }
        for child in parts.children {
            if child.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let Some((name, child_scope)) = font_child_start(&child, &scope)? else {
                return Ok(true);
            };
            let mut reader = Reader::from_reader(child.as_slice());
            let start = match reader.read_event()? {
                Event::Start(start) | Event::Empty(start) => start,
                _ => return Ok(true),
            };
            let allowed = if name == b"font" {
                vec!["typeface", "panose", "pitchFamily", "charset"]
            } else if EmbeddedFontKind::from_name(&name).is_some() {
                Vec::new()
            } else {
                return Ok(true);
            };
            if all_attributes(&start)?.iter().any(|(key, _)| {
                !(key == "xmlns" || key.starts_with("xmlns:"))
                    && !allowed.contains(&key.as_str())
                    && !(name != b"font"
                        && local_name(key.as_bytes()) == b"id"
                        && child_scope.attribute_uri(key.as_bytes()) == Some(R_NS))
            }) {
                return Ok(true);
            }
            if font_element_parts(&child)?
                .children
                .iter()
                .any(|raw| !raw.iter().all(u8::is_ascii_whitespace))
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EmbeddedFontListChild {
    Entry(CT_EmbeddedFontListEntry),
    Opaque(Vec<u8>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EmbeddedFontList {
    original: Vec<u8>,
    start: Vec<u8>,
    end: Vec<u8>,
    children: Vec<EmbeddedFontListChild>,
    namespaces: Vec<(String, String)>,
    dirty: bool,
}

impl EmbeddedFontList {
    fn xml(&self) -> Vec<u8> {
        if !self.dirty {
            return self.original.clone();
        }
        let mut xml = self.start.clone();
        for child in &self.children {
            match child {
                EmbeddedFontListChild::Entry(entry) => xml.extend_from_slice(&entry.raw),
                EmbeddedFontListChild::Opaque(raw) => xml.extend_from_slice(raw),
            }
        }
        xml.extend_from_slice(&self.end);
        xml
    }

    fn matching_typefaces(&self, typeface: &str) -> Result<usize> {
        let inherited = NamespaceBindings::from_entries(&self.namespaces);
        let mut count = 0;
        for child in &self.children {
            let raw = match child {
                EmbeddedFontListChild::Entry(entry) => &entry.raw,
                EmbeddedFontListChild::Opaque(raw) => raw,
            };
            if let Some((name, scope)) = font_child_start(raw, &inherited)?
                && name == b"embeddedFont"
            {
                for item in font_element_parts(raw)?.children {
                    if font_child_start(&item, &scope)?.is_some_and(|(name, _)| name == b"font") {
                        let mut reader = Reader::from_reader(item.as_slice());
                        if let Event::Start(start) | Event::Empty(start) = reader.read_event()?
                            && font_attribute(&start, "typeface")?.as_deref() == Some(typeface)
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        Ok(count)
    }
}

struct FontElementParts {
    start: Vec<u8>,
    children: Vec<Vec<u8>>,
    end: Vec<u8>,
}

impl FontElementParts {
    fn join(&self) -> Vec<u8> {
        let mut xml = self.start.clone();
        for child in &self.children {
            xml.extend_from_slice(child);
        }
        xml.extend_from_slice(&self.end);
        xml
    }
}

fn font_element_parts(xml: &[u8]) -> Result<FontElementParts> {
    let mut reader = Reader::from_reader(xml);
    let event = reader.read_event()?;
    let (name, empty) = match event {
        Event::Start(start) => (start.name().as_ref().to_vec(), false),
        Event::Empty(start) => (start.name().as_ref().to_vec(), true),
        _ => return Err(invalid_value("missing font element start".into())),
    };
    let start_end = reader.buffer_position() as usize;
    if empty {
        let mut start = xml[..start_end - 2].to_vec();
        start.push(b'>');
        return Ok(FontElementParts {
            start,
            children: Vec::new(),
            end: [b"</".as_slice(), &name, b">"].concat(),
        });
    }
    let mut children = Vec::new();
    loop {
        let before = reader.buffer_position() as usize;
        match reader.read_event()? {
            Event::Start(start) => {
                reader.read_to_end(start.name())?;
            }
            Event::End(_) => {
                return Ok(FontElementParts {
                    start: xml[..start_end].to_vec(),
                    children,
                    end: xml[before..].to_vec(),
                });
            }
            Event::Eof => return Err(invalid_value("missing font element end".into())),
            _ => {}
        }
        children.push(xml[before..reader.buffer_position() as usize].to_vec());
    }
}

fn font_child_start(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<Option<(Vec<u8>, NamespaceBindings)>> {
    let mut reader = Reader::from_reader(xml);
    match reader.read_event()? {
        Event::Start(start) | Event::Empty(start) => {
            let scope = inherited.with_start(&start)?;
            Ok((scope.element_uri(start.name().as_ref()) == Some(P_NS))
                .then(|| (local_name(start.name().as_ref()).to_vec(), scope)))
        }
        _ => Ok(None),
    }
}

fn validate_font_relationship_id(value: &str) -> Result<()> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err(invalid_value(
            "embedded font relationship id must be nonempty and contain no whitespace".into(),
        ));
    }
    Ok(())
}

fn font_exact_attributes(start: &BytesStart<'_>) -> Result<RawAttributes> {
    let mut attributes = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute?;
        attributes.push((
            std::str::from_utf8(attribute.key.as_ref())?.to_owned(),
            attribute
                .decoded_and_normalized_value(XmlVersion::Implicit1_0, start.decoder())?
                .into_owned(),
        ));
    }
    Ok(attributes)
}

/// Opaque values may refer to any inherited prefix. Preserve the complete
/// decoded context instead of guessing which producer strings are QNames.
fn font_opaque_namespace_context(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<Vec<Vec<(String, String)>>> {
    let mut reader = Reader::from_reader(xml);
    let mut scopes = vec![inherited.clone()];
    let mut contexts = Vec::new();
    loop {
        let event = reader.read_event()?;
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(start) | Event::Empty(start) => {
                let scope = scopes
                    .last()
                    .ok_or_else(|| invalid_value("missing embedded font namespace scope".into()))?
                    .with_start(&start)?;
                let mut entries = scope.entries();
                if !entries.iter().any(|(prefix, _)| prefix.is_empty()) {
                    entries.push((String::new(), String::new()));
                }
                entries.sort();
                contexts.push(entries);
                if !empty {
                    scopes.push(scope);
                }
            }
            Event::End(_) => {
                scopes.pop();
            }
            Event::Eof => return Ok(contexts),
            _ => {}
        }
    }
}

fn font_expanded_names(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<Vec<(Vec<u8>, Option<String>)>> {
    let mut reader = Reader::from_reader(xml);
    let mut scopes = vec![inherited.clone()];
    let mut names = Vec::new();
    loop {
        let event = reader.read_event()?;
        let empty = matches!(&event, Event::Empty(_));
        match event {
            Event::Start(start) | Event::Empty(start) => {
                let scope = scopes
                    .last()
                    .ok_or_else(|| invalid_value("missing embedded font namespace scope".into()))?
                    .with_start(&start)?;
                names.push((
                    start.name().as_ref().to_vec(),
                    scope
                        .element_uri(start.name().as_ref())
                        .filter(|uri| !uri.is_empty())
                        .map(str::to_owned),
                ));
                for (key, _) in font_exact_attributes(&start)? {
                    if key == "xmlns" || key.starts_with("xmlns:") {
                        continue;
                    }
                    names.push((
                        key.as_bytes().to_vec(),
                        scope
                            .attribute_uri(key.as_bytes())
                            .filter(|uri| !uri.is_empty())
                            .map(str::to_owned),
                    ));
                }
                if !empty {
                    scopes.push(scope);
                }
            }
            Event::End(_) => {
                scopes.pop();
            }
            Event::Eof => return Ok(names),
            _ => {}
        }
    }
}

fn font_attribute(start: &BytesStart<'_>, name: &str) -> Result<Option<String>> {
    for attribute in start.attributes() {
        let attribute = attribute?;
        if attribute.key.as_ref() == name.as_bytes() {
            return Ok(Some(
                attribute
                    .decoded_and_normalized_value(XmlVersion::Implicit1_0, start.decoder())?
                    .into_owned(),
            ));
        }
    }
    Ok(None)
}

fn parse_font_entry(xml: &[u8], inherited: &NamespaceBindings) -> Result<CT_EmbeddedFontListEntry> {
    let Some((name, scope)) = font_child_start(xml, inherited)? else {
        return Err(invalid_value("foreign embedded font entry".into()));
    };
    if name != b"embeddedFont" {
        return Err(invalid_value("not an embedded font entry".into()));
    }
    let parts = font_element_parts(xml)?;
    let mut description = None;
    let mut faces = [None, None, None, None];
    let mut boundary = 0;
    for child in parts.children {
        let Some((name, child_scope)) = font_child_start(&child, &scope)? else {
            continue;
        };
        let mut reader = Reader::from_reader(child.as_slice());
        let start = match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => start,
            _ => unreachable!(),
        };
        let attributes = font_exact_attributes(&start)?;
        if name == b"font" {
            if description.is_some() || boundary > 0 {
                return Err(duplicate("embedded font description"));
            }
            let get = |key: &str| font_attribute(&start, key);
            let typeface =
                get("typeface")?.ok_or_else(|| missing_attribute("embedded font", "typeface"))?;
            if typeface.trim().is_empty() {
                return Err(invalid_value("embedded font typeface is empty".into()));
            }
            let pitch = get("pitchFamily")?
                .map(|value| {
                    value
                        .parse::<i8>()
                        .map_err(|_| invalid_value("invalid font pitchFamily".into()))
                })
                .transpose()?;
            let charset = get("charset")?
                .map(|value| {
                    value
                        .parse::<i8>()
                        .map_err(|_| invalid_value("invalid font charset".into()))
                })
                .transpose()?;
            description = Some((typeface, get("panose")?, pitch, charset));
            boundary = 1;
        } else if let Some(kind) = EmbeddedFontKind::from_name(&name) {
            if description.is_none()
                || boundary > kind as usize + 1
                || faces[kind as usize].is_some()
            {
                return Err(invalid_value(
                    "ambiguous or out-of-order embedded font face".into(),
                ));
            }
            let ids = attributes
                .iter()
                .filter(|(key, _)| {
                    local_name(key.as_bytes()) == b"id"
                        && child_scope.attribute_uri(key.as_bytes()) == Some(R_NS)
                })
                .collect::<Vec<_>>();
            if ids.len() != 1 {
                return Err(invalid_value(
                    "missing or ambiguous embedded font face relationship".into(),
                ));
            }
            validate_font_relationship_id(&ids[0].1)?;
            faces[kind as usize] = Some(ids[0].1.clone());
            boundary = kind as usize + 2;
        }
    }
    let (typeface, panose, pitch_family, charset) =
        description.ok_or_else(|| missing_attribute("embedded font", "p:font"))?;
    Ok(CT_EmbeddedFontListEntry {
        typeface,
        panose,
        pitch_family,
        charset,
        faces,
        raw: xml.to_vec(),
        namespaces: scope.entries(),
    })
}

fn parse_font_list(xml: &[u8], inherited: &NamespaceBindings) -> Result<EmbeddedFontList> {
    let Some((name, scope)) = font_child_start(xml, inherited)? else {
        return Err(invalid_value("foreign embedded font list".into()));
    };
    if name != b"embeddedFontLst" {
        return Err(invalid_value("not an embedded font list".into()));
    }
    let parts = font_element_parts(xml)?;
    let children = parts
        .children
        .into_iter()
        .map(|raw| match parse_font_entry(&raw, &scope) {
            Ok(entry) => EmbeddedFontListChild::Entry(entry),
            Err(_) => EmbeddedFontListChild::Opaque(raw),
        })
        .collect();
    Ok(EmbeddedFontList {
        original: xml.to_vec(),
        start: parts.start,
        end: parts.end,
        children,
        namespaces: scope.entries(),
        dirty: false,
    })
}

fn font_relationship_reference_count(xml: &[u8], relationship_id: &str) -> Result<usize> {
    let mut reader = Reader::from_reader(xml);
    let mut scopes = vec![NamespaceBindings::default()];
    let mut count = 0;
    loop {
        let event = reader.read_event()?;
        let is_empty = matches!(&event, Event::Empty(_));
        match event {
            Event::Start(start) | Event::Empty(start) => {
                let empty = is_empty;
                let scope = scopes
                    .last()
                    .ok_or_else(|| invalid_value("missing font reference scope".into()))?
                    .with_start(&start)?;
                count += font_exact_attributes(&start)?
                    .iter()
                    .filter(|(key, value)| {
                        scope.attribute_uri(key.as_bytes()) == Some(R_NS)
                            && value == relationship_id
                    })
                    .count();
                if !empty {
                    scopes.push(scope);
                }
            }
            Event::End(_) => {
                scopes.pop();
            }
            Event::Eof => return Ok(count),
            _ => {}
        }
    }
}

/// The typed shell for `/ppt/presentation.xml`.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CT_Presentation {
    pub slide_master_ids: Vec<CT_SlideMasterId>,
    pub slide_ids: Vec<CT_SlideId>,
    pub slide_size: Option<CT_SlideSize>,
    pub notes_size: CT_SlideSize,
    pub default_text_style: Option<CT_TextListStyle>,
    embedded_font_list: Option<EmbeddedFontList>,
    embedded_font_list_ambiguous: bool,
    sections: Vec<Section>,
    extension_list_raw: Option<Vec<u8>>,
    section_list_range: Option<(usize, usize)>,
    section_list_namespaces: Vec<(String, String)>,
    sections_dirty: bool,
    raw_attributes: Vec<(String, String)>,
    slide_master_list_present: bool,
    slide_master_list_attributes: Vec<(String, String)>,
    slide_master_list_raw_children: OrderedRawChildren,
    slide_id_list_present: bool,
    slide_id_list_attributes: Vec<(String, String)>,
    slide_id_list_raw_children: OrderedRawChildren,
    original_slide_relationship_ids: Vec<String>,
    raw_children: OrderedRawChildren,
}

#[derive(Default)]
struct PresentationParseState {
    raw_attributes: RawAttributes,
    slide_master_ids: Option<Vec<CT_SlideMasterId>>,
    slide_master_list_attributes: RawAttributes,
    slide_master_list_raw_children: OrderedRawChildren,
    slide_ids: Option<Vec<CT_SlideId>>,
    slide_id_list_attributes: RawAttributes,
    slide_id_list_raw_children: OrderedRawChildren,
    slide_size: Option<CT_SlideSize>,
    notes_size: Option<CT_SlideSize>,
    default_text_style: Option<CT_TextListStyle>,
    embedded_font_list: Option<EmbeddedFontList>,
    embedded_font_list_ambiguous: bool,
    sections: Vec<Section>,
    extension_list_raw: Option<Vec<u8>>,
    section_list_range: Option<(usize, usize)>,
    section_list_namespaces: Vec<(String, String)>,
    raw_children: OrderedRawChildren,
    boundary: usize,
}

impl PresentationParseState {
    fn capture_child(
        &mut self,
        name: &[u8],
        is_presentationml: bool,
        namespaces: &NamespaceBindings,
        raw: Vec<u8>,
    ) -> Result<()> {
        if !is_presentationml {
            self.raw_children.push(self.boundary, raw);
            return Ok(());
        }

        match name {
            b"sldMasterIdLst" => {
                namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                if self.slide_master_ids.is_some() {
                    return Err(duplicate("sldMasterIdLst"));
                }
                let (items, attributes, children) = parse_master_list(&raw, namespaces)?;
                self.slide_master_ids = Some(items);
                self.slide_master_list_attributes = attributes;
                self.slide_master_list_raw_children = children;
                self.boundary = self.boundary.max(1);
            }
            b"sldIdLst" => {
                namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                if self.slide_ids.is_some() {
                    return Err(duplicate("sldIdLst"));
                }
                let (items, attributes, children) = parse_slide_list(&raw, namespaces)?;
                self.slide_ids = Some(items);
                self.slide_id_list_attributes = attributes;
                self.slide_id_list_raw_children = children;
                self.boundary = self.boundary.max(4);
            }
            b"sldSz" => {
                namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                if self.slide_size.is_some() {
                    return Err(duplicate("sldSz"));
                }
                self.slide_size = Some(parse_size(&raw, "sldSz")?);
                self.boundary = self.boundary.max(5);
            }
            b"notesSz" => {
                namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                if self.notes_size.is_some() {
                    return Err(duplicate("notesSz"));
                }
                self.notes_size = Some(parse_size(&raw, "notesSz")?);
                self.boundary = self.boundary.max(6);
            }
            b"defaultTextStyle" => {
                namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                if self.default_text_style.is_some() {
                    return Err(duplicate("defaultTextStyle"));
                }
                self.default_text_style = Some(
                    CT_TextListStyle::from_xml_with_inherited_namespaces(
                        &raw,
                        &namespaces.entries(),
                    )
                    .map_err(|error| invalid_value(error.to_string()))?,
                );
                self.boundary = self.boundary.max(13);
            }
            b"embeddedFontLst" => {
                if self.embedded_font_list.is_some() {
                    self.embedded_font_list_ambiguous = true;
                    self.raw_children.push(8, raw);
                    self.boundary = self.boundary.max(8);
                    return Ok(());
                }
                self.embedded_font_list = Some(parse_font_list(&raw, namespaces)?);
                self.boundary = self.boundary.max(8);
            }
            b"extLst" => {
                if self.extension_list_raw.is_some() {
                    return Err(duplicate("extLst"));
                }
                let (sections, range, section_namespaces) =
                    parse_sections_from_extension(&raw, namespaces)?;
                self.sections = sections;
                self.section_list_range = range;
                self.section_list_namespaces = section_namespaces;
                self.extension_list_raw = Some(raw);
                self.boundary = self.boundary.max(15);
            }
            _ => {
                let slot = root_schema_boundary(name).unwrap_or(self.boundary);
                self.raw_children.push(slot, raw);
                self.boundary = self.boundary.max(slot);
            }
        }
        Ok(())
    }

    fn finish(self) -> Result<CT_Presentation> {
        let notes_size = self
            .notes_size
            .ok_or_else(|| OxmlError::MissingElement("p:notesSz".to_owned()))?;
        let original_slide_relationship_ids = self
            .slide_ids
            .as_ref()
            .map(|slides| {
                slides
                    .iter()
                    .map(|slide| slide.relationship_id.clone())
                    .collect()
            })
            .unwrap_or_default();
        let presentation = CT_Presentation {
            slide_master_list_present: self.slide_master_ids.is_some(),
            slide_master_ids: self.slide_master_ids.unwrap_or_default(),
            slide_id_list_present: self.slide_ids.is_some(),
            slide_ids: self.slide_ids.unwrap_or_default(),
            slide_size: self.slide_size,
            notes_size,
            default_text_style: self.default_text_style,
            embedded_font_list: self.embedded_font_list,
            embedded_font_list_ambiguous: self.embedded_font_list_ambiguous,
            sections: self.sections,
            extension_list_raw: self.extension_list_raw,
            section_list_range: self.section_list_range,
            section_list_namespaces: self.section_list_namespaces,
            sections_dirty: false,
            raw_attributes: self.raw_attributes,
            slide_master_list_attributes: self.slide_master_list_attributes,
            slide_master_list_raw_children: self.slide_master_list_raw_children,
            slide_id_list_attributes: self.slide_id_list_attributes,
            slide_id_list_raw_children: self.slide_id_list_raw_children,
            original_slide_relationship_ids,
            raw_children: self.raw_children,
        };
        presentation.validate()?;
        Ok(presentation)
    }
}

impl CT_Presentation {
    pub fn new(notes_size: CT_SlideSize) -> Self {
        Self {
            slide_master_ids: Vec::new(),
            slide_ids: Vec::new(),
            slide_size: None,
            notes_size,
            default_text_style: None,
            embedded_font_list: None,
            embedded_font_list_ambiguous: false,
            sections: Vec::new(),
            extension_list_raw: None,
            section_list_range: None,
            section_list_namespaces: Vec::new(),
            sections_dirty: false,
            raw_attributes: Vec::new(),
            slide_master_list_present: false,
            slide_master_list_attributes: Vec::new(),
            slide_master_list_raw_children: OrderedRawChildren::default(),
            slide_id_list_present: false,
            slide_id_list_attributes: Vec::new(),
            slide_id_list_raw_children: OrderedRawChildren::default(),
            original_slide_relationship_ids: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        }
    }

    /// Supported font entries in original list order. Unsupported source remains opaque.
    pub fn embedded_fonts(&self) -> impl Iterator<Item = &CT_EmbeddedFontListEntry> {
        self.embedded_font_list
            .iter()
            .flat_map(|list| list.children.iter())
            .filter_map(|child| match child {
                EmbeddedFontListChild::Entry(entry) => Some(entry),
                EmbeddedFontListChild::Opaque(_) => None,
            })
    }

    /// Refuses ambiguous producer typefaces rather than editing one arbitrary owner.
    pub fn embedded_font_mut(
        &mut self,
        typeface: &str,
    ) -> Result<Option<&mut CT_EmbeddedFontListEntry>> {
        if self.embedded_font_list_ambiguous {
            return Err(invalid_value(
                "ambiguous embedded font lists cannot be edited safely".into(),
            ));
        }
        let Some(list) = &mut self.embedded_font_list else {
            return Ok(None);
        };
        let matches = list.matching_typefaces(typeface)?;
        if matches > 1 {
            return Err(invalid_value("ambiguous embedded font typeface".into()));
        }
        for child in &mut list.children {
            if let EmbeddedFontListChild::Entry(entry) = child
                && entry.typeface == typeface
            {
                list.dirty = true;
                return Ok(Some(entry));
            }
        }
        if matches != 0 {
            return Err(invalid_value(
                "unsupported embedded font source cannot be edited safely".into(),
            ));
        }
        Ok(None)
    }

    pub fn push_embedded_font(&mut self, entry: CT_EmbeddedFontListEntry) -> Result<()> {
        if self.embedded_font_list_ambiguous {
            return Err(invalid_value(
                "ambiguous embedded font lists cannot be edited safely".into(),
            ));
        }
        if let Some(list) = &self.embedded_font_list
            && list.matching_typefaces(&entry.typeface)? != 0
        {
            return Err(invalid_value(
                "embedded font typeface already exists".into(),
            ));
        }
        let xml = self.to_xml()?;
        let mut reader = Reader::from_reader(xml.as_slice());
        let root = match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                NamespaceBindings::default().with_start(&start)?
            }
            _ => unreachable!(),
        };
        let destination = if let Some(list) = &self.embedded_font_list {
            font_child_start(&list.xml(), &root)?
                .ok_or_else(|| invalid_value("invalid embedded font list context".into()))?
                .1
        } else {
            root
        };
        let entry = entry.into_namespace_context(&destination)?;
        if self.embedded_font_list.is_none() {
            let raw = format!(r#"<p:embeddedFontLst xmlns:p="{P_NS}"/>"#);
            self.embedded_font_list = Some(parse_font_list(
                raw.as_bytes(),
                &NamespaceBindings::default(),
            )?);
        }
        let list = self
            .embedded_font_list
            .as_mut()
            .expect("font list initialized");
        list.children.push(EmbeddedFontListChild::Entry(entry));
        list.dirty = true;
        Ok(())
    }

    /// Leaves the list shell and all unrelated entries intact.
    pub fn remove_embedded_font(
        &mut self,
        typeface: &str,
    ) -> Result<Option<CT_EmbeddedFontListEntry>> {
        if self.embedded_font_list_ambiguous {
            return Err(invalid_value(
                "ambiguous embedded font lists cannot be edited safely".into(),
            ));
        }
        let Some(list) = &mut self.embedded_font_list else {
            return Ok(None);
        };
        let matches = list.matching_typefaces(typeface)?;
        if matches == 0 {
            return Ok(None);
        }
        if matches != 1 {
            return Err(invalid_value("ambiguous embedded font typeface".into()));
        }
        let index = list.children.iter().position(|child| matches!(child, EmbeddedFontListChild::Entry(entry) if entry.typeface == typeface)).ok_or_else(|| invalid_value("unsupported embedded font source cannot be removed safely".into()))?;
        if let EmbeddedFontListChild::Entry(entry) = &list.children[index]
            && entry.has_opaque_source()?
        {
            return Err(invalid_value(
                "removing embedded font would discard opaque source".into(),
            ));
        }
        let EmbeddedFontListChild::Entry(entry) = list.children.remove(index) else {
            unreachable!()
        };
        list.dirty = true;
        Ok(Some(entry))
    }

    /// Counts qualified relationships throughout presentation XML, including preserved source.
    pub fn embedded_font_reference_count(&self, relationship_id: &str) -> Result<usize> {
        font_relationship_reference_count(&self.to_xml()?, relationship_id)
    }

    /// Registers the single global notes master in its schema-ordered list.
    /// Existing owned lists must agree, and all foreign children remain verbatim.
    pub fn ensure_notes_master_relationship(&mut self, relationship_id: &str) -> Result<()> {
        if relationship_id.is_empty() {
            return Err(invalid_value("empty notes master relationship".into()));
        }
        let xml = self.to_xml()?;
        let mut reader = Reader::from_reader(xml.as_slice());
        let mut scopes = vec![NamespaceBindings::default()];
        let mut owned_list = None;
        let mut lists = 0;
        let mut references = Vec::new();
        loop {
            let event = reader.read_event()?;
            let empty = matches!(&event, Event::Empty(_));
            match event {
                Event::Start(start) | Event::Empty(start) => {
                    let scope = scopes
                        .last()
                        .expect("root namespace scope")
                        .with_start(&start)?;
                    let name = start.name();
                    if scopes.len() == 2
                        && scope.element_uri(name.as_ref()) == Some(P_NS)
                        && name.local_name().as_ref() == b"notesMasterIdLst"
                    {
                        lists += 1;
                        if !empty {
                            owned_list = Some(scopes.len());
                        }
                    } else if owned_list == Some(scopes.len() - 1)
                        && scope.element_uri(name.as_ref()) == Some(P_NS)
                        && name.local_name().as_ref() == b"notesMasterId"
                    {
                        let mut ids =
                            font_exact_attributes(&start)?
                                .into_iter()
                                .filter(|(key, _)| {
                                    key.as_bytes().rsplit(|byte| *byte == b':').next()
                                        == Some(b"id")
                                        && scope.attribute_uri(key.as_bytes()) == Some(R_NS)
                                });
                        let (_, id) = ids
                            .next()
                            .ok_or_else(|| invalid_value("notes master id lacks r:id".into()))?;
                        if ids.next().is_some() {
                            return Err(invalid_value("ambiguous notes master r:id".into()));
                        }
                        references.push(id);
                    }
                    if !empty {
                        scopes.push(scope);
                    }
                }
                Event::End(_) => {
                    scopes.pop();
                    if owned_list == Some(scopes.len()) {
                        owned_list = None;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        if lists > 0 {
            if lists != 1 || references != [relationship_id] {
                return Err(invalid_value(
                    "notes master list disagrees with its global relationship".into(),
                ));
            }
            return Ok(());
        }
        let id = quick_xml::escape::escape(relationship_id);
        self.raw_children.push(
            2,
            format!("<p:notesMasterIdLst><p:notesMasterId r:id=\"{id}\"/></p:notesMasterIdLst>")
                .into_bytes(),
        );
        Ok(())
    }

    pub fn embed_true_type_fonts(&self) -> Result<Option<bool>> {
        self.font_root_boolean("embedTrueTypeFonts")
    }
    pub fn save_subset_fonts(&self) -> Result<Option<bool>> {
        self.font_root_boolean("saveSubsetFonts")
    }
    pub fn set_embed_true_type_fonts(&mut self, value: Option<bool>) {
        self.set_font_root_boolean("embedTrueTypeFonts", value);
    }
    pub fn set_save_subset_fonts(&mut self, value: Option<bool>) {
        self.set_font_root_boolean("saveSubsetFonts", value);
    }

    fn font_root_boolean(&self, name: &str) -> Result<Option<bool>> {
        self.raw_attributes
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| match value.trim() {
                "1" | "true" => Ok(true),
                "0" | "false" => Ok(false),
                _ => Err(invalid_value(format!(
                    "invalid embedded font Boolean {name}: {value}"
                ))),
            })
            .transpose()
    }

    fn set_font_root_boolean(&mut self, name: &str, value: Option<bool>) {
        if self.font_root_boolean(name).ok() == Some(value) {
            return;
        }
        if let Some(index) = self.raw_attributes.iter().position(|(key, _)| key == name) {
            match value {
                Some(value) => self.raw_attributes[index].1 = if value { "1" } else { "0" }.into(),
                None => {
                    self.raw_attributes.remove(index);
                }
            }
        } else if let Some(value) = value {
            self.raw_attributes
                .push((name.into(), if value { "1" } else { "0" }.into()));
        }
    }

    /// Presence-sensitive displayed number of the first slide. Physical page
    /// ordinals remain independent. The schema permits an explicit zero.
    pub fn first_slide_number(&self) -> Result<Option<u32>> {
        self.raw_attributes
            .iter()
            .find(|(name, _)| name == "firstSlideNum")
            .map(|(_, value)| {
                value
                    .trim()
                    .parse::<u32>()
                    .map_err(|_| OxmlError::InvalidValue(format!("invalid firstSlideNum: {value}")))
            })
            .transpose()
    }

    /// Patches only the qualified root's unqualified displayed-number attribute.
    pub fn set_first_slide_number(&mut self, value: Option<u32>) {
        self.raw_attributes
            .retain(|(name, _)| name != "firstSlideNum");
        if let Some(value) = value {
            self.raw_attributes
                .push(("firstSlideNum".into(), value.to_string()));
        }
    }

    /// Retains presence of title-layout special-placeholder visibility policy.
    pub fn show_special_placeholders_on_title_slide(&self) -> Result<Option<bool>> {
        self.raw_attributes
            .iter()
            .find(|(name, _)| name == "showSpecialPlsOnTitleSld")
            .map(|(_, value)| match value.as_str() {
                "1" | "true" | "on" => Ok(true),
                "0" | "false" | "off" => Ok(false),
                _ => Err(OxmlError::InvalidValue(
                    "invalid title-placeholder visibility".into(),
                )),
            })
            .transpose()
    }

    /// Patches title-placeholder visibility, retaining every other root operand.
    pub fn set_show_special_placeholders_on_title_slide(&mut self, value: Option<bool>) {
        self.raw_attributes
            .retain(|(name, _)| name != "showSpecialPlsOnTitleSld");
        if let Some(value) = value {
            self.raw_attributes.push((
                "showSpecialPlsOnTitleSld".into(),
                if value { "1" } else { "0" }.into(),
            ));
        }
    }

    /// Replaces only the slide dimensions, preserving size kind and raw data.
    pub fn set_slide_size(&mut self, cx: Emu, cy: Emu) -> Result<()> {
        validate_dimension("sldSz", "cx", cx)?;
        validate_dimension("sldSz", "cy", cy)?;
        if let Some(size) = &mut self.slide_size {
            size.cx = cx;
            size.cy = cy;
        } else {
            self.slide_size = Some(CT_SlideSize::new(cx, cy)?);
        }
        Ok(())
    }

    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn set_sections(&mut self, sections: Vec<Section>) -> Result<()> {
        validate_sections(&sections, &self.slide_ids)?;
        if sections.is_empty()
            && let (Some(extension_list), Some((start, end))) =
                (&self.extension_list_raw, self.section_list_range)
            && section_list_has_raw_content(
                &extension_list[start..end],
                &self.section_list_namespaces,
            )?
        {
            return Err(invalid_value(
                "cannot clear sections while preserving direct section-list raw payload".to_owned(),
            ));
        }
        self.sections = sections;
        self.sections_dirty = true;
        Ok(())
    }

    pub fn remove_slide_from_sections(&mut self, slide_id: u32) {
        for section in &mut self.sections {
            let before = section.slide_ids.len();
            section.slide_ids.retain(|id| *id != slide_id);
            self.sections_dirty |= section.slide_ids.len() != before;
        }
    }

    /// Parses a complete PresentationML presentation root with any prefix.
    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer)? {
                Event::Start(element) => {
                    let namespaces = NamespaceBindings::default().with_start(&element)?;
                    if local_name(element.name().as_ref()) == b"presentation"
                        && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                    {
                        namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                        return Self::from_element(&mut reader, &element, &namespaces);
                    }
                    return Err(OxmlError::UnexpectedElement(element_name(&element)));
                }
                Event::Empty(element) => {
                    let namespaces = NamespaceBindings::default().with_start(&element)?;
                    if local_name(element.name().as_ref()) == b"presentation"
                        && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                    {
                        namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                        return Err(OxmlError::MissingElement("p:notesSz".to_owned()));
                    }
                    return Err(OxmlError::UnexpectedElement(element_name(&element)));
                }
                Event::Eof => {
                    return Err(OxmlError::MissingElement(
                        "PresentationML presentation".to_owned(),
                    ));
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    fn from_element(
        reader: &mut Reader<&[u8]>,
        start: &BytesStart<'_>,
        root_namespaces: &NamespaceBindings,
    ) -> Result<Self> {
        let mut state = PresentationParseState {
            raw_attributes: root_attributes(start, FIXED_MODEL_PREFIXES)?,
            ..PresentationParseState::default()
        };
        let mut buffer = Vec::new();

        loop {
            match reader.read_event_into(&mut buffer)? {
                Event::Start(element) => {
                    let name = local_name(element.name().as_ref()).to_vec();
                    let namespaces = root_namespaces.with_start(&element)?;
                    let is_presentationml =
                        namespaces.element_uri(element.name().as_ref()) == Some(P_NS);
                    let raw = capture_element(reader, &element)?;
                    state.capture_child(&name, is_presentationml, &namespaces, raw)?;
                }
                Event::Empty(element) => {
                    let name = local_name(element.name().as_ref()).to_vec();
                    let namespaces = root_namespaces.with_start(&element)?;
                    let is_presentationml =
                        namespaces.element_uri(element.name().as_ref()) == Some(P_NS);
                    let raw = capture_empty_element(&element)?;
                    state.capture_child(&name, is_presentationml, &namespaces, raw)?;
                }
                Event::End(element) if local_name(element.name().as_ref()) == b"presentation" => {
                    return state.finish();
                }
                Event::Eof => {
                    return Err(OxmlError::MissingElement(
                        "closing p:presentation".to_owned(),
                    ));
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    fn validate(&self) -> Result<()> {
        if let Some(size) = &self.slide_size {
            validate_size("sldSz", size)?;
        }
        validate_size("notesSz", &self.notes_size)
    }

    /// Writes fixed `p`, `a`, and `r` prefixes in presentation schema order.
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut writer = Writer::new(Vec::new());
        let mut root = BytesStart::new("p:presentation");
        root.push_attribute(("xmlns:p", P_NS));
        root.push_attribute(("xmlns:a", A_NS));
        root.push_attribute(("xmlns:r", R_NS));
        push_attributes(&mut root, &self.raw_attributes);
        writer.write_event(Event::Start(root))?;

        emit_raw(&mut writer, self.raw_children.at(0))?;
        if self.slide_master_list_present || !self.slide_master_ids.is_empty() {
            write_master_list(self, &mut writer)?;
        }
        emit_raw(&mut writer, self.raw_children.at(1))?;
        emit_raw(&mut writer, self.raw_children.at(2))?;
        emit_raw(&mut writer, self.raw_children.at(3))?;
        if self.slide_id_list_present || !self.slide_ids.is_empty() {
            write_slide_list(self, &mut writer)?;
        }
        emit_raw(&mut writer, self.raw_children.at(4))?;
        if let Some(size) = &self.slide_size {
            write_size(size, "p:sldSz", &mut writer)?;
        }
        emit_raw(&mut writer, self.raw_children.at(5))?;
        write_size(&self.notes_size, "p:notesSz", &mut writer)?;
        emit_raw(&mut writer, self.raw_children.at(6))?;
        emit_raw(&mut writer, self.raw_children.at(7))?;
        if let Some(fonts) = &self.embedded_font_list {
            writer.get_mut().write_all(&fonts.xml())?;
        }
        for boundary in 8..=12 {
            emit_raw(&mut writer, self.raw_children.at(boundary))?;
        }
        if let Some(style) = &self.default_text_style {
            style
                .write_xml_as(&mut writer, "p:defaultTextStyle")
                .map_err(|error| invalid_value(error.to_string()))?;
        }
        emit_raw(&mut writer, self.raw_children.at(13))?;
        emit_raw(&mut writer, self.raw_children.at(14))?;
        if let Some(extension_list) = write_extension_list(
            self.extension_list_raw.as_deref(),
            self.section_list_range,
            &self.sections,
            self.sections_dirty,
            &self.section_list_namespaces,
        )? {
            writer.get_mut().write_all(&extension_list)?;
        }
        emit_raw(&mut writer, self.raw_children.at(15))?;
        writer.write_event(Event::End(BytesEnd::new("p:presentation")))?;
        Ok(writer.into_inner())
    }

    /// Removes references to one presentation relationship from custom shows.
    pub fn remove_custom_show_slide(&mut self, relationship_id: &str) -> Result<()> {
        let xml = self.to_xml()?;
        let rewritten = remove_custom_show_slide_references(&xml, relationship_id)?;
        *self = Self::from_xml(&rewritten)?;
        Ok(())
    }
}

/// Returns relationship ids referenced by slides inside preserved custom shows.
pub fn custom_show_relationship_ids(xml: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut scopes = vec![NamespaceBindings::default()];
    let mut depth = 0usize;
    let mut custom_show_depth = None;
    let mut ids = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) => {
                let parent = scopes.last().ok_or_else(|| {
                    invalid_value("missing custom-show namespace scope".to_owned())
                })?;
                let scope = parent.with_start(&element)?;
                let is_custom_show = scope.element_uri(element.name().as_ref()) == Some(P_NS)
                    && local_name(element.name().as_ref()) == b"custShowLst";
                if custom_show_depth.is_some() || is_custom_show {
                    collect_custom_show_slide_id(&element, &scope, &mut ids)?;
                }
                scopes.push(scope);
                depth += 1;
                if is_custom_show {
                    custom_show_depth = Some(depth);
                }
            }
            Event::Empty(element) => {
                let parent = scopes.last().ok_or_else(|| {
                    invalid_value("missing custom-show namespace scope".to_owned())
                })?;
                let scope = parent.with_start(&element)?;
                if custom_show_depth.is_some() {
                    collect_custom_show_slide_id(&element, &scope, &mut ids)?;
                }
            }
            Event::End(_) => {
                if custom_show_depth == Some(depth) {
                    custom_show_depth = None;
                }
                if depth == 0 || scopes.len() == 1 {
                    return Err(invalid_value(
                        "custom-show XML has an unmatched closing tag".to_owned(),
                    ));
                }
                depth -= 1;
                scopes.pop();
            }
            Event::Eof => {
                if depth != 0 || scopes.len() != 1 {
                    return Err(invalid_value(
                        "custom-show XML ended before its root closed".to_owned(),
                    ));
                }
                return Ok(ids);
            }
            _ => {}
        }
        buffer.clear();
    }
}

fn collect_custom_show_slide_id(
    element: &BytesStart<'_>,
    scope: &NamespaceBindings,
    ids: &mut Vec<String>,
) -> Result<()> {
    if scope.element_uri(element.name().as_ref()) != Some(P_NS)
        || local_name(element.name().as_ref()) != b"sld"
    {
        return Ok(());
    }
    for attribute in element.attributes() {
        let attribute = attribute?;
        if scope.attribute_uri(attribute.key.as_ref()) == Some(R_NS)
            && local_name(attribute.key.as_ref()) == b"id"
        {
            ids.push(
                attribute
                    .decoded_and_normalized_value(XmlVersion::Implicit1_0, element.decoder())?
                    .into_owned(),
            );
        }
    }
    Ok(())
}

fn remove_custom_show_slide_references(xml: &[u8], relationship_id: &str) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut scopes = vec![NamespaceBindings::default()];
    let mut depth = 0usize;
    let mut custom_show_depth = None;
    let mut removals = Vec::new();
    loop {
        let event_start = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) => {
                let parent = scopes.last().ok_or_else(|| {
                    invalid_value("missing custom-show namespace scope".to_owned())
                })?;
                let scope = parent.with_start(&element)?;
                let is_custom_show = scope.element_uri(element.name().as_ref()) == Some(P_NS)
                    && local_name(element.name().as_ref()) == b"custShowLst";
                if custom_show_depth.is_some()
                    && custom_show_slide_relationship_id(&element, &scope)?.as_deref()
                        == Some(relationship_id)
                {
                    reader.read_to_end(element.name())?;
                    removals.push(event_start..reader.buffer_position() as usize);
                } else {
                    scopes.push(scope);
                    depth += 1;
                    if is_custom_show {
                        custom_show_depth = Some(depth);
                    }
                }
            }
            Event::Empty(element) => {
                let parent = scopes.last().ok_or_else(|| {
                    invalid_value("missing custom-show namespace scope".to_owned())
                })?;
                let scope = parent.with_start(&element)?;
                if custom_show_depth.is_some()
                    && custom_show_slide_relationship_id(&element, &scope)?.as_deref()
                        == Some(relationship_id)
                {
                    removals.push(event_start..reader.buffer_position() as usize);
                }
            }
            Event::End(_) => {
                if custom_show_depth == Some(depth) {
                    custom_show_depth = None;
                }
                if depth == 0 || scopes.len() == 1 {
                    return Err(invalid_value(
                        "custom-show XML has an unmatched closing tag".to_owned(),
                    ));
                }
                depth -= 1;
                scopes.pop();
            }
            Event::Eof => break,
            _ => {}
        }
        buffer.clear();
    }
    if depth != 0 || scopes.len() != 1 {
        return Err(invalid_value(
            "custom-show XML ended before its root closed".to_owned(),
        ));
    }
    let mut rewritten = xml.to_vec();
    for range in removals.into_iter().rev() {
        rewritten.drain(range);
    }
    Ok(rewritten)
}

fn custom_show_slide_relationship_id(
    element: &BytesStart<'_>,
    scope: &NamespaceBindings,
) -> Result<Option<String>> {
    if scope.element_uri(element.name().as_ref()) != Some(P_NS)
        || local_name(element.name().as_ref()) != b"sld"
    {
        return Ok(None);
    }
    for attribute in element.attributes() {
        let attribute = attribute?;
        if scope.attribute_uri(attribute.key.as_ref()) == Some(R_NS)
            && local_name(attribute.key.as_ref()) == b"id"
        {
            return Ok(Some(
                attribute
                    .decoded_and_normalized_value(XmlVersion::Implicit1_0, element.decoder())?
                    .into_owned(),
            ));
        }
    }
    Ok(None)
}

fn parse_sections_from_extension(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<ParsedSectionList> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut scopes = vec![inherited.clone()];
    let mut depth = 0usize;
    let mut section_extension_depth = None;
    loop {
        let start_position = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) => {
                let parent = scopes
                    .last()
                    .ok_or_else(|| invalid_value("missing section namespace scope".to_owned()))?;
                let scope = parent.with_start(&element)?;
                if depth == 1
                    && local_name(element.name().as_ref()) == b"ext"
                    && scope.element_uri(element.name().as_ref()) == Some(P_NS)
                    && all_attributes(&element)?
                        .into_iter()
                        .any(|(name, value)| name == "uri" && value == SECTION_EXTENSION_URI)
                {
                    section_extension_depth = Some(depth + 1);
                }
                if section_extension_depth == Some(depth)
                    && local_name(element.name().as_ref()) == b"sectionLst"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    let raw = capture_element(&mut reader, &element)?;
                    let section_namespaces = scope
                        .entries()
                        .into_iter()
                        .filter(|(prefix, _)| raw_uses_namespace_prefix(&raw, prefix))
                        .collect();
                    let end_position = reader.buffer_position() as usize;
                    return Ok((
                        parse_section_list(&raw, &scope)?,
                        Some((start_position, end_position)),
                        section_namespaces,
                    ));
                }
                scopes.push(scope);
                depth += 1;
            }
            Event::Empty(element) => {
                let parent = scopes
                    .last()
                    .ok_or_else(|| invalid_value("missing section namespace scope".to_owned()))?;
                let scope = parent.with_start(&element)?;
                if section_extension_depth == Some(depth)
                    && local_name(element.name().as_ref()) == b"sectionLst"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    return Err(OxmlError::MissingElement(
                        "p14:sectionLst requires p14:section".to_owned(),
                    ));
                }
            }
            Event::End(_) => {
                if section_extension_depth == Some(depth) {
                    section_extension_depth = None;
                }
                if scopes.len() > 1 {
                    scopes.pop();
                    depth -= 1;
                }
            }
            Event::Eof => return Ok((Vec::new(), None, Vec::new())),
            _ => {}
        }
        buffer.clear();
    }
}

fn parse_section_list(xml: &[u8], inherited: &NamespaceBindings) -> Result<Vec<Section>> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut root_scope = inherited.clone();
    let mut sections = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == b"sectionLst" => {
                root_scope = inherited.with_start(&element)?;
            }
            Event::Start(element) => {
                let scope = root_scope.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"section"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    sections.push(parse_section(&raw, &scope)?);
                }
            }
            Event::Empty(element) => {
                let scope = root_scope.with_start(&element)?;
                if local_name(element.name().as_ref()) == b"section"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    return Err(OxmlError::MissingElement(
                        "p14:section requires p14:sldIdLst".to_owned(),
                    ));
                }
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sectionLst" => break,
            Event::Eof => {
                return Err(OxmlError::MissingElement(
                    "closing p14:sectionLst".to_owned(),
                ));
            }
            _ => {}
        }
        buffer.clear();
    }
    if sections.is_empty() {
        return Err(OxmlError::MissingElement(
            "p14:sectionLst requires p14:section".to_owned(),
        ));
    }
    Ok(sections)
}

fn parse_section(xml: &[u8], inherited: &NamespaceBindings) -> Result<Section> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut id = None;
    let mut name = None;
    let mut raw_attributes = Vec::new();
    let mut slide_ids = None;
    let mut slide_id_list_attributes = Vec::new();
    let mut slide_id_list_raw_children = OrderedRawChildren::default();
    let mut slide_id_sidecars = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut section_scope = inherited.clone();
    let mut boundary = 0;
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == b"section" => {
                section_scope = inherited.with_start(&element)?;
                for (attribute, value) in all_attributes(&element)? {
                    match attribute.as_str() {
                        "id" if id.is_none() => id = Some(value),
                        "name" if name.is_none() => name = Some(value),
                        "id" | "name" => return Err(duplicate("section attribute")),
                        _ => {}
                    }
                }
                raw_attributes = lexical_attributes(&element, &["id", "name"])?;
                raw_attributes.extend_from_slice(&dependent_p14_shadow_attributes(&element, xml)?);
                raw_attributes.extend_from_slice(&inherited_p14_shadow_attributes(
                    inherited,
                    &element,
                    xml,
                    &raw_attributes,
                ));
            }
            Event::Start(element) => {
                let scope = section_scope.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"sldIdLst"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                    && slide_ids.is_none()
                {
                    let parsed = parse_section_slide_ids(&raw, &scope)?;
                    slide_ids = Some(parsed.0);
                    slide_id_list_attributes = parsed.1;
                    slide_id_list_raw_children = parsed.2;
                    slide_id_sidecars = parsed.3;
                    boundary = 1;
                } else {
                    raw_children.push(boundary, raw);
                }
            }
            Event::Empty(element) => {
                let scope = section_scope.with_start(&element)?;
                let raw = capture_empty_element(&element)?;
                if local_name(element.name().as_ref()) == b"sldIdLst"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                    && slide_ids.is_none()
                {
                    slide_ids = Some(Vec::new());
                    slide_id_list_attributes = lexical_attributes(&element, &[])?;
                    slide_id_list_attributes
                        .extend_from_slice(&dependent_p14_shadow_attributes(&element, &raw)?);
                    slide_id_list_attributes.extend_from_slice(&inherited_p14_shadow_attributes(
                        &section_scope,
                        &element,
                        &raw,
                        &slide_id_list_attributes,
                    ));
                    boundary = 1;
                } else {
                    raw_children.push(boundary, raw);
                }
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"section" => break,
            Event::Eof => return Err(OxmlError::MissingElement("closing p14:section".to_owned())),
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => raw_children.push(boundary, capture_misc_event(event)?),
            _ => {}
        }
        buffer.clear();
    }
    if let Some(value) = &id {
        validate_guid("section", value)?;
    }
    let slide_ids = slide_ids
        .ok_or_else(|| OxmlError::MissingElement("p14:section requires p14:sldIdLst".to_owned()))?;
    Ok(Section {
        id,
        name,
        original_slide_ids: slide_ids.clone(),
        slide_ids,
        raw_attributes,
        slide_id_list_attributes,
        slide_id_list_raw_children,
        slide_id_sidecars,
        raw_children,
    })
}

fn parse_section_slide_ids(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<ParsedSectionSlideIds> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut root_scope = inherited.clone();
    let mut attributes = Vec::new();
    let mut ids = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut sidecars = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == b"sldIdLst" => {
                root_scope = inherited.with_start(&element)?;
                attributes = lexical_attributes(&element, &[])?;
                attributes.extend_from_slice(&dependent_p14_shadow_attributes(&element, xml)?);
                attributes.extend_from_slice(&inherited_p14_shadow_attributes(
                    inherited,
                    &element,
                    xml,
                    &attributes,
                ));
            }
            Event::Start(element) => {
                let scope = root_scope.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"sldId"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    let sidecar = parse_section_slide_id_sidecar(&raw, &scope)?;
                    ids.push(sidecar.id);
                    sidecars.push(sidecar);
                } else {
                    raw_children.push(ids.len(), raw);
                }
            }
            Event::Empty(element) => {
                let scope = root_scope.with_start(&element)?;
                let raw = capture_empty_element(&element)?;
                if local_name(element.name().as_ref()) == b"sldId"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    let id = parse_section_slide_id(&element)?;
                    ids.push(id);
                    let mut raw_attributes = lexical_attributes(&element, &["id"])?;
                    raw_attributes
                        .extend_from_slice(&dependent_p14_shadow_attributes(&element, &raw)?);
                    raw_attributes.extend_from_slice(&inherited_p14_shadow_attributes(
                        &root_scope,
                        &element,
                        &raw,
                        &raw_attributes,
                    ));
                    sidecars.push(SectionSlideIdSidecar {
                        id,
                        raw_attributes,
                        raw_children: Vec::new(),
                    });
                } else {
                    raw_children.push(ids.len(), raw);
                }
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sldIdLst" => break,
            Event::Eof => return Err(OxmlError::MissingElement("closing p14:sldIdLst".to_owned())),
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => raw_children.push(ids.len(), capture_misc_event(event)?),
            _ => {}
        }
        buffer.clear();
    }
    Ok((ids, attributes, raw_children, sidecars))
}

fn parse_section_slide_id(element: &BytesStart<'_>) -> Result<u32> {
    all_attributes(element)?
        .into_iter()
        .find(|(name, _)| name == "id")
        .ok_or_else(|| missing_attribute("p14:sldId", "id"))?
        .1
        .parse()
        .map_err(|_| invalid_value("p14:sldId has malformed @id".to_owned()))
}

fn parse_section_slide_id_sidecar(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<SectionSlideIdSidecar> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut id = None;
    let mut raw_attributes = Vec::new();
    let mut raw_children = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if id.is_none() => {
                id = Some(parse_section_slide_id(&element)?);
                raw_attributes = lexical_attributes(&element, &["id"])?;
                raw_attributes.extend_from_slice(&dependent_p14_shadow_attributes(&element, xml)?);
                raw_attributes.extend_from_slice(&inherited_p14_shadow_attributes(
                    inherited,
                    &element,
                    xml,
                    &raw_attributes,
                ));
            }
            Event::Start(element) => {
                raw_children.extend_from_slice(&capture_element(&mut reader, &element)?);
            }
            Event::Empty(element) => {
                raw_children.extend_from_slice(&capture_empty_element(&element)?);
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sldId" => break,
            Event::Eof => return Err(OxmlError::MissingElement("closing p14:sldId".to_owned())),
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => raw_children.extend_from_slice(&capture_misc_event(event)?),
            _ => {}
        }
        buffer.clear();
    }
    Ok(SectionSlideIdSidecar {
        id: id.ok_or_else(|| missing_attribute("p14:sldId", "id"))?,
        raw_attributes,
        raw_children,
    })
}

fn write_extension_list(
    original: Option<&[u8]>,
    section_range: Option<(usize, usize)>,
    sections: &[Section],
    sections_dirty: bool,
    section_list_namespaces: &[(String, String)],
) -> Result<Option<Vec<u8>>> {
    if original.is_none() && sections.is_empty() {
        return Ok(None);
    }
    if let Some(original) = original {
        if let Some((start, end)) = section_range {
            if !sections_dirty {
                return Ok(Some(original.to_vec()));
            }
            if sections.is_empty() {
                if section_list_has_raw_content(&original[start..end], section_list_namespaces)? {
                    return Err(invalid_value(
                        "cannot clear sections while preserving direct section-list raw payload"
                            .to_owned(),
                    ));
                }
                let mut output = Vec::with_capacity(original.len() - (end - start));
                output.extend_from_slice(&original[..start]);
                output.extend_from_slice(&original[end..]);
                return Ok(Some(output));
            }
            let replacement = write_section_list_preserving_raw(
                &original[start..end],
                sections,
                section_list_namespaces,
            )?;
            let mut output = Vec::with_capacity(original.len() - (end - start) + replacement.len());
            output.extend_from_slice(&original[..start]);
            output.extend_from_slice(&replacement);
            output.extend_from_slice(&original[end..]);
            return Ok(Some(output));
        }
        if sections.is_empty() {
            return Ok(Some(original.to_vec()));
        }
        let section_xml = write_section_list(sections)?;
        if original.trim_ascii_end().ends_with(b"/>") {
            let slash = original
                .windows(2)
                .rposition(|window| window == b"/>")
                .expect("trimmed empty element ends with its slash");
            let name_end = original[1..]
                .iter()
                .position(|byte| byte.is_ascii_whitespace() || matches!(byte, b'/' | b'>'))
                .map(|index| index + 1)
                .ok_or_else(|| invalid_value("malformed empty p:extLst".to_owned()))?;
            let mut output = Vec::with_capacity(original.len() + section_xml.len() + 80);
            output.extend_from_slice(&original[..slash]);
            output.extend_from_slice(b">");
            output.extend_from_slice(format!("<p:ext uri=\"{SECTION_EXTENSION_URI}\">").as_bytes());
            output.extend_from_slice(&section_xml);
            output.extend_from_slice(b"</p:ext></");
            output.extend_from_slice(&original[1..name_end]);
            output.extend_from_slice(b">");
            return Ok(Some(output));
        }
        let closing = original
            .windows(b"</".len())
            .rposition(|window| window == b"</")
            .ok_or_else(|| invalid_value("p:extLst has no closing tag".to_owned()))?;
        let mut output = Vec::with_capacity(original.len() + section_xml.len() + 80);
        output.extend_from_slice(&original[..closing]);
        output.extend_from_slice(format!("<p:ext uri=\"{SECTION_EXTENSION_URI}\">").as_bytes());
        output.extend_from_slice(&section_xml);
        output.extend_from_slice(b"</p:ext>");
        output.extend_from_slice(&original[closing..]);
        return Ok(Some(output));
    }
    let section_xml = write_section_list(sections)?;
    let mut output = format!("<p:extLst><p:ext uri=\"{SECTION_EXTENSION_URI}\">").into_bytes();
    output.extend_from_slice(&section_xml);
    output.extend_from_slice(b"</p:ext></p:extLst>");
    Ok(Some(output))
}

fn section_list_has_raw_content(
    original: &[u8],
    inherited_namespaces: &[(String, String)],
) -> Result<bool> {
    let mut reader = Reader::from_reader(original);
    let mut buffer = Vec::new();
    let mut root_scope = NamespaceBindings::from_entries(inherited_namespaces);
    let mut saw_root = false;
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if !saw_root => {
                saw_root = true;
                root_scope = root_scope.with_start(&element)?;
                if all_attributes(&element)?.into_iter().any(|(name, value)| {
                    !((name == "xmlns" || name.starts_with("xmlns:")) && value == P14_NS)
                }) {
                    return Ok(true);
                }
            }
            Event::Start(element) => {
                let scope = root_scope.with_start(&element)?;
                if local_name(element.name().as_ref()) != b"section"
                    || scope.element_uri(element.name().as_ref()) != Some(P14_NS)
                {
                    return Ok(true);
                }
                capture_element(&mut reader, &element)?;
            }
            Event::Empty(element) => {
                let scope = root_scope.with_start(&element)?;
                if local_name(element.name().as_ref()) != b"section"
                    || scope.element_uri(element.name().as_ref()) != Some(P14_NS)
                {
                    return Ok(true);
                }
            }
            Event::Text(text) => {
                let bytes: &[u8] = text.as_ref();
                if bytes.iter().any(|byte| !byte.is_ascii_whitespace()) {
                    return Ok(true);
                }
            }
            Event::Comment(_) | Event::PI(_) | Event::CData(_) | Event::DocType(_) => {
                return Ok(true);
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sectionLst" => {
                return Ok(false);
            }
            Event::Eof => return Ok(false),
            _ => {}
        }
        buffer.clear();
    }
}

fn write_section_list(sections: &[Section]) -> Result<Vec<u8>> {
    let mut writer = Writer::new(Vec::new());
    let mut root = BytesStart::new("p14:sectionLst");
    root.push_attribute(("xmlns:p14", P14_NS));
    writer.write_event(Event::Start(root))?;
    for section in sections {
        write_section(&mut writer, section)?;
    }
    writer.write_event(Event::End(BytesEnd::new("p14:sectionLst")))?;
    Ok(writer.into_inner())
}

fn write_section_list_preserving_raw(
    original: &[u8],
    sections: &[Section],
    inherited_namespaces: &[(String, String)],
) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(original);
    let mut buffer = Vec::new();
    let mut root_scope = NamespaceBindings::from_entries(inherited_namespaces);
    let mut opening = Vec::new();
    let mut closing = Vec::new();
    let mut originals = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    loop {
        let event_start = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if opening.is_empty() => {
                root_scope = root_scope.with_start(&element)?;
                opening.extend_from_slice(&original[..reader.buffer_position() as usize]);
            }
            Event::Start(element) => {
                let scope = root_scope.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"section"
                    && scope.element_uri(element.name().as_ref()) == Some(P14_NS)
                {
                    originals.push(parse_section(&raw, &scope)?);
                } else {
                    raw_children.push(originals.len(), raw);
                }
            }
            Event::Empty(element) => {
                raw_children.push(originals.len(), capture_empty_element(&element)?);
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sectionLst" => {
                closing
                    .extend_from_slice(&original[event_start..reader.buffer_position() as usize]);
                break;
            }
            Event::Eof => {
                return Err(OxmlError::MissingElement(
                    "closing p14:sectionLst".to_owned(),
                ));
            }
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => {
                let mut event_writer = Writer::new(Vec::new());
                event_writer.write_event(event.into_owned())?;
                raw_children.push(originals.len(), event_writer.into_inner());
            }
            _ => {}
        }
        buffer.clear();
    }
    let original_to_current = originals
        .iter()
        .map(|original| {
            sections
                .iter()
                .position(|section| section.id == original.id)
        })
        .collect::<Vec<_>>();
    let mut writer = Writer::new(Vec::new());
    writer.get_mut().write_all(&opening)?;
    for (index, section) in sections.iter().enumerate() {
        emit_raw(
            &mut writer,
            raw_children.at_reconciled(index, 0, &original_to_current, sections.len()),
        )?;
        write_section(&mut writer, section)?;
    }
    emit_raw(
        &mut writer,
        raw_children.at_reconciled(sections.len(), 0, &original_to_current, sections.len()),
    )?;
    writer.get_mut().write_all(&closing)?;
    Ok(writer.into_inner())
}

fn write_section<W: Write>(writer: &mut Writer<W>, section: &Section) -> Result<()> {
    let prefix = section_model_prefix(&section.raw_attributes, "p14")?;
    let tag = format!("{prefix}:section");
    let mut start = BytesStart::new(&tag);
    push_section_namespace(&mut start, prefix);
    if let Some(name) = &section.name {
        start.push_attribute(("name", name.as_str()));
    }
    if let Some(id) = &section.id {
        start.push_attribute(("id", id.as_str()));
    }
    write_start_with_raw(writer, &start, &section.raw_attributes, false)?;
    emit_raw(writer, section.raw_children.at(0))?;
    let list_prefix = section_model_prefix(&section.slide_id_list_attributes, prefix)?;
    let list_tag = format!("{list_prefix}:sldIdLst");
    let mut list = BytesStart::new(&list_tag);
    if list_prefix != prefix {
        push_section_namespace(&mut list, list_prefix);
    }
    write_start_with_raw(writer, &list, &section.slide_id_list_attributes, false)?;
    let original_to_current = section
        .original_slide_ids
        .iter()
        .map(|id| section.slide_ids.iter().position(|current| current == id))
        .collect::<Vec<_>>();
    for (index, id) in section.slide_ids.iter().enumerate() {
        emit_raw(
            writer,
            section.slide_id_list_raw_children.at_reconciled(
                index,
                0,
                &original_to_current,
                section.slide_ids.len(),
            ),
        )?;
        let value = id.to_string();
        let sidecar = section.slide_id_sidecars.iter().find(|item| item.id == *id);
        let slide_prefix = if let Some(sidecar) = sidecar {
            section_model_prefix(&sidecar.raw_attributes, list_prefix)?
        } else {
            list_prefix
        };
        let slide_tag = format!("{slide_prefix}:sldId");
        let mut slide = BytesStart::new(&slide_tag);
        if slide_prefix != list_prefix {
            push_section_namespace(&mut slide, slide_prefix);
        }
        slide.push_attribute(("id", value.as_str()));
        if let Some(sidecar) = sidecar {
            if sidecar.raw_children.is_empty() {
                write_start_with_raw(writer, &slide, &sidecar.raw_attributes, true)?;
            } else {
                write_start_with_raw(writer, &slide, &sidecar.raw_attributes, false)?;
                writer.get_mut().write_all(&sidecar.raw_children)?;
                writer.write_event(Event::End(BytesEnd::new(&slide_tag)))?;
            }
        } else {
            writer.write_event(Event::Empty(slide))?;
        }
    }
    emit_raw(
        writer,
        section.slide_id_list_raw_children.at_reconciled(
            section.slide_ids.len(),
            0,
            &original_to_current,
            section.slide_ids.len(),
        ),
    )?;
    writer.write_event(Event::End(BytesEnd::new(&list_tag)))?;
    emit_raw(writer, section.raw_children.at(1))?;
    writer.write_event(Event::End(BytesEnd::new(&tag)))?;
    Ok(())
}

fn lexical_attributes(start: &BytesStart<'_>, modeled: &[&str]) -> Result<Vec<u8>> {
    let _ = all_attributes(start)?;
    let raw = start.attributes_raw();
    let mut output = Vec::new();
    let mut cursor = 0usize;
    while cursor < raw.len() {
        let span_start = cursor;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == raw.len() {
            output.extend_from_slice(&raw[span_start..]);
            break;
        }
        let name_start = cursor;
        while cursor < raw.len() && !raw[cursor].is_ascii_whitespace() && raw[cursor] != b'=' {
            cursor += 1;
        }
        let name = &raw[name_start..cursor];
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        cursor += 1;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        let quote = raw[cursor];
        cursor += 1;
        while cursor < raw.len() && raw[cursor] != quote {
            cursor += 1;
        }
        cursor += 1;
        let is_modeled =
            !name.contains(&b':') && modeled.iter().any(|candidate| name == candidate.as_bytes());
        let fixed_namespace = matches!(name, b"xmlns:p14" | b"xmlns:p14m" | b"xmlns:p14model");
        if !is_modeled && !fixed_namespace {
            output.extend_from_slice(&raw[span_start..cursor]);
        }
    }
    Ok(output)
}

fn section_model_prefix<'a>(raw_attributes: &[u8], inherited: &'a str) -> Result<&'a str> {
    if !has_raw_namespace_declaration(raw_attributes, inherited) {
        return Ok(inherited);
    }
    ["p14", "p14m", "p14model"]
        .into_iter()
        .find(|prefix| !has_raw_namespace_declaration(raw_attributes, prefix))
        .ok_or_else(|| invalid_value("no unshadowed section model prefix is available".to_owned()))
}

fn push_section_namespace(start: &mut BytesStart<'_>, prefix: &str) {
    match prefix {
        "p14" => start.push_attribute(("xmlns:p14", P14_NS)),
        "p14m" => start.push_attribute(("xmlns:p14m", P14_NS)),
        "p14model" => start.push_attribute(("xmlns:p14model", P14_NS)),
        _ => unreachable!("section model prefix is selected from fixed candidates"),
    }
}

fn has_raw_namespace_declaration(raw_attributes: &[u8], prefix: &str) -> bool {
    let declaration = format!("xmlns:{prefix}");
    raw_attributes
        .windows(declaration.len())
        .enumerate()
        .any(|(index, window)| {
            window == declaration.as_bytes()
                && raw_attributes
                    .get(index + declaration.len())
                    .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'=')
        })
}

fn dependent_p14_shadow_attributes(start: &BytesStart<'_>, xml: &[u8]) -> Result<Vec<u8>> {
    let decoded = all_attributes(start)?;
    let descendants = xml
        .iter()
        .position(|byte| *byte == b'>')
        .map_or(&[][..], |index| &xml[index + 1..]);
    let raw = start.attributes_raw();
    let mut preserved = Vec::new();
    let mut cursor = 0usize;
    while cursor < raw.len() {
        let span_start = cursor;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        let name_start = cursor;
        while cursor < raw.len() && !raw[cursor].is_ascii_whitespace() && raw[cursor] != b'=' {
            cursor += 1;
        }
        let name = &raw[name_start..cursor];
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == raw.len() {
            break;
        }
        cursor += 1;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        let quote = *raw
            .get(cursor)
            .ok_or_else(|| invalid_value("malformed p14 namespace shadow".to_owned()))?;
        cursor += 1;
        while cursor < raw.len() && raw[cursor] != quote {
            cursor += 1;
        }
        cursor += 1;
        let prefix = match name {
            b"xmlns:p14" => b"p14".as_slice(),
            b"xmlns:p14m" => b"p14m".as_slice(),
            b"xmlns:p14model" => b"p14model".as_slice(),
            _ => continue,
        };
        let value = decoded
            .iter()
            .find(|(candidate, _)| candidate.as_bytes() == name)
            .map(|(_, value)| value.as_str());
        if value != Some(P14_NS)
            && (shell_attribute_depends_on_prefix(start, prefix)
                || descendant_depends_on_prefix(descendants, prefix))
        {
            preserved.extend_from_slice(&raw[span_start..cursor]);
        }
    }
    Ok(preserved)
}

fn inherited_p14_shadow_attributes(
    inherited: &NamespaceBindings,
    start: &BytesStart<'_>,
    xml: &[u8],
    local_attributes: &[u8],
) -> Vec<u8> {
    let descendants = xml
        .iter()
        .position(|byte| *byte == b'>')
        .map_or(&[][..], |index| &xml[index + 1..]);
    let mut preserved = Vec::new();
    for (prefix, uri) in inherited.entries() {
        if !matches!(prefix.as_str(), "p14" | "p14m" | "p14model")
            || uri == P14_NS
            || has_raw_namespace_declaration(local_attributes, &prefix)
            || !(shell_attribute_depends_on_prefix(start, prefix.as_bytes())
                || descendant_depends_on_prefix(descendants, prefix.as_bytes()))
        {
            continue;
        }
        let escaped = quick_xml::escape::escape(&uri);
        preserved.extend_from_slice(format!(" xmlns:{prefix}=\"{escaped}\"").as_bytes());
    }
    preserved
}

fn shell_attribute_depends_on_prefix(start: &BytesStart<'_>, prefix: &[u8]) -> bool {
    let qualified = [prefix, b":"].concat();
    start
        .attributes()
        .with_checks(false)
        .filter_map(std::result::Result::ok)
        .any(|attribute| attribute.key.as_ref().starts_with(&qualified))
}

fn descendant_depends_on_prefix(xml: &[u8], prefix: &[u8]) -> bool {
    let qualified = [prefix, b":"].concat();
    let declaration = [b"xmlns:".as_slice(), prefix].concat();
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let mut shadowed = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(element)) => {
                let current = shadowed.last().copied().unwrap_or(false)
                    || element.attributes().with_checks(false).any(|attribute| {
                        attribute.is_ok_and(|attribute| attribute.key.as_ref() == declaration)
                    });
                if !current && element_uses_prefix(&element, &qualified) {
                    return true;
                }
                shadowed.push(current);
            }
            Ok(Event::Empty(element)) => {
                let current = shadowed.last().copied().unwrap_or(false)
                    || element.attributes().with_checks(false).any(|attribute| {
                        attribute.is_ok_and(|attribute| attribute.key.as_ref() == declaration)
                    });
                if !current && element_uses_prefix(&element, &qualified) {
                    return true;
                }
            }
            Ok(Event::End(_)) => {
                shadowed.pop();
            }
            Ok(Event::Eof) | Err(_) => return false,
            _ => {}
        }
        buffer.clear();
    }
}

fn element_uses_prefix(element: &BytesStart<'_>, qualified: &[u8]) -> bool {
    element.name().as_ref().starts_with(qualified)
        || element
            .attributes()
            .with_checks(false)
            .filter_map(std::result::Result::ok)
            .any(|attribute| attribute.key.as_ref().starts_with(qualified))
}

fn raw_uses_namespace_prefix(xml: &[u8], prefix: &str) -> bool {
    if prefix.is_empty() {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Start(element) | Event::Empty(element)) => {
                    if !element.name().as_ref().contains(&b':') {
                        return true;
                    }
                }
                Ok(Event::Eof) | Err(_) => return false,
                _ => {}
            }
            buffer.clear();
        }
    }
    let qualified = format!("{prefix}:");
    xml.windows(qualified.len())
        .any(|window| window == qualified.as_bytes())
}

fn write_start_with_raw<W: Write>(
    writer: &mut Writer<W>,
    start: &BytesStart<'_>,
    raw_attributes: &[u8],
    empty: bool,
) -> Result<()> {
    writer.get_mut().write_all(b"<")?;
    writer.get_mut().write_all(start.as_ref())?;
    writer.get_mut().write_all(raw_attributes)?;
    writer
        .get_mut()
        .write_all(if empty { b"/>" } else { b">" })?;
    Ok(())
}

fn capture_misc_event(event: Event<'_>) -> Result<Vec<u8>> {
    let mut writer = Writer::new(Vec::new());
    writer.write_event(event.into_owned())?;
    Ok(writer.into_inner())
}

fn validate_sections(sections: &[Section], slides: &[CT_SlideId]) -> Result<()> {
    let known = slides
        .iter()
        .map(|slide| slide.id)
        .collect::<std::collections::HashSet<_>>();
    let mut section_ids = std::collections::HashSet::new();
    let mut assigned_slides = std::collections::HashSet::new();
    for section in sections {
        let id = section
            .id
            .as_ref()
            .ok_or_else(|| invalid_value("section mutation requires an id".to_owned()))?;
        validate_guid("section", id)?;
        if !section_ids.insert(id) {
            return Err(invalid_value(format!("duplicate section id {id}")));
        }
        for slide_id in &section.slide_ids {
            if !known.contains(slide_id) {
                return Err(invalid_value(format!(
                    "unknown section slide id {slide_id}"
                )));
            }
            if !assigned_slides.insert(*slide_id) {
                return Err(invalid_value(format!(
                    "slide id {slide_id} belongs to more than one section"
                )));
            }
        }
    }
    Ok(())
}

fn validate_guid(kind: &str, value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    let valid = bytes.len() == 38
        && bytes[0] == b'{'
        && bytes[37] == b'}'
        && [9, 14, 19, 24]
            .into_iter()
            .all(|index| bytes[index] == b'-')
        && bytes[1..37]
            .iter()
            .enumerate()
            .all(|(index, byte)| [8, 13, 18, 23].contains(&index) || byte.is_ascii_hexdigit());
    if valid {
        Ok(())
    } else {
        Err(invalid_value(format!(
            "{kind} id is not a braced GUID: {value}"
        )))
    }
}

fn parse_slide_list(xml: &[u8], inherited: &NamespaceBindings) -> Result<ParsedSlideList> {
    let mut reader = Reader::from_reader(xml);
    let mut list_namespaces = inherited.clone();
    let mut items = Vec::new();
    let mut attributes = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == b"sldIdLst" => {
                attributes = all_attributes(&element)?;
                list_namespaces = inherited.with_start(&element)?;
            }
            Event::Empty(element) if local_name(element.name().as_ref()) == b"sldIdLst" => {
                attributes = all_attributes(&element)?;
                return Ok((items, attributes, raw_children));
            }
            Event::Start(element) => {
                let namespaces = list_namespaces.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"sldId"
                    && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                {
                    namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                    items.push(parse_slide_id(&raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::Empty(element) => {
                let namespaces = list_namespaces.with_start(&element)?;
                let raw = capture_empty_element(&element)?;
                if local_name(element.name().as_ref()) == b"sldId"
                    && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                {
                    namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                    items.push(parse_slide_id(&raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sldIdLst" => {
                return Ok((items, attributes, raw_children));
            }
            Event::Eof => return Err(OxmlError::MissingElement("closing p:sldIdLst".to_owned())),
            _ => {}
        }
        buffer.clear();
    }
}

fn parse_master_list(xml: &[u8], inherited: &NamespaceBindings) -> Result<ParsedMasterList> {
    let mut reader = Reader::from_reader(xml);
    let mut list_namespaces = inherited.clone();
    let mut items = Vec::new();
    let mut attributes = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == b"sldMasterIdLst" => {
                attributes = all_attributes(&element)?;
                list_namespaces = inherited.with_start(&element)?;
            }
            Event::Empty(element) if local_name(element.name().as_ref()) == b"sldMasterIdLst" => {
                attributes = all_attributes(&element)?;
                return Ok((items, attributes, raw_children));
            }
            Event::Start(element) => {
                let namespaces = list_namespaces.with_start(&element)?;
                let raw = capture_element(&mut reader, &element)?;
                if local_name(element.name().as_ref()) == b"sldMasterId"
                    && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                {
                    namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                    items.push(parse_master_id(&raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::Empty(element) => {
                let namespaces = list_namespaces.with_start(&element)?;
                let raw = capture_empty_element(&element)?;
                if local_name(element.name().as_ref()) == b"sldMasterId"
                    && namespaces.element_uri(element.name().as_ref()) == Some(P_NS)
                {
                    namespaces.reject_writer_conflicts(FIXED_MODEL_PREFIXES)?;
                    items.push(parse_master_id(&raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::End(element) if local_name(element.name().as_ref()) == b"sldMasterIdLst" => {
                return Ok((items, attributes, raw_children));
            }
            Event::Eof => {
                return Err(OxmlError::MissingElement(
                    "closing p:sldMasterIdLst".to_owned(),
                ));
            }
            _ => {}
        }
        buffer.clear();
    }
}

fn parse_slide_id(xml: &[u8], namespaces: &NamespaceBindings) -> Result<CT_SlideId> {
    let (id, relationship_id, raw_attributes, raw_children) =
        parse_identifier(xml, "sldId", namespaces)?;
    let id = id.ok_or_else(|| missing_attribute("sldId", "id"))?;
    Ok(CT_SlideId {
        id,
        relationship_id,
        raw_attributes,
        raw_children,
    })
}

fn parse_master_id(xml: &[u8], namespaces: &NamespaceBindings) -> Result<CT_SlideMasterId> {
    let (id, relationship_id, raw_attributes, raw_children) =
        parse_identifier(xml, "sldMasterId", namespaces)?;
    Ok(CT_SlideMasterId {
        id,
        relationship_id,
        raw_attributes,
        raw_children,
    })
}

fn parse_identifier(
    xml: &[u8],
    expected: &str,
    inherited: &NamespaceBindings,
) -> Result<ParsedIdentifier> {
    let mut reader = Reader::from_reader(xml);
    let mut id = None;
    let mut relationship_id = None;
    let mut raw_attributes = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                let namespaces = inherited.with_start(&element)?;
                parse_identifier_attributes(
                    &element,
                    expected,
                    &namespaces,
                    &mut id,
                    &mut relationship_id,
                    &mut raw_attributes,
                )?;
            }
            Event::Empty(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                let namespaces = inherited.with_start(&element)?;
                parse_identifier_attributes(
                    &element,
                    expected,
                    &namespaces,
                    &mut id,
                    &mut relationship_id,
                    &mut raw_attributes,
                )?;
                break;
            }
            Event::Start(element) => raw_children.push(0, capture_element(&mut reader, &element)?),
            Event::Empty(element) => raw_children.push(0, capture_empty_element(&element)?),
            Event::End(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                break;
            }
            Event::Eof => break,
            _ => {}
        }
        buffer.clear();
    }
    Ok((
        id,
        relationship_id.ok_or_else(|| missing_attribute(expected, "r:id"))?,
        raw_attributes,
        raw_children,
    ))
}

fn parse_identifier_attributes(
    element: &BytesStart<'_>,
    expected: &str,
    namespaces: &NamespaceBindings,
    id: &mut Option<u32>,
    relationship_id: &mut Option<String>,
    raw_attributes: &mut Vec<(String, String)>,
) -> Result<()> {
    for (name, value) in all_attributes(element)? {
        if name == "id" {
            if id.is_some() {
                return Err(duplicate("id attribute"));
            }
            *id = Some(parse_u32(expected, "id", &value)?);
        } else if local_name(name.as_bytes()) == b"id"
            && namespaces.attribute_uri(name.as_bytes()) == Some(R_NS)
        {
            if relationship_id.replace(value).is_some() {
                return Err(duplicate("relationship id attribute"));
            }
        } else {
            raw_attributes.push((name, value));
        }
    }
    Ok(())
}

fn parse_size(xml: &[u8], expected: &str) -> Result<CT_SlideSize> {
    let mut reader = Reader::from_reader(xml);
    let mut cx = None;
    let mut cy = None;
    let mut kind = None;
    let mut raw_attributes = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                parse_size_attributes(
                    &element,
                    expected,
                    &mut cx,
                    &mut cy,
                    &mut kind,
                    &mut raw_attributes,
                )?;
            }
            Event::Empty(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                parse_size_attributes(
                    &element,
                    expected,
                    &mut cx,
                    &mut cy,
                    &mut kind,
                    &mut raw_attributes,
                )?;
                break;
            }
            Event::Start(element) => raw_children.push(0, capture_element(&mut reader, &element)?),
            Event::Empty(element) => raw_children.push(0, capture_empty_element(&element)?),
            Event::End(element) if local_name(element.name().as_ref()) == expected.as_bytes() => {
                break;
            }
            Event::Eof => break,
            _ => {}
        }
        buffer.clear();
    }
    let size = CT_SlideSize {
        cx: cx.ok_or_else(|| missing_attribute(expected, "cx"))?,
        cy: cy.ok_or_else(|| missing_attribute(expected, "cy"))?,
        kind,
        raw_attributes,
        raw_children,
    };
    validate_size(expected, &size)?;
    Ok(size)
}

fn parse_size_attributes(
    element: &BytesStart<'_>,
    expected: &str,
    cx: &mut Option<Emu>,
    cy: &mut Option<Emu>,
    kind: &mut Option<String>,
    raw_attributes: &mut Vec<(String, String)>,
) -> Result<()> {
    for (name, value) in all_attributes(element)? {
        match name.as_str() {
            "cx" if cx.is_none() => *cx = Some(parse_emu(expected, "cx", &value)?),
            "cy" if cy.is_none() => *cy = Some(parse_emu(expected, "cy", &value)?),
            "type" if kind.is_none() => *kind = Some(value),
            "cx" | "cy" | "type" => return Err(duplicate(&format!("{expected} @{name}"))),
            _ => raw_attributes.push((name, value)),
        }
    }
    Ok(())
}

fn write_master_list<W: Write>(
    presentation: &CT_Presentation,
    writer: &mut Writer<W>,
) -> Result<()> {
    let mut list = BytesStart::new("p:sldMasterIdLst");
    push_attributes(&mut list, &presentation.slide_master_list_attributes);
    if presentation.slide_master_ids.is_empty()
        && presentation.slide_master_list_raw_children.is_empty()
    {
        writer.write_event(Event::Empty(list))?;
        return Ok(());
    }
    writer.write_event(Event::Start(list))?;
    emit_raw(writer, presentation.slide_master_list_raw_children.at(0))?;
    for (index, item) in presentation.slide_master_ids.iter().enumerate() {
        write_identifier(
            "p:sldMasterId",
            item.id,
            &item.relationship_id,
            &item.raw_attributes,
            &item.raw_children,
            writer,
        )?;
        emit_raw(
            writer,
            presentation.slide_master_list_raw_children.at(index + 1),
        )?;
    }
    writer.write_event(Event::End(BytesEnd::new("p:sldMasterIdLst")))?;
    Ok(())
}

fn write_slide_list<W: Write>(
    presentation: &CT_Presentation,
    writer: &mut Writer<W>,
) -> Result<()> {
    let mut list = BytesStart::new("p:sldIdLst");
    push_attributes(&mut list, &presentation.slide_id_list_attributes);
    if presentation.slide_ids.is_empty() && presentation.slide_id_list_raw_children.is_empty() {
        writer.write_event(Event::Empty(list))?;
        return Ok(());
    }
    writer.write_event(Event::Start(list))?;
    let original_to_current = presentation
        .original_slide_relationship_ids
        .iter()
        .map(|relationship_id| {
            presentation
                .slide_ids
                .iter()
                .position(|slide| slide.relationship_id == *relationship_id)
        })
        .collect::<Vec<_>>();
    for (index, item) in presentation.slide_ids.iter().enumerate() {
        emit_raw(
            writer,
            presentation.slide_id_list_raw_children.at_reconciled(
                index,
                0,
                &original_to_current,
                presentation.slide_ids.len(),
            ),
        )?;
        write_identifier(
            "p:sldId",
            Some(item.id),
            &item.relationship_id,
            &item.raw_attributes,
            &item.raw_children,
            writer,
        )?;
    }
    emit_raw(
        writer,
        presentation.slide_id_list_raw_children.at_reconciled(
            presentation.slide_ids.len(),
            0,
            &original_to_current,
            presentation.slide_ids.len(),
        ),
    )?;
    writer.write_event(Event::End(BytesEnd::new("p:sldIdLst")))?;
    Ok(())
}

fn write_identifier<W: Write>(
    tag: &str,
    id: Option<u32>,
    relationship_id: &str,
    raw_attributes: &[(String, String)],
    raw_children: &OrderedRawChildren,
    writer: &mut Writer<W>,
) -> Result<()> {
    let mut start = BytesStart::new(tag);
    let id_value = id.map(|value| value.to_string());
    if let Some(id_value) = &id_value {
        start.push_attribute(("id", id_value.as_str()));
    }
    start.push_attribute(("r:id", relationship_id));
    push_attributes(&mut start, raw_attributes);
    if raw_children.is_empty() {
        writer.write_event(Event::Empty(start))?;
    } else {
        writer.write_event(Event::Start(start))?;
        emit_raw(writer, raw_children.at(0))?;
        writer.write_event(Event::End(BytesEnd::new(tag)))?;
    }
    Ok(())
}

fn write_size<W: Write>(size: &CT_SlideSize, tag: &str, writer: &mut Writer<W>) -> Result<()> {
    validate_size(tag, size)?;
    let cx = size.cx.0.to_string();
    let cy = size.cy.0.to_string();
    let mut start = BytesStart::new(tag);
    start.push_attribute(("cx", cx.as_str()));
    start.push_attribute(("cy", cy.as_str()));
    if let Some(kind) = &size.kind {
        start.push_attribute(("type", kind.as_str()));
    }
    push_attributes(&mut start, &size.raw_attributes);
    if size.raw_children.is_empty() {
        writer.write_event(Event::Empty(start))?;
    } else {
        writer.write_event(Event::Start(start))?;
        emit_raw(writer, size.raw_children.at(0))?;
        writer.write_event(Event::End(BytesEnd::new(tag)))?;
    }
    Ok(())
}

fn validate_slide_id(id: u32) -> Result<()> {
    if (MIN_SLIDE_ID..=MAX_SLIDE_ID).contains(&id) {
        Ok(())
    } else {
        Err(invalid_value(format!(
            "PowerPoint slide id {id} is outside {MIN_SLIDE_ID}..={MAX_SLIDE_ID}"
        )))
    }
}

fn validate_size(element: &str, size: &CT_SlideSize) -> Result<()> {
    validate_dimension(element, "cx", size.cx)?;
    validate_dimension(element, "cy", size.cy)
}

fn validate_dimension(element: &str, attribute: &str, value: Emu) -> Result<()> {
    if value.0 > 0 {
        Ok(())
    } else {
        Err(invalid_value(format!(
            "{element} @{attribute} must be a positive EMU value"
        )))
    }
}

fn parse_emu(element: &str, attribute: &str, value: &str) -> Result<Emu> {
    let parsed = value
        .parse::<i64>()
        .map_err(|_| invalid_value(format!("{element} has malformed @{attribute}: {value}")))?;
    let emu = Emu(parsed);
    validate_dimension(element, attribute, emu)?;
    Ok(emu)
}

fn parse_u32(element: &str, attribute: &str, value: &str) -> Result<u32> {
    value
        .parse::<u32>()
        .map_err(|_| invalid_value(format!("{element} has malformed @{attribute}: {value}")))
}

fn push_attributes(start: &mut BytesStart<'_>, attributes: &[(String, String)]) {
    for (name, value) in attributes {
        start.push_attribute((name.as_str(), value.as_str()));
    }
}

fn emit_raw<'a, W: Write>(
    writer: &mut Writer<W>,
    children: impl Iterator<Item = &'a [u8]>,
) -> Result<()> {
    for child in children {
        writer.get_mut().write_all(child)?;
    }
    Ok(())
}

fn root_schema_boundary(name: &[u8]) -> Option<usize> {
    match name {
        b"notesMasterIdLst" => Some(2),
        b"handoutMasterIdLst" => Some(3),
        b"smartTags" => Some(7),
        b"custShowLst" => Some(9),
        b"photoAlbum" => Some(10),
        b"custDataLst" => Some(11),
        b"kiosk" => Some(12),
        b"modifyVerifier" => Some(14),
        b"extLst" => Some(15),
        _ => None,
    }
}

fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn element_name(element: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(element.name().as_ref()).into_owned()
}

fn missing_attribute(element: &str, attribute: &str) -> OxmlError {
    OxmlError::MissingElement(format!("{element} requires @{attribute}"))
}

fn duplicate(element: &str) -> OxmlError {
    invalid_value(format!("duplicate PresentationML {element}"))
}

fn invalid_value(message: String) -> OxmlError {
    OxmlError::InvalidValue(message)
}
