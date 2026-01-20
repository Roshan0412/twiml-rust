use twiml_rust::xml_escape::{escape_xml_attr, escape_xml_text};

#[test]
fn test_escape_xml_text_basic() {
    assert_eq!(escape_xml_text("Hello World"), "Hello World");
}

#[test]
fn test_escape_xml_text_ampersand() {
    assert_eq!(escape_xml_text("Tom & Jerry"), "Tom &amp; Jerry");
}

#[test]
fn test_escape_xml_text_less_than() {
    assert_eq!(escape_xml_text("5 < 10"), "5 &lt; 10");
}

#[test]
fn test_escape_xml_text_greater_than() {
    assert_eq!(escape_xml_text("10 > 5"), "10 &gt; 5");
}

#[test]
fn test_escape_xml_text_tags() {
    assert_eq!(
        escape_xml_text("<script>alert('xss')</script>"),
        "&lt;script&gt;alert('xss')&lt;/script&gt;"
    );
}

#[test]
fn test_escape_xml_text_multiple() {
    assert_eq!(escape_xml_text("A & B < C > D"), "A &amp; B &lt; C &gt; D");
}

#[test]
fn test_escape_xml_text_injection_attempt() {
    // Note: escape_xml_text doesn't escape quotes - only < > &
    // Quotes are only escaped in attributes via escape_xml_attr
    assert_eq!(
        escape_xml_text("Hello</Body><Message to=\"+1-attacker\"><Body>Hacked"),
        "Hello&lt;/Body&gt;&lt;Message to=\"+1-attacker\"&gt;&lt;Body&gt;Hacked"
    );
}

#[test]
fn test_escape_xml_attr_basic() {
    assert_eq!(escape_xml_attr("Hello World"), "Hello World");
}

#[test]
fn test_escape_xml_attr_double_quotes() {
    assert_eq!(escape_xml_attr("Say \"Hello\""), "Say &quot;Hello&quot;");
}

#[test]
fn test_escape_xml_attr_single_quotes() {
    assert_eq!(escape_xml_attr("It's fine"), "It&apos;s fine");
}

#[test]
fn test_escape_xml_attr_all_special_chars() {
    assert_eq!(escape_xml_attr("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
}

#[test]
fn test_escape_xml_attr_injection_attempt() {
    assert_eq!(
        escape_xml_attr("\" onload=\"alert('xss')"),
        "&quot; onload=&quot;alert(&apos;xss&apos;)"
    );
}

#[test]
fn test_escape_xml_attr_url_with_params() {
    assert_eq!(
        escape_xml_attr("https://example.com?a=1&b=2"),
        "https://example.com?a=1&amp;b=2"
    );
}
