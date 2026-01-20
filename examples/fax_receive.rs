//! Fax TwiML examples
//!
//! Run this example with:
//! ```bash
//! cargo run --example fax_receive
//! ```

use twiml_rust::{
    fax::{ReceiveAttributes, ReceiveMediaType, ReceivePageSize},
    FaxResponse, TwiML,
};

fn main() {
    println!("=== TwiML Fax Examples ===\n");

    // Example 1: Simple fax receive
    println!("1. Simple Fax Receive:");
    simple_receive();

    // Example 2: Fax receive with PDF storage
    println!("\n2. Fax Receive with PDF Storage:");
    receive_as_pdf();

    // Example 3: Fax receive with all options
    println!("\n3. Fax Receive with All Options:");
    receive_with_all_options();

    // Example 4: Fax receive without storage
    println!("\n4. Fax Receive without Storage:");
    receive_no_storage();
}

/// Simple fax receive with defaults
fn simple_receive() {
    let response = FaxResponse::new().receive(Some(ReceiveAttributes::new()));

    println!("{}", response.to_xml());
}

/// Receive fax and store as PDF
fn receive_as_pdf() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new()
            .action("https://example.com/fax-received")
            .media_type(ReceiveMediaType::ApplicationPdf),
    ));

    println!("{}", response.to_xml());
}

/// Receive fax with all configuration options
fn receive_with_all_options() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new()
            .action("https://example.com/fax-received")
            .method("POST")
            .media_type(ReceiveMediaType::ApplicationPdf)
            .page_size(ReceivePageSize::Letter)
            .store_media(true),
    ));

    println!("{}", response.to_xml());
}

/// Receive fax without storing media
fn receive_no_storage() {
    let response = FaxResponse::new().receive(Some(
        ReceiveAttributes::new()
            .action("https://example.com/fax-metadata")
            .store_media(false),
    ));

    println!("{}", response.to_xml());
}
