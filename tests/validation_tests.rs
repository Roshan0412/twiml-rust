use twiml_rust::validation::{
    validate_twiml, validate_twiml_strict, TwiMLValidator, ValidationErrorType,
};
use twiml_rust::voice::{Dial, DialNumber, Gather, Say, VoiceResponse};
use twiml_rust::TwiML;

#[test]
fn test_valid_xml() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello World</Say>
</Response>"#;

    let validator = TwiMLValidator::new();
    assert!(validator.validate_xml(xml).is_ok());
}

#[test]
fn test_missing_xml_declaration() {
    let xml = r#"<Response>
  <Say>Hello World</Say>
</Response>"#;

    let validator = TwiMLValidator::new();
    assert!(validator.validate_xml(xml).is_err());
}

#[test]
fn test_missing_response_element() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Say>Hello World</Say>"#;

    let validator = TwiMLValidator::new();
    assert!(validator.validate_xml(xml).is_err());
}

#[test]
fn test_unbalanced_tags() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello World
</Response>"#;

    let validator = TwiMLValidator::new();
    assert!(validator.validate_xml(xml).is_err());
}

#[test]
fn test_validate_complete_response() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello World</Say>
  <Play>https://example.com/audio.mp3</Play>
</Response>"#;

    let errors = validate_twiml(xml).unwrap();
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_validate_content_length() {
    let long_text = "a".repeat(5000);
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>{}</Say>
</Response>"#,
        long_text
    );

    let errors = validate_twiml(&xml).unwrap();
    assert!(errors
        .iter()
        .any(|e| matches!(e.error_type, ValidationErrorType::ContentTooLong)));
}

#[test]
fn test_strict_url_validation() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Gather action="invalid-url">
    <Say>Press 1</Say>
  </Gather>
</Response>"#;

    let errors = validate_twiml_strict(xml).unwrap();
    assert!(errors
        .iter()
        .any(|e| matches!(e.error_type, ValidationErrorType::InvalidUrl)));
}

#[test]
fn test_real_world_voice_response() {
    // Create a realistic voice response
    let gather = Gather::new()
        .input(vec!["dtmf".to_string()])
        .action("https://example.com/process")
        .timeout(10)
        .add_say(Say::new("Press 1 for sales, 2 for support"));

    let dial = Dial::new()
        .timeout(30)
        .add_number(DialNumber::new("+15551234567"));

    let response = VoiceResponse::new()
        .say("Welcome to our service!")
        .gather(gather)
        .dial_with(dial)
        .say("Thank you for calling. Goodbye!")
        .hangup();

    // Validate using trait method
    let errors = response.validate().unwrap();
    assert_eq!(errors.len(), 0, "Expected no validation errors");

    // Validate using strict mode
    let errors = response.validate_strict().unwrap();
    assert_eq!(
        errors.len(),
        0,
        "Expected no validation errors in strict mode"
    );
}

#[test]
fn test_malformed_xml_detection() {
    let validator = TwiMLValidator::new();

    // Test unclosed tag
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello World
</Response>"#;

    let errors = validator.validate(xml).unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].message.contains("Mismatched closing tag")
            || errors[0].message.contains("Malformed XML")
    );

    // Test extra closing tag
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello</Say>
  </Say>
</Response>"#;

    let errors = validator.validate(xml).unwrap();
    assert_eq!(errors.len(), 1);
    assert!(
        errors[0].message.contains("Mismatched closing tag")
            || errors[0].message.contains("Malformed XML")
    );
}

#[test]
fn test_message_body_length() {
    let validator = TwiMLValidator::new();

    // Test Message body that's too long
    let long_text = "a".repeat(2000);
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Message>
    <Body>{}</Body>
  </Message>
</Response>"#,
        long_text
    );

    let errors = validator.validate(&xml).unwrap();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Message body exceeds 1600 characters"));
}
