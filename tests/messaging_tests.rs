use twiml_rust::messaging::{
    Body, Media, Message, MessageAttributes, MessagingResponse, RedirectAttributes,
};
use twiml_rust::TwiML;

#[test]
fn test_simple_message() {
    let response = MessagingResponse::new().message("Hello, World!");

    let xml = response.to_xml();
    println!("XML: {}", xml);
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Body>Hello, World!</Body>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_message_with_recipient() {
    let response = MessagingResponse::new()
        .message_with_attributes(MessageAttributes::new().to("+1234567890"), "Hello!");

    let xml = response.to_xml();
    assert!(xml.contains("to=\"+1234567890\""));
    assert!(xml.contains("<Body>Hello!</Body>"));
}

#[test]
fn test_message_with_all_attributes() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new()
            .to("+1234567890")
            .from("+0987654321")
            .action("https://example.com/action")
            .method("POST")
            .status_callback("https://example.com/status"),
        "Hello!",
    );

    let xml = response.to_xml();
    assert!(xml.contains("to=\"+1234567890\""));
    assert!(xml.contains("from=\"+0987654321\""));
    assert!(xml.contains("action=\"https://example.com/action\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("<Body>Hello!</Body>"));
}

#[test]
fn test_redirect() {
    let response = MessagingResponse::new().redirect("https://example.com/next");

    let xml = response.to_xml();
    assert!(xml.contains("<Redirect>https://example.com/next</Redirect>"));
}

#[test]
fn test_redirect_with_method() {
    let response = MessagingResponse::new().redirect_with_attributes(
        RedirectAttributes::new().method("POST"),
        "https://example.com/next",
    );

    let xml = response.to_xml();
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains(">https://example.com/next</Redirect>"));
}

#[test]
fn test_multiple_messages() {
    let response = MessagingResponse::new()
        .message("First message")
        .message_with_attributes(MessageAttributes::new().to("+1234567890"), "Second message");

    let xml = response.to_xml();
    assert!(xml.contains("<Body>First message</Body>"));
    assert!(xml.contains("<Body>Second message</Body>"));
    assert!(xml.contains("to=\"+1234567890\""));
}

#[test]
fn test_message_and_redirect() {
    let response = MessagingResponse::new()
        .message("Hello!")
        .redirect("https://example.com/next");

    let xml = response.to_xml();
    assert!(xml.contains("<Body>Hello!</Body>"));
    assert!(xml.contains("<Redirect>https://example.com/next</Redirect>"));
}

#[test]
fn test_comment_before() {
    let response = MessagingResponse::new()
        .comment_before("This is a comment before the response")
        .message("Hello!");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- This is a comment before the response -->"));
    // Comment should appear before <Response>
    let comment_pos = xml
        .find("<!-- This is a comment before the response -->")
        .unwrap();
    let response_pos = xml.find("<Response>").unwrap();
    assert!(comment_pos < response_pos);
}

#[test]
fn test_comment_inside() {
    let response = MessagingResponse::new()
        .message("Hello!")
        .comment("This is a comment inside the response");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- This is a comment inside the response -->"));
    // Comment should appear after <Response> and before </Response>
    let response_start = xml.find("<Response>").unwrap();
    let comment_pos = xml
        .find("<!-- This is a comment inside the response -->")
        .unwrap();
    let response_end = xml.find("</Response>").unwrap();
    assert!(response_start < comment_pos);
    assert!(comment_pos < response_end);
}

#[test]
fn test_comment_after() {
    let response = MessagingResponse::new()
        .message("Hello!")
        .comment_after("This is a comment after the response");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- This is a comment after the response -->"));
    // Comment should appear after </Response>
    let response_pos = xml.find("</Response>").unwrap();
    let comment_pos = xml
        .find("<!-- This is a comment after the response -->")
        .unwrap();
    assert!(response_pos < comment_pos);
}

#[test]
fn test_multiple_comments() {
    let response = MessagingResponse::new()
        .comment_before("Before 1")
        .comment_before("Before 2")
        .comment("Inside 1")
        .message("Hello!")
        .comment("Inside 2")
        .comment_after("After 1")
        .comment_after("After 2");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Before 1 -->"));
    assert!(xml.contains("<!-- Before 2 -->"));
    assert!(xml.contains("<!-- Inside 1 -->"));
    assert!(xml.contains("<!-- Inside 2 -->"));
    assert!(xml.contains("<!-- After 1 -->"));
    assert!(xml.contains("<!-- After 2 -->"));
}

