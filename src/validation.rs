//! TwiML Validation
//!
//! This module provides validation for TwiML responses to ensure they conform
//! to Twilio's requirements and will be accepted by Twilio's servers.

use crate::error::{Error, Result};

/// Validation error details
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// The type of validation error
    pub error_type: ValidationErrorType,
    /// Human-readable error message
    pub message: String,
    /// Optional context (e.g., verb name, attribute name)
    pub context: Option<String>,
}

/// Types of validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorType {
    /// XML is not well-formed
    MalformedXml,
    /// Missing required attribute
    MissingRequiredAttribute,
    /// Invalid attribute value
    InvalidAttributeValue,
    /// Invalid verb nesting
    InvalidNesting,
    /// Content exceeds maximum length
    ContentTooLong,
    /// Invalid URL format
    InvalidUrl,
    /// Invalid phone number format
    InvalidPhoneNumber,
    /// Empty required field
    EmptyRequiredField,
    /// Invalid enum value
    InvalidEnumValue,
    /// Unsupported feature combination
    UnsupportedCombination,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(error_type: ValidationErrorType, message: impl Into<String>) -> Self {
        Self {
            error_type,
            message: message.into(),
            context: None,
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(context) = &self.context {
            write!(f, "[{}] {}: {}", context, self.error_type, self.message)
        } else {
            write!(f, "{}: {}", self.error_type, self.message)
        }
    }
}

impl std::fmt::Display for ValidationErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedXml => write!(f, "Malformed XML"),
            Self::MissingRequiredAttribute => write!(f, "Missing Required Attribute"),
            Self::InvalidAttributeValue => write!(f, "Invalid Attribute Value"),
            Self::InvalidNesting => write!(f, "Invalid Nesting"),
            Self::ContentTooLong => write!(f, "Content Too Long"),
            Self::InvalidUrl => write!(f, "Invalid URL"),
            Self::InvalidPhoneNumber => write!(f, "Invalid Phone Number"),
            Self::EmptyRequiredField => write!(f, "Empty Required Field"),
            Self::InvalidEnumValue => write!(f, "Invalid Enum Value"),
            Self::UnsupportedCombination => write!(f, "Unsupported Combination"),
        }
    }
}

/// TwiML Validator
pub struct TwiMLValidator {
    /// Whether to perform strict validation
    strict: bool,
}

impl TwiMLValidator {
    /// Create a new validator with default settings
    pub fn new() -> Self {
        Self { strict: false }
    }

    /// Create a new validator with strict validation enabled
    pub fn strict() -> Self {
        Self { strict: true }
    }

    /// Enable or disable strict validation
    pub fn set_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Validate XML well-formedness
    pub fn validate_xml(&self, xml: &str) -> Result<()> {
        // Check for basic XML structure
        if !xml.contains("<?xml") {
            return Err(Error::Validation("XML declaration missing".to_string()));
        }

        if !xml.contains("<Response>") || !xml.contains("</Response>") {
            return Err(Error::Validation(
                "Response element missing or malformed".to_string(),
            ));
        }

        // Check for balanced tags (basic check)
        let open_tags = xml.matches('<').count();
        let close_tags = xml.matches('>').count();
        if open_tags != close_tags {
            return Err(Error::Validation("Unbalanced XML tags".to_string()));
        }

        // More sophisticated check: ensure all opening tags have closing tags
        // This is a simple heuristic - count opening tags vs closing tags
        let mut tag_stack = Vec::new();
        let mut in_tag = false;
        let mut tag_name = String::new();
        let mut is_closing = false;
        let mut is_self_closing = false;
        let mut in_attributes = false; // Track if we're in the attributes section

        for ch in xml.chars() {
            if ch == '<' {
                in_tag = true;
                tag_name.clear();
                is_closing = false;
                is_self_closing = false;
                in_attributes = false;
            } else if ch == '>' && in_tag {
                in_tag = false;

                // Skip XML declaration, comments, and processing instructions
                if tag_name.starts_with('?') || tag_name.starts_with('!') {
                    continue;
                }

                // Handle self-closing tags
                if is_self_closing {
                    continue;
                }

                // Extract just the tag name (before any space or attribute)
                let tag = tag_name
                    .trim()
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim();

                if is_closing {
                    // Closing tag - should match the last opening tag
                    if let Some(last) = tag_stack.pop() {
                        if last != tag {
                            return Err(Error::Validation(format!(
                                "Mismatched closing tag: expected </{}>, found </{}>",
                                last, tag
                            )));
                        }
                    } else {
                        return Err(Error::Validation(format!(
                            "Unexpected closing tag: </{}>",
                            tag
                        )));
                    }
                } else if !tag.is_empty() {
                    // Opening tag
                    tag_stack.push(tag.to_string());
                }
            } else if in_tag {
                if ch == '/' {
                    if tag_name.is_empty() {
                        // This is a closing tag like </Say>
                        is_closing = true;
                    } else {
                        // This might be a self-closing tag like <Hangup />
                        // or a / in the attributes (like in https://)
                        // We'll mark it as self-closing and check later
                        is_self_closing = true;
                    }
                } else if is_self_closing && ch != ' ' {
                    // If we already marked as self-closing and we see a non-space character,
                    // it means the / was in the middle of attributes (like https://),
                    // so reset is_self_closing
                    is_self_closing = false;
                } else if ch == ' ' && !in_attributes {
                    // Space marks the end of the tag name (start of attributes)
                    in_attributes = true;
                } else if !in_attributes && !is_self_closing {
                    // Only add to tag_name if we're not in attributes yet and not self-closing
                    tag_name.push(ch);
                }
            }
        }

        // Check if there are unclosed tags
        if !tag_stack.is_empty() {
            return Err(Error::Validation(format!(
                "Unclosed tags: {}",
                tag_stack.join(", ")
            )));
        }

        Ok(())
    }

