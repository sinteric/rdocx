use std::collections::HashMap;
use std::io::Write;

use oxml_core::OxmlError;
use oxml_core::raw_xml::{capture_element, capture_empty_element};
use oxml_core::xml::reject_non_xml_characters;
use oxml_drawing::namespace::A_NS;
use oxml_drawing::order::OrderedRawChildren;
use oxml_drawing::text::CT_TextBody;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};

use crate::namespace::{NamespaceBindings, P_NS, all_attributes};

pub const P188_NS: &str = "http://schemas.microsoft.com/office/powerpoint/2018/8/main";
const POWERPOINT_COMMAND_NS: &str =
    "http://schemas.microsoft.com/office/powerpoint/2013/main/command";
const DRAWING_COMMAND_NS: &str = "http://schemas.microsoft.com/office/drawing/2013/main/command";

pub type Result<T> = std::result::Result<T, OxmlError>;
type RawAttributes = Vec<u8>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentAuthor {
    pub id: String,
    pub name: String,
    pub initials: Option<String>,
    pub user_id: String,
    pub provider_id: String,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
}

impl CommentAuthor {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        initials: Option<&str>,
        user_id: impl Into<String>,
        provider_id: impl Into<String>,
    ) -> Result<Self> {
        let id = id.into();
        validate_guid("comment author", &id)?;
        let (name, user_id, provider_id) = (name.into(), user_id.into(), provider_id.into());
        reject_non_xml_characters("comment author name", &name)?;
        reject_non_xml_characters("comment author initials", initials.unwrap_or_default())?;
        reject_non_xml_characters("comment author user id", &user_id)?;
        reject_non_xml_characters("comment author provider id", &provider_id)?;
        Ok(Self {
            id,
            name,
            initials: initials.map(str::to_owned),
            user_id,
            provider_id,
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentAuthorList {
    pub authors: Vec<CommentAuthor>,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
    original_author_ids: Vec<String>,
}

impl Default for CommentAuthorList {
    fn default() -> Self {
        Self::new()
    }
}

impl CommentAuthorList {
    pub fn new() -> Self {
        Self {
            authors: Vec::new(),
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
            original_author_ids: Vec::new(),
        }
    }

    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer)? {
                Event::Start(start) => {
                    let namespaces = NamespaceBindings::default().with_start(&start)?;
                    require_element(&start, &namespaces, b"authorLst", P188_NS)?;
                    let mut raw_attributes = model_attributes(&start, &["p188", "a"])?;
                    raw_attributes
                        .extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                    let (authors, raw_children) = parse_ordered_children(
                        &mut reader,
                        &namespaces,
                        b"authorLst",
                        b"author",
                        |raw, ns| parse_author(&raw, ns),
                    )?;
                    return Ok(Self {
                        original_author_ids: authors
                            .iter()
                            .map(|author| author.id.clone())
                            .collect(),
                        authors,
                        raw_attributes,
                        raw_children,
                    });
                }
                Event::Empty(start) => {
                    let namespaces = NamespaceBindings::default().with_start(&start)?;
                    require_element(&start, &namespaces, b"authorLst", P188_NS)?;
                    let mut raw_attributes = model_attributes(&start, &["p188", "a"])?;
                    raw_attributes
                        .extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                    return Ok(Self {
                        authors: Vec::new(),
                        raw_attributes,
                        raw_children: OrderedRawChildren::default(),
                        original_author_ids: Vec::new(),
                    });
                }
                Event::Eof => return Err(missing("p188:authorLst")),
                _ => {}
            }
            buffer.clear();
        }
    }

    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(Vec::new());
        let prefix = comment_model_prefix(&self.raw_attributes, "p188")?;
        let tag = format!("{prefix}:authorLst");
        let mut root = BytesStart::new(&tag);
        push_model_namespace(&mut root, prefix);
        if !has_namespace_shadow(&self.raw_attributes, "a") {
            root.push_attribute(("xmlns:a", A_NS));
        }
        write_start_with_raw(&mut writer, &root, &self.raw_attributes, false)?;
        let original_to_current = self
            .original_author_ids
            .iter()
            .map(|id| self.authors.iter().position(|author| author.id == *id))
            .collect::<Vec<_>>();
        for (index, author) in self.authors.iter().enumerate() {
            emit_raw(
                &mut writer,
                self.raw_children
                    .at_reconciled(index, 0, &original_to_current, self.authors.len()),
            )?;
            write_author(&mut writer, author, prefix)?;
        }
        emit_raw(
            &mut writer,
            self.raw_children.at_reconciled(
                self.authors.len(),
                0,
                &original_to_current,
                self.authors.len(),
            ),
        )?;
        writer.write_event(Event::End(BytesEnd::new(&tag)))?;
        Ok(writer.into_inner())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentReply {
    pub id: String,
    pub author_id: String,
    pub status: Option<String>,
    pub created: String,
    text_body: Option<CT_TextBody>,
    text_body_attributes: RawAttributes,
    text_body_raw: Option<Vec<u8>>,
    text_body_dirty: bool,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
}

impl CommentReply {
    pub fn new(
        id: impl Into<String>,
        author_id: impl Into<String>,
        created: impl Into<String>,
        text: &str,
    ) -> Result<Self> {
        let id = id.into();
        let author_id = author_id.into();
        let created = created.into();
        validate_guid("comment reply", &id)?;
        validate_guid("comment reply author", &author_id)?;
        validate_rfc3339(&created)?;
        let mut text_body = CT_TextBody::new();
        text_body.set_text(text);
        Ok(Self {
            id,
            author_id,
            status: None,
            created,
            text_body: Some(text_body),
            text_body_attributes: Vec::new(),
            text_body_raw: None,
            text_body_dirty: true,
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }

    pub fn text(&self) -> String {
        self.text_body
            .as_ref()
            .map_or_else(String::new, CT_TextBody::plain_text)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_body
            .get_or_insert_with(CT_TextBody::new)
            .set_text(text);
        self.text_body_dirty = true;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comment {
    pub id: String,
    pub author_id: String,
    pub status: Option<String>,
    pub created: String,
    anchor: Vec<u8>,
    position: Option<Vec<u8>>,
    replies: Vec<CommentReply>,
    original_reply_ids: Vec<String>,
    reply_list_attributes: RawAttributes,
    reply_list_raw_children: OrderedRawChildren,
    text_body: Option<CT_TextBody>,
    text_body_attributes: RawAttributes,
    text_body_raw: Option<Vec<u8>>,
    text_body_dirty: bool,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
}

impl Comment {
    pub fn new(
        id: impl Into<String>,
        author_id: impl Into<String>,
        created: impl Into<String>,
        text: &str,
    ) -> Result<Self> {
        let id = id.into();
        let author_id = author_id.into();
        let created = created.into();
        validate_guid("comment", &id)?;
        validate_guid("comment author", &author_id)?;
        validate_rfc3339(&created)?;
        let mut text_body = CT_TextBody::new();
        text_body.set_text(text);
        Ok(Self {
            id,
            author_id,
            status: None,
            created,
            anchor: b"<p188:unknownAnchor/>".to_vec(),
            position: None,
            replies: Vec::new(),
            original_reply_ids: Vec::new(),
            reply_list_attributes: Vec::new(),
            reply_list_raw_children: OrderedRawChildren::default(),
            text_body: Some(text_body),
            text_body_attributes: Vec::new(),
            text_body_raw: None,
            text_body_dirty: true,
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
        })
    }

    /// The preserved anchor element, including its namespace declarations.
    pub fn anchor_xml(&self) -> &[u8] {
        &self.anchor
    }

    /// Anchors this comment to a drawing element, or to a range of its text.
    /// `monikers` contains the schema-ordered drawing path for a shape anchor.
    pub fn set_drawing_anchor(
        &mut self,
        slide_id: u32,
        monikers: &[(&str, u32)],
        text_range: Option<(usize, usize)>,
        text_context: Option<&str>,
    ) -> Result<()> {
        if slide_id < 256 || monikers.is_empty() {
            return Err(invalid(
                "comment anchor requires a slide and drawing element".into(),
            ));
        }
        let mut anchor = format!(
            "<ac:{} xmlns:ac=\"{}\" xmlns:pc=\"{}\"><pc:docMk/><pc:sldMk sldId=\"{}\"/>",
            if text_range.is_some() {
                "txMkLst"
            } else {
                "deMkLst"
            },
            DRAWING_COMMAND_NS,
            POWERPOINT_COMMAND_NS,
            slide_id,
        );
        if text_range.is_none() {
            for (kind, id) in monikers {
                if !matches!(
                    *kind,
                    "spMk" | "grpSpMk" | "graphicFrameMk" | "cxnSpMk" | "picMk"
                ) || *id == 0
                {
                    return Err(invalid("invalid comment drawing moniker".into()));
                }
                anchor.push_str(&format!("<ac:{kind} id=\"{id}\"/>"));
            }
        }
        if let Some((start, length)) = text_range {
            let start = i32::try_from(start)
                .map_err(|_| invalid("comment text start exceeds i32".into()))?;
            let length = i32::try_from(length)
                .map_err(|_| invalid("comment text length exceeds i32".into()))?;
            let context =
                text_context.ok_or_else(|| invalid("comment text context is missing".into()))?;
            let context_length = context.encode_utf16().count();
            let hash = context.encode_utf16().fold(0u32, |hash, code| {
                hash.wrapping_mul(33).wrapping_add(u32::from(code))
            });
            anchor.push_str(&format!("<ac:txMk cp=\"{start}\" len=\"{length}\"><ac:context len=\"{context_length}\" hash=\"{hash}\"/></ac:txMk>"));
        }
        anchor.push_str(if text_range.is_some() {
            "</ac:txMkLst>"
        } else {
            "</ac:deMkLst>"
        });
        self.anchor = anchor.into_bytes();
        Ok(())
    }

    pub fn text(&self) -> String {
        self.text_body
            .as_ref()
            .map_or_else(String::new, CT_TextBody::plain_text)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_body
            .get_or_insert_with(CT_TextBody::new)
            .set_text(text);
        self.text_body_dirty = true;
    }

    pub fn replies(&self) -> &[CommentReply] {
        &self.replies
    }

    pub fn add_reply(&mut self, reply: CommentReply) -> Result<()> {
        if reply.id == self.id || self.replies.iter().any(|existing| existing.id == reply.id) {
            return Err(invalid(format!("duplicate modern comment id {}", reply.id)));
        }
        self.replies.push(reply);
        Ok(())
    }

    pub fn move_reply(&mut self, from: usize, to: usize) -> Result<()> {
        move_item(&mut self.replies, from, to, "comment reply")
    }

    /// Removes the reply with `id` and reports whether one was removed.
    pub fn remove_reply(&mut self, id: &str) -> bool {
        let count = self.replies.len();
        self.replies.retain(|reply| reply.id != id);
        self.replies.len() != count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommentList {
    pub comments: Vec<Comment>,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
    original_comment_ids: Vec<String>,
}

impl Default for CommentList {
    fn default() -> Self {
        Self::new()
    }
}

impl CommentList {
    pub fn new() -> Self {
        Self {
            comments: Vec::new(),
            raw_attributes: Vec::new(),
            raw_children: OrderedRawChildren::default(),
            original_comment_ids: Vec::new(),
        }
    }

    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer)? {
                Event::Start(start) => {
                    let namespaces = NamespaceBindings::default().with_start(&start)?;
                    require_element(&start, &namespaces, b"cmLst", P188_NS)?;
                    let mut raw_attributes = model_attributes(&start, &["p188", "a"])?;
                    raw_attributes
                        .extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                    let (comments, raw_children) = parse_ordered_children(
                        &mut reader,
                        &namespaces,
                        b"cmLst",
                        b"cm",
                        |raw, ns| parse_comment(&raw, ns),
                    )?;
                    return Ok(Self {
                        original_comment_ids: comments
                            .iter()
                            .map(|comment| comment.id.clone())
                            .collect(),
                        comments,
                        raw_attributes,
                        raw_children,
                    });
                }
                Event::Empty(start) => {
                    let namespaces = NamespaceBindings::default().with_start(&start)?;
                    require_element(&start, &namespaces, b"cmLst", P188_NS)?;
                    let mut raw_attributes = model_attributes(&start, &["p188", "a"])?;
                    raw_attributes
                        .extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                    return Ok(Self {
                        comments: Vec::new(),
                        raw_attributes,
                        raw_children: OrderedRawChildren::default(),
                        original_comment_ids: Vec::new(),
                    });
                }
                Event::Eof => return Err(missing("p188:cmLst")),
                _ => {}
            }
            buffer.clear();
        }
    }

    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(Vec::new());
        let prefix = comment_model_prefix(&self.raw_attributes, "p188")?;
        let a_shadow = has_namespace_shadow(&self.raw_attributes, "a");
        let tag = format!("{prefix}:cmLst");
        let mut root = BytesStart::new(&tag);
        push_model_namespace(&mut root, prefix);
        if !a_shadow {
            root.push_attribute(("xmlns:a", A_NS));
        }
        write_start_with_raw(&mut writer, &root, &self.raw_attributes, false)?;
        let original_to_current = self
            .original_comment_ids
            .iter()
            .map(|id| self.comments.iter().position(|comment| comment.id == *id))
            .collect::<Vec<_>>();
        for (index, comment) in self.comments.iter().enumerate() {
            emit_raw(
                &mut writer,
                self.raw_children.at_reconciled(
                    index,
                    0,
                    &original_to_current,
                    self.comments.len(),
                ),
            )?;
            write_comment(&mut writer, comment, prefix, a_shadow)?;
        }
        emit_raw(
            &mut writer,
            self.raw_children.at_reconciled(
                self.comments.len(),
                0,
                &original_to_current,
                self.comments.len(),
            ),
        )?;
        writer.write_event(Event::End(BytesEnd::new(&tag)))?;
        Ok(writer.into_inner())
    }

    pub fn move_comment(&mut self, from: usize, to: usize) -> Result<()> {
        move_item(&mut self.comments, from, to, "comment")
    }

    /// Returns schema-owned author references, including assigned task authors.
    pub fn referenced_author_ids(&self) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        for comment in &self.comments {
            ids.push(comment.author_id.clone());
            ids.extend(comment.replies.iter().map(|reply| reply.author_id.clone()));
            ids.extend(assigned_author_ids(&comment.raw_attributes)?);
        }
        Ok(ids)
    }

    /// Rebinds a copy of this list to a copied slide.
    ///
    /// Comment and reply ids found in `ids`, and author ids found in
    /// `author_ids`, take their mapped value. Every anchor names `slide_id`,
    /// and its drawing monikers take the shape ids of `shape_ids`.
    pub fn remap_for_copied_slide(
        &mut self,
        ids: &HashMap<String, String>,
        author_ids: &HashMap<String, String>,
        slide_id: u32,
        shape_ids: &HashMap<u32, u32>,
    ) -> Result<()> {
        fn rename(id: &mut String, map: &HashMap<String, String>) {
            if let Some(mapped) = map.get(id.as_str()) {
                id.clone_from(mapped);
            }
        }
        for id in &mut self.original_comment_ids {
            rename(id, ids);
        }
        for comment in &mut self.comments {
            rename(&mut comment.id, ids);
            rename(&mut comment.author_id, author_ids);
            comment.raw_attributes = remap_assigned_authors(&comment.raw_attributes, author_ids)?;
            for id in &mut comment.original_reply_ids {
                rename(id, ids);
            }
            for reply in &mut comment.replies {
                rename(&mut reply.id, ids);
                rename(&mut reply.author_id, author_ids);
            }
            comment.anchor = remap_anchor(&comment.anchor, slide_id, shape_ids)?;
        }
        Ok(())
    }
}

fn assigned_author_ids(attributes: &[u8]) -> Result<Vec<String>> {
    let xml = [b"<record".as_slice(), attributes, b"/>"].concat();
    let mut reader = Reader::from_reader(xml.as_slice());
    let Event::Empty(start) = reader.read_event()? else {
        return Err(invalid("invalid comment attributes".into()));
    };
    let mut result = Vec::new();
    for (name, value) in all_attributes(&start)? {
        if name == "assignedTo" {
            for id in value.split_whitespace() {
                validate_guid("assigned comment author", id)?;
                result.push(id.to_owned());
            }
        }
    }
    Ok(result)
}

fn remap_assigned_authors(attributes: &[u8], authors: &HashMap<String, String>) -> Result<Vec<u8>> {
    let xml = [b"<record".as_slice(), attributes, b"/>"].concat();
    let mut reader = Reader::from_reader(xml.as_slice());
    let Event::Empty(start) = reader.read_event()? else {
        return Err(invalid("invalid comment attributes".into()));
    };
    let mut changes = Vec::new();
    for attribute in start.attributes() {
        let attribute = attribute?;
        if attribute.key.as_ref() == b"assignedTo" {
            let value = attribute.decoded_and_normalized_value(
                quick_xml::XmlVersion::Implicit1_0,
                reader.decoder(),
            )?;
            let ids = value
                .split_whitespace()
                .map(|id| {
                    authors.get(id).cloned().ok_or_else(|| {
                        invalid(format!("assigned comment author {id} was not mapped"))
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let mapped = ids.join(" ");
            if mapped != value {
                let begin = (attribute.value.as_ptr() as usize)
                    .checked_sub(xml.as_ptr() as usize)
                    .ok_or_else(|| invalid("assigned author attribute outside input".into()))?;
                changes.push((begin..begin + attribute.value.len(), mapped.into_bytes()));
            }
        }
    }
    let rewritten = classic_apply(&xml, changes)?;
    Ok(rewritten[b"<record".len()..rewritten.len() - 2].to_vec())
}

/// Rewrites the slide id of `pc:sldMk` and the shape ids of drawing monikers.
fn remap_anchor(anchor: &[u8], slide_id: u32, shape_ids: &HashMap<u32, u32>) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(anchor);
    let mut scopes = vec![NamespaceBindings::default()];
    let mut changes = Vec::new();
    loop {
        let event = reader.read_event()?;
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(start) | Event::Empty(start) => {
                let scope = scopes
                    .last()
                    .ok_or_else(|| {
                        OxmlError::InvalidValue("missing anchor namespace scope".into())
                    })?
                    .with_start(&start)?;
                let key = match (
                    scope.element_uri(start.name().as_ref()),
                    local_name(start.name().as_ref()),
                ) {
                    (Some(POWERPOINT_COMMAND_NS), b"sldMk") => Some(b"sldId".as_slice()),
                    (
                        Some(DRAWING_COMMAND_NS),
                        b"spMk" | b"grpSpMk" | b"graphicFrameMk" | b"cxnSpMk" | b"picMk",
                    ) => Some(b"id".as_slice()),
                    _ => None,
                };
                if let Some(key) = key {
                    for attribute in start.attributes() {
                        let attribute = attribute?;
                        if attribute.key.as_ref() != key {
                            continue;
                        }
                        let value = attribute.decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            reader.decoder(),
                        )?;
                        let replacement = if key == b"sldId" {
                            Some(slide_id)
                        } else {
                            value
                                .parse::<u32>()
                                .ok()
                                .and_then(|old| shape_ids.get(&old).copied())
                        };
                        if let Some(replacement) = replacement {
                            let begin = (attribute.value.as_ptr() as usize)
                                .checked_sub(anchor.as_ptr() as usize)
                                .ok_or_else(|| invalid("anchor attribute outside input".into()))?;
                            let end = begin
                                .checked_add(attribute.value.len())
                                .filter(|end| *end <= anchor.len())
                                .ok_or_else(|| invalid("anchor attribute outside input".into()))?;
                            changes.push((begin, end, replacement.to_string()));
                        }
                    }
                }
                if !empty {
                    scopes.push(scope);
                }
            }
            Event::End(_) => {
                scopes.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    let mut result = anchor.to_vec();
    for (start, end, value) in changes.into_iter().rev() {
        result.splice(start..end, value.bytes());
    }
    Ok(result)
}

fn parse_author(xml: &[u8], inherited: &NamespaceBindings) -> Result<CommentAuthor> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(start) => {
                let namespaces = inherited.with_start(&start)?;
                require_element(&start, &namespaces, b"author", P188_NS)?;
                let mut fields = AuthorFields::from_start(&start, xml, &namespaces)?;
                fields.raw_children = capture_all_children(&mut reader, b"author")?;
                return fields.finish();
            }
            Event::Empty(start) => {
                let namespaces = inherited.with_start(&start)?;
                return AuthorFields::from_start(&start, xml, &namespaces)?.finish();
            }
            Event::Eof => return Err(missing("p188:author")),
            _ => {}
        }
        buffer.clear();
    }
}

#[derive(Default)]
struct AuthorFields {
    id: Option<String>,
    name: Option<String>,
    initials: Option<String>,
    user_id: Option<String>,
    provider_id: Option<String>,
    raw_attributes: RawAttributes,
    raw_children: OrderedRawChildren,
}

impl AuthorFields {
    fn from_start(
        start: &BytesStart<'_>,
        xml: &[u8],
        inherited: &NamespaceBindings,
    ) -> Result<Self> {
        let mut fields = Self::default();
        for (name, value) in all_attributes(start)? {
            match name.as_str() {
                "id" => set_once(&mut fields.id, value, "author @id")?,
                "name" => set_once(&mut fields.name, value, "author @name")?,
                "initials" => set_once(&mut fields.initials, value, "author @initials")?,
                "userId" => set_once(&mut fields.user_id, value, "author @userId")?,
                "providerId" => set_once(&mut fields.provider_id, value, "author @providerId")?,
                _ => {}
            }
        }
        fields.raw_attributes =
            model_attributes(start, &["id", "name", "initials", "userId", "providerId"])?;
        fields
            .raw_attributes
            .extend_from_slice(&dependent_fixed_shadow_attributes(start, xml)?);
        fields
            .raw_attributes
            .extend_from_slice(&inherited_fixed_shadow_attributes(
                inherited,
                start,
                xml,
                &fields.raw_attributes,
            ));
        Ok(fields)
    }

    fn finish(self) -> Result<CommentAuthor> {
        let id = required(self.id, "author @id")?;
        validate_guid("comment author", &id)?;
        Ok(CommentAuthor {
            id,
            name: required(self.name, "author @name")?,
            initials: self.initials,
            user_id: required(self.user_id, "author @userId")?,
            provider_id: required(self.provider_id, "author @providerId")?,
            raw_attributes: self.raw_attributes,
            raw_children: self.raw_children,
        })
    }
}

fn parse_comment(xml: &[u8], inherited: &NamespaceBindings) -> Result<Comment> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(start) => {
                let namespaces = inherited.with_start(&start)?;
                require_element(&start, &namespaces, b"cm", P188_NS)?;
                let attributes = parse_comment_attributes(&start, "comment", xml, &namespaces)?;
                return parse_comment_children(&mut reader, &namespaces, attributes);
            }
            Event::Empty(_) => return Err(missing("modern comment anchor")),
            Event::Eof => return Err(missing("p188:cm")),
            _ => {}
        }
        buffer.clear();
    }
}

