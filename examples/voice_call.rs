//! Voice call TwiML examples
//!
//! Run this example with:
//! ```bash
//! cargo run --example voice_call
//! ```

use twiml_rust::{
    voice::{Dial, DialNumber, Gather, Record, Say},
    TwiML, VoiceResponse,
};

fn main() {
    println!("=== TwiML Voice Call Examples ===\n");

    // Example 1: Simple greeting
    println!("1. Simple Greeting:");
    simple_greeting();

    // Example 2: Interactive voice response (IVR)
    println!("\n2. Interactive Voice Response (IVR):");
    ivr_menu();

    // Example 3: Call forwarding
    println!("\n3. Call Forwarding:");
    call_forwarding();

    // Example 4: Voicemail recording
    println!("\n4. Voicemail Recording:");
    voicemail();

    // Example 5: Advanced SSML
    println!("\n5. Advanced SSML:");
    advanced_ssml();
}

/// Simple greeting with text-to-speech
fn simple_greeting() {
    let response = VoiceResponse::new()
        .say("Hello! Welcome to TwiML Rust.")
        .say("This is a simple voice response example.")
        .hangup();

    println!("{}", response.to_xml());
}

/// Interactive voice response menu
fn ivr_menu() {
    let gather = Gather::new()
        .input(vec!["dtmf".to_string(), "speech".to_string()])
        .action("https://example.com/handle-input")
        .method("POST")
        .timeout(10)
        .num_digits(1)
        .add_say(Say::new(
            "Press 1 for sales, 2 for support, or 3 for billing. You can also say the department name.",
        ));

    let response = VoiceResponse::new()
        .say("Welcome to our automated phone system.")
        .gather(gather)
        .say("We didn't receive any input.")
        .redirect("https://example.com/main-menu");

    println!("{}", response.to_xml());
}

/// Call forwarding to a phone number
fn call_forwarding() {
    let dial = Dial::new()
        .timeout(30)
        .add_number(DialNumber::new("+15559876543").send_digits("wwww1234"));

    let response = VoiceResponse::new()
        .say("Please wait while we connect your call.")
        .dial_with(dial)
        .say("The call could not be completed. Please try again later.")
        .hangup();

    println!("{}", response.to_xml());
}

/// Voicemail recording system
fn voicemail() {
    let record = Record::new()
        .action("https://example.com/handle-recording")
        .method("POST")
        .max_length(120)
        .finish_on_key("#")
        .transcribe(true)
        .transcribe_callback("https://example.com/transcription");

    let response = VoiceResponse::new()
        .say("Please leave a message after the beep. Press the pound key when finished.")
        .record(record)
        .say("Thank you for your message. Goodbye!")
        .hangup();

    println!("{}", response.to_xml());
}

/// Advanced SSML features
fn advanced_ssml() {
    let say = Say::new("Welcome to our service")
        .voice("Polly.Joanna")
        .language("en-US")
        .add_break(Some("medium".to_string()), None)
        .add_emphasis(Some("strong".to_string()), "Please listen carefully")
        .add_break(None, Some("1s".to_string()))
        .add_prosody(
            Some("high".to_string()),
            Some("slow".to_string()),
            None,
            "This is important information",
        );

    let response = VoiceResponse::new()
        .say_with(say)
        .play("https://example.com/music.mp3")
        .hangup();

    println!("{}", response.to_xml());
}
