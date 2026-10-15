#![allow(non_camel_case_types)]

pub mod comments;
pub mod connector;
pub mod diagram;
pub mod graphic_frame;
pub mod namespace;
pub mod notes_parts;
pub mod picture;
pub mod placeholder;
pub mod presentation;
pub mod relmap;
pub mod shape_tree;
pub mod slide_parts;
pub mod timing;

/// The canonical PresentationML main-part name.
pub const PRESENTATION_PART: &str = "/ppt/presentation.xml";

/// Maximum recognized group, compatibility or timing-owner nesting depth.
/// Root cycle owners count as depth one. Depth 32 is accepted and depth 33 is refused.
pub const MAX_PRESENTATION_NESTING: usize = 32;

pub(crate) fn check_presentation_nesting(
    cycle: &str,
    depth: usize,
) -> Result<(), oxml_core::OxmlError> {
    if depth > MAX_PRESENTATION_NESTING {
        return Err(oxml_core::OxmlError::InvalidValue(format!(
            "{cycle} nesting exceeds limit {MAX_PRESENTATION_NESTING}, offending depth {depth}"
        )));
    }
    Ok(())
}