struct CommentAttributes {
    id: String,
    author_id: String,
    status: Option<String>,
    created: String,
    raw: RawAttributes,
}

fn parse_comment_attributes(
    start: &BytesStart<'_>,
    kind: &str,
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<CommentAttributes> {
    let mut id = None;
    let mut author_id = None;
    let mut status = None;
    let mut created = None;
    for (name, value) in all_attributes(start)? {
        match name.as_str() {
            "id" => set_once(&mut id, value, &format!("{kind} @id"))?,
            "authorId" => set_once(&mut author_id, value, &format!("{kind} @authorId"))?,
            "status" => set_once(&mut status, value, &format!("{kind} @status"))?,
            "created" => set_once(&mut created, value, &format!("{kind} @created"))?,
            _ => {}
        }
    }
    let id = required(id, &format!("{kind} @id"))?;
    let author_id = required(author_id, &format!("{kind} @authorId"))?;
    let created = required(created, &format!("{kind} @created"))?;
    validate_guid(kind, &id)?;
    validate_guid(&format!("{kind} author"), &author_id)?;
    validate_rfc3339(&created)?;
    validate_status(status.as_deref())?;
    let mut raw = model_attributes(start, &["id", "authorId", "status", "created"])?;
    raw.extend_from_slice(&dependent_fixed_shadow_attributes(start, xml)?);
    raw.extend_from_slice(&inherited_fixed_shadow_attributes(
        inherited, start, xml, &raw,
    ));
    Ok(CommentAttributes {
        id,
        author_id,
        status,
        created,
        raw,
    })
}

fn parse_comment_children(
    reader: &mut Reader<&[u8]>,
    inherited: &NamespaceBindings,
    attributes: CommentAttributes,
) -> Result<Comment> {
    let mut anchor = None;
    let mut position = None;
    let mut replies = None;
    let mut reply_list_attributes = Vec::new();
    let mut reply_list_raw_children = OrderedRawChildren::default();
    let mut text_body = None;
    let mut text_body_attributes = Vec::new();
    let mut text_body_raw = None;
    let mut raw_children = OrderedRawChildren::default();
    let mut boundary = 0usize;
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(child) => {
                let namespaces = inherited.with_start(&child)?;
                let raw = capture_element(reader, &child)?;
                let qualified_name = child.name().as_ref().to_vec();
                let local = local_name(&qualified_name);
                let uri = namespaces.element_uri(child.name().as_ref());
                if boundary == 0 && is_comment_anchor(local, uri) {
                    anchor = Some(raw);
                    boundary = 1;
                } else if local == b"pos"
                    && uri == Some(P188_NS)
                    && boundary <= 1
                    && position.is_none()
                {
                    position = Some(raw);
                    boundary = 2;
                } else if local == b"replyLst"
                    && uri == Some(P188_NS)
                    && boundary <= 2
                    && replies.is_none()
                {
                    let parsed = parse_reply_list(&raw, &namespaces)?;
                    replies = Some(parsed.0);
                    reply_list_attributes = parsed.1;
                    reply_list_raw_children = parsed.2;
                    boundary = 3;
                } else if local == b"txBody"
                    && uri == Some(P188_NS)
                    && boundary <= 3
                    && text_body.is_none()
                {
                    text_body_attributes = text_body_root_attributes(&child, &raw)?;
                    text_body = Some(
                        CT_TextBody::from_xml_with_inherited_namespaces(
                            &raw,
                            &namespaces.entries(),
                        )
                        .map_err(text_error)?,
                    );
                    text_body_raw = Some(raw);
                    boundary = 4;
                } else if local == b"extLst" && uri == Some(P_NS) {
                    raw_children.push(4, raw);
                    boundary = 4;
                } else {
                    raw_children.push(boundary, raw);
                }
            }
            Event::Empty(child) => {
                let namespaces = inherited.with_start(&child)?;
                let raw = capture_empty_element(&child)?;
                let qualified_name = child.name().as_ref().to_vec();
                let local = local_name(&qualified_name);
                let uri = namespaces.element_uri(child.name().as_ref());
                if boundary == 0 && is_comment_anchor(local, uri) {
                    anchor = Some(raw);
                    boundary = 1;
                } else if local == b"pos"
                    && uri == Some(P188_NS)
                    && boundary <= 1
                    && position.is_none()
                {
                    position = Some(raw);
                    boundary = 2;
                } else if local == b"replyLst"
                    && uri == Some(P188_NS)
                    && boundary <= 2
                    && replies.is_none()
                {
                    let parsed = parse_reply_list(&raw, &namespaces)?;
                    replies = Some(parsed.0);
                    reply_list_attributes = parsed.1;
                    reply_list_raw_children = parsed.2;
                    boundary = 3;
                } else if local == b"txBody" && uri == Some(P188_NS) {
                    return Err(missing("comment text body properties"));
                } else if local == b"extLst" && uri == Some(P_NS) {
                    raw_children.push(4, raw);
                    boundary = 4;
                } else {
                    raw_children.push(boundary, raw);
                }
            }
            Event::End(end) if local_name(end.name().as_ref()) == b"cm" => break,
            Event::Eof => return Err(missing("closing p188:cm")),
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => raw_children.push(boundary, capture_event(event)?),
            _ => {}
        }
        buffer.clear();
    }
    let replies = replies.unwrap_or_default();
    let original_reply_ids = replies.iter().map(|reply| reply.id.clone()).collect();
    Ok(Comment {
        id: attributes.id,
        author_id: attributes.author_id,
        status: attributes.status,
        created: attributes.created,
        anchor: required(anchor, "modern comment anchor")?,
        position,
        replies,
        original_reply_ids,
        reply_list_attributes,
        reply_list_raw_children,
        text_body,
        text_body_attributes,
        text_body_raw,
        text_body_dirty: false,
        raw_attributes: attributes.raw,
        raw_children,
    })
}

