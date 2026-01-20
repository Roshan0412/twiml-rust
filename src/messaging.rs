//! TwiML generation for SMS and MMS messaging responses.
//!
//! This module provides types and builders for creating TwiML responses
//! for SMS and MMS messages. The main entry point is [`MessagingResponse`],
//! which can contain [`Message`] verbs with [`Body`] and [`Media`] nouns.
//!
//! # Examples
//!
//! ## Simple SMS
//!
//! ```rust
//! use twiml_rust::{MessagingResponse, TwiML};
//!
//! let response = MessagingResponse::new()
//!     .message("Thanks for your message!");
//!
//! println!("{}", response.to_xml());
//! ```
//!
//! ## MMS with Multiple Media
//!
//! ```rust
//! use twiml_rust::{MessagingResponse, messaging::{Message, MessageAttributes, Body, Media}, TwiML};
//!
//! let message = Message::with_nouns(MessageAttributes::new())
//!     .body(Body::new("Check out these photos!"))
//!     .add_media(Media::new("https://example.com/photo1.jpg"))
//!     .add_media(Media::new("https://example.com/photo2.jpg"));
//!
//! let response = MessagingResponse::new()
//!     .message_with_nouns(message);
//!
//! println!("{}", response.to_xml());
//! ```

use crate::xml_escape::{escape_xml_attr, escape_xml_text};
use crate::TwiML;

/// Attributes to pass to message
#[derive(Debug, Clone, Default)]
pub struct MessageAttributes {
    /// action - A URL specifying where Twilio should send status callbacks for the created outbound message.
    pub action: Option<String>,
    /// from - Phone Number to send Message from
    pub from: Option<String>,
    /// method - Action URL Method
    pub method: Option<String>,
    /// statusCallback - Status callback URL. Deprecated in favor of action.
    pub status_callback: Option<String>,
    /// to - Phone Number to send Message to
    pub to: Option<String>,
}

impl MessageAttributes {
    /// Create a new MessageAttributes
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the action URL
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Set the from phone number
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set the HTTP method
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Set the status callback URL
    pub fn status_callback(mut self, status_callback: impl Into<String>) -> Self {
        self.status_callback = Some(status_callback.into());
        self
    }

    /// Set the to phone number
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }
}

/// Attributes to pass to redirect
#[derive(Debug, Clone, Default)]
pub struct RedirectAttributes {
    /// method - Redirect URL method
    pub method: Option<String>,
}

impl RedirectAttributes {
    /// Create a new RedirectAttributes
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the HTTP method
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}

/// <Body> TwiML Noun
#[derive(Debug, Clone)]
pub struct Body {
    message: String,
}

impl Body {
    /// Create a new Body noun
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn to_xml(&self) -> String {
        format!("<Body>{}</Body>", escape_xml_text(&self.message))
    }
}

/// <Media> TwiML Noun
#[derive(Debug, Clone)]
pub struct Media {
    url: String,
}

impl Media {
    /// Create a new Media noun
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    fn to_xml(&self) -> String {
        format!("<Media>{}</Media>", escape_xml_text(&self.url))
    }
}

/// <Message> TwiML Verb
#[derive(Debug, Clone)]
pub struct Message {
    attributes: MessageAttributes,
    body: Option<Body>,
    media: Vec<Media>,
}

impl Message {
    /// Create a new Message verb with plain text (backward compatible)
    pub(crate) fn new(attributes: MessageAttributes, body: Option<String>) -> Self {
        Self {
            attributes,
            body: body.map(Body::new),
            media: Vec::new(),
        }
    }

    /// Create a new Message verb with Body and Media nouns
    pub fn with_nouns(attributes: MessageAttributes) -> Self {
        Self {
            attributes,
            body: None,
            media: Vec::new(),
        }
    }

    /// Add a Body noun to the Message
    pub fn body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    /// Add a Media noun to the Message
    pub fn add_media(mut self, media: Media) -> Self {
        self.media.push(media);
        self
    }

    /// Add multiple Media nouns to the Message
    pub fn media(mut self, media: Vec<Media>) -> Self {
        self.media = media;
        self
    }

    fn to_xml(&self) -> String {
        let mut xml = String::from("<Message");

        // Add attributes
        if let Some(ref action) = self.attributes.action {
            xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(action)));
        }
        if let Some(ref from) = self.attributes.from {
            xml.push_str(&format!(" from=\"{}\"", escape_xml_attr(from)));
        }
        if let Some(ref method) = self.attributes.method {
            xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(method)));
        }
        if let Some(ref status_callback) = self.attributes.status_callback {
            xml.push_str(&format!(
                " statusCallback=\"{}\"",
                escape_xml_attr(status_callback)
            ));
        }
        if let Some(ref to) = self.attributes.to {
            xml.push_str(&format!(" to=\"{}\"", escape_xml_attr(to)));
        }

        // Check if we have body or media
        if self.body.is_some() || !self.media.is_empty() {
            xml.push('>');

            // Add Body noun if present
            if let Some(ref body) = self.body {
                xml.push_str(&body.to_xml());
            }

            // Add Media nouns
            for media in &self.media {
                xml.push_str(&media.to_xml());
            }

            xml.push_str("</Message>");
        } else {
            xml.push_str(" />");
        }

        xml
    }
}