    /// Validate a complete TwiML response
    pub fn validate(&self, xml: &str) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate XML well-formedness
        if let Err(e) = self.validate_xml(xml) {
            errors.push(ValidationError::new(
                ValidationErrorType::MalformedXml,
                e.to_string(),
            ));
            // If XML is malformed, return early
            return Ok(errors);
        }

        // Validate URLs in the TwiML
        errors.extend(self.validate_urls(xml));

        // Validate phone numbers in the TwiML
        errors.extend(self.validate_phone_numbers(xml));

        // Validate content lengths
        errors.extend(self.validate_content_lengths(xml));

        Ok(errors)
    }

    /// Validate URLs in TwiML
    fn validate_urls(&self, xml: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Common URL attributes to check
        let url_attributes = [
            "action=",
            "url=",
            "statusCallback=",
            "recordingStatusCallback=",
            "transcribeCallback=",
            "statusCallbackUrl=",
            "fallbackUrl=",
        ];

        for attr in &url_attributes {
            if let Some(start) = xml.find(attr) {
                let after_attr = &xml[start + attr.len()..];
                if let Some(quote_start) = after_attr.find('"') {
                    let url_part = &after_attr[quote_start + 1..];
                    if let Some(quote_end) = url_part.find('"') {
                        let url = &url_part[..quote_end];

                        // Basic URL validation
                        if !url.is_empty()
                            && !url.starts_with("http://")
                            && !url.starts_with("https://")
                            && !url.starts_with('/')
                        {
                            if self.strict {
                                errors.push(
                                    ValidationError::new(
                                        ValidationErrorType::InvalidUrl,
                                        format!(
                                            "URL should start with http://, https://, or /: {}",
                                            url
                                        ),
                                    )
                                    .with_context(attr.trim_end_matches('=')),
                                );
                            }
                        }
                    }
                }
            }
        }

        errors
    }

    /// Validate phone numbers in TwiML
    fn validate_phone_numbers(&self, xml: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check for Number elements
        if xml.contains("<Number>") {
            let parts: Vec<&str> = xml.split("<Number>").collect();
            for (i, part) in parts.iter().enumerate().skip(1) {
                if let Some(end) = part.find("</Number>") {
                    let number = &part[..end];

                    // Basic phone number validation
                    if !number.is_empty()
                        && !number.starts_with('+')
                        && !number.starts_with("client:")
                        && !number.starts_with("sip:")
                    {
                        if self.strict {
                            errors.push(
                                ValidationError::new(
                                    ValidationErrorType::InvalidPhoneNumber,
                                    format!("Phone number should start with + or be a client/sip identifier: {}", number),
                                )
                                .with_context(format!("Number element #{}", i)),
                            );
                        }
                    }
                }
            }
        }

        errors
    }

    /// Validate content lengths
    fn validate_content_lengths(&self, xml: &str) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check Say content length (max 4096 characters for text, more for SSML)
        if xml.contains("<Say>") {
            let parts: Vec<&str> = xml.split("<Say>").collect();
            for (i, part) in parts.iter().enumerate().skip(1) {
                if let Some(end) = part.find("</Say>") {
                    let content = &part[..end];

                    // Basic length check (4096 for plain text)
                    if content.len() > 4096 && !content.contains('<') {
                        errors.push(
                            ValidationError::new(
                                ValidationErrorType::ContentTooLong,
                                format!(
                                    "Say content exceeds 4096 characters: {} characters",
                                    content.len()
                                ),
                            )
                            .with_context(format!("Say element #{}", i)),
                        );
                    }
                }
            }
        }

        // Check Message body length (max 1600 characters)
        if xml.contains("<Body>") {
            let parts: Vec<&str> = xml.split("<Body>").collect();
            for (i, part) in parts.iter().enumerate().skip(1) {
                if let Some(end) = part.find("</Body>") {
                    let content = &part[..end];

                    if content.len() > 1600 {
                        errors.push(
                            ValidationError::new(
                                ValidationErrorType::ContentTooLong,
                                format!(
                                    "Message body exceeds 1600 characters: {} characters",
                                    content.len()
                                ),
                            )
                            .with_context(format!("Body element #{}", i)),
                        );
                    }
                }
            }
        }

        errors
    }
}

impl Default for TwiMLValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a TwiML XML string
pub fn validate_twiml(xml: &str) -> Result<Vec<ValidationError>> {
    TwiMLValidator::new().validate(xml)
}

/// Validate a TwiML XML string with strict validation
pub fn validate_twiml_strict(xml: &str) -> Result<Vec<ValidationError>> {
    TwiMLValidator::strict().validate(xml)
}