fn parse_reply_list(
    xml: &[u8],
    inherited: &NamespaceBindings,
) -> Result<(Vec<CommentReply>, RawAttributes, OrderedRawChildren)> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(start) => {
                let namespaces = inherited.with_start(&start)?;
                let mut attributes = model_attributes(&start, &["p188", "a"])?;
                attributes.extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                attributes.extend_from_slice(&inherited_fixed_shadow_attributes(
                    &namespaces,
                    &start,
                    xml,
                    &attributes,
                ));
                let (items, children) = parse_ordered_children(
                    &mut reader,
                    &namespaces,
                    b"replyLst",
                    b"reply",
                    |raw, ns| parse_reply(&raw, ns),
                )?;
                return Ok((items, attributes, children));
            }
            Event::Empty(start) => {
                let namespaces = inherited.with_start(&start)?;
                let mut attributes = model_attributes(&start, &["p188", "a"])?;
                attributes.extend_from_slice(&dependent_fixed_shadow_attributes(&start, xml)?);
                attributes.extend_from_slice(&inherited_fixed_shadow_attributes(
                    &namespaces,
                    &start,
                    xml,
                    &attributes,
                ));
                return Ok((Vec::new(), attributes, OrderedRawChildren::default()));
            }
            Event::Eof => return Err(missing("p188:replyLst")),
            _ => {}
        }
        buffer.clear();
    }
}

