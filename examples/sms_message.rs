//! SMS/MMS messaging TwiML examples
//!
//! Run this example with:
//! ```bash
//! cargo run --example sms_message
//! ```

use twiml_rust::{
    messaging::{Body, Media, Message, MessageAttributes, RedirectAttributes},
    MessagingResponse, TwiML,
};

fn main() {
    println!("=== TwiML Messaging Examples ===\n");

    // Example 1: Simple SMS
    println!("1. Simple SMS:");
    simple_sms();

    // Example 2: SMS with attributes
    println!("\n2. SMS with Attributes:");
    sms_with_attributes();

    // Example 3: MMS with single media
    println!("\n3. MMS with Single Media:");
    mms_single_media();

    // Example 4: MMS with multiple media
    println!("\n4. MMS with Multiple Media:");
    mms_multiple_media();

    // Example 5: Message with redirect
    println!("\n5. Message with Redirect:");
    message_with_redirect();
}

/// Simple SMS message
fn simple_sms() {
    let response =
        MessagingResponse::new().message("Thanks for your message! We'll get back to you soon.");

    println!("{}", response.to_xml());
}

/// SMS with custom attributes
fn sms_with_attributes() {
    let response = MessagingResponse::new().message_with_attributes(
        MessageAttributes::new()
            .to("+15551234567")
            .from("+15559876543")
            .action("https://example.com/message-status")
            .method("POST")
            .status_callback("https://example.com/status"),
        "Your order #12345 has been shipped!",
    );

    println!("{}", response.to_xml());
}

/// MMS with a single media attachment
fn mms_single_media() {
    let message = Message::with_nouns(MessageAttributes::new())
        .body(Body::new("Check out this image!"))
        .add_media(Media::new("https://example.com/image.jpg"));

    let response = MessagingResponse::new().message_with_nouns(message);

    println!("{}", response.to_xml());
}

/// MMS with multiple media attachments
fn mms_multiple_media() {
    let message = Message::with_nouns(
        MessageAttributes::new()
            .to("+15551234567")
            .from("+15559876543"),
    )
    .body(Body::new("Here are the photos from today's event!"))
    .add_media(Media::new("https://example.com/photo1.jpg"))
    .add_media(Media::new("https://example.com/photo2.jpg"))
    .add_media(Media::new("https://example.com/photo3.jpg"));

    let response = MessagingResponse::new().message_with_nouns(message);

    println!("{}", response.to_xml());
}

/// Message followed by a redirect
fn message_with_redirect() {
    let response = MessagingResponse::new()
        .message("Processing your request...")
        .redirect_with_attributes(
            RedirectAttributes::new().method("POST"),
            "https://example.com/next-step",
        );

    println!("{}", response.to_xml());
}
