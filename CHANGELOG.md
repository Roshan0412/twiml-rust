# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Released]

## [0.1.0] - 2026-01-21

### Added
- Initial release of twiml-rust
- Voice TwiML support with comprehensive verb coverage:
  - `<Say>` with full SSML support (break, emphasis, prosody, lang, say-as, etc.)
  - `<Play>` for audio playback
  - `<Dial>` with support for numbers, SIP, clients, conferences, and queues
  - `<Gather>` for DTMF and speech input collection
  - `<Record>` for voice recording
  - `<Pause>` for delays
  - `<Hangup>` to end calls
  - `<Redirect>` for TwiML flow control
  - `<Reject>` for call rejection
  - `<Enqueue>` for queue management
  - `<Connect>` for streams, conversations, and autopilot
  - `<Pay>` for payment processing
  - `<Refer>` for SIP REFER
  - `<Start>` and `<Stop>` for stream control
  - And more...
- Messaging TwiML support:
  - `<Message>` with body and media support
  - `<Body>` and `<Media>` nouns for MMS
  - `<Redirect>` for messaging flow control
  - Support for multiple media attachments
- Fax TwiML support:
  - `<Receive>` with configurable media types and page sizes
  - Support for PDF and TIFF formats
  - Configurable storage options
- Built-in validation system:
  - XML well-formedness validation
  - TwiML structure validation
  - Warning system for potential logic issues
- Type-safe builder patterns for all TwiML elements
- XML escaping for security
- Comment support in TwiML responses
- Zero external dependencies
- Comprehensive test suite with 167+ tests
- Full documentation with examples

### Features
- Type-safe API with builder patterns
- Fluent interface for easy TwiML construction
- Automatic XML escaping to prevent injection attacks
- Validation helpers to ensure TwiML correctness
- Support for all major TwiML verbs and nouns
- SSML support for advanced speech synthesis
- MMS support with multiple media attachments

[0.1.0]: https://github.com/Roshan0412/twiml-rust/releases/tag/v0.1.0