/// <Redirect> TwiML Verb
#[derive(Debug, Clone)]
pub struct Redirect {
    attributes: RedirectAttributes,
    url: String,
}

impl Redirect {
    /// Create a new Redirect verb
    pub(crate) fn new(attributes: RedirectAttributes, url: impl Into<String>) -> Self {
        Self {
            attributes,
            url: url.into(),
        }
    }

    fn to_xml(&self) -> String {
        let mut xml = String::from("<Redirect");

        if let Some(ref method) = self.attributes.method {
            xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(method)));
        }

        xml.push_str(&format!(">{}</Redirect>", escape_xml_text(&self.url)));
        xml
    }
}

/// Top-level TwiML verbs for messaging
#[derive(Debug, Clone)]
pub(crate) enum MessagingVerb {
    Message(Message),
    Redirect(Redirect),
}

/// <Response> TwiML for Messages
#[derive(Debug, Clone, Default)]
pub struct MessagingResponse {
    pub(crate) verbs: Vec<MessagingVerb>,
    comments_before: Vec<String>,
    comments: Vec<String>,
    comments_after: Vec<String>,
}

impl MessagingResponse {
    /// Create a new MessagingResponse
    ///
    /// <Response> TwiML for Messages
    pub fn new() -> Self {
        Self::default()
    }

    /// <Message> TwiML Verb
    ///
    /// Supports two calling patterns:
    /// - `message(body)` - Simple message with just body text
    /// - Use `message_with_attributes` for attributes + body
    ///
    /// # Arguments
    /// * `body` - Message Body
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn message(mut self, body: impl Into<String>) -> Self {
        let message = Message::new(MessageAttributes::default(), Some(body.into()));
        self.verbs.push(MessagingVerb::Message(message));
        self
    }

    /// <Message> TwiML Verb with attributes
    ///
    /// # Arguments
    /// * `attributes` - TwiML attributes
    /// * `body` - Message Body
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn message_with_attributes(
        mut self,
        attributes: MessageAttributes,
        body: impl Into<String>,
    ) -> Self {
        let message = Message::new(attributes, Some(body.into()));
        self.verbs.push(MessagingVerb::Message(message));
        self
    }

    /// <Message> TwiML Verb with Body and Media nouns
    ///
    /// This allows you to create messages with proper <Body> and <Media> nouns,
    /// supporting multiple media attachments for MMS.
    ///
    /// # Arguments
    /// * `message` - Pre-configured Message with Body and/or Media nouns
    ///
    /// # Returns
    /// Returns self for method chaining
    ///
    /// # Example
    /// ```
    /// use twiml_rust::{MessagingResponse, Message, MessageAttributes, Body, Media, TwiML};
    ///
    /// let message = Message::with_nouns(MessageAttributes::new())
    ///     .body(Body::new("Check out these images!"))
    ///     .add_media(Media::new("https://example.com/image1.jpg"))
    ///     .add_media(Media::new("https://example.com/image2.jpg"));
    ///
    /// let response = MessagingResponse::new().message_with_nouns(message);
    /// let xml = response.to_xml();
    /// ```
    pub fn message_with_nouns(mut self, message: Message) -> Self {
        self.verbs.push(MessagingVerb::Message(message));
        self
    }

    /// <Redirect> TwiML Verb
    ///
    /// Supports two calling patterns:
    /// - `redirect(url)` - Simple redirect with just URL
    /// - Use `redirect_with_attributes` for attributes + URL
    ///
    /// # Arguments
    /// * `url` - Redirect URL
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn redirect(mut self, url: impl Into<String>) -> Self {
        let redirect = Redirect::new(RedirectAttributes::default(), url);
        self.verbs.push(MessagingVerb::Redirect(redirect));
        self
    }

    /// <Redirect> TwiML Verb with attributes
    ///
    /// # Arguments
    /// * `attributes` - TwiML attributes
    /// * `url` - Redirect URL
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn redirect_with_attributes(
        mut self,
        attributes: RedirectAttributes,
        url: impl Into<String>,
    ) -> Self {
        let redirect = Redirect::new(attributes, url);
        self.verbs.push(MessagingVerb::Redirect(redirect));
        self
    }

    /// Comments in <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comments.push(comment.into());
        self
    }

    /// Comments after <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn comment_after(mut self, comment: impl Into<String>) -> Self {
        self.comments_after.push(comment.into());
        self
    }

    /// Comments before <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn comment_before(mut self, comment: impl Into<String>) -> Self {
        self.comments_before.push(comment.into());
        self
    }
}

impl TwiML for MessagingResponse {
    fn to_xml(&self) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

        // Add comments before Response
        for comment in &self.comments_before {
            xml.push_str(&format!("<!-- {} -->\n", escape_xml_text(comment)));
        }

        xml.push_str("<Response>");

        // Add comments inside Response
        for comment in &self.comments {
            xml.push_str(&format!("\n  <!-- {} -->", escape_xml_text(comment)));
        }

        for verb in &self.verbs {
            match verb {
                MessagingVerb::Message(message) => {
                    xml.push_str(&message.to_xml());
                }
                MessagingVerb::Redirect(redirect) => {
                    xml.push_str(&redirect.to_xml());
                }
            }
        }

        xml.push_str("</Response>");

        // Add comments after Response
        for comment in &self.comments_after {
            xml.push_str(&format!("\n<!-- {} -->", escape_xml_text(comment)));
        }

        xml
    }
}