#[test]
fn test_comments_with_all_features() {
    let response = MessagingResponse::new()
        .comment_before("Generated by TwiML Rust")
        .comment("Main message section")
        .message_with_attributes(
            MessageAttributes::new()
                .to("+1234567890")
                .from("+0987654321"),
            "Hello!",
        )
        .comment("Redirect section")
        .redirect("https://example.com/next")
        .comment_after("End of TwiML");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Generated by TwiML Rust -->"));
    assert!(xml.contains("<!-- Main message section -->"));
    assert!(xml.contains("<!-- Redirect section -->"));
    assert!(xml.contains("<!-- End of TwiML -->"));
    assert!(xml.contains("<Body>Hello!</Body>"));
    assert!(xml.contains("to=\"+1234567890\""));
}

#[test]
fn test_empty_response() {
    let response = MessagingResponse::new();

    let xml = response.to_xml();
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<Response>"));
    assert!(xml.contains("</Response>"));
}

#[test]
fn test_message_with_body_noun() {
    let body = Body::new("Hello from Body noun!");
    let message = Message::with_nouns(MessageAttributes::new()).body(body);
    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Body>Hello from Body noun!</Body>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_message_with_single_media() {
    let media = Media::new("https://example.com/image.jpg");
    let message = Message::with_nouns(MessageAttributes::new()).add_media(media);
    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Media>https://example.com/image.jpg</Media>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_message_with_multiple_media() {
    let media1 = Media::new("https://example.com/image1.jpg");
    let media2 = Media::new("https://example.com/image2.jpg");
    let media3 = Media::new("https://example.com/image3.jpg");

    let message = Message::with_nouns(MessageAttributes::new())
        .add_media(media1)
        .add_media(media2)
        .add_media(media3);

    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Media>https://example.com/image1.jpg</Media>"));
    assert!(xml.contains("<Media>https://example.com/image2.jpg</Media>"));
    assert!(xml.contains("<Media>https://example.com/image3.jpg</Media>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_message_with_body_and_media() {
    let body = Body::new("Check out these images!");
    let media1 = Media::new("https://example.com/image1.jpg");
    let media2 = Media::new("https://example.com/image2.jpg");

    let message = Message::with_nouns(MessageAttributes::new())
        .body(body)
        .add_media(media1)
        .add_media(media2);

    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Body>Check out these images!</Body>"));
    assert!(xml.contains("<Media>https://example.com/image1.jpg</Media>"));
    assert!(xml.contains("<Media>https://example.com/image2.jpg</Media>"));
    assert!(xml.contains("</Message>"));

    // Verify Body comes before Media
    let body_pos = xml.find("<Body>").unwrap();
    let media_pos = xml.find("<Media>").unwrap();
    assert!(body_pos < media_pos);
}

#[test]
fn test_message_with_body_and_media_with_attributes() {
    let body = Body::new("Hello!");
    let media = Media::new("https://example.com/image.jpg");

    let message = Message::with_nouns(
        MessageAttributes::new()
            .to("+1234567890")
            .from("+0987654321"),
    )
    .body(body)
    .add_media(media);

    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("to=\"+1234567890\""));
    assert!(xml.contains("from=\"+0987654321\""));
    assert!(xml.contains("<Body>Hello!</Body>"));
    assert!(xml.contains("<Media>https://example.com/image.jpg</Media>"));
}

#[test]
fn test_message_media_only_no_body() {
    let media = Media::new("https://example.com/image.jpg");
    let message = Message::with_nouns(MessageAttributes::new()).add_media(media);
    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(!xml.contains("<Body>"));
    assert!(xml.contains("<Media>https://example.com/image.jpg</Media>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_message_with_media_vec() {
    let media_vec = vec![
        Media::new("https://example.com/image1.jpg"),
        Media::new("https://example.com/image2.jpg"),
    ];

    let message = Message::with_nouns(MessageAttributes::new()).media(media_vec);
    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("<Media>https://example.com/image1.jpg</Media>"));
    assert!(xml.contains("<Media>https://example.com/image2.jpg</Media>"));
}

#[test]
fn test_backward_compatibility_simple_message() {
    // Old API should still work
    let response = MessagingResponse::new().message("Hello, World!");

    let xml = response.to_xml();
    assert!(xml.contains("<Message>"));
    assert!(xml.contains("<Body>Hello, World!</Body>"));
    assert!(xml.contains("</Message>"));
}

#[test]
fn test_backward_compatibility_message_with_attributes() {
    // Old API should still work
    let response = MessagingResponse::new()
        .message_with_attributes(MessageAttributes::new().to("+1234567890"), "Hello!");

    let xml = response.to_xml();
    assert!(xml.contains("to=\"+1234567890\""));
    assert!(xml.contains("<Body>Hello!</Body>"));
}

// ========================================================================
// XML Injection Security Tests
// ========================================================================

#[test]
fn test_xml_injection_in_body() {
    let response =
        MessagingResponse::new().message("Hello</Body><Message to=\"+1-attacker\"><Body>Hacked");

    let xml = response.to_xml();
    // Should escape the XML tags
    assert!(xml.contains("&lt;/Body&gt;"));
    assert!(xml.contains("&lt;Message"));
    assert!(!xml.contains("</Body><Message"));
}

#[test]
fn test_xml_injection_in_media_url() {
    let media =
        Media::new("https://example.com/image.jpg</Media><Message to=\"+1-attacker\"><Body>Hacked");
    let message = Message::with_nouns(MessageAttributes::new()).add_media(media);
    let response = MessagingResponse::new().message_with_nouns(message);

    let xml = response.to_xml();
    assert!(xml.contains("&lt;/Media&gt;"));
    assert!(xml.contains("&lt;Message"));
    assert!(!xml.contains("</Media><Message"));
}

#[test]
fn test_xml_injection_in_to_attribute() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new().to("+1234\" onload=\"alert('xss')"),
        "Hello!",
    );

    let xml = response.to_xml();
    assert!(xml.contains("&quot;"));
    // The injection is prevented because quotes are escaped
    assert!(xml.contains("&quot; onload=&quot;"));
    // Make sure the actual attack string is not present unescaped
    assert!(!xml.contains("\" onload=\""));
}

#[test]
fn test_xml_injection_in_from_attribute() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new().from("+1234\"><script>alert('xss')</script>"),
        "Hello!",
    );

    let xml = response.to_xml();
    assert!(xml.contains("&quot;&gt;&lt;script&gt;"));
    assert!(!xml.contains("<script>"));
}

