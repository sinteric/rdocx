use oxml_core::OxmlError;
use quick_xml::XmlVersion;
use quick_xml::events::BytesStart;

pub const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
pub const A_PREFIX: &str = "a";
pub const PIC_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/picture";
pub const PIC_PREFIX: &str = "pic";

/// Rejects a local binding that would change the meaning of a fixed `a:` tag.
///
/// Call this only for elements that a typed parser rewrites. Opaque elements
/// keep their producer namespace declarations and are not inspected.
pub(crate) fn reject_conflicting_a_prefix(element: &BytesStart<'_>) -> Result<(), OxmlError> {
    for attribute in element.attributes() {
        let attribute = attribute.map_err(OxmlError::from)?;
        if attribute.key.as_ref() == b"xmlns:a" {
            let value = attribute
                .decoded_and_normalized_value(XmlVersion::Implicit1_0, element.decoder())
                .map_err(OxmlError::from)?;
            if value != A_NS {
                return Err(OxmlError::InvalidValue(
                    "xmlns:a conflicts with the fixed DrawingML writer namespace".to_owned(),
                ));
            }
        }
    }
    Ok(())
}

/// Resolves the actual property fragment scope without rewriting source declarations.
pub(crate) fn property_namespace_scope(
    element: &BytesStart<'_>,
    inherited: &[(String, String)],
) -> Result<Vec<(String, String)>, OxmlError> {
    let mut scope = inherited.to_vec();
    if inherited.is_empty() {
        scope.push(("a".to_owned(), A_NS.to_owned()));
        let name = element.name();
        let spelling = std::str::from_utf8(name.as_ref()).map_err(OxmlError::from)?;
        let prefix = spelling.split_once(':').map_or("", |(prefix, _)| prefix);
        if !scope.iter().any(|(name, _)| name == prefix) {
            scope.push((prefix.to_owned(), A_NS.to_owned()));
        }
    }
    for attribute in element.attributes() {
        let attribute = attribute.map_err(OxmlError::from)?;
        let key = attribute.key.as_ref();
        let prefix = if key == b"xmlns" {
            Some("")
        } else if let Some(prefix) = key.strip_prefix(b"xmlns:") {
            Some(std::str::from_utf8(prefix).map_err(OxmlError::from)?)
        } else {
            None
        };
        if let Some(prefix) = prefix {
            let value = attribute
                .decoded_and_normalized_value(XmlVersion::Implicit1_0, element.decoder())
                .map_err(OxmlError::from)?
                .into_owned();
            if let Some((_, old)) = scope.iter_mut().find(|(name, _)| name == prefix) {
                *old = value;
            } else {
                scope.push((prefix.to_owned(), value));
            }
        }
    }
    Ok(scope)
}

/// Checks a property child's expanded namespace before typed dispatch.
pub(crate) fn property_is_drawing(
    element: &BytesStart<'_>,
    inherited: &[(String, String)],
) -> Result<bool, OxmlError> {
    let scope = property_namespace_scope(element, inherited)?;
    let name = element.name();
    let spelling = std::str::from_utf8(name.as_ref()).map_err(OxmlError::from)?;
    let prefix = spelling.split_once(':').map_or("", |(prefix, _)| prefix);
    Ok(scope
        .iter()
        .any(|(name, uri)| name == prefix && uri == A_NS))
}

/// Typed property writers use `a:` tags. Opaque source never calls this check.
pub(crate) fn property_reject_writer_conflicts(
    scope: &[(String, String)],
) -> Result<(), OxmlError> {
    if scope
        .iter()
        .any(|(prefix, uri)| prefix == "a" && uri != A_NS)
    {
        return Err(OxmlError::InvalidValue(
            "unsupported typed property binding: in-scope a namespace conflicts with the fixed DrawingML writer".to_owned(),
        ));
    }
    Ok(())
}