fn parse_reply(xml: &[u8], inherited: &NamespaceBindings) -> Result<CommentReply> {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(start) => {
                let namespaces = inherited.with_start(&start)?;
                require_element(&start, &namespaces, b"reply", P188_NS)?;
                let attributes =
                    parse_comment_attributes(&start, "comment reply", xml, &namespaces)?;
                let mut text_body = None;
                let mut text_body_attributes = Vec::new();
                let mut text_body_raw = None;
                let mut raw_children = OrderedRawChildren::default();
                let mut boundary = 0;
                loop {
                    buffer.clear();
                    match reader.read_event_into(&mut buffer)? {
                        Event::Start(child) => {
                            let child_ns = namespaces.with_start(&child)?;
                            let raw = capture_element(&mut reader, &child)?;
                            if local_name(child.name().as_ref()) == b"txBody"
                                && child_ns.element_uri(child.name().as_ref()) == Some(P188_NS)
                                && text_body.is_none()
                            {
                                text_body_attributes = text_body_root_attributes(&child, &raw)?;
                                text_body = Some(
                                    CT_TextBody::from_xml_with_inherited_namespaces(
                                        &raw,
                                        &child_ns.entries(),
                                    )
                                    .map_err(text_error)?,
                                );
                                text_body_raw = Some(raw);
                                boundary = 1;
                            } else if local_name(child.name().as_ref()) == b"extLst"
                                && child_ns.element_uri(child.name().as_ref()) == Some(P_NS)
                            {
                                raw_children.push(1, raw);
                                boundary = 1;
                            } else {
                                raw_children.push(boundary, raw);
                            }
                        }
                        Event::Empty(child) => {
                            let child_ns = namespaces.with_start(&child)?;
                            let raw = capture_empty_element(&child)?;
                            if local_name(child.name().as_ref()) == b"extLst"
                                && child_ns.element_uri(child.name().as_ref()) == Some(P_NS)
                            {
                                raw_children.push(1, raw);
                                boundary = 1;
                            } else {
                                raw_children.push(boundary, raw);
                            }
                        }
                        Event::End(end) if local_name(end.name().as_ref()) == b"reply" => break,
                        Event::Eof => return Err(missing("closing p188:reply")),
                        event @ (Event::Text(_)
                        | Event::CData(_)
                        | Event::Comment(_)
                        | Event::PI(_)
                        | Event::DocType(_)) => raw_children.push(boundary, capture_event(event)?),
                        _ => {}
                    }
                }
                return Ok(CommentReply {
                    id: attributes.id,
                    author_id: attributes.author_id,
                    status: attributes.status,
                    created: attributes.created,
                    text_body,
                    text_body_attributes,
                    text_body_raw,
                    text_body_dirty: false,
                    raw_attributes: attributes.raw,
                    raw_children,
                });
            }
            Event::Empty(start) => {
                let namespaces = inherited.with_start(&start)?;
                let attributes =
                    parse_comment_attributes(&start, "comment reply", xml, &namespaces)?;
                return Ok(CommentReply {
                    id: attributes.id,
                    author_id: attributes.author_id,
                    status: attributes.status,
                    created: attributes.created,
                    text_body: None,
                    text_body_attributes: Vec::new(),
                    text_body_raw: None,
                    text_body_dirty: false,
                    raw_attributes: attributes.raw,
                    raw_children: OrderedRawChildren::default(),
                });
            }
            Event::Eof => return Err(missing("p188:reply")),
            _ => {}
        }
        buffer.clear();
    }
}

fn write_author<W: Write>(
    writer: &mut Writer<W>,
    author: &CommentAuthor,
    inherited_prefix: &str,
) -> Result<()> {
    validate_guid("comment author", &author.id)?;
    let prefix = comment_model_prefix(&author.raw_attributes, inherited_prefix)?;
    let tag = format!("{prefix}:author");
    let mut start = BytesStart::new(&tag);
    if prefix != inherited_prefix {
        push_model_namespace(&mut start, prefix);
    }
    start.push_attribute(("id", author.id.as_str()));
    start.push_attribute(("name", author.name.as_str()));
    if let Some(initials) = &author.initials {
        start.push_attribute(("initials", initials.as_str()));
    }
    start.push_attribute(("userId", author.user_id.as_str()));
    start.push_attribute(("providerId", author.provider_id.as_str()));
    write_shell(
        writer,
        start,
        &author.raw_attributes,
        &tag,
        &author.raw_children,
    )
}

