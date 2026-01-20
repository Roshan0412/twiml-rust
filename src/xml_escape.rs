//! XML escaping utilities to prevent XML injection attacks.
//!
//! This module provides functions to safely escape user-provided content
//! before inserting it into XML documents.

/// Escape XML text content.
///
/// Escapes the following characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
///
/// This function should be used for text content between XML tags.
///
/// # Arguments
/// * `text` - The text to escape
///
/// # Returns
/// The escaped text safe for use in XML content
///
/// # Example
/// ```
/// use twiml_rust::xml_escape::escape_xml_text;
///
/// let safe = escape_xml_text("Hello <script>alert('xss')</script>");
/// assert_eq!(safe, "Hello &lt;script&gt;alert('xss')&lt;/script&gt;");
/// ```
pub fn escape_xml_text(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect::<Vec<_>>(),
            '>' => "&gt;".chars().collect::<Vec<_>>(),
            _ => vec![c],
        })
        .collect()
}

/// Escape XML attribute values.
///
/// Escapes the following characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&apos;`
///
/// This function should be used for attribute values in XML tags.
///
/// # Arguments
/// * `text` - The attribute value to escape
///
/// # Returns
/// The escaped text safe for use in XML attributes
///
/// # Example
/// ```
/// use twiml_rust::xml_escape::escape_xml_attr;
///
/// let safe = escape_xml_attr("value with \"quotes\" and <tags>");
/// assert_eq!(safe, "value with &quot;quotes&quot; and &lt;tags&gt;");
/// ```
pub fn escape_xml_attr(text: &str) -> String {
    text.chars()
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect::<Vec<_>>(),
            '>' => "&gt;".chars().collect::<Vec<_>>(),
            '"' => "&quot;".chars().collect::<Vec<_>>(),
            '\'' => "&apos;".chars().collect::<Vec<_>>(),
            _ => vec![c],
        })
        .collect()
}