#[test]
fn test_xml_injection_in_action_url() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new().action("https://example.com?a=1&b=2<script>"),
        "Hello!",
    );

    let xml = response.to_xml();
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("&lt;script&gt;"));
    assert!(!xml.contains("<script>"));
}

#[test]
fn test_xml_injection_in_redirect_url() {
    let response = MessagingResponse::new()
        .redirect("https://example.com</Redirect><Message to=\"+1-attacker\"><Body>Hacked");

    let xml = response.to_xml();
    assert!(xml.contains("&lt;/Redirect&gt;"));
    assert!(!xml.contains("</Redirect><Message"));
}

#[test]
fn test_xml_injection_in_comments() {
    let response = MessagingResponse::new()
        .comment_before("Comment with </Response><script>alert('xss')</script>")
        .message("Hello!")
        .comment("Inside comment --><script>alert('xss')</script><!--")
        .comment_after("After comment -->");

    let xml = response.to_xml();
    assert!(xml.contains("&lt;/Response&gt;"));
    assert!(xml.contains("&lt;script&gt;"));
    assert!(!xml.contains("<script>"));
    assert!(!xml.contains("--><script>"));
}

#[test]
fn test_ampersand_in_url() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new().action("https://example.com?a=1&b=2&c=3"),
        "Hello!",
    );

    let xml = response.to_xml();
    assert!(xml.contains("a=1&amp;b=2&amp;c=3"));
    assert!(!xml.contains("a=1&b=2"));
}

#[test]
fn test_special_chars_in_body() {
    let response = MessagingResponse::new().message("Tom & Jerry say: <Hello> & 'Goodbye'");

    let xml = response.to_xml();
    assert!(xml.contains("Tom &amp; Jerry"));
    assert!(xml.contains("&lt;Hello&gt;"));
    assert!(!xml.contains("Tom & Jerry"));
    assert!(!xml.contains("<Hello>"));
}