fn write_comment<W: Write>(
    writer: &mut Writer<W>,
    comment: &Comment,
    inherited_prefix: &str,
    inherited_a_shadow: bool,
) -> Result<()> {
    validate_guid("comment", &comment.id)?;
    validate_guid("comment author", &comment.author_id)?;
    validate_rfc3339(&comment.created)?;
    validate_status(comment.status.as_deref())?;
    let prefix = comment_model_prefix(&comment.raw_attributes, inherited_prefix)?;
    let a_shadow = inherited_a_shadow || has_namespace_shadow(&comment.raw_attributes, "a");
    let tag = format!("{prefix}:cm");
    let mut start = BytesStart::new(&tag);
    if prefix != inherited_prefix {
        push_model_namespace(&mut start, prefix);
    }
    push_comment_attributes(
        &mut start,
        &comment.id,
        &comment.author_id,
        comment.status.as_deref(),
        &comment.created,
    );
    write_start_with_raw(writer, &start, &comment.raw_attributes, false)?;
    emit_raw(writer, comment.raw_children.at(0))?;
    if prefix != "p188" && comment.anchor == b"<p188:unknownAnchor/>" {
        writer
            .get_mut()
            .write_all(format!("<{prefix}:unknownAnchor/>").as_bytes())?;
    } else {
        writer.get_mut().write_all(&comment.anchor)?;
    }
    emit_raw(writer, comment.raw_children.at(1))?;
    if let Some(position) = &comment.position {
        writer.get_mut().write_all(position)?;
    }
    emit_raw(writer, comment.raw_children.at(2))?;
    if !comment.replies.is_empty()
        || !comment.reply_list_attributes.is_empty()
        || !comment.reply_list_raw_children.is_empty()
    {
        let list_prefix = comment_model_prefix(&comment.reply_list_attributes, prefix)?;
        let list_a_shadow = a_shadow || has_namespace_shadow(&comment.reply_list_attributes, "a");
        let list_tag = format!("{list_prefix}:replyLst");
        let mut list = BytesStart::new(&list_tag);
        if list_prefix != prefix {
            push_model_namespace(&mut list, list_prefix);
        }
        write_start_with_raw(writer, &list, &comment.reply_list_attributes, false)?;
        let original_to_current = comment
            .original_reply_ids
            .iter()
            .map(|id| comment.replies.iter().position(|reply| reply.id == *id))
            .collect::<Vec<_>>();
        for (index, reply) in comment.replies.iter().enumerate() {
            emit_raw(
                writer,
                comment.reply_list_raw_children.at_reconciled(
                    index,
                    0,
                    &original_to_current,
                    comment.replies.len(),
                ),
            )?;
            write_reply(writer, reply, list_prefix, list_a_shadow)?;
        }
        emit_raw(
            writer,
            comment.reply_list_raw_children.at_reconciled(
                comment.replies.len(),
                0,
                &original_to_current,
                comment.replies.len(),
            ),
        )?;
        writer.write_event(Event::End(BytesEnd::new(&list_tag)))?;
    }
    emit_raw(writer, comment.raw_children.at(3))?;
    if let Some(text_body) = &comment.text_body {
        if !comment.text_body_dirty
            && let Some(raw) = &comment.text_body_raw
        {
            writer.get_mut().write_all(raw)?;
        } else {
            write_text_body(
                writer,
                text_body,
                &comment.text_body_attributes,
                comment.text_body_raw.as_deref(),
                prefix,
                a_shadow,
            )?;
        }
    }
    emit_raw(writer, comment.raw_children.at(4))?;
    writer.write_event(Event::End(BytesEnd::new(&tag)))?;
    Ok(())
}

fn write_reply<W: Write>(
    writer: &mut Writer<W>,
    reply: &CommentReply,
    inherited_prefix: &str,
    inherited_a_shadow: bool,
) -> Result<()> {
    validate_guid("comment reply", &reply.id)?;
    validate_guid("comment reply author", &reply.author_id)?;
    validate_rfc3339(&reply.created)?;
    validate_status(reply.status.as_deref())?;
    let prefix = comment_model_prefix(&reply.raw_attributes, inherited_prefix)?;
    let tag = format!("{prefix}:reply");
    let mut start = BytesStart::new(&tag);
    if prefix != inherited_prefix {
        push_model_namespace(&mut start, prefix);
    }
    push_comment_attributes(
        &mut start,
        &reply.id,
        &reply.author_id,
        reply.status.as_deref(),
        &reply.created,
    );
    write_start_with_raw(writer, &start, &reply.raw_attributes, false)?;
    emit_raw(writer, reply.raw_children.at(0))?;
    if let Some(text_body) = &reply.text_body {
        if !reply.text_body_dirty
            && let Some(raw) = &reply.text_body_raw
        {
            writer.get_mut().write_all(raw)?;
        } else {
            write_text_body(
                writer,
                text_body,
                &reply.text_body_attributes,
                reply.text_body_raw.as_deref(),
                prefix,
                inherited_a_shadow || has_namespace_shadow(&reply.raw_attributes, "a"),
            )?;
        }
    }
    emit_raw(writer, reply.raw_children.at(1))?;
    writer.write_event(Event::End(BytesEnd::new(&tag)))?;
    Ok(())
}

fn push_comment_attributes(
    start: &mut BytesStart<'_>,
    id: &str,
    author_id: &str,
    status: Option<&str>,
    created: &str,
) {
    start.push_attribute(("id", id));
    start.push_attribute(("authorId", author_id));
    if let Some(status) = status {
        start.push_attribute(("status", status));
    }
    start.push_attribute(("created", created));
}

fn parse_ordered_children<T>(
    reader: &mut Reader<&[u8]>,
    inherited: &NamespaceBindings,
    closing: &[u8],
    child_name: &[u8],
    mut parse: impl FnMut(Vec<u8>, &NamespaceBindings) -> Result<T>,
) -> Result<(Vec<T>, OrderedRawChildren)> {
    let mut items = Vec::new();
    let mut raw_children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(child) => {
                let namespaces = inherited.with_start(&child)?;
                let raw = capture_element(reader, &child)?;
                if local_name(child.name().as_ref()) == child_name
                    && namespaces.element_uri(child.name().as_ref()) == Some(P188_NS)
                {
                    items.push(parse(raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::Empty(child) => {
                let namespaces = inherited.with_start(&child)?;
                let raw = capture_empty_element(&child)?;
                if local_name(child.name().as_ref()) == child_name
                    && namespaces.element_uri(child.name().as_ref()) == Some(P188_NS)
                {
                    items.push(parse(raw, &namespaces)?);
                } else {
                    raw_children.push(items.len(), raw);
                }
            }
            Event::End(end) if local_name(end.name().as_ref()) == closing => break,
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => raw_children.push(items.len(), capture_event(event)?),
            Event::Eof => {
                return Err(missing(&format!(
                    "closing {}",
                    String::from_utf8_lossy(closing)
                )));
            }
            _ => {}
        }
        buffer.clear();
    }
    Ok((items, raw_children))
}

fn capture_all_children(reader: &mut Reader<&[u8]>, closing: &[u8]) -> Result<OrderedRawChildren> {
    let mut children = OrderedRawChildren::default();
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer)? {
            Event::Start(child) => children.push(0, capture_element(reader, &child)?),
            Event::Empty(child) => children.push(0, capture_empty_element(&child)?),
            Event::End(end) if local_name(end.name().as_ref()) == closing => break,
            event @ (Event::Text(_)
            | Event::CData(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)) => children.push(0, capture_event(event)?),
            Event::Eof => {
                return Err(missing(&format!(
                    "closing {}",
                    String::from_utf8_lossy(closing)
                )));
            }
            _ => {}
        }
        buffer.clear();
    }
    Ok(children)
}

fn write_shell<W: Write>(
    writer: &mut Writer<W>,
    start: BytesStart<'_>,
    raw_attributes: &RawAttributes,
    tag: &str,
    children: &OrderedRawChildren,
) -> Result<()> {
    if children.is_empty() {
        write_start_with_raw(writer, &start, raw_attributes, true)?;
    } else {
        write_start_with_raw(writer, &start, raw_attributes, false)?;
        emit_raw(writer, children.at(0))?;
        writer.write_event(Event::End(BytesEnd::new(tag)))?;
    }
    Ok(())
}

fn is_comment_anchor(local: &[u8], uri: Option<&str>) -> bool {
    (local == b"unknownAnchor" && uri == Some(P188_NS))
        || (local == b"sldMkLst" && uri == Some(POWERPOINT_COMMAND_NS))
        || (matches!(local, b"deMkLst" | b"txMkLst") && uri == Some(DRAWING_COMMAND_NS))
}

fn model_attributes(start: &BytesStart<'_>, modeled: &[&str]) -> Result<RawAttributes> {
    let _ = all_attributes(start)?;
    let raw = start.attributes_raw();
    let mut preserved = Vec::new();
    let mut cursor = 0usize;
    while cursor < raw.len() {
        let span_start = cursor;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == raw.len() {
            preserved.extend_from_slice(&raw[span_start..]);
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
        if raw.get(cursor) != Some(&b'=') {
            return Err(invalid("malformed raw comment attribute".to_owned()));
        }
        cursor += 1;
        while cursor < raw.len() && raw[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        let quote = *raw
            .get(cursor)
            .filter(|byte| matches!(byte, b'\'' | b'"'))
            .ok_or_else(|| invalid("malformed raw comment attribute value".to_owned()))?;
        cursor += 1;
        while cursor < raw.len() && raw[cursor] != quote {
            cursor += 1;
        }
        if cursor == raw.len() {
            return Err(invalid(
                "unterminated raw comment attribute value".to_owned(),
            ));
        }
        cursor += 1;
        let modeled =
            !name.contains(&b':') && modeled.iter().any(|candidate| name == candidate.as_bytes());
        let fixed_namespace = matches!(
            name,
            b"xmlns:p188" | b"xmlns:p188m" | b"xmlns:p188model" | b"xmlns:a"
        );
        if !modeled && !fixed_namespace {
            preserved.extend_from_slice(&raw[span_start..cursor]);
        }
    }
    Ok(preserved)
}

fn comment_model_prefix<'a>(raw_attributes: &RawAttributes, inherited: &'a str) -> Result<&'a str> {
    if !has_namespace_shadow(raw_attributes, inherited) {
        return Ok(inherited);
    }
    ["p188", "p188m", "p188model"]
        .into_iter()
        .find(|prefix| !has_namespace_shadow(raw_attributes, prefix))
        .ok_or_else(|| invalid("no unshadowed modern comment model prefix is available".to_owned()))
}

