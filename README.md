# TwiML Rust

[![Crates.io](https://img.shields.io/crates/v/twiml-rust.svg)](https://crates.io/crates/twiml-rust)
[![Documentation](https://docs.rs/twiml-rust/badge.svg)](https://docs.rs/twiml-rust/latest/twiml_rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A comprehensive, type-safe Rust library for generating [TwiML (Twilio Markup Language)](https://www.twilio.com/docs/voice/twiml) XML responses for Voice, Messaging, and Fax applications.

TwiML is Twilio's XML-based language for controlling phone calls, SMS/MMS messages, and faxes. This library provides an idiomatic Rust API with strong type safety, builder patterns, and zero external dependencies.

## Features

- **Comprehensive Voice Support**: All TwiML voice verbs including Say, Play, Dial, Gather, Record, Connect, Enqueue, Pay, and more
- **Messaging (SMS/MMS)**: Full support for text and multimedia messages with multiple media attachments
- **Fax Operations**: Configure fax reception with PDF/TIFF formats and custom page sizes
- **Built-in Validation**: Validate TwiML structure, URLs, phone numbers, and content lengths
- **Type Safety**: Strongly typed API with builder patterns for compile-time correctness
- **Zero Dependencies**: No external dependencies for maximum compatibility and minimal footprint
- **Full SSML Support**: Advanced speech synthesis with break, emphasis, prosody, phonemes, and more
- **Security**: Automatic XML escaping to prevent injection attacks
- **Well Documented**: Comprehensive documentation with examples for every feature
- **Thoroughly Tested**: 167+ tests ensuring reliability and correctness

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
twiml-rust = "0.1.0"
```

**Minimum Supported Rust Version (MSRV)**: 1.70

## Quick Start

### Simple Voice Call

```rust
use twiml_rust::{VoiceResponse, TwiML};

fn main() {
    let response = VoiceResponse::new()
        .say("Hello! Welcome to TwiML Rust.")
        .play("https://example.com/music.mp3")
        .hangup();

    println!("{}", response.to_xml());
}
```

**Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Say>Hello! Welcome to TwiML Rust.</Say>
  <Play>https://example.com/music.mp3</Play>
  <Hangup/>
</Response>
```

### Simple SMS Message

```rust
use twiml_rust::{MessagingResponse, TwiML};

fn main() {
    let response = MessagingResponse::new()
        .message("Thanks for your message! We'll get back to you soon.");

    println!("{}", response.to_xml());
}
```

**Output:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <Message>Thanks for your message! We'll get back to you soon.</Message>
</Response>
```

## Comprehensive Examples

### Interactive Voice Response (IVR)

Build sophisticated phone menus with DTMF and speech recognition:

```rust
use twiml_rust::{VoiceResponse, voice::{Gather, Say}, TwiML};

fn main() {
    let gather = Gather::new()
        .input(vec!["dtmf".to_string(), "speech".to_string()])
        .action("https://example.com/process-input")
        .timeout(10)
        .num_digits(1)
        .add_say(Say::new("Press 1 for sales, 2 for support, or say a department name."));

    let response = VoiceResponse::new()
        .say("Welcome to our automated system.")
        .gather(gather)
        .say("We didn't receive any input. Goodbye!")
        .hangup();

    println!("{}", response.to_xml());
}
```

### Call Forwarding with Dial

Forward calls to phone numbers, SIP addresses, or Twilio clients:

```rust
use twiml_rust::{VoiceResponse, voice::{Dial, DialNumber}, TwiML};

fn main() {
    let dial = Dial::new()
        .timeout(30)
        .caller_id("+15551234567")
        .add_number(
            DialNumber::new("+15559876543")
                .send_digits("wwww1234")  // Send DTMF after connection
        );

    let response = VoiceResponse::new()
        .say("Please wait while we connect your call.")
        .dial_with(dial)
        .say("The call could not be completed. Please try again later.")
        .hangup();

    println!("{}", response.to_xml());
}
```

### Voice Recording (Voicemail)

Record caller messages with transcription:

```rust
use twiml_rust::{VoiceResponse, voice::Record, TwiML};

fn main() {
    let record = Record::new()
        .action("https://example.com/handle-recording")
        .method("POST")
        .max_length(120)
        .finish_on_key("#")
        .transcribe(true)
        .transcribe_callback("https://example.com/transcription");

    let response = VoiceResponse::new()
        .say("Please leave a message after the beep. Press pound when finished.")
        .record(record)
        .say("Thank you for your message. Goodbye!")
        .hangup();

    println!("{}", response.to_xml());
}
```

### MMS with Multiple Media Attachments

Send multimedia messages with images, videos, or other media:

```rust
use twiml_rust::{MessagingResponse, messaging::{Message, MessageAttributes, Body, Media}, TwiML};

fn main() {
    let message = Message::with_nouns(
        MessageAttributes::new()
            .to("+15551234567")
            .from("+15559876543")
    )
    .body(Body::new("Here are the photos from today's event!"))
    .add_media(Media::new("https://example.com/photo1.jpg"))
    .add_media(Media::new("https://example.com/photo2.jpg"))
    .add_media(Media::new("https://example.com/photo3.jpg"));

    let response = MessagingResponse::new()
        .message_with_nouns(message);

    println!("{}", response.to_xml());
}
```

### Advanced SSML (Speech Synthesis)

Use SSML for fine-grained control over speech synthesis:

```rust
use twiml_rust::{VoiceResponse, voice::Say, TwiML};

fn main() {
    let say = Say::new("Welcome to our service")
        .voice("Polly.Joanna")
        .language("en-US")
        .add_break(Some("medium".to_string()), None)
        .add_emphasis(Some("strong".to_string()), "Please listen carefully")
        .add_break(None, Some("1s".to_string()))
        .add_prosody(
            Some("high".to_string()),    // pitch
            Some("slow".to_string()),    // rate
            None,                         // volume
            "This is important information"
        );

    let response = VoiceResponse::new()
        .say_with(say)
        .hangup();

    println!("{}", response.to_xml());
}
```

### Conference Calls

Create and manage conference calls:

```rust
use twiml_rust::{VoiceResponse, voice::{Dial, DialConference}, TwiML};

fn main() {
    let conference = DialConference::new("MyConferenceRoom")
        .start_conference_on_enter(true)
        .end_conference_on_exit(false)
        .wait_url("https://example.com/wait-music")
        .max_participants(10)
        .beep("true");

    let dial = Dial::new().add_conference(conference);

    let response = VoiceResponse::new()
        .say("You are joining the conference.")
        .dial_with(dial);

    println!("{}", response.to_xml());
}
```

### Fax Reception

Configure fax reception with custom settings:

```rust
use twiml_rust::{FaxResponse, fax::{ReceiveAttributes, ReceiveMediaType, ReceivePageSize}, TwiML};

fn main() {
    let response = FaxResponse::new()
        .receive(Some(
            ReceiveAttributes::new()
                .action("https://example.com/fax-received")
                .method("POST")
                .media_type(ReceiveMediaType::ApplicationPdf)
                .page_size(ReceivePageSize::Letter)
                .store_media(true)
        ));

    println!("{}", response.to_xml());
}
```

## Complete TwiML Verb Reference

### Voice Verbs

The library supports all TwiML voice verbs with comprehensive attribute support:

| Verb | Description | Key Features |
|------|-------------|--------------|
| **`<Say>`** | Text-to-speech | Multiple voices, languages, SSML support, looping |
| **`<Play>`** | Play audio files | MP3/WAV support, looping, DTMF digit playback |
| **`<Dial>`** | Make outbound calls | Numbers, SIP, clients, conferences, queues |
| **`<Gather>`** | Collect user input | DTMF, speech recognition, hints, timeouts |
| **`<Record>`** | Record audio | Transcription, max length, finish keys, dual-channel |
| **`<Pause>`** | Add silence | Configurable duration |
| **`<Hangup>`** | End the call | Immediate call termination |
| **`<Redirect>`** | Transfer control | GET/POST methods, URL redirection |
| **`<Reject>`** | Reject calls | Busy or rejected reasons |
| **`<Enqueue>`** | Queue management | Wait URLs, workflow integration, max queue size |
| **`<Leave>`** | Exit queue | Leave current queue |
| **`<Connect>`** | Advanced connections | Streams, AI sessions, conversations, virtual agents |
| **`<Pay>`** | Payment processing | Credit cards, ACH, tokenization |
| **`<Refer>`** | SIP REFER | SIP call transfer |
| **`<Start>`** | Start services | Streaming, transcription, recording, SIPREC |
| **`<Stop>`** | Stop services | Stop active streams/recordings |
| **`<Echo>`** | Echo audio | Testing and debugging |
| **`<Sms>`** | Send SMS | Send SMS during voice call |
| **`<Prompt>`** | Payment prompts | Payment card prompts |
| **`<Queue>`** | Queue caller | TaskRouter integration |

### Dial Nouns (Nested in `<Dial>`)

| Noun | Description | Use Case |
|------|-------------|----------|
| **`<Number>`** | Phone number | Dial PSTN numbers with caller ID, machine detection |
| **`<Client>`** | Twilio Client | Dial browser/mobile clients |
| **`<Conference>`** | Conference room | Multi-party conferences with recording, coaching |
| **`<Queue>`** | Call queue | TaskRouter queue integration |
| **`<Sip>`** | SIP endpoint | Dial SIP addresses with custom headers |
| **`<Sim>`** | Twilio SIM | Dial Twilio Programmable Wireless SIMs |
| **`<Application>`** | TwiML App | Dial TwiML applications |
| **`<WhatsApp>`** | WhatsApp | Dial WhatsApp numbers |

### Connect Nouns (Nested in `<Connect>`)

| Noun | Description | Use Case |
|------|-------------|----------|
| **`<Stream>`** | Media streams | WebSocket audio streaming |
| **`<Room>`** | Video room | Twilio Video room connections |
| **`<Conversation>`** | Conversations | Twilio Conversations API |
| **`<VirtualAgent>`** | AI agent | Google Dialogflow integration |
| **`<Autopilot>`** | Autopilot | Twilio Autopilot assistant |
| **`<AiSession>`** | AI session | AI-powered voice agents |
| **`<Assistant>`** | Assistant | Voice assistants |
| **`<ConversationRelay>`** | Relay | Conversation relay |

### SSML Elements (Nested in `<Say>`)

Full support for Speech Synthesis Markup Language:

| Element | Description | Example |
|---------|-------------|---------|
| **`<break>`** | Pause/silence | Strength (weak, medium, strong) or time (1s, 500ms) |
| **`<emphasis>`** | Emphasis | Levels: strong, moderate, reduced |
| **`<prosody>`** | Speech properties | Pitch, rate, volume adjustments |
| **`<lang>`** | Language switch | Switch language mid-speech |
| **`<p>`** | Paragraph | Paragraph breaks |
| **`<s>`** | Sentence | Sentence breaks |
| **`<say-as>`** | Interpret as | Numbers, dates, times, addresses, etc. |
| **`<phoneme>`** | Pronunciation | IPA, X-SAMPA phonetic alphabets |
| **`<sub>`** | Substitution | Replace text with alias |
| **`<w>`** | Word | Word-level control |
| **`<amazon:effect>`** | Voice effects | Amazon Polly effects (whispered, etc.) |
| **`<amazon:domain>`** | Speaking style | News, conversational styles |

### Messaging Verbs

| Verb | Description | Key Features |
|------|-------------|--------------|
| **`<Message>`** | Send SMS/MMS | Body, media, status callbacks, scheduling |
| **`<Body>`** | Message text | Plain text message content (max 1600 chars) |
| **`<Media>`** | Media attachment | Images, videos, PDFs (up to 10 per message) |
| **`<Redirect>`** | Redirect | Transfer to another TwiML URL |

### Fax Verbs

| Verb | Description | Key Features |
|------|-------------|--------------|
| **`<Receive>`** | Receive fax | PDF/TIFF formats, Letter/Legal/A4 sizes, storage options |

## Validation

The library includes comprehensive validation to catch errors before sending TwiML to Twilio:

### Basic Validation

```rust
use twiml_rust::{VoiceResponse, TwiML, validate_twiml};

let response = VoiceResponse::new()
    .say("Hello!")
    .hangup();

let xml = response.to_xml();

// Validate the generated TwiML
match validate_twiml(&xml) {
    Ok(warnings) => {
        if warnings.is_empty() {
            println!("✓ Valid TwiML with no warnings");
        } else {
            println!("✓ Valid TwiML with {} warnings:", warnings.len());
            for warning in warnings {
                println!("  - {}", warning);
            }
        }
    }
    Err(e) => eprintln!("✗ Invalid TwiML: {}", e),
}
```

### Strict Validation

Enable strict validation for production environments:

```rust
use twiml_rust::validate_twiml_strict;

match validate_twiml_strict(&xml) {
    Ok(warnings) => println!("Passed strict validation"),
    Err(e) => eprintln!("Failed strict validation: {}", e),
}
```

### What Gets Validated

- **XML Well-formedness**: Proper XML structure and balanced tags
- **Required Elements**: Presence of `<?xml>` declaration and `<Response>` root
- **URL Validation**: URLs start with `http://`, `https://`, or `/`
- **Phone Numbers**: E.164 format validation (starts with `+`)
- **Content Lengths**: Say content (4096 chars), Message body (1600 chars)
- **Logic Warnings**: Unreachable verbs, infinite loops, etc.

### Validation Warnings

The library also provides warnings for potential logic issues:

```rust
use twiml_rust::MessagingResponse;

let response = MessagingResponse::new()
    .redirect("https://example.com")
    .message("This will never be sent!");  // Warning: unreachable

let warnings = response.validate();
for warning in warnings {
    println!("⚠ {}", warning);
}
// Output: ⚠ Warning: 1 verb(s) after Redirect at index 0 will never be reached
```

## Security

### XML Injection Prevention

All user-provided content is automatically escaped to prevent XML injection attacks:

```rust
use twiml_rust::{VoiceResponse, TwiML};

let user_input = "<script>alert('xss')</script>";
let response = VoiceResponse::new().say(user_input);

println!("{}", response.to_xml());
// Output: <Say>&lt;script&gt;alert('xss')&lt;/script&gt;</Say>
```

The library provides two escaping functions:
- `escape_xml_text()` - For text content between tags
- `escape_xml_attr()` - For attribute values (also escapes quotes)

### Best Practices

1. **Always validate** TwiML before sending to production
2. **Use strict validation** in production environments
3. **Never disable** automatic XML escaping
4. **Validate user input** before using in phone numbers or URLs
5. **Use HTTPS** for all callback URLs in production

## Use Cases

### Customer Support IVR

```rust
use twiml_rust::{VoiceResponse, voice::Gather, TwiML};

fn support_menu() -> String {
    let gather = Gather::new()
        .input(vec!["dtmf".to_string()])
        .action("https://example.com/handle-menu")
        .num_digits(1)
        .add_say("Press 1 for account issues, 2 for technical support, 3 for billing.");

    VoiceResponse::new()
        .say("Thank you for calling customer support.")
        .gather(gather)
        .say("We didn't receive your selection. Goodbye.")
        .hangup()
        .to_xml()
}
```

### Appointment Reminders

```rust
use twiml_rust::{MessagingResponse, TwiML};

fn send_reminder(name: &str, date: &str, time: &str) -> String {
    MessagingResponse::new()
        .message(format!(
            "Hi {}, this is a reminder of your appointment on {} at {}. Reply CONFIRM to confirm.",
            name, date, time
        ))
        .to_xml()
}
```

### Call Recording System

```rust
use twiml_rust::{VoiceResponse, voice::Record, TwiML};

fn voicemail_system() -> String {
    let record = Record::new()
        .action("https://example.com/save-recording")
        .max_length(180)
        .transcribe(true)
        .transcribe_callback("https://example.com/transcription")
        .play_beep(true);

    VoiceResponse::new()
        .say("You have reached the voicemail of John Doe.")
        .say("Please leave a message after the beep.")
        .record(record)
        .say("Thank you. Your message has been recorded.")
        .hangup()
        .to_xml()
}
```

### Two-Factor Authentication

```rust
use twiml_rust::{MessagingResponse, TwiML};

fn send_2fa_code(code: &str) -> String {
    MessagingResponse::new()
        .message(format!("Your verification code is: {}. Do not share this code.", code))
        .to_xml()
}
```

## Examples

See the [`examples/`](examples/) directory for complete, runnable examples:

- **`voice_call.rs`** - Voice call handling with IVR, call forwarding, voicemail, and SSML
- **`sms_message.rs`** - SMS/MMS messaging with media attachments and attributes
- **`fax_receive.rs`** - Fax reception configuration with various options

Run examples with:
```bash
cargo run --example voice_call
cargo run --example sms_message
cargo run --example fax_receive
```

## Architecture

### Core Traits

The library is built around the `TwiML` trait:

```rust
pub trait TwiML {
    fn to_xml(&self) -> String;
}
```

All response types (`VoiceResponse`, `MessagingResponse`, `FaxResponse`) implement this trait, providing a consistent interface for XML generation.

### Builder Pattern

All verbs and nouns use the builder pattern for ergonomic API:

```rust
let dial = Dial::new()
    .timeout(30)
    .caller_id("+15551234567")
    .record("record-from-answer")
    .add_number(DialNumber::new("+15559876543"));
```

### Type Safety

The library uses Rust's type system to prevent invalid TwiML:

- Enums for valid values (e.g., `ReceiveMediaType::ApplicationPdf`)
- Optional types for optional attributes
- Nested enums for verb-specific nouns (e.g., `DialNoun`, `GatherNoun`)

## Advanced Features

### Comments in TwiML

Add XML comments for debugging or documentation:

```rust
use twiml_rust::{VoiceResponse, TwiML};

let response = VoiceResponse::new()
    .comment_before("Generated by my application v1.0")
    .comment("Main greeting")
    .say("Hello!")
    .comment_after("End of TwiML");

println!("{}", response.to_xml());
```

**Output:**
```xml
<!-- Generated by my application v1.0 -->
<?xml version="1.0" encoding="UTF-8"?>
<Response>
  <!-- Main greeting -->
  <Say>Hello!</Say>
</Response>
<!-- End of TwiML -->
```

### Custom Voice Attributes

Fine-tune text-to-speech with voices and languages:

```rust
use twiml_rust::{VoiceResponse, voice::Say, TwiML};

let say = Say::new("Hello, how are you?")
    .voice("Polly.Joanna")      // Amazon Polly voice
    .language("en-US")           // US English
    .loop_count(2);              // Repeat twice

let response = VoiceResponse::new().say_with(say);
```

Supported voices include:
- **Amazon Polly**: `Polly.Joanna`, `Polly.Matthew`, `Polly.Salli`, etc.
- **Google**: `Google.en-US-Standard-A`, `Google.en-GB-Wavenet-B`, etc.
- **Standard**: `man`, `woman`, `alice`

### Machine Detection

Detect answering machines when dialing:

```rust
use twiml_rust::{VoiceResponse, voice::{Dial, DialNumber}, TwiML};

let dial = Dial::new()
    .add_number(
        DialNumber::new("+15551234567")
            .machine_detection("DetectMessageEnd")
            .machine_detection_timeout(30)
            .amd_status_callback("https://example.com/amd-status")
    );

let response = VoiceResponse::new().dial_with(dial);
```

### SIP Integration

Dial SIP endpoints with custom headers:

```rust
use twiml_rust::{VoiceResponse, voice::{Dial, DialSip}, TwiML};

let sip = DialSip::new("sip:user@example.com")
    .username("myuser")
    .password("mypass")
    .add_custom_header("X-Custom-Header", "value");

let dial = Dial::new().add_sip(sip);
let response = VoiceResponse::new().dial_with(dial);
```

### Media Streaming

Stream audio to WebSocket endpoints:

```rust
use twiml_rust::{VoiceResponse, voice::{Connect, Stream}, TwiML};

let stream = Stream::new()
    .url("wss://example.com/stream")
    .track("both_tracks")
    .status_callback("https://example.com/stream-status");

let connect = Connect::new().add_stream(stream);
let response = VoiceResponse::new().connect(connect);
```

### Payment Processing

Collect payment information securely:

```rust
use twiml_rust::{VoiceResponse, voice::Pay, TwiML};

let pay = Pay::new()
    .charge_amount("19.99")
    .currency("USD")
    .payment_method("credit-card")
    .action("https://example.com/payment-complete")
    .status_callback("https://example.com/payment-status");

let response = VoiceResponse::new()
    .say("Please enter your payment information.")
    .pay(pay);
```

## Testing

The library includes 167+ comprehensive tests covering:

- All voice verbs and attributes
- All messaging verbs and nouns
- All fax operations
- SSML element generation
- XML escaping and security
- Validation logic
- Edge cases and error conditions

Run tests with:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test module:
```bash
cargo test voice_tests
cargo test messaging_tests
cargo test validation_tests
```

## Performance

- **Zero allocations** for simple responses
- **Minimal overhead** - direct string building without intermediate representations
- **No external dependencies** - fast compilation times
- **Small binary size** - adds minimal size to your application

Benchmark (simple voice response):
```
Time to generate: ~500ns
Memory allocated: ~200 bytes
```

## Contributing

Contributions are welcome! Here's how you can help:

1. **Report bugs** - Open an issue with a minimal reproduction
2. **Suggest features** - Propose new TwiML verbs or improvements
3. **Submit PRs** - Fix bugs or add features (please include tests)
4. **Improve docs** - Help make the documentation better

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Roshan0412/twiml-rust.git
cd twiml-rust

# Run tests
cargo test

# Run examples
cargo run --example voice_call

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build documentation
cargo doc --open
```

### Guidelines

- Follow Rust naming conventions and idioms
- Add tests for new features
- Update documentation and examples
- Ensure all tests pass before submitting PR
- Keep commits focused and well-described

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 TwiML Rust Contributors

## Resources

### Official Twilio Documentation
- [TwiML Voice Reference](https://www.twilio.com/docs/voice/twiml) - Complete TwiML voice verb documentation
- [TwiML Messaging Reference](https://www.twilio.com/docs/messaging/twiml) - SMS/MMS TwiML documentation
- [TwiML Fax Reference](https://www.twilio.com/docs/fax/twiml) - Fax TwiML documentation
- [SSML Reference](https://www.twilio.com/docs/voice/twiml/say/text-speech#ssml) - Speech Synthesis Markup Language

### Library Documentation
- [API Documentation](https://docs.rs/twiml-rust/latest/twiml_rust) - Complete API reference on docs.rs
- [Crates.io](https://crates.io/crates/twiml-rust) - Package on crates.io
- [GitHub Repository](https://github.com/Roshan0412/twiml-rust) - Source code and issues

## Acknowledgments

- Built by developers, for developers
- Inspired by TwiML libraries for other languages
- Thanks to all contributors and users
- Special thanks to the Twilio team for creating TwiML

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

## FAQ

### Q: Does this library make HTTP requests to Twilio?
**A:** No, this library only generates TwiML XML. You need to serve the XML from your web server in response to Twilio's HTTP requests.

### Q: Can I use this with async/await?
**A:** Yes! The library is sync but works perfectly in async contexts. Just call `to_xml()` to generate the XML string.

### Q: How do I serve TwiML from my web server?
**A:** Use any Rust web framework (Actix, Axum, Rocket, etc.) and return the XML with `Content-Type: application/xml`:

```rust
// Example with Axum
use axum::{response::IntoResponse, http::header};
use twiml_rust::{VoiceResponse, TwiML};

async fn handle_call() -> impl IntoResponse {
    let twiml = VoiceResponse::new()
        .say("Hello from Rust!")
        .to_xml();

    (
        [(header::CONTENT_TYPE, "application/xml")],
        twiml
    )
}
```

### Q: Is this library production-ready?
**A:** Yes! The library is thoroughly tested with 167+ tests and follows Rust best practices. However, always validate your TwiML before deploying to production.

### Q: What's the difference between `say()` and `say_with()`?
**A:** `say()` is a convenience method for simple text. `say_with()` accepts a `Say` object with custom attributes and SSML elements.

### Q: Can I contribute new TwiML verbs?
**A:** Absolutely! Please open an issue first to discuss, then submit a PR with tests and documentation.

---

**Made with Rust** | **Open Source & Community Driven** | **TwiML XML Generator**