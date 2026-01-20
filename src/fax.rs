//! TwiML generation for fax responses.
//!
//! This module provides types and builders for creating TwiML responses
//! that handle incoming faxes. The main entry point is [`FaxResponse`],
//! which can contain a [`Receive`] verb to configure how faxes are received.
//!
//! # Example
//!
//! ```rust
//! use twiml_rust::{FaxResponse, fax::{ReceiveAttributes, ReceiveMediaType}, TwiML};
//!
//! let response = FaxResponse::new()
//!     .receive(Some(
//!         ReceiveAttributes::new()
//!             .action("https://example.com/fax-received")
//!             .media_type(ReceiveMediaType::ApplicationPdf)
//!     ));
//!
//! println!("{}", response.to_xml());
//! ```

use crate::xml_escape::{escape_xml_attr, escape_xml_text};
use crate::TwiML;

/// Media type for fax storage
#[derive(Debug, Clone, PartialEq)]
pub enum ReceiveMediaType {
    /// PDF format - "application/pdf"
    ApplicationPdf,
    /// TIFF format - "image/tiff"
    ImageTiff,
}

impl ReceiveMediaType {
    fn as_str(&self) -> &str {
        match self {
            ReceiveMediaType::ApplicationPdf => "application/pdf",
            ReceiveMediaType::ImageTiff => "image/tiff",
        }
    }
}

/// Page size for received faxes
#[derive(Debug, Clone, PartialEq)]
pub enum ReceivePageSize {
    /// Letter size (8.5" x 11")
    Letter,
    /// Legal size (8.5" x 14")
    Legal,
    /// A4 size (210mm x 297mm)
    A4,
}

impl ReceivePageSize {
    fn as_str(&self) -> &str {
        match self {
            ReceivePageSize::Letter => "letter",
            ReceivePageSize::Legal => "legal",
            ReceivePageSize::A4 => "a4",
        }
    }
}

/// Attributes to pass to receive
#[derive(Debug, Clone, Default)]
pub struct ReceiveAttributes {
    /// action - Receive action URL
    pub action: Option<String>,
    /// mediaType - The media type used to store media in the fax media store
    pub media_type: Option<ReceiveMediaType>,
    /// method - Receive action URL method
    pub method: Option<String>,
    /// pageSize - What size to interpret received pages as
    pub page_size: Option<ReceivePageSize>,
    /// storeMedia - Whether or not to store received media in the fax media store
    pub store_media: Option<bool>,
}

impl ReceiveAttributes {
    /// Create a new ReceiveAttributes
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the action URL
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Set the media type
    pub fn media_type(mut self, media_type: ReceiveMediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    /// Set the HTTP method
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Set the page size
    pub fn page_size(mut self, page_size: ReceivePageSize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Set whether to store media
    pub fn store_media(mut self, store_media: bool) -> Self {
        self.store_media = Some(store_media);
        self
    }
}

/// <Receive> TwiML Verb
#[derive(Debug, Clone)]
pub struct Receive {
    attributes: ReceiveAttributes,
}

impl Receive {
    /// Create a new Receive verb with attributes
    pub(crate) fn new(attributes: Option<ReceiveAttributes>) -> Self {
        Self {
            attributes: attributes.unwrap_or_default(),
        }
    }

    fn to_xml(&self) -> String {
        let mut attrs = Vec::new();

        if let Some(ref action) = self.attributes.action {
            attrs.push(format!(" action=\"{}\"", escape_xml_attr(action)));
        }
        if let Some(ref media_type) = self.attributes.media_type {
            attrs.push(format!(" mediaType=\"{}\"", media_type.as_str()));
        }
        if let Some(ref method) = self.attributes.method {
            attrs.push(format!(" method=\"{}\"", escape_xml_attr(method)));
        }
        if let Some(ref page_size) = self.attributes.page_size {
            attrs.push(format!(" pageSize=\"{}\"", page_size.as_str()));
        }
        if let Some(store_media) = self.attributes.store_media {
            attrs.push(format!(" storeMedia=\"{}\"", store_media));
        }

        format!("<Receive{}/>", attrs.join(""))
    }
}

/// <Response> TwiML for Faxes
#[derive(Debug, Clone, Default)]
pub struct FaxResponse {
    receive: Option<Receive>,
    comments_before: Vec<String>,
    comments: Vec<String>,
    comments_after: Vec<String>,
}

impl FaxResponse {
    /// Create a new FaxResponse
    ///
    /// <Response> TwiML for Faxes
    pub fn new() -> Self {
        Self::default()
    }

    /// <Receive> TwiML Verb
    ///
    /// # Arguments
    /// * `attributes` - TwiML attributes
    ///
    /// # Returns
    /// Returns self for method chaining
    pub fn receive(mut self, attributes: Option<ReceiveAttributes>) -> Self {
        self.receive = Some(Receive::new(attributes));
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

impl TwiML for FaxResponse {
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

        if let Some(ref receive) = self.receive {
            xml.push_str(&receive.to_xml());
        }

        xml.push_str("</Response>");

        // Add comments after Response
        for comment in &self.comments_after {
            xml.push_str(&format!("\n<!-- {} -->", escape_xml_text(comment)));
        }

        xml
    }
}
