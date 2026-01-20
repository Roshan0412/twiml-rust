use twiml_rust::fax::{FaxResponse, ReceiveAttributes, ReceiveMediaType, ReceivePageSize};
use twiml_rust::TwiML;

#[test]
fn test_simple_receive() {
    let response = FaxResponse::new().receive(None);

    let xml = response.to_xml();
    assert!(xml.contains("<Receive/>"));
}

#[test]
fn test_receive_with_action() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new().action("https://example.com/fax-received"),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("action=\"https://example.com/fax-received\""));
}

#[test]
fn test_receive_with_all_attributes() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new()
            .action("https://example.com/fax-received")
            .method("POST")
            .media_type(ReceiveMediaType::ApplicationPdf)
            .page_size(ReceivePageSize::Letter)
            .store_media(true),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("action=\"https://example.com/fax-received\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("mediaType=\"application/pdf\""));
    assert!(xml.contains("pageSize=\"letter\""));
    assert!(xml.contains("storeMedia=\"true\""));
}

#[test]
fn test_receive_with_tiff_media_type() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new().media_type(ReceiveMediaType::ImageTiff),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("mediaType=\"image/tiff\""));
}

#[test]
fn test_receive_with_legal_page_size() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new().page_size(ReceivePageSize::Legal),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("pageSize=\"legal\""));
}

#[test]
fn test_receive_with_a4_page_size() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new().page_size(ReceivePageSize::A4),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("pageSize=\"a4\""));
}

#[test]
fn test_receive_with_store_media_false() {
    let response = FaxResponse::new().receive(Some(ReceiveAttributes::new().store_media(false)));

    let xml = response.to_xml();
    assert!(xml.contains("storeMedia=\"false\""));
}

#[test]
fn test_receive_with_method_get() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new()
            .action("https://example.com/fax")
            .method("GET"),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("method=\"GET\""));
}

#[test]
fn test_empty_fax_response() {
    let response = FaxResponse::new();

    let xml = response.to_xml();
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<Response>"));
    assert!(xml.contains("</Response>"));
    assert!(!xml.contains("<Receive"));
}

#[test]
fn test_comment_before() {
    let response = FaxResponse::new()
        .comment_before("This is a comment before the response")
        .receive(None);

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
    let response = FaxResponse::new()
        .receive(None)
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
    let response = FaxResponse::new()
        .receive(None)
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
    let response = FaxResponse::new()
        .comment_before("Before 1")
        .comment_before("Before 2")
        .comment("Inside 1")
        .receive(None)
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
    let response = FaxResponse::new()
        .comment_before("Generated by TwiML Rust")
        .comment("Fax receive configuration")
        .receive(Some(
            ReceiveAttributes::new()
                .action("https://example.com/fax-received")
                .method("POST")
                .media_type(ReceiveMediaType::ApplicationPdf)
                .page_size(ReceivePageSize::Letter)
                .store_media(true),
        ))
        .comment_after("End of TwiML");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Generated by TwiML Rust -->"));
    assert!(xml.contains("<!-- Fax receive configuration -->"));
    assert!(xml.contains("<!-- End of TwiML -->"));
    assert!(xml.contains("action=\"https://example.com/fax-received\""));
    assert!(xml.contains("mediaType=\"application/pdf\""));
}

#[test]
fn test_empty_response_with_comments() {
    let response = FaxResponse::new()
        .comment_before("Before")
        .comment("Inside")
        .comment_after("After");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Before -->"));
    assert!(xml.contains("<!-- Inside -->"));
    assert!(xml.contains("<!-- After -->"));
    assert!(xml.contains("<Response>"));
    assert!(xml.contains("</Response>"));
    assert!(!xml.contains("<Receive"));
}
