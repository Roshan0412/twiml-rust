//! # TwiML Rust
//!
//! A Rust library for generating TwiML (Twilio Markup Language) XML responses.
//!
//! TwiML is an XML-based language for controlling phone calls, SMS messages, and faxes.
//! This library provides a type-safe, idiomatic Rust API for generating TwiML responses.
//!
//! ## Features
//!
//! - **Voice Responses**: Generate TwiML for voice calls with verbs like Say, Play, Dial, Gather, etc.
//! - **Messaging Responses**: Generate TwiML for SMS/MMS messages
//! - **Fax Responses**: Generate TwiML for fax operations
//! - **Validation**: Built-in validation to ensure TwiML conforms to Twilio's requirements
//! - **Type Safety**: Strongly typed API with builder patterns
//!
//! ## Example
//!
//! ```rust
//! use twiml_rust::{VoiceResponse, TwiML};
//!
//! let response = VoiceResponse::new()
//!     .say("Hello, World!")
//!     .play("https://example.com/audio.mp3");
//!
//! let xml = response.to_xml();
//! println!("{}", xml);
//! ```

pub mod error;
pub mod fax;
pub mod messaging;
pub mod validation;
pub mod validation_warnings;
pub mod voice;
pub mod xml_escape;

pub use error::{Error, Result};
pub use fax::{FaxResponse, Receive, ReceiveAttributes, ReceiveMediaType, ReceivePageSize};
pub use messaging::{
    Body, Media, Message, MessageAttributes, MessagingResponse, Redirect, RedirectAttributes,
};
pub use validation::{
    validate_twiml, validate_twiml_strict, TwiMLValidator, ValidationError, ValidationErrorType,
};
pub use voice::VoiceResponse;

/// Common trait for TwiML responses
pub trait TwiML {
    /// Convert the TwiML to an XML string
    fn to_xml(&self) -> String;

    /// Validate the TwiML
    fn validate(&self) -> Result<Vec<ValidationError>> {
        validate_twiml(&self.to_xml())
    }

    /// Validate the TwiML with strict validation
    fn validate_strict(&self) -> Result<Vec<ValidationError>> {
        validate_twiml_strict(&self.to_xml())
    }
}