fn push_model_namespace(start: &mut BytesStart<'_>, prefix: &str) {
    match prefix {
        "p188" => start.push_attribute(("xmlns:p188", P188_NS)),
        "p188m" => start.push_attribute(("xmlns:p188m", P188_NS)),
        "p188model" => start.push_attribute(("xmlns:p188model", P188_NS)),
        _ => unreachable!("comment model prefix is selected from fixed candidates"),
    }
}

fn has_namespace_shadow(raw_attributes: &RawAttributes, prefix: &str) -> bool {
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

fn text_body_root_attributes(start: &BytesStart<'_>, xml: &[u8]) -> Result<RawAttributes> {
    let mut attributes = model_attributes(start, &["p188", "a"])?;
    attributes.extend_from_slice(&dependent_fixed_shadow_attributes(start, xml)?);
    Ok(attributes)
}

fn write_text_body<W: Write>(
    writer: &mut Writer<W>,
    text_body: &CT_TextBody,
    raw_attributes: &RawAttributes,
    original_xml: Option<&[u8]>,
    inherited_prefix: &str,
    a_shadow: bool,
) -> Result<()> {
    if has_namespace_shadow(raw_attributes, "a") {
        return Err(invalid(
            "comment text body cannot preserve a producer-owned a prefix".to_owned(),
        ));
    }
    if a_shadow && original_xml.is_some_and(text_body_depends_on_inherited_a) {
        return Err(invalid(
            "cannot rewrite comment text body with producer-owned a descendants".to_owned(),
        ));
    }
    let prefix = comment_model_prefix(raw_attributes, inherited_prefix)?;
    let mut temporary = Writer::new(Vec::new());
    text_body
        .write_xml_as(&mut temporary, &format!("{prefix}:txBody"))
        .map_err(text_error)?;
    let xml = temporary.into_inner();
    if !raw_attributes.is_empty() || a_shadow || prefix != inherited_prefix {
        let closing = xml
            .iter()
            .position(|byte| *byte == b'>')
            .ok_or_else(|| invalid("malformed comment text body".to_owned()))?;
        writer.get_mut().write_all(&xml[..closing])?;
        if prefix != inherited_prefix {
            writer
                .get_mut()
                .write_all(format!(" xmlns:{prefix}=\"{P188_NS}\"").as_bytes())?;
        }
        writer.get_mut().write_all(raw_attributes)?;
        if a_shadow {
            writer
                .get_mut()
                .write_all(format!(" xmlns:a=\"{A_NS}\"").as_bytes())?;
        }
        writer.get_mut().write_all(&xml[closing..])?;
    } else {
        writer.get_mut().write_all(&xml)?;
    }
    Ok(())
}

fn text_body_depends_on_inherited_a(xml: &[u8]) -> bool {
    let mut reader = Reader::from_reader(xml);
    let mut buffer = Vec::new();
    let locally_declared = match reader.read_event_into(&mut buffer) {
        Ok(Event::Start(start)) | Ok(Event::Empty(start)) => start
            .attributes()
            .with_checks(false)
            .any(|attribute| attribute.is_ok_and(|attribute| attribute.key.as_ref() == b"xmlns:a")),
        _ => return false,
    };
    if locally_declared {
        return false;
    }
    let start = reader.buffer_position() as usize;
    descendant_depends_on_prefix(&xml[start..], b"a")
}

fn dependent_fixed_shadow_attributes(start: &BytesStart<'_>, xml: &[u8]) -> Result<RawAttributes> {
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
            .ok_or_else(|| invalid("malformed fixed namespace shadow".to_owned()))?;
        cursor += 1;
        while cursor < raw.len() && raw[cursor] != quote {
            cursor += 1;
        }
        cursor += 1;
        let (prefix, expected) = match name {
            b"xmlns:p188" => (b"p188".as_slice(), P188_NS),
            b"xmlns:p188m" => (b"p188m".as_slice(), P188_NS),
            b"xmlns:p188model" => (b"p188model".as_slice(), P188_NS),
            b"xmlns:a" => (b"a".as_slice(), A_NS),
            _ => continue,
        };
        let value = decoded
            .iter()
            .find(|(candidate, _)| candidate.as_bytes() == name)
            .map(|(_, value)| value.as_str());
        if value != Some(expected)
            && (shell_attribute_depends_on_prefix(start, prefix)
                || descendant_depends_on_prefix(descendants, prefix))
        {
            preserved.extend_from_slice(&raw[span_start..cursor]);
        }
    }
    Ok(preserved)
}

fn inherited_fixed_shadow_attributes(
    inherited: &NamespaceBindings,
    start: &BytesStart<'_>,
    xml: &[u8],
    local_attributes: &RawAttributes,
) -> RawAttributes {
    let descendants = xml
        .iter()
        .position(|byte| *byte == b'>')
        .map_or(&[][..], |index| &xml[index + 1..]);
    let mut preserved = Vec::new();
    for (prefix, uri) in inherited.entries() {
        let expected = match prefix.as_str() {
            "p188" | "p188m" | "p188model" => P188_NS,
            "a" => A_NS,
            _ => continue,
        };
        if uri == expected
            || has_namespace_shadow(local_attributes, &prefix)
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

fn require_element(
    start: &BytesStart<'_>,
    namespaces: &NamespaceBindings,
    local: &[u8],
    uri: &str,
) -> Result<()> {
    if local_name(start.name().as_ref()) == local
        && namespaces.element_uri(start.name().as_ref()) == Some(uri)
    {
        Ok(())
    } else {
        Err(OxmlError::UnexpectedElement(
            String::from_utf8_lossy(start.name().as_ref()).into_owned(),
        ))
    }
}

fn move_item<T>(items: &mut Vec<T>, from: usize, to: usize, kind: &str) -> Result<()> {
    if from >= items.len() || to >= items.len() {
        return Err(invalid(format!("{kind} move index is out of range")));
    }
    if from != to {
        let item = items.remove(from);
        items.insert(to, item);
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
        Err(invalid(format!("{kind} id is not a braced GUID: {value}")))
    }
}

fn validate_rfc3339(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    let basic = bytes.len() >= 20
        && bytes.is_ascii()
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes
            .get(10)
            .is_some_and(|byte| matches!(byte, b'T' | b't'))
        && bytes.get(13) == Some(&b':')
        && bytes.get(16) == Some(&b':')
        && bytes[..19]
            .iter()
            .enumerate()
            .all(|(index, byte)| [4, 7, 10, 13, 16].contains(&index) || byte.is_ascii_digit());
    let time_end = bytes
        .get(19..)
        .and_then(|tail| {
            tail.iter()
                .position(|byte| matches!(byte, b'Z' | b'z' | b'+' | b'-'))
        })
        .map(|index| 19 + index);
    let timezone_ok = time_end.is_some_and(|index| {
        let zone = &bytes[index..];
        matches!(zone, b"Z" | b"z")
            || (zone.len() == 6
                && matches!(zone[0], b'+' | b'-')
                && zone[3] == b':'
                && parse_digits(&zone[1..3]).is_some_and(|hour| hour <= 23)
                && parse_digits(&zone[4..6]).is_some_and(|minute| minute <= 59))
    });
    let fraction_ok = time_end.is_some_and(|index| {
        index == 19
            || (bytes.get(19) == Some(&b'.')
                && index > 20
                && bytes[20..index].iter().all(|byte| byte.is_ascii_digit()))
    });
    let date_time_ok = basic
        && parse_digits(&bytes[0..4]).is_some_and(|year| {
            parse_digits(&bytes[5..7]).is_some_and(|month| {
                parse_digits(&bytes[8..10])
                    .is_some_and(|day| day != 0 && day <= days_in_month(year, month))
            })
        })
        && parse_digits(&bytes[11..13]).is_some_and(|hour| hour <= 23)
        && parse_digits(&bytes[14..16]).is_some_and(|minute| minute <= 59)
        && parse_digits(&bytes[17..19]).is_some_and(|second| second <= 60);
    if date_time_ok && timezone_ok && fraction_ok {
        Ok(())
    } else {
        Err(invalid(format!(
            "comment timestamp is not RFC 3339: {value}"
        )))
    }
}

fn validate_status(status: Option<&str>) -> Result<()> {
    if status.is_none_or(|value| matches!(value, "active" | "resolved" | "closed")) {
        Ok(())
    } else {
        Err(invalid(format!(
            "comment status is not active, resolved, or closed: {}",
            status.unwrap_or_default()
        )))
    }
}

fn parse_digits(bytes: &[u8]) -> Option<u32> {
    bytes.iter().try_fold(0_u32, |value, byte| {
        byte.is_ascii_digit()
            .then(|| value * 10 + u32::from(*byte - b'0'))
    })
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => 0,
    }
}

fn text_error(error: impl std::fmt::Display) -> OxmlError {
    invalid(error.to_string())
}

fn set_once<T>(slot: &mut Option<T>, value: T, name: &str) -> Result<()> {
    if slot.replace(value).is_some() {
        Err(invalid(format!("duplicate {name}")))
    } else {
        Ok(())
    }
}

fn required<T>(value: Option<T>, name: &str) -> Result<T> {
    value.ok_or_else(|| missing(name))
}

fn write_start_with_raw<W: Write>(
    writer: &mut Writer<W>,
    start: &BytesStart<'_>,
    raw_attributes: &RawAttributes,
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

fn capture_event(event: Event<'_>) -> Result<Vec<u8>> {
    let mut writer = Writer::new(Vec::new());
    writer.write_event(event.into_owned())?;
    Ok(writer.into_inner())
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

fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn missing(name: &str) -> OxmlError {
    OxmlError::MissingElement(name.to_owned())
}

fn invalid(message: String) -> OxmlError {
    OxmlError::InvalidValue(message)
}

const P15_NS: &str = "http://schemas.microsoft.com/office/powerpoint/2012/main";

#[derive(Clone)]
struct ClassicElement {
    start: usize,
    end: usize,
    attributes: HashMap<String, (String, std::ops::Range<usize>)>,
    namespaces: NamespaceBindings,
}

struct ClassicList {
    elements: Vec<ClassicElement>,
    insert_at: usize,
}

#[derive(Eq, PartialEq)]
struct ClassicIdentity {
    attributes: Vec<(String, String)>,
    namespaces: Vec<(String, String)>,
    payload: Vec<u8>,
}

/// One required parent comment with its original relationship scope.
pub struct ClassicCommentDependency {
    pub source_part: String,
    pub xml: Vec<u8>,
}

/// Resolves parent-comment closure without importing unrelated slides or comments.
pub fn classic_comment_dependencies(
    selected: &[(String, Vec<u8>)],
    all_parts: &[(String, Vec<u8>)],
) -> Result<HashMap<String, Vec<ClassicCommentDependency>>> {
    let mut placed = std::collections::HashSet::new();
    let mut queues = Vec::new();
    for (part, xml) in selected {
        for comment in classic_list(xml, b"cmLst", b"cm")?.elements {
            placed.insert((
                classic_number(&comment, "authorId")?,
                classic_number(&comment, "idx")?,
            ));
        }
        let parents = classic_parent_references(xml)?
            .into_iter()
            .map(|parent| classic_parent_identity(&parent))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        queues.push((part, parents));
    }
    if queues.iter().all(|(_, parents)| parents.is_empty()) {
        return Ok(HashMap::new());
    }
    let mut available = HashMap::new();
    for (part, xml) in all_parts {
        for comment in classic_list(xml, b"cmLst", b"cm")?.elements {
            let key = (
                classic_number(&comment, "authorId")?,
                classic_number(&comment, "idx")?,
            );
            if available.insert(key, (part, xml, comment)).is_some() {
                return Err(invalid("duplicate source classic parent identity".into()));
            }
        }
    }
    let mut output = HashMap::new();
    for (part, parents) in queues {
        let mut pending = std::collections::VecDeque::from(parents);
        let mut added = Vec::new();
        while let Some(key) = pending.pop_front() {
            if !placed.insert(key) {
                continue;
            }
            let (origin, xml, comment) = available.get(&key).ok_or_else(|| {
                invalid(format!(
                    "classic parent comment {},{} is absent",
                    key.0, key.1
                ))
            })?;
            let raw = close_classic_element(&xml[comment.start..comment.end], &comment.namespaces)?;
            // A single comment is wrapped only to reuse the exact schema-path reader.
            let wrapped = [
                format!("<p:cmLst xmlns:p=\"{P_NS}\">").as_bytes(),
                raw.as_slice(),
                b"</p:cmLst>",
            ]
            .concat();
            for parent in classic_parent_references(&wrapped)? {
                if let Some(key) = classic_parent_identity(&parent)? {
                    pending.push_back(key);
                }
            }
            added.push(ClassicCommentDependency {
                source_part: (*origin).clone(),
                xml: raw,
            });
        }
        if !added.is_empty() {
            output.insert((*part).clone(), added);
        }
    }
    Ok(output)
}

/// Appends an already closed and relationship-rebound parent fragment.
pub fn append_classic_comment_dependency(xml: &[u8], dependency: &[u8]) -> Result<Vec<u8>> {
    let list = classic_list(xml, b"cmLst", b"cm")?;
    classic_apply(
        xml,
        vec![(list.insert_at..list.insert_at, dependency.to_vec())],
    )
}

fn close_classic_element(xml: &[u8], namespaces: &NamespaceBindings) -> Result<Vec<u8>> {
    let mut reader = Reader::from_reader(xml);
    let start = match reader.read_event()? {
        Event::Start(start) | Event::Empty(start) => start,
        _ => return Err(invalid("classic element root missing".into())),
    };
    let declared = all_attributes(&start)?
        .into_iter()
        .map(|(name, _)| name)
        .collect::<std::collections::HashSet<_>>();
    let end = reader.buffer_position() as usize;
    let position = end
        - if xml.get(end - 2) == Some(&b'/') {
            2
        } else {
            1
        };
    let mut declarations = String::new();
    for (prefix, uri) in namespaces.entries() {
        let name = if prefix.is_empty() {
            "xmlns".to_owned()
        } else {
            format!("xmlns:{prefix}")
        };
        if !declared.contains(&name) {
            declarations.push_str(&format!(" {name}=\"{}\"", quick_xml::escape::escape(&uri)));
        }
    }
    classic_apply(xml, vec![(position..position, declarations.into_bytes())])
}

/// Prepared classic batch payloads, published together by the caller.
pub struct ClassicCommentImport {
    pub authors_xml: Vec<u8>,
    pub comment_parts: HashMap<String, Vec<u8>>,
}

/// Prepares classic authors and numeric comment references for one selected batch.
/// Only PresentationML comment identities and p15 parent references are rewritten.
/// Foreign extension payload remains byte-identical.
pub fn import_classic_comments(
    source_authors: &[u8],
    destination_authors: Option<&[u8]>,
    source_parts: &[(String, Vec<u8>)],
    destination_parts: &[Vec<u8>],
) -> Result<ClassicCommentImport> {
    let empty = format!(r#"<p:cmAuthorLst xmlns:p="{P_NS}"></p:cmAuthorLst>"#).into_bytes();
    let destination = destination_authors.unwrap_or(&empty);
    let source = classic_list(source_authors, b"cmAuthorLst", b"cmAuthor")?;
    let existing = classic_list(destination, b"cmAuthorLst", b"cmAuthor")?;
    let mut used = std::collections::HashSet::new();
    let mut high_water = HashMap::<u32, u32>::new();
    for author in &existing.elements {
        let id = classic_number(author, "id")?;
        if !used.insert(id) {
            return Err(invalid("duplicate classic author id".into()));
        }
        high_water.insert(id, classic_number(author, "lastIdx")?);
    }
    for xml in destination_parts {
        for comment in classic_list(xml, b"cmLst", b"cm")?.elements {
            let id = classic_number(&comment, "authorId")?;
            let index = classic_number(&comment, "idx")?;
            let high = high_water
                .get_mut(&id)
                .ok_or_else(|| invalid("classic comment has no author".into()))?;
            *high = (*high).max(index);
        }
    }
    let mut author_ids = HashMap::new();
    let mut new_authors = Vec::new();
    for author in &source.elements {
        let old = classic_number(author, "id")?;
        if author_ids.contains_key(&old) {
            return Err(invalid("duplicate source classic author id".into()));
        }
        let identity = classic_identity(source_authors, author)?;
        let mut matched = None;
        for target in &existing.elements {
            if classic_identity(destination, target)? == identity {
                matched = Some(classic_number(target, "id")?);
                break;
            }
        }
        let id = if let Some(id) = matched {
            id
        } else {
            let id = (0..=u32::MAX)
                .find(|id| !used.contains(id))
                .ok_or_else(|| invalid("classic author ids exhausted".into()))?;
            used.insert(id);
            high_water.insert(id, 0);
            new_authors.push((author, id));
            id
        };
        author_ids.insert(old, id);
    }
    let mut comments = HashMap::new();
    let mut parsed = Vec::new();
    for (part, xml) in source_parts {
        let list = classic_list(xml, b"cmLst", b"cm")?;
        for comment in &list.elements {
            let old = classic_number(comment, "authorId")?;
            let old_index = classic_number(comment, "idx")?;
            let id = *author_ids
                .get(&old)
                .ok_or_else(|| invalid("source classic comment has no author".into()))?;
            if comments.contains_key(&(old, old_index)) {
                return Err(invalid("duplicate source classic comment identity".into()));
            }
            let high = high_water
                .get_mut(&id)
                .expect("every mapped author has a counter");
            *high = high
                .checked_add(1)
                .ok_or_else(|| invalid("classic comment indices exhausted".into()))?;
            comments.insert((old, old_index), (id, *high));
        }
        parsed.push((part, xml, list));
    }
    let mut output = HashMap::new();
    for (part, xml, list) in parsed {
        let mut changes = Vec::new();
        for comment in &list.elements {
            let key = (
                classic_number(comment, "authorId")?,
                classic_number(comment, "idx")?,
            );
            let &(author, index) = comments
                .get(&key)
                .expect("every selected comment was allocated");
            classic_change(comment, "authorId", author, &mut changes)?;
            classic_change(comment, "idx", index, &mut changes)?;
        }
        for parent in classic_parent_references(xml)? {
            let Some(key) = classic_parent_identity(&parent)? else {
                continue;
            };
            let &(author, index) = comments.get(&key).ok_or_else(|| {
                invalid(format!(
                    "classic parent comment {},{} is outside the selected comment graph",
                    key.0, key.1
                ))
            })?;
            classic_change(&parent, "authorId", author, &mut changes)?;
            classic_change(&parent, "idx", index, &mut changes)?;
        }
        output.insert(part.clone(), classic_apply(xml, changes)?);
    }
    let mut changes = Vec::new();
    for author in &existing.elements {
        let id = classic_number(author, "id")?;
        classic_change(author, "lastIdx", high_water[&id], &mut changes)?;
    }
    let mut added = Vec::new();
    for (author, id) in new_authors {
        let mut local = Vec::new();
        classic_change(author, "id", id, &mut local)?;
        classic_change(author, "lastIdx", high_water[&id], &mut local)?;
        let mut raw = classic_apply(
            &source_authors[author.start..author.end],
            local
                .into_iter()
                .map(|(range, value)| {
                    (
                        (range.start - author.start)..(range.end - author.start),
                        value,
                    )
                })
                .collect(),
        )?;
        raw = close_classic_element(&raw, &author.namespaces)?;
        added.extend(raw);
    }
    changes.push((existing.insert_at..existing.insert_at, added));
    Ok(ClassicCommentImport {
        authors_xml: classic_apply(destination, changes)?,
        comment_parts: output,
    })
}

fn classic_parent_identity(parent: &ClassicElement) -> Result<Option<(u32, u32)>> {
    match (
        parent.attributes.contains_key("authorId"),
        parent.attributes.contains_key("idx"),
    ) {
        (false, false) => Ok(None),
        (true, true) => Ok(Some((
            classic_number(parent, "authorId")?,
            classic_number(parent, "idx")?,
        ))),
        _ => Err(invalid("incomplete classic parent identity".into())),
    }
}

fn classic_number(element: &ClassicElement, name: &str) -> Result<u32> {
    element
        .attributes
        .get(name)
        .ok_or_else(|| invalid(format!("classic {name} missing")))?
        .0
        .parse()
        .map_err(|_| invalid(format!("classic {name} is not an unsigned integer")))
}

fn classic_change(
    element: &ClassicElement,
    name: &str,
    value: u32,
    changes: &mut Vec<(std::ops::Range<usize>, Vec<u8>)>,
) -> Result<()> {
    let (_, range) = element
        .attributes
        .get(name)
        .ok_or_else(|| invalid(format!("classic {name} missing")))?;
    changes.push((range.clone(), value.to_string().into_bytes()));
    Ok(())
}

fn classic_apply(
    xml: &[u8],
    mut changes: Vec<(std::ops::Range<usize>, Vec<u8>)>,
) -> Result<Vec<u8>> {
    changes.sort_by_key(|(range, _)| range.start);
    let mut end = 0;
    for (range, _) in &changes {
        if range.start < end || range.start > range.end || range.end > xml.len() {
            return Err(invalid("overlapping classic XML edit".into()));
        }
        end = range.end;
    }
    let mut output = xml.to_vec();
    for (range, value) in changes.into_iter().rev() {
        output.splice(range, value);
    }
    Ok(output)
}

fn classic_identity(xml: &[u8], element: &ClassicElement) -> Result<ClassicIdentity> {
    let mut attributes = element
        .attributes
        .iter()
        .filter(|(name, _)| {
            !matches!(name.as_str(), "id" | "lastIdx") && !name.starts_with("xmlns")
        })
        .map(|(name, (value, _))| (name.clone(), value.clone()))
        .collect::<Vec<_>>();
    attributes.sort();
    let raw = &xml[element.start..element.end];
    let mut reader = Reader::from_reader(raw);
    reader.read_event()?;
    Ok(ClassicIdentity {
        attributes,
        namespaces: element.namespaces.entries(),
        payload: raw[reader.buffer_position() as usize..].to_vec(),
    })
}

fn classic_list(xml: &[u8], root: &[u8], child: &[u8]) -> Result<ClassicList> {
    classic_elements(xml, Some(root), P_NS, child, true)
}

fn classic_parent_references(xml: &[u8]) -> Result<Vec<ClassicElement>> {
    Ok(classic_elements(xml, Some(b"cmLst"), P15_NS, b"parentCm", false)?.elements)
}

fn classic_elements(
    xml: &[u8],
    root: Option<&[u8]>,
    uri: &str,
    child: &[u8],
    direct: bool,
) -> Result<ClassicList> {
    let mut reader = Reader::from_reader(xml);
    let mut scopes = vec![NamespaceBindings::default()];
    let mut elements = Vec::new();
    let mut pending = Vec::new();
    let mut path: Vec<(Option<String>, Vec<u8>, Option<String>)> = Vec::new();
    let mut insert_at = 0;
    let mut seen_root = false;
    loop {
        let before = reader.buffer_position() as usize;
        let event = reader.read_event()?;
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(start) | Event::Empty(start) => {
                let scope = scopes
                    .last()
                    .ok_or_else(|| invalid("classic namespace scope missing".into()))?
                    .with_start(&start)?;
                if scopes.len() == 1 {
                    if seen_root
                        || root.is_some_and(|root| local_name(start.name().as_ref()) != root)
                        || scope.element_uri(start.name().as_ref()) != Some(P_NS)
                        || empty
                    {
                        return Err(invalid("invalid classic comment list root".into()));
                    }
                    seen_root = true;
                    insert_at = reader.buffer_position() as usize;
                }
                let parent_path = path.len() == 5
                    && path[0].0.as_deref() == Some(P_NS)
                    && path[0].1 == b"cmLst"
                    && path[1].0.as_deref() == Some(P_NS)
                    && path[1].1 == b"cm"
                    && path[2].0.as_deref() == Some(P_NS)
                    && path[2].1 == b"extLst"
                    && path[3].0.as_deref() == Some(P_NS)
                    && path[3].1 == b"ext"
                    && path[3].2.as_deref().is_some_and(|uri: &str| {
                        uri.eq_ignore_ascii_case("{C676402C-5697-4E1C-873F-D02D1690AC5C}")
                    })
                    && path[4].0.as_deref() == Some(P15_NS)
                    && path[4].1 == b"threadingInfo";
                let owned = (if direct {
                    scopes.len() == 2
                } else {
                    parent_path
                }) && local_name(start.name().as_ref()) == child
                    && scope.element_uri(start.name().as_ref()) == Some(uri);
                if owned {
                    let mut attributes = HashMap::new();
                    for attribute in start.attributes() {
                        let attribute = attribute?;
                        let begin = (attribute.value.as_ptr() as usize)
                            .checked_sub(xml.as_ptr() as usize)
                            .ok_or_else(|| invalid("classic XML attribute outside input".into()))?;
                        let end = begin
                            .checked_add(attribute.value.len())
                            .filter(|end| *end <= xml.len())
                            .ok_or_else(|| invalid("classic XML attribute outside input".into()))?;
                        let name = std::str::from_utf8(attribute.key.as_ref())?.to_owned();
                        let value = attribute
                            .decoded_and_normalized_value(
                                quick_xml::XmlVersion::Implicit1_0,
                                reader.decoder(),
                            )?
                            .into_owned();
                        if attributes.insert(name, (value, begin..end)).is_some() {
                            return Err(invalid("duplicate classic attribute".into()));
                        }
                    }
                    let element = ClassicElement {
                        start: before,
                        end: reader.buffer_position() as usize,
                        attributes,
                        namespaces: scope.clone(),
                    };
                    if empty {
                        if direct {
                            insert_at = element.end;
                        }
                        elements.push(element);
                    } else {
                        pending.push((scopes.len(), element));
                    }
                }
                if !empty {
                    let extension = all_attributes(&start)?
                        .into_iter()
                        .find(|(name, _)| name == "uri")
                        .map(|(_, value)| value);
                    path.push((
                        scope.element_uri(start.name().as_ref()).map(str::to_owned),
                        local_name(start.name().as_ref()).to_vec(),
                        extension,
                    ));
                    scopes.push(scope);
                }
            }
            Event::End(_) => {
                scopes.pop();
                path.pop();
                if pending
                    .last()
                    .is_some_and(|(depth, _)| *depth == scopes.len())
                {
                    let (_, mut element) = pending.pop().expect("checked pending element");
                    element.end = reader.buffer_position() as usize;
                    if direct {
                        insert_at = element.end;
                    }
                    elements.push(element);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if !seen_root || scopes.len() != 1 || !pending.is_empty() {
        return Err(invalid("incomplete classic comment list".into()));
    }
    Ok(ClassicList {
        elements,
        insert_at,
    })
}
