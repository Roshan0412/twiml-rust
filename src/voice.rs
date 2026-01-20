//! TwiML generation for voice call responses.
//!
//! This module provides comprehensive support for creating TwiML responses
//! for voice calls. The main entry point is [`VoiceResponse`], which can
//! contain various voice verbs like [`Say`], [`Play`], [`Dial`], [`Gather`],
//! [`Record`], and many more.
//!
//! # Examples
//!
//! ## Simple Voice Response
//!
//! ```rust
//! use twiml_rust::{VoiceResponse, TwiML};
//!
//! let response = VoiceResponse::new()
//!     .say("Hello! Welcome to our service.")
//!     .play("https://example.com/music.mp3")
//!     .hangup();
//!
//! println!("{}", response.to_xml());
//! ```
//!
//! ## Interactive Voice Response (IVR)
//!
//! ```rust
//! use twiml_rust::{VoiceResponse, voice::{Gather, Say}, TwiML};
//!
//! let gather = Gather::new()
//!     .input(vec!["dtmf".to_string(), "speech".to_string()])
//!     .action("https://example.com/process")
//!     .timeout(10)
//!     .add_say(Say::new("Press 1 for sales, 2 for support"));
//!
//! let response = VoiceResponse::new()
//!     .say("Welcome!")
//!     .gather(gather)
//!     .hangup();
//!
//! println!("{}", response.to_xml());
//! ```
//!
//! ## Call Forwarding
//!
//! ```rust
//! use twiml_rust::{VoiceResponse, voice::{Dial, DialNumber}, TwiML};
//!
//! let dial = Dial::new()
//!     .timeout(30)
//!     .add_number(DialNumber::new("+15551234567"));
//!
//! let response = VoiceResponse::new()
//!     .say("Connecting your call...")
//!     .dial_with(dial);
//!
//! println!("{}", response.to_xml());
//! ```

use crate::xml_escape::{escape_xml_attr, escape_xml_text};
use crate::TwiML;

// ===================================
// Type Aliases
// ===================================

pub type ApplicationEvent = &'static str; // "initiated" | "ringing" | "answered" | "completed"
pub type ClientEvent = &'static str; // "initiated" | "ringing" | "answered" | "completed"
pub type ConferenceBeep = &'static str; // "true" | "false" | "onEnter" | "onExit"
pub type ConferenceEvent = &'static str; // "start" | "end" | "join" | "leave" | "mute" | "hold" | "modify" | "speaker" | "announcement"
pub type ConferenceJitterBufferSize = &'static str; // "large" | "medium" | "small" | "off"
pub type ConferenceRecord = &'static str; // "do-not-record" | "record-from-start"
pub type ConferenceRecordingEvent = &'static str; // "in-progress" | "completed" | "absent"
pub type ConferenceRegion = &'static str; // "us1" | "us2" | "ie1" | "sg1" | "br1" | "au1" | "jp1" | "de1"
pub type ConferenceTrim = &'static str; // "trim-silence" | "do-not-trim"
pub type ConversationEvent = &'static str; // "call-initiated" | "call-ringing" | "call-answered" | "call-completed"
pub type ConversationRecord = &'static str; // "do-not-record" | "record-from-answer" | "record-from-ringing" | "record-from-answer-dual" | "record-from-ringing-dual" | "true" | "false"
pub type ConversationRecordingEvent = &'static str; // "in-progress" | "completed" | "absent"
pub type ConversationTrim = &'static str; // "trim-silence" | "do-not-trim"
pub type DialEvents = &'static str; // "call-progress-event"
pub type DialRecord = &'static str; // "do-not-record" | "record-from-answer" | "record-from-ringing" | "record-from-answer-dual" | "record-from-ringing-dual"
pub type DialRecordingEvent = &'static str; // "in-progress" | "completed" | "absent"
pub type DialRecordingTrack = &'static str; // "both" | "inbound" | "outbound"
pub type DialRingTone = &'static str; // "at" | "au" | "bg" | "br" | "be" | "ch" | "cl" | "cn" | "cz" | "de" | "dk" | "ee" | "es" | "fi" | "fr" | "gr" | "hu" | "il" | "in" | "it" | "lt" | "jp" | "mx" | "my" | "nl" | "no" | "nz" | "ph" | "pl" | "pt" | "ru" | "se" | "sg" | "th" | "uk" | "us" | "us-old" | "tw" | "ve" | "za"
pub type DialTrim = &'static str; // "trim-silence" | "do-not-trim"
pub type GatherInput = &'static str; // "dtmf" | "speech"
pub type GatherLanguage = &'static str; // "af-ZA" | "am-ET" | "ar-AE" | ... (many language codes)
pub type NumberEvent = &'static str; // "initiated" | "ringing" | "answered" | "completed"
pub type PayBankAccountType = &'static str; // "consumer-checking" | "consumer-savings" | "commercial-checking" | "commercial-savings"
pub type PayInput = &'static str; // "dtmf"
pub type PayLanguage = &'static str; // "de-DE" | "en-AU" | "en-CA" | "en-GB" | "en-IN" | "en-IE" | "en-NZ" | "en-PH" | "en-ZA" | "en-US" | "es-ES" | "es-US" | "fr-CA" | "fr-FR" | "it-IT"
pub type PayPaymentMethod = &'static str; // "ach-debit" | "credit-card"
pub type PayStatusCallbackMethod = &'static str; // "GET" | "POST"
pub type PayTokenType = &'static str; // "one-time" | "reusable" | "payment-method"
pub type PayValidCardTypes = &'static str; // "visa" | "mastercard" | "amex" | "maestro" | "discover" | "optima" | "jcb" | "diners-club" | "enroute"
pub type PromptCardType = &'static str; // "visa" | "mastercard" | "amex" | "maestro" | "discover" | "optima" | "jcb" | "diners-club" | "enroute"
pub type PromptErrorType = &'static str; // "timeout" | "invalid-card-number" | "invalid-card-type" | "invalid-date" | "invalid-security-code" | "internal-error" | "input-matching-failed"
pub type PromptFor = &'static str; // "payment-card-number" | "expiration-date" | "security-code" | "postal-code" | "payment-processing" | "bank-account-number" | "bank-routing-number"
pub type RecordRecordingEvent = &'static str; // "in-progress" | "completed" | "absent"
pub type RecordTrim = &'static str; // "trim-silence" | "do-not-trim"
pub type RecordingChannels = &'static str; // "mono" | "dual"
pub type RecordingEvent = &'static str; // "in-progress" | "completed" | "absent"
pub type RecordingRecordingStatusCallbackMethod = &'static str; // "GET" | "POST"
pub type RecordingTrack = &'static str; // "inbound" | "outbound" | "both"
pub type RecordingTrim = &'static str; // "trim-silence" | "do-not-trim"
pub type RejectReason = &'static str; // "rejected" | "busy"
pub type SayLanguage = &'static str; // "af-ZA" | "am-ET" | "ar-AE" | ... (many language codes)
pub type SayVoice = &'static str; // "man" | "woman" | "alice" | "Google.*" | "Polly.*" | ... (many voices)
pub type SipEvent = &'static str; // "initiated" | "ringing" | "answered" | "completed"
pub type SiprecStatusCallbackMethod = &'static str; // "GET" | "POST"
pub type SiprecTrack = &'static str; // "inbound_track" | "outbound_track" | "both_tracks"
pub type SsmlBreakStrength = &'static str; // "none" | "x-weak" | "weak" | "medium" | "strong" | "x-strong"
pub type SsmlEmphasisLevel = &'static str; // "strong" | "moderate" | "reduced"
pub type SsmlLangXmlLang = &'static str; // "arb" | "ar-AE" | "ca-ES" | ... (many language codes)
pub type SsmlPhonemeAlphabet = &'static str; // "ipa" | "x-sampa" | "x-amazon-jyutping" | "x-amazon-pinyin" | "x-amazon-pron-kana" | "x-amazon-yomigana"
pub type SsmlSayAsFormat = &'static str; // "mdy" | "dmy" | "ymd" | "md" | "dm" | "ym" | "my" | "d" | "m" | "y" | "yyyymmdd"
pub type SsmlSayAsInterpretAs = &'static str; // "characters" | "spell-out" | "cardinal" | "number" | "ordinal" | "digits" | "fraction" | "unit" | "date" | "time" | "address" | "expletive" | "telephone"
pub type StreamStatusCallbackMethod = &'static str; // "GET" | "POST"
pub type StreamTrack = &'static str; // "inbound_track" | "outbound_track" | "both_tracks"
pub type TranscriptionStatusCallbackMethod = &'static str; // "GET" | "POST"
pub type TranscriptionTrack = &'static str; // "inbound_track" | "outbound_track" | "both_tracks"
pub type WhatsAppEvent = &'static str; // "initiated" | "ringing" | "answered" | "completed"

// ================================================
// Attribute Structs
// ================================================

#[derive(Debug, Clone, Default)]
pub struct ConnectAttributes {
    pub action: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DialAttributes {
    pub action: Option<String>,
    pub answer_on_bridge: Option<bool>,
    pub caller_id: Option<String>,
    pub call_reason: Option<String>,
    pub events: Option<String>,
    pub hangup_on_star: Option<bool>,
    pub method: Option<String>,
    pub record: Option<String>,
    pub recording_status_callback: Option<String>,
    pub recording_status_callback_event: Option<Vec<String>>,
    pub recording_status_callback_method: Option<String>,
    pub recording_track: Option<String>,
    pub refer_method: Option<String>,
    pub refer_url: Option<String>,
    pub ring_tone: Option<String>,
    pub sequential: Option<bool>,
    pub time_limit: Option<u32>,
    pub timeout: Option<u32>,
    pub trim: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EnqueueAttributes {
    pub action: Option<String>,
    pub max_queue_size: Option<u32>,
    pub method: Option<String>,
    pub wait_url: Option<String>,
    pub wait_url_method: Option<String>,
    pub workflow_sid: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct GatherAttributes {
    pub action: Option<String>,
    pub action_on_empty_result: Option<bool>,
    pub barge_in: Option<bool>,
    pub debug: Option<bool>,
    pub dtmf_detection: Option<bool>,
    pub enhanced: Option<bool>,
    pub finish_on_key: Option<String>,
    pub hints: Option<String>,
    pub input: Option<Vec<String>>,
    pub language: Option<String>,
    pub max_speech_time: Option<u32>,
    pub method: Option<String>,
    pub num_digits: Option<u32>,
    pub partial_result_callback: Option<String>,
    pub partial_result_callback_method: Option<String>,
    pub profanity_filter: Option<bool>,
    pub speech_model: Option<String>,
    pub speech_timeout: Option<String>,
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct PauseAttributes {
    pub length: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct PayAttributes {
    pub action: Option<String>,
    pub bank_account_type: Option<String>,
    pub charge_amount: Option<String>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub input: Option<String>,
    pub language: Option<String>,
    pub max_attempts: Option<u32>,
    pub method: Option<String>,
    pub min_postal_code_length: Option<u32>,
    pub payment_connector: Option<String>,
    pub payment_method: Option<String>,
    pub postal_code: Option<bool>,
    pub security_code: Option<bool>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub timeout: Option<u32>,
    pub token_type: Option<String>,
    pub valid_card_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct PlayAttributes {
    pub digits: Option<String>,
    pub loop_count: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct PromptAttributes {
    pub attempt: Option<Vec<u32>>,
    pub card_type: Option<Vec<String>>,
    pub error_type: Option<Vec<String>>,
    pub for_attr: Option<String>,
    pub require_matching_inputs: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct QueueAttributes {
    pub method: Option<String>,
    pub post_work_activity_sid: Option<String>,
    pub reservation_sid: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RecordAttributes {
    pub action: Option<String>,
    pub finish_on_key: Option<String>,
    pub max_length: Option<u32>,
    pub method: Option<String>,
    pub play_beep: Option<bool>,
    pub recording_status_callback: Option<String>,
    pub recording_status_callback_event: Option<Vec<String>>,
    pub recording_status_callback_method: Option<String>,
    pub timeout: Option<u32>,
    pub transcribe: Option<bool>,
    pub transcribe_callback: Option<String>,
    pub trim: Option<String>,
    pub recording_channels: Option<String>, // "mono" | "dual"
    pub recording_track: Option<String>,    // "inbound" | "outbound" | "both"
}

#[derive(Debug, Clone, Default)]
pub struct RedirectAttributes {
    pub method: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ReferAttributes {
    pub action: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RejectAttributes {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SayAttributes {
    pub language: Option<String>,
    pub loop_count: Option<u32>,
    pub voice: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SmsAttributes {
    pub action: Option<String>,
    pub from: Option<String>,
    pub method: Option<String>,
    pub status_callback: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StartAttributes {
    pub action: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StreamAttributes {
    pub connector_name: Option<String>,
    pub name: Option<String>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub track: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SiprecAttributes {
    pub connector_name: Option<String>,
    pub name: Option<String>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub track: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TranscriptionAttributes {
    pub enable_automatic_punctuation: Option<bool>,
    pub hints: Option<String>,
    pub inbound_track_label: Option<String>,
    pub intelligence_service: Option<String>,
    pub language_code: Option<String>,
    pub name: Option<String>,
    pub outbound_track_label: Option<String>,
    pub partial_results: Option<bool>,
    pub profanity_filter: Option<bool>,
    pub speech_model: Option<String>,
    pub status_callback_method: Option<String>,
    pub status_callback_url: Option<String>,
    pub track: Option<String>,
    pub transcription_engine: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigAttributes {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ParameterAttributes {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RecordingAttributes {
    pub channels: Option<String>,
    pub recording_status_callback: Option<String>,
    pub recording_status_callback_event: Option<Vec<String>>,
    pub recording_status_callback_method: Option<String>,
    pub track: Option<String>,
    pub trim: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlBreakAttributes {
    pub strength: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlEmphasisAttributes {
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlLangAttributes {
    pub xml_lang: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlPhonemeAttributes {
    pub alphabet: Option<String>,
    pub ph: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlProsodyAttributes {
    pub pitch: Option<String>,
    pub rate: Option<String>,
    pub volume: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlSayAsAttributes {
    pub format: Option<String>,
    pub interpret_as: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlSubAttributes {
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SsmlWAttributes {
    pub role: Option<String>,
}

// ============================================================================
// VoiceResponse - Main Response Class
// ============================================================================

/// <Response> TwiML for Voice
#[derive(Debug, Clone, Default)]
pub struct VoiceResponse {
    verbs: Vec<VoiceVerb>,
    comments_before: Vec<String>,
    comments: Vec<String>,
    comments_after: Vec<String>,
}

/// Top-level TwiML verbs for voice calls
#[derive(Debug, Clone)]
pub enum VoiceVerb {
    Connect(Connect),
    Dial(Dial),
    Echo(Echo),
    Enqueue(Enqueue),
    Gather(Gather),
    Hangup(Hangup),
    Leave(Leave),
    Pause(Pause),
    Pay(Pay),
    Play(Play),
    Prompt(Prompt),
    Queue(Queue),
    Record(Record),
    Redirect(Redirect),
    Refer(Refer),
    Reject(Reject),
    Say(Say),
    Sms(Sms),
    Start(Start),
    Stop(Stop),
}

// ============================================================================
// Connect Verb - Connect to other services
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct Connect {
    pub attributes: ConnectAttributes,
    pub nested: Vec<ConnectNoun>,
}

impl Connect {
    pub fn new() -> Self {
        Self {
            attributes: ConnectAttributes::default(),
            nested: Vec::new(),
        }
    }

    pub fn with_attributes(attributes: ConnectAttributes) -> Self {
        Self {
            attributes,
            nested: Vec::new(),
        }
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn add_stream(mut self, stream: Stream) -> Self {
        self.nested.push(ConnectNoun::Stream(stream));
        self
    }

    pub fn add_room(mut self, room: Room) -> Self {
        self.nested.push(ConnectNoun::Room(room));
        self
    }

    pub fn add_conversation(mut self, conversation: Conversation) -> Self {
        self.nested.push(ConnectNoun::Conversation(conversation));
        self
    }

    pub fn add_virtual_agent(mut self, agent: VirtualAgent) -> Self {
        self.nested.push(ConnectNoun::VirtualAgent(agent));
        self
    }

    pub fn add_autopilot(mut self, autopilot: Autopilot) -> Self {
        self.nested.push(ConnectNoun::Autopilot(autopilot));
        self
    }

    pub fn add_ai_session(mut self, ai_session: AiSession) -> Self {
        self.nested.push(ConnectNoun::AiSession(ai_session));
        self
    }

    pub fn add_conversation_relay_session(mut self, session: ConversationRelaySession) -> Self {
        self.nested
            .push(ConnectNoun::ConversationRelaySession(session));
        self
    }

    pub fn add_assistant(mut self, assistant: Assistant) -> Self {
        self.nested.push(ConnectNoun::Assistant(assistant));
        self
    }

    pub fn add_conversation_relay(mut self, relay: ConversationRelay) -> Self {
        self.nested.push(ConnectNoun::ConversationRelay(relay));
        self
    }
}

// ============================================================================
// Dial Verb - Connect to Another Number
// ============================================================================

#[derive(Debug, Clone)]
pub struct Dial {
    pub attributes: DialAttributes,
    pub number: Option<String>,
    pub nested: Vec<DialNoun>,
}

impl Dial {
    pub fn new() -> Self {
        Self {
            attributes: DialAttributes::default(),
            number: None,
            nested: Vec::new(),
        }
    }

    pub fn with_attributes(attributes: DialAttributes) -> Self {
        Self {
            attributes,
            number: None,
            nested: Vec::new(),
        }
    }

    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = Some(number.into());
        self
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.attributes.timeout = Some(timeout);
        self
    }

    pub fn call_reason(mut self, call_reason: impl Into<String>) -> Self {
        self.attributes.call_reason = Some(call_reason.into());
        self
    }

    pub fn add_number(mut self, number: DialNumber) -> Self {
        self.nested.push(DialNoun::Number(number));
        self
    }

    pub fn add_client(mut self, client: DialClient) -> Self {
        self.nested.push(DialNoun::Client(client));
        self
    }

    pub fn add_conference(mut self, conference: DialConference) -> Self {
        self.nested.push(DialNoun::Conference(conference));
        self
    }

    pub fn add_queue(mut self, queue: DialQueue) -> Self {
        self.nested.push(DialNoun::Queue(queue));
        self
    }

    pub fn add_sip(mut self, sip: DialSip) -> Self {
        self.nested.push(DialNoun::Sip(sip));
        self
    }

    pub fn add_sim(mut self, sim: DialSim) -> Self {
        self.nested.push(DialNoun::Sim(sim));
        self
    }

    pub fn add_application(mut self, application: DialApplication) -> Self {
        self.nested.push(DialNoun::Application(application));
        self
    }

    pub fn add_whatsapp(mut self, whatsapp: DialWhatsApp) -> Self {
        self.nested.push(DialNoun::WhatsApp(whatsapp));
        self
    }
}

// ============================================================================
// Echo Verb
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self
    }
}

// ============================================================================
// Enqueue Verb - Add call to queue
// ============================================================================

#[derive(Debug, Clone)]
pub struct Enqueue {
    pub attributes: EnqueueAttributes,
    pub name: Option<String>,
    pub task: Option<Task>,
}

impl Enqueue {
    pub fn new() -> Self {
        Self {
            attributes: EnqueueAttributes::default(),
            name: None,
            task: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn wait_url(mut self, wait_url: impl Into<String>) -> Self {
        self.attributes.wait_url = Some(wait_url.into());
        self
    }

    pub fn wait_url_method(mut self, method: impl Into<String>) -> Self {
        self.attributes.wait_url_method = Some(method.into());
        self
    }

    pub fn workflow_sid(mut self, workflow_sid: impl Into<String>) -> Self {
        self.attributes.workflow_sid = Some(workflow_sid.into());
        self
    }
}

// ============================================================================
// Gather Verb - Collect User Input
// ============================================================================

#[derive(Debug, Clone)]
pub struct Gather {
    pub attributes: GatherAttributes,
    pub nested: Vec<GatherNoun>,
}

impl Gather {
    pub fn new() -> Self {
        Self {
            attributes: GatherAttributes::default(),
            nested: Vec::new(),
        }
    }

    pub fn with_attributes(attributes: GatherAttributes) -> Self {
        Self {
            attributes,
            nested: Vec::new(),
        }
    }

    pub fn input(mut self, input: Vec<String>) -> Self {
        self.attributes.input = Some(input);
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.attributes.timeout = Some(timeout);
        self
    }

    pub fn finish_on_key(mut self, key: impl Into<String>) -> Self {
        self.attributes.finish_on_key = Some(key.into());
        self
    }

    pub fn num_digits(mut self, num_digits: u32) -> Self {
        self.attributes.num_digits = Some(num_digits);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.attributes.language = Some(language.into());
        self
    }

    pub fn hints(mut self, hints: impl Into<String>) -> Self {
        self.attributes.hints = Some(hints.into());
        self
    }

    pub fn action_on_empty_result(mut self, action_on_empty_result: bool) -> Self {
        self.attributes.action_on_empty_result = Some(action_on_empty_result);
        self
    }

    pub fn barge_in(mut self, barge_in: bool) -> Self {
        self.attributes.barge_in = Some(barge_in);
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.attributes.debug = Some(debug);
        self
    }

    pub fn dtmf_detection(mut self, dtmf_detection: bool) -> Self {
        self.attributes.dtmf_detection = Some(dtmf_detection);
        self
    }

    pub fn enhanced(mut self, enhanced: bool) -> Self {
        self.attributes.enhanced = Some(enhanced);
        self
    }

    pub fn max_speech_time(mut self, max_speech_time: u32) -> Self {
        self.attributes.max_speech_time = Some(max_speech_time);
        self
    }

    pub fn partial_result_callback(mut self, callback: impl Into<String>) -> Self {
        self.attributes.partial_result_callback = Some(callback.into());
        self
    }

    pub fn partial_result_callback_method(mut self, method: impl Into<String>) -> Self {
        self.attributes.partial_result_callback_method = Some(method.into());
        self
    }

    pub fn profanity_filter(mut self, profanity_filter: bool) -> Self {
        self.attributes.profanity_filter = Some(profanity_filter);
        self
    }

    pub fn speech_model(mut self, speech_model: impl Into<String>) -> Self {
        self.attributes.speech_model = Some(speech_model.into());
        self
    }

    pub fn speech_timeout(mut self, speech_timeout: impl Into<String>) -> Self {
        self.attributes.speech_timeout = Some(speech_timeout.into());
        self
    }

    pub fn add_say(mut self, say: Say) -> Self {
        self.nested.push(GatherNoun::Say(say));
        self
    }

    pub fn add_play(mut self, play: Play) -> Self {
        self.nested.push(GatherNoun::Play(play));
        self
    }

    pub fn add_pause(mut self, pause: Pause) -> Self {
        self.nested.push(GatherNoun::Pause(pause));
        self
    }
}

// ============================================================================
// Hangup Verb
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct Hangup;

impl Hangup {
    pub fn new() -> Self {
        Self
    }
}

// ============================================================================
// Leave Verb
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct Leave;

impl Leave {
    pub fn new() -> Self {
        Self
    }
}

// ============================================================================
// Pause Verb - Silent Pause
// ============================================================================

#[derive(Debug, Clone)]
pub struct Pause {
    pub attributes: PauseAttributes,
}

impl Pause {
    pub fn new(attributes: Option<PauseAttributes>) -> Self {
        Self {
            attributes: attributes.unwrap_or_default(),
        }
    }
}

// ============================================================================
// Pay Verb - Collect payment information
// ============================================================================

#[derive(Debug, Clone)]
pub struct Pay {
    pub attributes: PayAttributes,
    pub prompts: Vec<Prompt>,
    pub parameters: Vec<Parameter>,
}

impl Pay {
    pub fn new() -> Self {
        Self {
            attributes: PayAttributes::default(),
            prompts: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.attributes.input = Some(input.into());
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn charge_amount(mut self, charge_amount: impl Into<String>) -> Self {
        self.attributes.charge_amount = Some(charge_amount.into());
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.attributes.currency = Some(currency.into());
        self
    }

    pub fn payment_connector(mut self, payment_connector: impl Into<String>) -> Self {
        self.attributes.payment_connector = Some(payment_connector.into());
        self
    }

    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.attributes.payment_method = Some(payment_method.into());
        self
    }
}

// ============================================================================
// Play Verb - Play Audio
// ============================================================================

#[derive(Debug, Clone)]
pub struct Play {
    pub attributes: PlayAttributes,
    pub url: Option<String>,
}

impl Play {
    pub fn new() -> Self {
        Self {
            attributes: PlayAttributes::default(),
            url: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn loop_count(mut self, loop_count: u32) -> Self {
        self.attributes.loop_count = Some(loop_count);
        self
    }

    pub fn digits(mut self, digits: impl Into<String>) -> Self {
        self.attributes.digits = Some(digits.into());
        self
    }
}

// ============================================================================
// Prompt Verb
// ============================================================================

#[derive(Debug, Clone)]
pub struct Prompt {
    pub attributes: PromptAttributes,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            attributes: PromptAttributes::default(),
        }
    }

    pub fn with_attributes(attributes: PromptAttributes) -> Self {
        Self { attributes }
    }

    pub fn for_attr(mut self, for_attr: impl Into<String>) -> Self {
        self.attributes.for_attr = Some(for_attr.into());
        self
    }

    pub fn attempt(mut self, attempt: Vec<u32>) -> Self {
        self.attributes.attempt = Some(attempt);
        self
    }

    pub fn card_type(mut self, card_type: Vec<String>) -> Self {
        self.attributes.card_type = Some(card_type);
        self
    }

    pub fn error_type(mut self, error_type: Vec<String>) -> Self {
        self.attributes.error_type = Some(error_type);
        self
    }
}

// ============================================================================
// Queue Verb - Join a queue
// ============================================================================

#[derive(Debug, Clone)]
pub struct Queue {
    pub attributes: QueueAttributes,
    pub name: String,
}

impl Queue {
    pub fn new(attributes: QueueAttributes, name: String) -> Self {
        Self { attributes, name }
    }
}

// ============================================================================
// Record Verb - Record Audio
// ============================================================================

#[derive(Debug, Clone)]
pub struct Record {
    pub attributes: RecordAttributes,
}

impl Record {
    pub fn new() -> Self {
        Self {
            attributes: RecordAttributes::default(),
        }
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.attributes.timeout = Some(timeout);
        self
    }

    pub fn max_length(mut self, max_length: u32) -> Self {
        self.attributes.max_length = Some(max_length);
        self
    }

    pub fn play_beep(mut self, play_beep: bool) -> Self {
        self.attributes.play_beep = Some(play_beep);
        self
    }

    pub fn transcribe(mut self, transcribe: bool) -> Self {
        self.attributes.transcribe = Some(transcribe);
        self
    }

    pub fn transcribe_callback(mut self, callback: impl Into<String>) -> Self {
        self.attributes.transcribe_callback = Some(callback.into());
        self
    }

    pub fn recording_channels(mut self, channels: impl Into<String>) -> Self {
        self.attributes.recording_channels = Some(channels.into());
        self
    }

    pub fn recording_track(mut self, track: impl Into<String>) -> Self {
        self.attributes.recording_track = Some(track.into());
        self
    }

    pub fn finish_on_key(mut self, key: impl Into<String>) -> Self {
        self.attributes.finish_on_key = Some(key.into());
        self
    }

    pub fn trim(mut self, trim: impl Into<String>) -> Self {
        self.attributes.trim = Some(trim.into());
        self
    }

    pub fn recording_status_callback(mut self, callback: impl Into<String>) -> Self {
        self.attributes.recording_status_callback = Some(callback.into());
        self
    }

    pub fn recording_status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.attributes.recording_status_callback_method = Some(method.into());
        self
    }

    pub fn recording_status_callback_event(mut self, events: Vec<String>) -> Self {
        self.attributes.recording_status_callback_event = Some(events);
        self
    }
}

// ============================================================================
// Redirect Verb
// ============================================================================

#[derive(Debug, Clone)]
pub struct Redirect {
    pub attributes: RedirectAttributes,
    pub url: String,
}

impl Redirect {
    pub fn new(attributes: RedirectAttributes, url: String) -> Self {
        Self { attributes, url }
    }
}

// ============================================================================
// Refer Verb - SIP REFER
// ============================================================================

#[derive(Debug, Clone)]
pub struct Refer {
    pub attributes: ReferAttributes,
    pub refer_sip: Option<ReferSip>,
}

impl Refer {
    pub fn new() -> Self {
        Self {
            attributes: ReferAttributes::default(),
            refer_sip: None,
        }
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn add_refer_sip(mut self, refer_sip: ReferSip) -> Self {
        self.refer_sip = Some(refer_sip);
        self
    }
}

// ============================================================================
// Reject Verb
// ============================================================================

#[derive(Debug, Clone)]
pub struct Reject {
    pub attributes: RejectAttributes,
}

impl Reject {
    pub fn new() -> Self {
        Self {
            attributes: RejectAttributes::default(),
        }
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.attributes.reason = Some(reason.into());
        self
    }
}

// ============================================================================
// Say Verb - Text-to-Speech
// ============================================================================

#[derive(Debug, Clone)]
pub struct Say {
    pub attributes: SayAttributes,
    pub message: String,
    pub ssml_elements: Vec<SsmlElement>,
}

impl Say {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            attributes: SayAttributes::default(),
            message: message.into(),
            ssml_elements: Vec::new(),
        }
    }

    pub fn voice(mut self, voice: impl Into<String>) -> Self {
        self.attributes.voice = Some(voice.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.attributes.language = Some(language.into());
        self
    }

    pub fn loop_count(mut self, loop_count: u32) -> Self {
        self.attributes.loop_count = Some(loop_count);
        self
    }

    pub fn add_break(mut self, strength: Option<String>, time: Option<String>) -> Self {
        self.ssml_elements
            .push(SsmlElement::Break { strength, time });
        self
    }

    pub fn add_emphasis(mut self, level: Option<String>, text: impl Into<String>) -> Self {
        self.ssml_elements.push(SsmlElement::Emphasis {
            level,
            text: text.into(),
        });
        self
    }

    pub fn add_prosody(
        mut self,
        pitch: Option<String>,
        rate: Option<String>,
        volume: Option<String>,
        text: impl Into<String>,
    ) -> Self {
        self.ssml_elements.push(SsmlElement::Prosody {
            pitch,
            rate,
            volume,
            text: text.into(),
        });
        self
    }

    pub fn add_lang(mut self, xml_lang: impl Into<String>, text: impl Into<String>) -> Self {
        self.ssml_elements.push(SsmlElement::Lang {
            xml_lang: xml_lang.into(),
            text: text.into(),
        });
        self
    }

    pub fn add_p(mut self, text: impl Into<String>) -> Self {
        self.ssml_elements
            .push(SsmlElement::P { text: text.into() });
        self
    }

    pub fn add_s(mut self, text: impl Into<String>) -> Self {
        self.ssml_elements
            .push(SsmlElement::S { text: text.into() });
        self
    }

    pub fn add_phoneme(
        mut self,
        ph: impl Into<String>,
        text: impl Into<String>,
        alphabet: Option<String>,
    ) -> Self {
        self.ssml_elements.push(SsmlElement::Phoneme {
            alphabet,
            ph: ph.into(),
            text: text.into(),
        });
        self
    }

    pub fn add_say_as(
        mut self,
        interpret_as: impl Into<String>,
        text: impl Into<String>,
        format: Option<String>,
    ) -> Self {
        self.ssml_elements.push(SsmlElement::SayAs {
            interpret_as: interpret_as.into(),
            format,
            text: text.into(),
        });
        self
    }

    pub fn add_sub(mut self, alias: impl Into<String>, text: impl Into<String>) -> Self {
        self.ssml_elements.push(SsmlElement::Sub {
            alias: alias.into(),
            text: text.into(),
        });
        self
    }

    pub fn add_w(mut self, text: impl Into<String>, role: Option<String>) -> Self {
        self.ssml_elements.push(SsmlElement::W {
            role,
            text: text.into(),
        });
        self
    }

    pub fn add_amazon_effect(mut self, name: impl Into<String>, text: impl Into<String>) -> Self {
        self.ssml_elements.push(SsmlElement::AmazonEffect {
            name: name.into(),
            text: text.into(),
        });
        self
    }

    pub fn add_amazon_domain(mut self, name: impl Into<String>, text: impl Into<String>) -> Self {
        self.ssml_elements.push(SsmlElement::AmazonDomain {
            name: name.into(),
            text: text.into(),
        });
        self
    }
}

// ============================================================================
// Sms Verb - Send SMS during call
// ============================================================================

#[derive(Debug, Clone)]
pub struct Sms {
    pub attributes: SmsAttributes,
    pub message: String,
}

impl Sms {
    pub fn new(attributes: SmsAttributes, message: String) -> Self {
        Self {
            attributes,
            message,
        }
    }
}

// ============================================================================
// Start Verb - Start media streaming/recording
// ============================================================================

#[derive(Debug, Clone)]
pub struct Start {
    pub attributes: StartAttributes,
    pub nested: Vec<StartNoun>,
}

impl Start {
    pub fn new() -> Self {
        Self {
            attributes: StartAttributes::default(),
            nested: Vec::new(),
        }
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.attributes.action = Some(action.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.attributes.method = Some(method.into());
        self
    }

    pub fn add_stream(mut self, stream: Stream) -> Self {
        self.nested.push(StartNoun::Stream(stream));
        self
    }

    pub fn add_siprec(mut self, siprec: Siprec) -> Self {
        self.nested.push(StartNoun::Siprec(siprec));
        self
    }

    pub fn add_transcription(mut self, transcription: Transcription) -> Self {
        self.nested.push(StartNoun::Transcription(transcription));
        self
    }

    pub fn add_recording(mut self, recording: Recording) -> Self {
        self.nested.push(StartNoun::Recording(recording));
        self
    }
}

// ============================================================================
// Stop Verb
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct Stop;

impl Stop {
    pub fn new() -> Self {
        Self
    }
}

// ============================================================================
// Supporting Types and Nested Elements
// ============================================================================

// Stream - Media streaming
#[derive(Debug, Clone, Default)]
pub struct Stream {
    pub name: Option<String>,
    pub connector_name: Option<String>,
    pub url: Option<String>,
    pub track: Option<String>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub parameters: Vec<Parameter>,
}

impl Stream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn connector_name(mut self, connector_name: impl Into<String>) -> Self {
        self.connector_name = Some(connector_name.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn track(mut self, track: impl Into<String>) -> Self {
        self.track = Some(track.into());
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Self {
        self.status_callback = Some(callback.into());
        self
    }

    pub fn add_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }
}

// Room - Video room
#[derive(Debug, Clone, Default)]
pub struct Room {
    pub name: Option<String>,
    pub participant_identity: Option<String>,
}

impl Room {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            participant_identity: None,
        }
    }

    pub fn participant_identity(mut self, identity: impl Into<String>) -> Self {
        self.participant_identity = Some(identity.into());
        self
    }
}

// Conversation - Conversation API
#[derive(Debug, Clone, Default)]
pub struct Conversation {
    pub service_instance_sid: Option<String>,
}

impl Conversation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn service_instance_sid(mut self, sid: impl Into<String>) -> Self {
        self.service_instance_sid = Some(sid.into());
        self
    }
}

// VirtualAgent - AI Virtual Agent
#[derive(Debug, Clone, Default)]
pub struct VirtualAgent {
    pub connector_name: Option<String>,
    pub language: Option<String>,
    pub parameters: Vec<Parameter>,
}

impl VirtualAgent {
    pub fn new(connector_name: impl Into<String>) -> Self {
        Self {
            connector_name: Some(connector_name.into()),
            ..Default::default()
        }
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn add_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }
}

// Autopilot - Autopilot Assistant
#[derive(Debug, Clone, Default)]
pub struct Autopilot {
    pub name: Option<String>,
}

impl Autopilot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
        }
    }
}

// AiSession - AI Session
#[derive(Debug, Clone, Default)]
pub struct AiSession {
    pub assistant_sid: Option<String>,
}

impl AiSession {
    pub fn new(assistant_sid: impl Into<String>) -> Self {
        Self {
            assistant_sid: Some(assistant_sid.into()),
        }
    }
}

// ConversationRelaySession - Conversation Relay Session
#[derive(Debug, Clone, Default)]
pub struct ConversationRelaySession {
    pub connector: Option<String>,
    pub session_configuration: Option<String>,
}

impl ConversationRelaySession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connector(mut self, connector: impl Into<String>) -> Self {
        self.connector = Some(connector.into());
        self
    }

    pub fn session_configuration(mut self, session_configuration: impl Into<String>) -> Self {
        self.session_configuration = Some(session_configuration.into());
        self
    }
}

// Assistant - Assistant
#[derive(Debug, Clone, Default)]
pub struct Assistant {
    pub sid: Option<String>,
}

impl Assistant {
    pub fn new(sid: impl Into<String>) -> Self {
        Self {
            sid: Some(sid.into()),
        }
    }
}

// ConversationRelay - Conversation Relay
#[derive(Debug, Clone, Default)]
pub struct ConversationRelay {
    pub url: Option<String>,
    pub welcome_greeting: Option<String>,
    pub voice: Option<String>,
    pub language: Option<String>,
    pub dtmf_detection: Option<bool>,
    pub interruptible: Option<bool>,
    pub interruption_sensitivity: Option<String>,
    pub speech_model: Option<String>,
    pub profanity_filter: Option<bool>,
    pub transcription_enabled: Option<bool>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub max_duration: Option<u32>,
    pub languages: Vec<Language>,
    pub parameters: Vec<Parameter>,
}

impl ConversationRelay {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Default::default()
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn welcome_greeting(mut self, greeting: impl Into<String>) -> Self {
        self.welcome_greeting = Some(greeting.into());
        self
    }

    pub fn voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn dtmf_detection(mut self, enabled: bool) -> Self {
        self.dtmf_detection = Some(enabled);
        self
    }

    pub fn interruptible(mut self, enabled: bool) -> Self {
        self.interruptible = Some(enabled);
        self
    }

    pub fn interruption_sensitivity(mut self, sensitivity: impl Into<String>) -> Self {
        self.interruption_sensitivity = Some(sensitivity.into());
        self
    }

    pub fn speech_model(mut self, model: impl Into<String>) -> Self {
        self.speech_model = Some(model.into());
        self
    }

    pub fn profanity_filter(mut self, enabled: bool) -> Self {
        self.profanity_filter = Some(enabled);
        self
    }

    pub fn transcription_enabled(mut self, enabled: bool) -> Self {
        self.transcription_enabled = Some(enabled);
        self
    }

    pub fn status_callback(mut self, url: impl Into<String>) -> Self {
        self.status_callback = Some(url.into());
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn max_duration(mut self, seconds: u32) -> Self {
        self.max_duration = Some(seconds);
        self
    }

    pub fn add_language(mut self, language: Language) -> Self {
        self.languages.push(language);
        self
    }

    pub fn add_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }
}

// Parameter - Generic parameter for various verbs
#[derive(Debug, Clone, Default)]
pub struct Parameter {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl Parameter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

// ReferSip - SIP reference for Refer verb
#[derive(Debug, Clone)]
pub struct ReferSip {
    pub sip_url: String,
}

impl ReferSip {
    pub fn new(sip_url: impl Into<String>) -> Self {
        Self {
            sip_url: sip_url.into(),
        }
    }
}

// Transcription - Live transcription
#[derive(Debug, Clone, Default)]
pub struct Transcription {
    pub attributes: TranscriptionAttributes,
}

impl Transcription {
    pub fn new() -> Self {
        Self {
            attributes: TranscriptionAttributes::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.attributes.name = Some(name.into());
        self
    }

    pub fn track(mut self, track: impl Into<String>) -> Self {
        self.attributes.track = Some(track.into());
        self
    }

    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.attributes.language_code = Some(language_code.into());
        self
    }

    pub fn enable_automatic_punctuation(mut self, enable: bool) -> Self {
        self.attributes.enable_automatic_punctuation = Some(enable);
        self
    }

    pub fn hints(mut self, hints: impl Into<String>) -> Self {
        self.attributes.hints = Some(hints.into());
        self
    }

    pub fn inbound_track_label(mut self, label: impl Into<String>) -> Self {
        self.attributes.inbound_track_label = Some(label.into());
        self
    }

    pub fn intelligence_service(mut self, service: impl Into<String>) -> Self {
        self.attributes.intelligence_service = Some(service.into());
        self
    }

    pub fn outbound_track_label(mut self, label: impl Into<String>) -> Self {
        self.attributes.outbound_track_label = Some(label.into());
        self
    }

    pub fn partial_results(mut self, enable: bool) -> Self {
        self.attributes.partial_results = Some(enable);
        self
    }

    pub fn profanity_filter(mut self, enable: bool) -> Self {
        self.attributes.profanity_filter = Some(enable);
        self
    }

    pub fn speech_model(mut self, model: impl Into<String>) -> Self {
        self.attributes.speech_model = Some(model.into());
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.attributes.status_callback_method = Some(method.into());
        self
    }

    pub fn status_callback_url(mut self, url: impl Into<String>) -> Self {
        self.attributes.status_callback_url = Some(url.into());
        self
    }

    pub fn transcription_engine(mut self, engine: impl Into<String>) -> Self {
        self.attributes.transcription_engine = Some(engine.into());
        self
    }
}

// Recording - Recording configuration
#[derive(Debug, Clone, Default)]
pub struct Recording {
    pub recording_status_callback: Option<String>,
    pub recording_status_callback_method: Option<String>,
    pub recording_status_callback_event: Option<String>,
    pub trim: Option<String>,
    pub track: Option<String>,
    pub channels: Option<String>,
}

impl Recording {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recording_status_callback(mut self, callback: impl Into<String>) -> Self {
        self.recording_status_callback = Some(callback.into());
        self
    }

    pub fn recording_status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.recording_status_callback_method = Some(method.into());
        self
    }

    pub fn recording_status_callback_event(mut self, event: impl Into<String>) -> Self {
        self.recording_status_callback_event = Some(event.into());
        self
    }

    pub fn trim(mut self, trim: impl Into<String>) -> Self {
        self.trim = Some(trim.into());
        self
    }

    pub fn track(mut self, track: impl Into<String>) -> Self {
        self.track = Some(track.into());
        self
    }

    pub fn channels(mut self, channels: impl Into<String>) -> Self {
        self.channels = Some(channels.into());
        self
    }
}

// Siprec - SIPREC recording
#[derive(Debug, Clone, Default)]
pub struct Siprec {
    pub name: Option<String>,
    pub connector_name: Option<String>,
    pub track: Option<String>,
}

impl Siprec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn connector_name(mut self, connector_name: impl Into<String>) -> Self {
        self.connector_name = Some(connector_name.into());
        self
    }

    pub fn track(mut self, track: impl Into<String>) -> Self {
        self.track = Some(track.into());
        self
    }
}

// Task - Task for Autopilot
#[derive(Debug, Clone, Default)]
pub struct Task {
    pub task_sid: Option<String>,
}

impl Task {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn task_sid(mut self, task_sid: impl Into<String>) -> Self {
        self.task_sid = Some(task_sid.into());
        self
    }
}

// Identity - Identity for Client
#[derive(Debug, Clone)]
pub struct Identity {
    pub identity: String,
}

impl Identity {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
        }
    }
}

// Language - Language for transcription and ConversationRelay
#[derive(Debug, Clone, Default)]
pub struct Language {
    pub language_code: Option<String>,
    pub tts_provider: Option<String>,
    pub stt_provider: Option<String>,
}

impl Language {
    pub fn new(language_code: impl Into<String>) -> Self {
        Self {
            language_code: Some(language_code.into()),
            tts_provider: None,
            stt_provider: None,
        }
    }

    pub fn tts_provider(mut self, provider: impl Into<String>) -> Self {
        self.tts_provider = Some(provider.into());
        self
    }

    pub fn stt_provider(mut self, provider: impl Into<String>) -> Self {
        self.stt_provider = Some(provider.into());
        self
    }
}

// Application - Application for Dial
#[derive(Debug, Clone, Default)]
pub struct Application {
    pub application_sid: Option<String>,
    pub customer_id: Option<String>,
    pub copy_parent_to: Option<String>,
    pub parameters: Vec<Parameter>,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn application_sid(mut self, application_sid: impl Into<String>) -> Self {
        self.application_sid = Some(application_sid.into());
        self
    }

    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn copy_parent_to(mut self, copy_parent_to: impl Into<String>) -> Self {
        self.copy_parent_to = Some(copy_parent_to.into());
        self
    }

    pub fn add_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }
}

// ApplicationSid - Application SID
#[derive(Debug, Clone)]
pub struct ApplicationSid {
    pub sid: String,
}

impl ApplicationSid {
    pub fn new(sid: impl Into<String>) -> Self {
        Self { sid: sid.into() }
    }
}

// Config - Configuration for various services
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub name: Option<String>,
    pub value: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

/// SSML elements for enhanced speech synthesis
#[derive(Debug, Clone)]
pub enum SsmlElement {
    Break {
        strength: Option<String>,
        time: Option<String>,
    },
    Emphasis {
        level: Option<String>,
        text: String,
    },
    Lang {
        xml_lang: String,
        text: String,
    },
    P {
        text: String,
    },
    Phoneme {
        alphabet: Option<String>,
        ph: String,
        text: String,
    },
    Prosody {
        pitch: Option<String>,
        rate: Option<String>,
        volume: Option<String>,
        text: String,
    },
    S {
        text: String,
    },
    SayAs {
        interpret_as: String,
        format: Option<String>,
        text: String,
    },
    Sub {
        alias: String,
        text: String,
    },
    W {
        role: Option<String>,
        text: String,
    },
    AmazonEffect {
        name: String,
        text: String,
    },
    AmazonDomain {
        name: String,
        text: String,
    },
}

/// Nouns that can be nested within Dial
#[derive(Debug, Clone)]
pub enum DialNoun {
    Number(DialNumber),
    Client(DialClient),
    Conference(DialConference),
    Queue(DialQueue),
    Sip(DialSip),
    Sim(DialSim),
    Application(DialApplication),
    WhatsApp(DialWhatsApp),
}

/// Nouns that can be nested within Gather
#[derive(Debug, Clone)]
pub enum GatherNoun {
    Say(Say),
    Play(Play),
    Pause(Pause),
}

/// Nouns that can be nested within Connect
#[derive(Debug, Clone)]
pub enum ConnectNoun {
    Stream(Stream),
    Room(Room),
    Conversation(Conversation),
    VirtualAgent(VirtualAgent),
    Autopilot(Autopilot),
    AiSession(AiSession),
    ConversationRelaySession(ConversationRelaySession),
    Assistant(Assistant),
    ConversationRelay(ConversationRelay),
}

/// Nouns that can be nested within Start
#[derive(Debug, Clone)]
pub enum StartNoun {
    Stream(Stream),
    Siprec(Siprec),
    Transcription(Transcription),
    Recording(Recording),
}

// ============================================================================
// Dial Nested Elements
// ============================================================================

#[derive(Debug, Clone)]
pub struct DialNumber {
    pub number: String,
    pub send_digits: Option<String>,
    pub url: Option<String>,
    pub method: Option<String>,
    pub status_callback: Option<String>,
    pub status_callback_event: Option<Vec<String>>,
    pub status_callback_method: Option<String>,
    pub call_reason: Option<String>,
    pub byoc: Option<String>,
    pub machine_detection: Option<String>,
    pub machine_detection_timeout: Option<u32>,
    pub machine_detection_speech_threshold: Option<u32>,
    pub machine_detection_speech_end_threshold: Option<u32>,
    pub machine_detection_silence_timeout: Option<u32>,
    pub amd_status_callback: Option<String>,
    pub amd_status_callback_method: Option<String>,
}

impl DialNumber {
    pub fn new(number: impl Into<String>) -> Self {
        Self {
            number: number.into(),
            send_digits: None,
            url: None,
            method: None,
            status_callback: None,
            status_callback_event: None,
            status_callback_method: None,
            call_reason: None,
            byoc: None,
            machine_detection: None,
            machine_detection_timeout: None,
            machine_detection_speech_threshold: None,
            machine_detection_speech_end_threshold: None,
            machine_detection_silence_timeout: None,
            amd_status_callback: None,
            amd_status_callback_method: None,
        }
    }

    pub fn send_digits(mut self, digits: impl Into<String>) -> Self {
        self.send_digits = Some(digits.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Self {
        self.status_callback = Some(callback.into());
        self
    }

    pub fn status_callback_event(mut self, events: Vec<String>) -> Self {
        self.status_callback_event = Some(events);
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn call_reason(mut self, call_reason: impl Into<String>) -> Self {
        self.call_reason = Some(call_reason.into());
        self
    }

    pub fn byoc(mut self, byoc: impl Into<String>) -> Self {
        self.byoc = Some(byoc.into());
        self
    }

    pub fn machine_detection(mut self, machine_detection: impl Into<String>) -> Self {
        self.machine_detection = Some(machine_detection.into());
        self
    }

    pub fn machine_detection_timeout(mut self, timeout: u32) -> Self {
        self.machine_detection_timeout = Some(timeout);
        self
    }

    pub fn machine_detection_speech_threshold(mut self, threshold: u32) -> Self {
        self.machine_detection_speech_threshold = Some(threshold);
        self
    }

    pub fn machine_detection_speech_end_threshold(mut self, threshold: u32) -> Self {
        self.machine_detection_speech_end_threshold = Some(threshold);
        self
    }

    pub fn machine_detection_silence_timeout(mut self, timeout: u32) -> Self {
        self.machine_detection_silence_timeout = Some(timeout);
        self
    }

    pub fn amd_status_callback(mut self, callback: impl Into<String>) -> Self {
        self.amd_status_callback = Some(callback.into());
        self
    }

    pub fn amd_status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.amd_status_callback_method = Some(method.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialClient {
    pub identity: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub status_callback_event: Option<Vec<String>>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub client_notification_url: Option<String>,
}

impl DialClient {
    pub fn new(identity: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
            url: None,
            method: None,
            status_callback_event: None,
            status_callback: None,
            status_callback_method: None,
            client_notification_url: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn status_callback_event(mut self, events: Vec<String>) -> Self {
        self.status_callback_event = Some(events);
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Self {
        self.status_callback = Some(callback.into());
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn client_notification_url(mut self, url: impl Into<String>) -> Self {
        self.client_notification_url = Some(url.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialConference {
    pub name: String,
    pub muted: Option<bool>,
    pub beep: Option<String>,
    pub start_conference_on_enter: Option<bool>,
    pub end_conference_on_exit: Option<bool>,
    pub wait_url: Option<String>,
    pub wait_method: Option<String>,
    pub max_participants: Option<u32>,
    pub record: Option<String>,
    pub region: Option<String>,
    pub coach: Option<String>,
    pub trim: Option<String>,
    pub status_callback_event: Option<Vec<String>>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub recording_status_callback: Option<String>,
    pub recording_status_callback_method: Option<String>,
    pub recording_status_callback_event: Option<Vec<String>>,
    pub event_callback_url: Option<String>,
    pub jitter_buffer_size: Option<String>,
    pub participant_label: Option<String>,
    pub call_sid_to_coach: Option<String>,
    pub beep_on_customer_entrance: Option<bool>,
    pub coaching: Option<bool>,
}

impl DialConference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            muted: None,
            beep: None,
            start_conference_on_enter: None,
            end_conference_on_exit: None,
            wait_url: None,
            wait_method: None,
            max_participants: None,
            record: None,
            region: None,
            coach: None,
            trim: None,
            status_callback_event: None,
            status_callback: None,
            status_callback_method: None,
            recording_status_callback: None,
            recording_status_callback_method: None,
            recording_status_callback_event: None,
            event_callback_url: None,
            jitter_buffer_size: None,
            participant_label: None,
            call_sid_to_coach: None,
            beep_on_customer_entrance: None,
            coaching: None,
        }
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = Some(muted);
        self
    }

    pub fn beep(mut self, beep: impl Into<String>) -> Self {
        self.beep = Some(beep.into());
        self
    }

    pub fn start_conference_on_enter(mut self, start: bool) -> Self {
        self.start_conference_on_enter = Some(start);
        self
    }

    pub fn end_conference_on_exit(mut self, end: bool) -> Self {
        self.end_conference_on_exit = Some(end);
        self
    }

    pub fn max_participants(mut self, max: u32) -> Self {
        self.max_participants = Some(max);
        self
    }

    pub fn record(mut self, record: impl Into<String>) -> Self {
        self.record = Some(record.into());
        self
    }

    pub fn wait_url(mut self, url: impl Into<String>) -> Self {
        self.wait_url = Some(url.into());
        self
    }

    pub fn wait_method(mut self, method: impl Into<String>) -> Self {
        self.wait_method = Some(method.into());
        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn coach(mut self, coach: impl Into<String>) -> Self {
        self.coach = Some(coach.into());
        self
    }

    pub fn trim(mut self, trim: impl Into<String>) -> Self {
        self.trim = Some(trim.into());
        self
    }

    pub fn status_callback_event(mut self, events: Vec<String>) -> Self {
        self.status_callback_event = Some(events);
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Self {
        self.status_callback = Some(callback.into());
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn recording_status_callback(mut self, callback: impl Into<String>) -> Self {
        self.recording_status_callback = Some(callback.into());
        self
    }

    pub fn recording_status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.recording_status_callback_method = Some(method.into());
        self
    }

    pub fn recording_status_callback_event(mut self, events: Vec<String>) -> Self {
        self.recording_status_callback_event = Some(events);
        self
    }

    pub fn event_callback_url(mut self, url: impl Into<String>) -> Self {
        self.event_callback_url = Some(url.into());
        self
    }

    pub fn jitter_buffer_size(mut self, size: impl Into<String>) -> Self {
        self.jitter_buffer_size = Some(size.into());
        self
    }

    pub fn participant_label(mut self, label: impl Into<String>) -> Self {
        self.participant_label = Some(label.into());
        self
    }

    pub fn call_sid_to_coach(mut self, call_sid: impl Into<String>) -> Self {
        self.call_sid_to_coach = Some(call_sid.into());
        self
    }

    pub fn beep_on_customer_entrance(mut self, beep: bool) -> Self {
        self.beep_on_customer_entrance = Some(beep);
        self
    }

    pub fn coaching(mut self, coaching: bool) -> Self {
        self.coaching = Some(coaching);
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialQueue {
    pub name: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub reservation_sid: Option<String>,
    pub post_work_activity_sid: Option<String>,
}

impl DialQueue {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            method: None,
            reservation_sid: None,
            post_work_activity_sid: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialSip {
    pub sip_url: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub status_callback_event: Option<Vec<String>>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub codecs: Option<Vec<String>>,
    pub custom_headers: Option<Vec<(String, String)>>,
}

impl DialSip {
    pub fn new(sip_url: impl Into<String>) -> Self {
        Self {
            sip_url: sip_url.into(),
            url: None,
            method: None,
            username: None,
            password: None,
            status_callback_event: None,
            status_callback: None,
            status_callback_method: None,
            codecs: None,
            custom_headers: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn status_callback_event(mut self, events: Vec<String>) -> Self {
        self.status_callback_event = Some(events);
        self
    }

    pub fn status_callback(mut self, callback: impl Into<String>) -> Self {
        self.status_callback = Some(callback.into());
        self
    }

    pub fn status_callback_method(mut self, method: impl Into<String>) -> Self {
        self.status_callback_method = Some(method.into());
        self
    }

    pub fn codecs(mut self, codecs: Vec<String>) -> Self {
        self.codecs = Some(codecs);
        self
    }

    pub fn add_codec(mut self, codec: impl Into<String>) -> Self {
        if let Some(ref mut codecs) = self.codecs {
            codecs.push(codec.into());
        } else {
            self.codecs = Some(vec![codec.into()]);
        }
        self
    }

    pub fn custom_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.custom_headers = Some(headers);
        self
    }

    pub fn add_custom_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        if let Some(ref mut headers) = self.custom_headers {
            headers.push((name.into(), value.into()));
        } else {
            self.custom_headers = Some(vec![(name.into(), value.into())]);
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialSim {
    pub sim_sid: String,
}

impl DialSim {
    pub fn new(sim_sid: impl Into<String>) -> Self {
        Self {
            sim_sid: sim_sid.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DialApplication {
    pub application_sid: Option<String>,
    pub customer_id: Option<String>,
    pub copy_parent_to: Option<String>,
    pub parameters: Vec<Parameter>,
}

impl DialApplication {
    pub fn new(application_sid: impl Into<String>) -> Self {
        Self {
            application_sid: Some(application_sid.into()),
            customer_id: None,
            copy_parent_to: None,
            parameters: Vec::new(),
        }
    }

    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub fn copy_parent_to(mut self, copy_parent_to: impl Into<String>) -> Self {
        self.copy_parent_to = Some(copy_parent_to.into());
        self
    }

    pub fn add_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }
}

#[derive(Debug, Clone)]
pub struct DialWhatsApp {
    pub phone_number: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub status_callback_event: Option<Vec<String>>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
}

impl DialWhatsApp {
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            url: None,
            method: None,
            status_callback_event: None,
            status_callback: None,
            status_callback_method: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}

impl VoiceResponse {
    /// <Response> TwiML for Voice
    pub fn new() -> Self {
        Self::default()
    }

    /// Comments in <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comments.push(comment.into());
        self
    }

    /// Comments after <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    pub fn comment_after(mut self, comment: impl Into<String>) -> Self {
        self.comments_after.push(comment.into());
        self
    }

    /// Comments before <Response>
    ///
    /// # Arguments
    /// * `comment` - XML Comment
    pub fn comment_before(mut self, comment: impl Into<String>) -> Self {
        self.comments_before.push(comment.into());
        self
    }

    /// <Connect> TwiML Verb
    ///
    /// # Arguments
    /// * `connect` - Pre-configured Connect object
    pub fn connect(mut self, connect: Connect) -> Self {
        self.verbs.push(VoiceVerb::Connect(connect));
        self
    }

    /// <Dial> TwiML Verb
    ///
    /// # Arguments
    /// * `number` - Phone number to dial
    pub fn dial(mut self, number: impl Into<String>) -> Self {
        let dial = Dial::new().number(number);
        self.verbs.push(VoiceVerb::Dial(dial));
        self
    }

    /// <Dial> TwiML Verb with attributes
    ///
    /// # Arguments
    /// * `attributes` - TwiML attributes
    /// * `number` - Phone number to dial (optional)
    pub fn dial_with_attributes(
        mut self,
        attributes: DialAttributes,
        number: Option<impl Into<String>>,
    ) -> Dial {
        let mut dial = Dial::with_attributes(attributes);
        if let Some(n) = number {
            dial = dial.number(n);
        }
        self.verbs.push(VoiceVerb::Dial(dial.clone()));
        dial
    }

    /// <Echo> TwiML Verb
    pub fn echo(mut self) -> Self {
        let echo = Echo::new();
        self.verbs.push(VoiceVerb::Echo(echo));
        self
    }

    /// <Enqueue> TwiML Noun
    ///
    /// # Arguments
    /// * `enqueue` - Pre-configured Enqueue object
    pub fn enqueue(mut self, enqueue: Enqueue) -> Self {
        self.verbs.push(VoiceVerb::Enqueue(enqueue));
        self
    }

    /// <Gather> TwiML Verb
    ///
    /// # Arguments
    /// * `gather` - Pre-configured Gather object
    pub fn gather(mut self, gather: Gather) -> Self {
        self.verbs.push(VoiceVerb::Gather(gather));
        self
    }

    /// <Hangup> TwiML Verb
    pub fn hangup(mut self) -> Self {
        let hangup = Hangup::new();
        self.verbs.push(VoiceVerb::Hangup(hangup));
        self
    }

    /// <Leave> TwiML Verb
    pub fn leave(mut self) -> Self {
        let leave = Leave::new();
        self.verbs.push(VoiceVerb::Leave(leave));
        self
    }

    /// <Pause> TwiML Verb
    ///
    /// # Arguments
    /// * `length` - Pause length in seconds (optional)
    pub fn pause(mut self, length: Option<u32>) -> Self {
        let pause = Pause::new(length.map(|l| PauseAttributes { length: Some(l) }));
        self.verbs.push(VoiceVerb::Pause(pause));
        self
    }

    /// <Pay> TwiML Verb
    ///
    /// # Arguments
    /// * `pay` - Pre-configured Pay object
    pub fn pay(mut self, pay: Pay) -> Self {
        self.verbs.push(VoiceVerb::Pay(pay));
        self
    }

    /// <Play> TwiML Verb
    ///
    /// # Arguments
    /// * `url` - Media URL
    pub fn play(mut self, url: impl Into<String>) -> Self {
        let play = Play::new().url(url);
        self.verbs.push(VoiceVerb::Play(play));
        self
    }

    /// <Record> TwiML Verb
    ///
    /// # Arguments
    /// * `record` - Pre-configured Record object
    pub fn record(mut self, record: Record) -> Self {
        self.verbs.push(VoiceVerb::Record(record));
        self
    }

    /// <Redirect> TwiML Verb
    ///
    /// # Arguments
    /// * `url` - Redirect URL
    pub fn redirect(mut self, url: impl Into<String>) -> Self {
        let redirect = Redirect::new(RedirectAttributes::default(), url.into());
        self.verbs.push(VoiceVerb::Redirect(redirect));
        self
    }

    /// <Refer> TwiML Verb
    ///
    /// # Arguments
    /// * `refer` - Pre-configured Refer object
    pub fn refer(mut self, refer: Refer) -> Self {
        self.verbs.push(VoiceVerb::Refer(refer));
        self
    }

    /// <Reject> TwiML Verb
    ///
    /// # Arguments
    /// * `reject` - Pre-configured Reject object
    pub fn reject(mut self, reject: Reject) -> Self {
        self.verbs.push(VoiceVerb::Reject(reject));
        self
    }

    /// <Say> TwiML Verb
    ///
    /// # Arguments
    /// * `message` - Message to say
    pub fn say(mut self, message: impl Into<String>) -> Self {
        let say = Say::new(message);
        self.verbs.push(VoiceVerb::Say(say));
        self
    }

    /// <Start> TwiML Verb
    ///
    /// # Arguments
    /// * `start` - Pre-configured Start object
    pub fn start(mut self, start: Start) -> Self {
        self.verbs.push(VoiceVerb::Start(start));
        self
    }

    /// <Stop> TwiML Verb
    pub fn stop(mut self) -> Self {
        let stop = Stop::new();
        self.verbs.push(VoiceVerb::Stop(stop));
        self
    }

    /// <Sms> TwiML Noun
    ///
    /// # Arguments
    /// * `message` - SMS message body
    pub fn sms(mut self, message: impl Into<String>) -> Self {
        let sms = Sms::new(SmsAttributes::default(), message.into());
        self.verbs.push(VoiceVerb::Sms(sms));
        self
    }

    /// <Queue> TwiML Noun
    ///
    /// # Arguments
    /// * `name` - Queue name
    pub fn queue(mut self, name: impl Into<String>) -> Self {
        let queue = Queue::new(QueueAttributes::default(), name.into());
        self.verbs.push(VoiceVerb::Queue(queue));
        self
    }

    /// <Prompt> TwiML Verb
    ///
    /// # Arguments
    /// * `prompt` - Pre-configured Prompt object
    pub fn prompt(mut self, prompt: Prompt) -> Self {
        self.verbs.push(VoiceVerb::Prompt(prompt));
        self
    }

    // ========================================================================
    // Convenience methods for simpler API
    // ========================================================================

    /// Add a Say verb with a pre-configured Say object
    pub fn say_with(mut self, say: Say) -> Self {
        self.verbs.push(VoiceVerb::Say(say));
        self
    }

    /// Add a Play verb with a pre-configured Play object
    pub fn play_with(mut self, play: Play) -> Self {
        self.verbs.push(VoiceVerb::Play(play));
        self
    }

    /// Add a Dial verb with a pre-configured Dial object
    pub fn dial_with(mut self, dial: Dial) -> Self {
        self.verbs.push(VoiceVerb::Dial(dial));
        self
    }

    /// Add a Gather verb with a pre-configured Gather object
    pub fn gather_with(mut self, gather: Gather) -> Self {
        self.verbs.push(VoiceVerb::Gather(gather));
        self
    }

    /// Add a Record verb with a pre-configured Record object
    pub fn record_with(mut self, record: Record) -> Self {
        self.verbs.push(VoiceVerb::Record(record));
        self
    }

    /// Add a Connect verb with a pre-configured Connect object
    pub fn connect_with(mut self, connect: Connect) -> Self {
        self.verbs.push(VoiceVerb::Connect(connect));
        self
    }

    /// Add an Enqueue verb with a pre-configured Enqueue object
    pub fn enqueue_with(mut self, enqueue: Enqueue) -> Self {
        self.verbs.push(VoiceVerb::Enqueue(enqueue));
        self
    }

    /// Add a Pay verb with a pre-configured Pay object
    pub fn pay_with(mut self, pay: Pay) -> Self {
        self.verbs.push(VoiceVerb::Pay(pay));
        self
    }

    /// Add a Refer verb with a pre-configured Refer object
    pub fn refer_with(mut self, refer: Refer) -> Self {
        self.verbs.push(VoiceVerb::Refer(refer));
        self
    }

    /// Add a Reject verb with a pre-configured Reject object
    pub fn reject_with(mut self, reject: Reject) -> Self {
        self.verbs.push(VoiceVerb::Reject(reject));
        self
    }

    /// Add a Start verb with a pre-configured Start object
    pub fn start_with(mut self, start: Start) -> Self {
        self.verbs.push(VoiceVerb::Start(start));
        self
    }

    /// Simple pause with just a length parameter
    pub fn pause_simple(mut self, length: u32) -> Self {
        let pause = Pause::new(Some(PauseAttributes {
            length: Some(length),
        }));
        self.verbs.push(VoiceVerb::Pause(pause));
        self
    }

    /// Add an Sms verb with a pre-configured Sms object
    pub fn sms_with(mut self, sms: Sms) -> Self {
        self.verbs.push(VoiceVerb::Sms(sms));
        self
    }

    /// Add a Queue verb with a pre-configured Queue object
    pub fn queue_with(mut self, queue: Queue) -> Self {
        self.verbs.push(VoiceVerb::Queue(queue));
        self
    }

    /// Add a Prompt verb with a pre-configured Prompt object
    pub fn prompt_with(mut self, prompt: Prompt) -> Self {
        self.verbs.push(VoiceVerb::Prompt(prompt));
        self
    }
}

impl TwiML for VoiceResponse {
    fn to_xml(&self) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

        // Add comments before Response
        for comment in &self.comments_before {
            xml.push_str(&format!("<!-- {} -->\n", escape_xml_text(comment)));
        }

        xml.push_str("<Response>\n");

        // Add comments inside Response
        for comment in &self.comments {
            xml.push_str(&format!("  <!-- {} -->\n", escape_xml_text(comment)));
        }

        for verb in &self.verbs {
            match verb {
                VoiceVerb::Say(say) => {
                    xml.push_str("  <Say");
                    if let Some(v) = &say.attributes.voice {
                        xml.push_str(&format!(" voice=\"{}\"", escape_xml_attr(v)));
                    }
                    if let Some(l) = &say.attributes.language {
                        xml.push_str(&format!(" language=\"{}\"", escape_xml_attr(l)));
                    }
                    if let Some(lc) = say.attributes.loop_count {
                        xml.push_str(&format!(" loop=\"{}\"", lc));
                    }
                    xml.push_str(">");

                    // Add message
                    xml.push_str(&escape_xml_text(&say.message));

                    // Add SSML elements
                    for ssml in &say.ssml_elements {
                        match ssml {
                            SsmlElement::Break { strength, time } => {
                                xml.push_str("<break");
                                if let Some(s) = strength {
                                    xml.push_str(&format!(" strength=\"{}\"", escape_xml_attr(s)));
                                }
                                if let Some(t) = time {
                                    xml.push_str(&format!(" time=\"{}\"", escape_xml_attr(t)));
                                }
                                xml.push_str(" />");
                            }
                            SsmlElement::Emphasis { level, text } => {
                                xml.push_str("<emphasis");
                                if let Some(l) = level {
                                    xml.push_str(&format!(" level=\"{}\"", escape_xml_attr(l)));
                                }
                                xml.push_str(&format!(">{}</emphasis>", escape_xml_text(text)));
                            }
                            SsmlElement::Prosody {
                                pitch,
                                rate,
                                volume,
                                text,
                            } => {
                                xml.push_str("<prosody");
                                if let Some(p) = pitch {
                                    xml.push_str(&format!(" pitch=\"{}\"", escape_xml_attr(p)));
                                }
                                if let Some(r) = rate {
                                    xml.push_str(&format!(" rate=\"{}\"", escape_xml_attr(r)));
                                }
                                if let Some(v) = volume {
                                    xml.push_str(&format!(" volume=\"{}\"", escape_xml_attr(v)));
                                }
                                xml.push_str(&format!(">{}</prosody>", escape_xml_text(text)));
                            }
                            SsmlElement::SayAs {
                                interpret_as,
                                format,
                                text,
                            } => {
                                xml.push_str(&format!(
                                    "<say-as interpret-as=\"{}\"",
                                    escape_xml_attr(interpret_as)
                                ));
                                if let Some(f) = format {
                                    xml.push_str(&format!(" format=\"{}\"", escape_xml_attr(f)));
                                }
                                xml.push_str(&format!(">{}</say-as>", escape_xml_text(text)));
                            }
                            SsmlElement::Sub { alias, text } => {
                                xml.push_str(&format!(
                                    "<sub alias=\"{}\">{}</sub>",
                                    escape_xml_attr(alias),
                                    escape_xml_text(text)
                                ));
                            }
                            SsmlElement::P { text } => {
                                xml.push_str(&format!("<p>{}</p>", escape_xml_text(text)));
                            }
                            SsmlElement::S { text } => {
                                xml.push_str(&format!("<s>{}</s>", escape_xml_text(text)));
                            }
                            SsmlElement::Lang { xml_lang, text } => {
                                xml.push_str(&format!(
                                    "<lang xml:lang=\"{}\">{}</lang>",
                                    escape_xml_attr(xml_lang),
                                    escape_xml_text(text)
                                ));
                            }
                            SsmlElement::Phoneme { alphabet, ph, text } => {
                                xml.push_str("<phoneme");
                                if let Some(a) = alphabet {
                                    xml.push_str(&format!(" alphabet=\"{}\"", escape_xml_attr(a)));
                                }
                                xml.push_str(&format!(
                                    " ph=\"{}\">{}</phoneme>",
                                    escape_xml_attr(ph),
                                    escape_xml_text(text)
                                ));
                            }
                            SsmlElement::W { role, text } => {
                                xml.push_str("<w");
                                if let Some(r) = role {
                                    xml.push_str(&format!(" role=\"{}\"", escape_xml_attr(r)));
                                }
                                xml.push_str(&format!(">{}</w>", escape_xml_text(text)));
                            }
                            SsmlElement::AmazonEffect { name, text } => {
                                xml.push_str(&format!(
                                    "<amazon:effect name=\"{}\">{}</amazon:effect>",
                                    escape_xml_attr(name),
                                    escape_xml_text(text)
                                ));
                            }
                            SsmlElement::AmazonDomain { name, text } => {
                                xml.push_str(&format!(
                                    "<amazon:domain name=\"{}\">{}</amazon:domain>",
                                    escape_xml_attr(name),
                                    escape_xml_text(text)
                                ));
                            }
                        }
                    }

                    xml.push_str("</Say>\n");
                }
                VoiceVerb::Play(play) => {
                    xml.push_str("  <Play");
                    if let Some(d) = &play.attributes.digits {
                        xml.push_str(&format!(" digits=\"{}\"", escape_xml_attr(d)));
                    }
                    if let Some(lc) = play.attributes.loop_count {
                        xml.push_str(&format!(" loop=\"{}\"", lc));
                    }
                    xml.push_str(">");
                    if let Some(url) = &play.url {
                        xml.push_str(&escape_xml_text(url));
                    }
                    xml.push_str("</Play>\n");
                }
                VoiceVerb::Pause(pause) => {
                    xml.push_str("  <Pause");
                    if let Some(len) = pause.attributes.length {
                        xml.push_str(&format!(" length=\"{}\"", len));
                    }
                    xml.push_str(" />\n");
                }
                VoiceVerb::Dial(dial) => {
                    xml.push_str("  <Dial");
                    if let Some(a) = &dial.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &dial.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(t) = dial.attributes.timeout {
                        xml.push_str(&format!(" timeout=\"{}\"", t));
                    }
                    if let Some(h) = dial.attributes.hangup_on_star {
                        xml.push_str(&format!(" hangupOnStar=\"{}\"", h));
                    }
                    if let Some(tl) = dial.attributes.time_limit {
                        xml.push_str(&format!(" timeLimit=\"{}\"", tl));
                    }
                    if let Some(c) = &dial.attributes.caller_id {
                        xml.push_str(&format!(" callerId=\"{}\"", escape_xml_attr(c)));
                    }
                    if let Some(cr) = &dial.attributes.call_reason {
                        xml.push_str(&format!(" callReason=\"{}\"", escape_xml_attr(cr)));
                    }
                    if let Some(r) = &dial.attributes.record {
                        xml.push_str(&format!(" record=\"{}\"", escape_xml_attr(r)));
                    }
                    if let Some(tr) = &dial.attributes.trim {
                        xml.push_str(&format!(" trim=\"{}\"", escape_xml_attr(tr)));
                    }
                    if let Some(rsc) = &dial.attributes.recording_status_callback {
                        xml.push_str(&format!(
                            " recordingStatusCallback=\"{}\"",
                            escape_xml_attr(rsc)
                        ));
                    }
                    if let Some(aob) = dial.attributes.answer_on_bridge {
                        xml.push_str(&format!(" answerOnBridge=\"{}\"", aob));
                    }
                    if let Some(rt) = &dial.attributes.ring_tone {
                        xml.push_str(&format!(" ringTone=\"{}\"", escape_xml_attr(rt)));
                    }
                    if let Some(e) = &dial.attributes.events {
                        xml.push_str(&format!(" events=\"{}\"", escape_xml_attr(e)));
                    }
                    if let Some(rm) = &dial.attributes.refer_method {
                        xml.push_str(&format!(" referMethod=\"{}\"", escape_xml_attr(rm)));
                    }
                    if let Some(ru) = &dial.attributes.refer_url {
                        xml.push_str(&format!(" referUrl=\"{}\"", escape_xml_attr(ru)));
                    }
                    if let Some(seq) = dial.attributes.sequential {
                        xml.push_str(&format!(" sequential=\"{}\"", seq));
                    }
                    if let Some(rt) = &dial.attributes.recording_track {
                        xml.push_str(&format!(" recordingTrack=\"{}\"", escape_xml_attr(rt)));
                    }
                    if let Some(rsce) = &dial.attributes.recording_status_callback_event {
                        let events = rsce.join(" ");
                        xml.push_str(&format!(
                            " recordingStatusCallbackEvent=\"{}\"",
                            escape_xml_attr(&events)
                        ));
                    }
                    if let Some(rscm) = &dial.attributes.recording_status_callback_method {
                        xml.push_str(&format!(
                            " recordingStatusCallbackMethod=\"{}\"",
                            escape_xml_attr(rscm)
                        ));
                    }
                    xml.push_str(">");

                    // Add plain number if specified
                    if let Some(num) = &dial.number {
                        xml.push_str(num);
                    }

                    // Add nested nouns
                    for noun in &dial.nested {
                        match noun {
                            DialNoun::Number(n) => {
                                xml.push_str("\n    <Number");
                                if let Some(sd) = &n.send_digits {
                                    xml.push_str(&format!(
                                        " sendDigits=\"{}\"",
                                        escape_xml_attr(sd)
                                    ));
                                }
                                if let Some(u) = &n.url {
                                    xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                }
                                if let Some(m) = &n.method {
                                    xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                                }
                                if let Some(sc) = &n.status_callback {
                                    xml.push_str(&format!(
                                        " statusCallback=\"{}\"",
                                        escape_xml_attr(sc)
                                    ));
                                }
                                if let Some(sce) = &n.status_callback_event {
                                    let events = sce.join(" ");
                                    xml.push_str(&format!(
                                        " statusCallbackEvent=\"{}\"",
                                        escape_xml_attr(&events)
                                    ));
                                }
                                if let Some(scm) = &n.status_callback_method {
                                    xml.push_str(&format!(
                                        " statusCallbackMethod=\"{}\"",
                                        escape_xml_attr(scm)
                                    ));
                                }
                                if let Some(cr) = &n.call_reason {
                                    xml.push_str(&format!(
                                        " callReason=\"{}\"",
                                        escape_xml_attr(cr)
                                    ));
                                }
                                if let Some(byoc) = &n.byoc {
                                    xml.push_str(&format!(" byoc=\"{}\"", escape_xml_attr(byoc)));
                                }
                                if let Some(md) = &n.machine_detection {
                                    xml.push_str(&format!(
                                        " machineDetection=\"{}\"",
                                        escape_xml_attr(md)
                                    ));
                                }
                                if let Some(mdt) = n.machine_detection_timeout {
                                    xml.push_str(&format!(" machineDetectionTimeout=\"{}\"", mdt));
                                }
                                if let Some(mdst) = n.machine_detection_speech_threshold {
                                    xml.push_str(&format!(
                                        " machineDetectionSpeechThreshold=\"{}\"",
                                        mdst
                                    ));
                                }
                                if let Some(mdset) = n.machine_detection_speech_end_threshold {
                                    xml.push_str(&format!(
                                        " machineDetectionSpeechEndThreshold=\"{}\"",
                                        mdset
                                    ));
                                }
                                if let Some(mdsto) = n.machine_detection_silence_timeout {
                                    xml.push_str(&format!(
                                        " machineDetectionSilenceTimeout=\"{}\"",
                                        mdsto
                                    ));
                                }
                                if let Some(asc) = &n.amd_status_callback {
                                    xml.push_str(&format!(
                                        " amdStatusCallback=\"{}\"",
                                        escape_xml_attr(asc)
                                    ));
                                }
                                if let Some(ascm) = &n.amd_status_callback_method {
                                    xml.push_str(&format!(
                                        " amdStatusCallbackMethod=\"{}\"",
                                        escape_xml_attr(ascm)
                                    ));
                                }
                                xml.push_str(&format!(">{}</Number>", escape_xml_text(&n.number)));
                            }
                            DialNoun::Client(c) => {
                                xml.push_str("\n    <Client");
                                if let Some(u) = &c.url {
                                    xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                }
                                if let Some(m) = &c.method {
                                    xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                                }
                                if let Some(sce) = &c.status_callback_event {
                                    xml.push_str(&format!(
                                        " statusCallbackEvent=\"{}\"",
                                        escape_xml_attr(&sce.join(" "))
                                    ));
                                }
                                if let Some(sc) = &c.status_callback {
                                    xml.push_str(&format!(
                                        " statusCallback=\"{}\"",
                                        escape_xml_attr(sc)
                                    ));
                                }
                                if let Some(scm) = &c.status_callback_method {
                                    xml.push_str(&format!(
                                        " statusCallbackMethod=\"{}\"",
                                        escape_xml_attr(scm)
                                    ));
                                }
                                if let Some(cnu) = &c.client_notification_url {
                                    xml.push_str(&format!(
                                        " clientNotificationUrl=\"{}\"",
                                        escape_xml_attr(cnu)
                                    ));
                                }
                                xml.push_str(&format!(
                                    ">{}</Client>",
                                    escape_xml_text(&c.identity)
                                ));
                            }
                            DialNoun::Conference(conf) => {
                                xml.push_str("\n    <Conference");
                                if let Some(m) = conf.muted {
                                    xml.push_str(&format!(" muted=\"{}\"", m));
                                }
                                if let Some(b) = &conf.beep {
                                    xml.push_str(&format!(" beep=\"{}\"", escape_xml_attr(b)));
                                }
                                if let Some(s) = conf.start_conference_on_enter {
                                    xml.push_str(&format!(" startConferenceOnEnter=\"{}\"", s));
                                }
                                if let Some(e) = conf.end_conference_on_exit {
                                    xml.push_str(&format!(" endConferenceOnExit=\"{}\"", e));
                                }
                                if let Some(w) = &conf.wait_url {
                                    xml.push_str(&format!(" waitUrl=\"{}\"", escape_xml_attr(w)));
                                }
                                if let Some(wm) = &conf.wait_method {
                                    xml.push_str(&format!(
                                        " waitMethod=\"{}\"",
                                        escape_xml_attr(wm)
                                    ));
                                }
                                if let Some(mp) = conf.max_participants {
                                    xml.push_str(&format!(" maxParticipants=\"{}\"", mp));
                                }
                                if let Some(r) = &conf.record {
                                    xml.push_str(&format!(" record=\"{}\"", escape_xml_attr(r)));
                                }
                                if let Some(reg) = &conf.region {
                                    xml.push_str(&format!(" region=\"{}\"", escape_xml_attr(reg)));
                                }
                                if let Some(c) = &conf.coach {
                                    xml.push_str(&format!(" coach=\"{}\"", escape_xml_attr(c)));
                                }
                                if let Some(t) = &conf.trim {
                                    xml.push_str(&format!(" trim=\"{}\"", escape_xml_attr(t)));
                                }
                                if let Some(sce) = &conf.status_callback_event {
                                    let events = sce.join(" ");
                                    xml.push_str(&format!(
                                        " statusCallbackEvent=\"{}\"",
                                        escape_xml_attr(&events)
                                    ));
                                }
                                if let Some(sc) = &conf.status_callback {
                                    xml.push_str(&format!(
                                        " statusCallback=\"{}\"",
                                        escape_xml_attr(sc)
                                    ));
                                }
                                if let Some(scm) = &conf.status_callback_method {
                                    xml.push_str(&format!(
                                        " statusCallbackMethod=\"{}\"",
                                        escape_xml_attr(scm)
                                    ));
                                }
                                if let Some(rsc) = &conf.recording_status_callback {
                                    xml.push_str(&format!(
                                        " recordingStatusCallback=\"{}\"",
                                        escape_xml_attr(rsc)
                                    ));
                                }
                                if let Some(rscm) = &conf.recording_status_callback_method {
                                    xml.push_str(&format!(
                                        " recordingStatusCallbackMethod=\"{}\"",
                                        escape_xml_attr(rscm)
                                    ));
                                }
                                if let Some(rsce) = &conf.recording_status_callback_event {
                                    let events = rsce.join(" ");
                                    xml.push_str(&format!(
                                        " recordingStatusCallbackEvent=\"{}\"",
                                        escape_xml_attr(&events)
                                    ));
                                }
                                if let Some(ecu) = &conf.event_callback_url {
                                    xml.push_str(&format!(
                                        " eventCallbackUrl=\"{}\"",
                                        escape_xml_attr(ecu)
                                    ));
                                }
                                if let Some(jbs) = &conf.jitter_buffer_size {
                                    xml.push_str(&format!(
                                        " jitterBufferSize=\"{}\"",
                                        escape_xml_attr(jbs)
                                    ));
                                }
                                if let Some(pl) = &conf.participant_label {
                                    xml.push_str(&format!(
                                        " participantLabel=\"{}\"",
                                        escape_xml_attr(pl)
                                    ));
                                }
                                if let Some(cstc) = &conf.call_sid_to_coach {
                                    xml.push_str(&format!(
                                        " callSidToCoach=\"{}\"",
                                        escape_xml_attr(cstc)
                                    ));
                                }
                                if let Some(boce) = conf.beep_on_customer_entrance {
                                    xml.push_str(&format!(" beepOnCustomerEntrance=\"{}\"", boce));
                                }
                                if let Some(coaching) = conf.coaching {
                                    xml.push_str(&format!(" coaching=\"{}\"", coaching));
                                }
                                xml.push_str(&format!(
                                    ">{}</Conference>",
                                    escape_xml_text(&conf.name)
                                ));
                            }
                            DialNoun::Queue(q) => {
                                xml.push_str("\n    <Queue");
                                if let Some(u) = &q.url {
                                    xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                }
                                if let Some(m) = &q.method {
                                    xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                                }
                                xml.push_str(&format!(">{}</Queue>", escape_xml_text(&q.name)));
                            }
                            DialNoun::Sip(s) => {
                                xml.push_str("\n    <Sip");
                                if let Some(u) = &s.url {
                                    xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                }
                                if let Some(m) = &s.method {
                                    xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                                }
                                if let Some(user) = &s.username {
                                    xml.push_str(&format!(
                                        " username=\"{}\"",
                                        escape_xml_attr(user)
                                    ));
                                }
                                if let Some(pass) = &s.password {
                                    xml.push_str(&format!(
                                        " password=\"{}\"",
                                        escape_xml_attr(pass)
                                    ));
                                }
                                if let Some(sce) = &s.status_callback_event {
                                    let events = sce.join(" ");
                                    xml.push_str(&format!(
                                        " statusCallbackEvent=\"{}\"",
                                        escape_xml_attr(&events)
                                    ));
                                }
                                if let Some(sc) = &s.status_callback {
                                    xml.push_str(&format!(
                                        " statusCallback=\"{}\"",
                                        escape_xml_attr(sc)
                                    ));
                                }
                                if let Some(scm) = &s.status_callback_method {
                                    xml.push_str(&format!(
                                        " statusCallbackMethod=\"{}\"",
                                        escape_xml_attr(scm)
                                    ));
                                }
                                if let Some(codecs) = &s.codecs {
                                    let codec_list = codecs.join(",");
                                    xml.push_str(&format!(
                                        " codecs=\"{}\"",
                                        escape_xml_attr(&codec_list)
                                    ));
                                }
                                xml.push_str(">");

                                // Add custom SIP headers as child elements
                                if let Some(headers) = &s.custom_headers {
                                    for (name, value) in headers {
                                        xml.push_str(&format!(
                                            "\n      <SipHeader name=\"{}\" value=\"{}\"/>",
                                            escape_xml_attr(name),
                                            escape_xml_attr(value)
                                        ));
                                    }
                                    xml.push_str("\n      ");
                                }

                                xml.push_str(&format!("{}</Sip>", escape_xml_text(&s.sip_url)));
                            }
                            DialNoun::Sim(sim) => {
                                xml.push_str(&format!(
                                    "\n    <Sim>{}</Sim>",
                                    escape_xml_text(&sim.sim_sid)
                                ));
                            }
                            DialNoun::Application(app) => {
                                let has_nested = !app.parameters.is_empty();

                                xml.push_str("\n    <Application");
                                if let Some(sid) = &app.application_sid {
                                    xml.push_str(&format!(" sid=\"{}\"", escape_xml_attr(sid)));
                                }
                                if let Some(cid) = &app.customer_id {
                                    xml.push_str(&format!(
                                        " customerId=\"{}\"",
                                        escape_xml_attr(cid)
                                    ));
                                }
                                if let Some(cpt) = &app.copy_parent_to {
                                    xml.push_str(&format!(
                                        " copyParentTo=\"{}\"",
                                        escape_xml_attr(cpt)
                                    ));
                                }

                                if has_nested {
                                    xml.push_str(">");
                                    for param in &app.parameters {
                                        xml.push_str("\n      <Parameter");
                                        if let Some(name) = &param.name {
                                            xml.push_str(&format!(
                                                " name=\"{}\"",
                                                escape_xml_attr(name)
                                            ));
                                        }
                                        if let Some(value) = &param.value {
                                            xml.push_str(&format!(
                                                " value=\"{}\"",
                                                escape_xml_attr(value)
                                            ));
                                        }
                                        xml.push_str(" />");
                                    }
                                    xml.push_str("\n    </Application>");
                                } else {
                                    xml.push_str(" />");
                                }
                            }
                            DialNoun::WhatsApp(wa) => {
                                xml.push_str("\n    <WhatsApp");
                                if let Some(u) = &wa.url {
                                    xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                }
                                if let Some(m) = &wa.method {
                                    xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                                }
                                xml.push_str(&format!(
                                    ">{}</WhatsApp>",
                                    escape_xml_attr(&wa.phone_number)
                                ));
                            }
                        }
                    }

                    if !dial.nested.is_empty() {
                        xml.push_str("\n  ");
                    }
                    xml.push_str("</Dial>\n");
                }
                VoiceVerb::Hangup(_) => {
                    xml.push_str("  <Hangup />\n");
                }
                VoiceVerb::Redirect(redirect) => {
                    xml.push_str("  <Redirect");
                    if let Some(m) = &redirect.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    xml.push_str(&format!(">{}</Redirect>\n", escape_xml_text(&redirect.url)));
                }
                VoiceVerb::Reject(reject) => {
                    xml.push_str("  <Reject");
                    if let Some(r) = &reject.attributes.reason {
                        xml.push_str(&format!(" reason=\"{}\"", escape_xml_attr(r)));
                    }
                    xml.push_str(" />\n");
                }
                VoiceVerb::Gather(gather) => {
                    xml.push_str("  <Gather");
                    if let Some(i) = &gather.attributes.input {
                        xml.push_str(&format!(" input=\"{}\"", escape_xml_attr(&i.join(" "))));
                    }
                    if let Some(a) = &gather.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &gather.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(t) = gather.attributes.timeout {
                        xml.push_str(&format!(" timeout=\"{}\"", t));
                    }
                    if let Some(f) = &gather.attributes.finish_on_key {
                        xml.push_str(&format!(" finishOnKey=\"{}\"", escape_xml_attr(f)));
                    }
                    if let Some(n) = gather.attributes.num_digits {
                        xml.push_str(&format!(" numDigits=\"{}\"", n));
                    }
                    if let Some(p) = &gather.attributes.partial_result_callback {
                        xml.push_str(&format!(
                            " partialResultCallback=\"{}\"",
                            escape_xml_attr(p)
                        ));
                    }
                    if let Some(l) = &gather.attributes.language {
                        xml.push_str(&format!(" language=\"{}\"", escape_xml_attr(l)));
                    }
                    if let Some(h) = &gather.attributes.hints {
                        xml.push_str(&format!(" hints=\"{}\"", escape_xml_attr(h)));
                    }
                    if let Some(b) = gather.attributes.barge_in {
                        xml.push_str(&format!(" bargeIn=\"{}\"", b));
                    }
                    if let Some(st) = &gather.attributes.speech_timeout {
                        xml.push_str(&format!(" speechTimeout=\"{}\"", escape_xml_attr(st)));
                    }
                    if let Some(aer) = gather.attributes.action_on_empty_result {
                        xml.push_str(&format!(" actionOnEmptyResult=\"{}\"", aer));
                    }
                    if let Some(d) = gather.attributes.debug {
                        xml.push_str(&format!(" debug=\"{}\"", d));
                    }
                    if let Some(dd) = gather.attributes.dtmf_detection {
                        xml.push_str(&format!(" dtmfDetection=\"{}\"", dd));
                    }
                    if let Some(e) = gather.attributes.enhanced {
                        xml.push_str(&format!(" enhanced=\"{}\"", e));
                    }
                    if let Some(mst) = gather.attributes.max_speech_time {
                        xml.push_str(&format!(" maxSpeechTime=\"{}\"", mst));
                    }
                    if let Some(prcm) = &gather.attributes.partial_result_callback_method {
                        xml.push_str(&format!(
                            " partialResultCallbackMethod=\"{}\"",
                            escape_xml_attr(prcm)
                        ));
                    }
                    if let Some(pf) = gather.attributes.profanity_filter {
                        xml.push_str(&format!(" profanityFilter=\"{}\"", pf));
                    }
                    if let Some(sm) = &gather.attributes.speech_model {
                        xml.push_str(&format!(" speechModel=\"{}\"", escape_xml_attr(sm)));
                    }

                    if gather.nested.is_empty() {
                        xml.push_str(" />\n");
                    } else {
                        xml.push_str(">\n");
                        for noun in &gather.nested {
                            match noun {
                                GatherNoun::Say(say) => {
                                    xml.push_str("    <Say");
                                    if let Some(v) = &say.attributes.voice {
                                        xml.push_str(&format!(" voice=\"{}\"", escape_xml_attr(v)));
                                    }
                                    if let Some(l) = &say.attributes.language {
                                        xml.push_str(&format!(
                                            " language=\"{}\"",
                                            escape_xml_attr(l)
                                        ));
                                    }
                                    if let Some(lc) = say.attributes.loop_count {
                                        xml.push_str(&format!(" loop=\"{}\"", lc));
                                    }
                                    xml.push_str(&format!(
                                        ">{}</Say>\n",
                                        escape_xml_text(&say.message)
                                    ));
                                }
                                GatherNoun::Play(play) => {
                                    xml.push_str("    <Play");
                                    if let Some(d) = &play.attributes.digits {
                                        xml.push_str(&format!(
                                            " digits=\"{}\"",
                                            escape_xml_attr(d)
                                        ));
                                    }
                                    if let Some(lc) = play.attributes.loop_count {
                                        xml.push_str(&format!(" loop=\"{}\"", lc));
                                    }
                                    xml.push_str(">");
                                    if let Some(url) = &play.url {
                                        xml.push_str(&escape_xml_text(url));
                                    }
                                    xml.push_str("</Play>\n");
                                }
                                GatherNoun::Pause(pause) => {
                                    xml.push_str("    <Pause");
                                    if let Some(len) = pause.attributes.length {
                                        xml.push_str(&format!(" length=\"{}\"", len));
                                    }
                                    xml.push_str(" />\n");
                                }
                            }
                        }
                        xml.push_str("  </Gather>\n");
                    }
                }
                VoiceVerb::Record(record) => {
                    xml.push_str("  <Record");
                    if let Some(a) = &record.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &record.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(t) = record.attributes.timeout {
                        xml.push_str(&format!(" timeout=\"{}\"", t));
                    }
                    if let Some(f) = &record.attributes.finish_on_key {
                        xml.push_str(&format!(" finishOnKey=\"{}\"", escape_xml_attr(f)));
                    }
                    if let Some(ml) = record.attributes.max_length {
                        xml.push_str(&format!(" maxLength=\"{}\"", ml));
                    }
                    if let Some(pb) = record.attributes.play_beep {
                        xml.push_str(&format!(" playBeep=\"{}\"", pb));
                    }
                    if let Some(tr) = &record.attributes.trim {
                        xml.push_str(&format!(" trim=\"{}\"", escape_xml_attr(tr)));
                    }
                    if let Some(rsc) = &record.attributes.recording_status_callback {
                        xml.push_str(&format!(
                            " recordingStatusCallback=\"{}\"",
                            escape_xml_attr(rsc)
                        ));
                    }
                    if let Some(rsce) = &record.attributes.recording_status_callback_event {
                        let events = rsce.join(" ");
                        xml.push_str(&format!(
                            " recordingStatusCallbackEvent=\"{}\"",
                            escape_xml_attr(&events)
                        ));
                    }
                    if let Some(rscm) = &record.attributes.recording_status_callback_method {
                        xml.push_str(&format!(
                            " recordingStatusCallbackMethod=\"{}\"",
                            escape_xml_attr(rscm)
                        ));
                    }
                    if let Some(t) = record.attributes.transcribe {
                        xml.push_str(&format!(" transcribe=\"{}\"", t));
                    }
                    if let Some(tc) = &record.attributes.transcribe_callback {
                        xml.push_str(&format!(" transcribeCallback=\"{}\"", escape_xml_attr(tc)));
                    }
                    if let Some(rc) = &record.attributes.recording_channels {
                        xml.push_str(&format!(" recordingChannels=\"{}\"", escape_xml_attr(rc)));
                    }
                    if let Some(rt) = &record.attributes.recording_track {
                        xml.push_str(&format!(" recordingTrack=\"{}\"", escape_xml_attr(rt)));
                    }
                    xml.push_str(" />\n");
                }
                VoiceVerb::Connect(connect) => {
                    xml.push_str("  <Connect");
                    if let Some(a) = &connect.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &connect.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }

                    if connect.nested.is_empty() {
                        xml.push_str(" />\n");
                    } else {
                        xml.push_str(">");

                        for noun in &connect.nested {
                            match noun {
                                ConnectNoun::Stream(stream) => {
                                    xml.push_str("\n    <Stream");
                                    if let Some(n) = &stream.name {
                                        xml.push_str(&format!(" name=\"{}\"", escape_xml_attr(n)));
                                    }
                                    if let Some(cn) = &stream.connector_name {
                                        xml.push_str(&format!(
                                            " connectorName=\"{}\"",
                                            escape_xml_attr(cn)
                                        ));
                                    }
                                    if let Some(u) = &stream.url {
                                        xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                    }
                                    if let Some(t) = &stream.track {
                                        xml.push_str(&format!(" track=\"{}\"", escape_xml_attr(t)));
                                    }
                                    if stream.parameters.is_empty() {
                                        xml.push_str(" />");
                                    } else {
                                        xml.push_str(">");
                                        for param in &stream.parameters {
                                            xml.push_str("\n      <Parameter");
                                            if let Some(n) = &param.name {
                                                xml.push_str(&format!(
                                                    " name=\"{}\"",
                                                    escape_xml_attr(n)
                                                ));
                                            }
                                            if let Some(v) = &param.value {
                                                xml.push_str(&format!(
                                                    " value=\"{}\"",
                                                    escape_xml_attr(v)
                                                ));
                                            }
                                            xml.push_str(" />");
                                        }
                                        xml.push_str("\n    </Stream>");
                                    }
                                }
                                ConnectNoun::Room(room) => {
                                    xml.push_str("\n    <Room");
                                    if let Some(pi) = &room.participant_identity {
                                        xml.push_str(&format!(
                                            " participantIdentity=\"{}\"",
                                            escape_xml_attr(pi)
                                        ));
                                    }
                                    xml.push_str(">");
                                    if let Some(n) = &room.name {
                                        xml.push_str(&escape_xml_attr(n));
                                    }
                                    xml.push_str("</Room>");
                                }
                                ConnectNoun::Conversation(conv) => {
                                    xml.push_str("\n    <Conversation");
                                    if let Some(sid) = &conv.service_instance_sid {
                                        xml.push_str(&format!(
                                            " serviceInstanceSid=\"{}\"",
                                            escape_xml_attr(sid)
                                        ));
                                    }
                                    xml.push_str(" />");
                                }
                                ConnectNoun::VirtualAgent(va) => {
                                    xml.push_str("\n    <VirtualAgent");
                                    if let Some(cn) = &va.connector_name {
                                        xml.push_str(&format!(
                                            " connectorName=\"{}\"",
                                            escape_xml_attr(cn)
                                        ));
                                    }
                                    if let Some(l) = &va.language {
                                        xml.push_str(&format!(
                                            " language=\"{}\"",
                                            escape_xml_attr(l)
                                        ));
                                    }
                                    if va.parameters.is_empty() {
                                        xml.push_str(" />");
                                    } else {
                                        xml.push_str(">");
                                        for param in &va.parameters {
                                            xml.push_str("\n      <Parameter");
                                            if let Some(n) = &param.name {
                                                xml.push_str(&format!(
                                                    " name=\"{}\"",
                                                    escape_xml_attr(n)
                                                ));
                                            }
                                            if let Some(v) = &param.value {
                                                xml.push_str(&format!(
                                                    " value=\"{}\"",
                                                    escape_xml_attr(v)
                                                ));
                                            }
                                            xml.push_str(" />");
                                        }
                                        xml.push_str("\n    </VirtualAgent>");
                                    }
                                }
                                ConnectNoun::Autopilot(ap) => {
                                    xml.push_str("\n    <Autopilot");
                                    xml.push_str(">");
                                    if let Some(n) = &ap.name {
                                        xml.push_str(&escape_xml_attr(n));
                                    }
                                    xml.push_str("</Autopilot>");
                                }
                                ConnectNoun::AiSession(ai) => {
                                    xml.push_str("\n    <AiSession");
                                    if let Some(sid) = &ai.assistant_sid {
                                        xml.push_str(&format!(
                                            " assistantSid=\"{}\"",
                                            escape_xml_attr(sid)
                                        ));
                                    }
                                    xml.push_str(" />");
                                }
                                ConnectNoun::ConversationRelaySession(crs) => {
                                    xml.push_str("\n    <ConversationRelaySession");
                                    if let Some(c) = &crs.connector {
                                        xml.push_str(&format!(
                                            " connector=\"{}\"",
                                            escape_xml_attr(c)
                                        ));
                                    }
                                    if let Some(sc) = &crs.session_configuration {
                                        xml.push_str(&format!(
                                            " sessionConfiguration=\"{}\"",
                                            escape_xml_attr(sc)
                                        ));
                                    }
                                    xml.push_str(" />");
                                }
                                ConnectNoun::Assistant(asst) => {
                                    xml.push_str("\n    <Assistant");
                                    if let Some(sid) = &asst.sid {
                                        xml.push_str(&format!(" sid=\"{}\"", escape_xml_attr(sid)));
                                    }
                                    xml.push_str(" />");
                                }
                                ConnectNoun::ConversationRelay(cr) => {
                                    let has_nested =
                                        !cr.languages.is_empty() || !cr.parameters.is_empty();

                                    xml.push_str("\n    <ConversationRelay");
                                    if let Some(u) = &cr.url {
                                        xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                    }
                                    if let Some(g) = &cr.welcome_greeting {
                                        xml.push_str(&format!(
                                            " welcomeGreeting=\"{}\"",
                                            escape_xml_attr(g)
                                        ));
                                    }
                                    if let Some(v) = &cr.voice {
                                        xml.push_str(&format!(" voice=\"{}\"", escape_xml_attr(v)));
                                    }
                                    if let Some(l) = &cr.language {
                                        xml.push_str(&format!(
                                            " language=\"{}\"",
                                            escape_xml_attr(l)
                                        ));
                                    }
                                    if let Some(d) = cr.dtmf_detection {
                                        xml.push_str(&format!(" dtmfDetection=\"{}\"", d));
                                    }
                                    if let Some(i) = cr.interruptible {
                                        xml.push_str(&format!(" interruptible=\"{}\"", i));
                                    }
                                    if let Some(s) = &cr.interruption_sensitivity {
                                        xml.push_str(&format!(
                                            " interruptionSensitivity=\"{}\"",
                                            escape_xml_attr(s)
                                        ));
                                    }
                                    if let Some(m) = &cr.speech_model {
                                        xml.push_str(&format!(
                                            " speechModel=\"{}\"",
                                            escape_xml_attr(m)
                                        ));
                                    }
                                    if let Some(p) = cr.profanity_filter {
                                        xml.push_str(&format!(" profanityFilter=\"{}\"", p));
                                    }
                                    if let Some(t) = cr.transcription_enabled {
                                        xml.push_str(&format!(" transcriptionEnabled=\"{}\"", t));
                                    }
                                    if let Some(sc) = &cr.status_callback {
                                        xml.push_str(&format!(
                                            " statusCallback=\"{}\"",
                                            escape_xml_attr(sc)
                                        ));
                                    }
                                    if let Some(scm) = &cr.status_callback_method {
                                        xml.push_str(&format!(
                                            " statusCallbackMethod=\"{}\"",
                                            escape_xml_attr(scm)
                                        ));
                                    }
                                    if let Some(md) = cr.max_duration {
                                        xml.push_str(&format!(" maxDuration=\"{}\"", md));
                                    }

                                    if has_nested {
                                        xml.push_str(">");

                                        // Add Language elements
                                        for lang in &cr.languages {
                                            xml.push_str("\n      <Language");
                                            if let Some(code) = &lang.language_code {
                                                xml.push_str(&format!(
                                                    " code=\"{}\"",
                                                    escape_xml_attr(code)
                                                ));
                                            }
                                            if let Some(tts) = &lang.tts_provider {
                                                xml.push_str(&format!(
                                                    " ttsProvider=\"{}\"",
                                                    escape_xml_attr(tts)
                                                ));
                                            }
                                            if let Some(stt) = &lang.stt_provider {
                                                xml.push_str(&format!(
                                                    " sttProvider=\"{}\"",
                                                    escape_xml_attr(stt)
                                                ));
                                            }
                                            xml.push_str(" />");
                                        }

                                        // Add Parameter elements
                                        for param in &cr.parameters {
                                            xml.push_str("\n      <Parameter");
                                            if let Some(name) = &param.name {
                                                xml.push_str(&format!(
                                                    " name=\"{}\"",
                                                    escape_xml_attr(name)
                                                ));
                                            }
                                            if let Some(value) = &param.value {
                                                xml.push_str(&format!(
                                                    " value=\"{}\"",
                                                    escape_xml_attr(value)
                                                ));
                                            }
                                            xml.push_str(" />");
                                        }

                                        xml.push_str("\n    </ConversationRelay>");
                                    } else {
                                        xml.push_str(" />");
                                    }
                                }
                            }
                        }

                        xml.push_str("\n  </Connect>\n");
                    }
                }
                VoiceVerb::Stop(_) => {
                    xml.push_str("  <Stop />\n");
                }
                VoiceVerb::Echo(_) => {
                    xml.push_str("  <Echo />\n");
                }
                VoiceVerb::Leave(_) => {
                    xml.push_str("  <Leave />\n");
                }
                VoiceVerb::Sms(sms) => {
                    xml.push_str("  <Sms");
                    if let Some(t) = &sms.attributes.to {
                        xml.push_str(&format!(" to=\"{}\"", escape_xml_attr(t)));
                    }
                    if let Some(f) = &sms.attributes.from {
                        xml.push_str(&format!(" from=\"{}\"", escape_xml_attr(f)));
                    }
                    if let Some(a) = &sms.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &sms.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(sc) = &sms.attributes.status_callback {
                        xml.push_str(&format!(" statusCallback=\"{}\"", escape_xml_attr(sc)));
                    }
                    xml.push_str(&format!(">{}</Sms>\n", escape_xml_text(&sms.message)));
                }
                VoiceVerb::Enqueue(enqueue) => {
                    xml.push_str("  <Enqueue");
                    if let Some(a) = &enqueue.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &enqueue.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(wu) = &enqueue.attributes.wait_url {
                        xml.push_str(&format!(" waitUrl=\"{}\"", escape_xml_attr(wu)));
                    }
                    if let Some(wum) = &enqueue.attributes.wait_url_method {
                        xml.push_str(&format!(" waitUrlMethod=\"{}\"", escape_xml_attr(wum)));
                    }
                    if let Some(ws) = &enqueue.attributes.workflow_sid {
                        xml.push_str(&format!(" workflowSid=\"{}\"", escape_xml_attr(ws)));
                    }
                    if let Some(mqs) = enqueue.attributes.max_queue_size {
                        xml.push_str(&format!(" maxQueueSize=\"{}\"", mqs));
                    }

                    if enqueue.name.is_none() && enqueue.task.is_none() {
                        xml.push_str(" />\n");
                    } else {
                        xml.push_str(">");
                        if let Some(name) = &enqueue.name {
                            xml.push_str(&escape_xml_attr(name));
                        }
                        if let Some(task) = &enqueue.task {
                            xml.push_str("\n    <Task");
                            if let Some(sid) = &task.task_sid {
                                xml.push_str(&format!(" sid=\"{}\"", escape_xml_attr(sid)));
                            }
                            xml.push_str(" />");
                        }
                        xml.push_str("\n  </Enqueue>\n");
                    }
                }
                VoiceVerb::Queue(queue) => {
                    xml.push_str("  <Queue");
                    if let Some(u) = &queue.attributes.url {
                        xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                    }
                    if let Some(m) = &queue.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }
                    if let Some(rs) = &queue.attributes.reservation_sid {
                        xml.push_str(&format!(" reservationSid=\"{}\"", escape_xml_attr(rs)));
                    }
                    if let Some(pwas) = &queue.attributes.post_work_activity_sid {
                        xml.push_str(&format!(
                            " postWorkActivitySid=\"{}\"",
                            escape_xml_attr(pwas)
                        ));
                    }
                    xml.push_str(&format!(">{}</Queue>\n", escape_xml_attr(&queue.name)));
                }
                VoiceVerb::Pay(pay) => {
                    xml.push_str("  <Pay");
                    if let Some(i) = &pay.attributes.input {
                        xml.push_str(&format!(" input=\"{}\"", escape_xml_attr(i)));
                    }
                    if let Some(a) = &pay.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(ca) = &pay.attributes.charge_amount {
                        xml.push_str(&format!(" chargeAmount=\"{}\"", escape_xml_attr(ca)));
                    }
                    if let Some(c) = &pay.attributes.currency {
                        xml.push_str(&format!(" currency=\"{}\"", escape_xml_attr(c)));
                    }
                    if let Some(pc) = &pay.attributes.payment_connector {
                        xml.push_str(&format!(" paymentConnector=\"{}\"", escape_xml_attr(pc)));
                    }
                    if let Some(pm) = &pay.attributes.payment_method {
                        xml.push_str(&format!(" paymentMethod=\"{}\"", escape_xml_attr(pm)));
                    }
                    if let Some(t) = pay.attributes.timeout {
                        xml.push_str(&format!(" timeout=\"{}\"", t));
                    }
                    if let Some(scm) = &pay.attributes.status_callback_method {
                        xml.push_str(&format!(
                            " statusCallbackMethod=\"{}\"",
                            escape_xml_attr(scm)
                        ));
                    }
                    if let Some(sc) = &pay.attributes.status_callback {
                        xml.push_str(&format!(" statusCallback=\"{}\"", escape_xml_attr(sc)));
                    }

                    if pay.prompts.is_empty() && pay.parameters.is_empty() {
                        xml.push_str(" />\n");
                    } else {
                        xml.push_str(">");

                        for prompt in &pay.prompts {
                            xml.push_str("\n    <Prompt");
                            if let Some(f) = &prompt.attributes.for_attr {
                                xml.push_str(&format!(" for=\"{}\"", escape_xml_attr(f)));
                            }
                            if let Some(a) = &prompt.attributes.attempt {
                                let attempts = a
                                    .iter()
                                    .map(|n| n.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                xml.push_str(&format!(
                                    " attempt=\"{}\"",
                                    escape_xml_attr(&attempts)
                                ));
                            }
                            if let Some(ct) = &prompt.attributes.card_type {
                                let types = ct.join(" ");
                                xml.push_str(&format!(" cardType=\"{}\"", escape_xml_attr(&types)));
                            }
                            if let Some(et) = &prompt.attributes.error_type {
                                let types = et.join(" ");
                                xml.push_str(&format!(
                                    " errorType=\"{}\"",
                                    escape_xml_attr(&types)
                                ));
                            }
                            xml.push_str(" />");
                        }

                        for param in &pay.parameters {
                            xml.push_str("\n    <Parameter");
                            if let Some(n) = &param.name {
                                xml.push_str(&format!(" name=\"{}\"", escape_xml_attr(n)));
                            }
                            if let Some(v) = &param.value {
                                xml.push_str(&format!(" value=\"{}\"", escape_xml_attr(v)));
                            }
                            xml.push_str(" />");
                        }

                        xml.push_str("\n  </Pay>\n");
                    }
                }
                VoiceVerb::Prompt(prompt) => {
                    xml.push_str("  <Prompt");
                    if let Some(f) = &prompt.attributes.for_attr {
                        xml.push_str(&format!(" for=\"{}\"", escape_xml_attr(f)));
                    }
                    if let Some(a) = &prompt.attributes.attempt {
                        let attempts = a
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        xml.push_str(&format!(" attempt=\"{}\"", escape_xml_attr(&attempts)));
                    }
                    if let Some(ct) = &prompt.attributes.card_type {
                        let types = ct.join(" ");
                        xml.push_str(&format!(" cardType=\"{}\"", escape_xml_attr(&types)));
                    }
                    if let Some(et) = &prompt.attributes.error_type {
                        let types = et.join(" ");
                        xml.push_str(&format!(" errorType=\"{}\"", escape_xml_attr(&types)));
                    }
                    xml.push_str(" />\n");
                }
                VoiceVerb::Refer(refer) => {
                    xml.push_str("  <Refer");
                    if let Some(a) = &refer.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &refer.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }

                    if let Some(sip) = &refer.refer_sip {
                        xml.push_str(">");
                        xml.push_str(&format!(
                            "\n    <Sip>{}</Sip>",
                            escape_xml_attr(&sip.sip_url)
                        ));
                        xml.push_str("\n  </Refer>\n");
                    } else {
                        xml.push_str(" />\n");
                    }
                }
                VoiceVerb::Start(start) => {
                    xml.push_str("  <Start");
                    if let Some(a) = &start.attributes.action {
                        xml.push_str(&format!(" action=\"{}\"", escape_xml_attr(a)));
                    }
                    if let Some(m) = &start.attributes.method {
                        xml.push_str(&format!(" method=\"{}\"", escape_xml_attr(m)));
                    }

                    if start.nested.is_empty() {
                        xml.push_str(" />\n");
                    } else {
                        xml.push_str(">");

                        for noun in &start.nested {
                            match noun {
                                StartNoun::Stream(stream) => {
                                    xml.push_str("\n    <Stream");
                                    if let Some(n) = &stream.name {
                                        xml.push_str(&format!(" name=\"{}\"", escape_xml_attr(n)));
                                    }
                                    if let Some(cn) = &stream.connector_name {
                                        xml.push_str(&format!(
                                            " connectorName=\"{}\"",
                                            escape_xml_attr(cn)
                                        ));
                                    }
                                    if let Some(u) = &stream.url {
                                        xml.push_str(&format!(" url=\"{}\"", escape_xml_attr(u)));
                                    }
                                    if let Some(t) = &stream.track {
                                        xml.push_str(&format!(" track=\"{}\"", escape_xml_attr(t)));
                                    }
                                    if let Some(sc) = &stream.status_callback {
                                        xml.push_str(&format!(
                                            " statusCallback=\"{}\"",
                                            escape_xml_attr(sc)
                                        ));
                                    }
                                    if let Some(scm) = &stream.status_callback_method {
                                        xml.push_str(&format!(
                                            " statusCallbackMethod=\"{}\"",
                                            escape_xml_attr(scm)
                                        ));
                                    }

                                    if stream.parameters.is_empty() {
                                        xml.push_str(" />");
                                    } else {
                                        xml.push_str(">");
                                        for param in &stream.parameters {
                                            xml.push_str("\n      <Parameter");
                                            if let Some(n) = &param.name {
                                                xml.push_str(&format!(
                                                    " name=\"{}\"",
                                                    escape_xml_attr(n)
                                                ));
                                            }
                                            if let Some(v) = &param.value {
                                                xml.push_str(&format!(
                                                    " value=\"{}\"",
                                                    escape_xml_attr(v)
                                                ));
                                            }
                                            xml.push_str(" />");
                                        }
                                        xml.push_str("\n    </Stream>");
                                    }
                                }
                                StartNoun::Siprec(siprec) => {
                                    xml.push_str("\n    <Siprec");
                                    if let Some(n) = &siprec.name {
                                        xml.push_str(&format!(" name=\"{}\"", escape_xml_attr(n)));
                                    }
                                    if let Some(cn) = &siprec.connector_name {
                                        xml.push_str(&format!(
                                            " connectorName=\"{}\"",
                                            escape_xml_attr(cn)
                                        ));
                                    }
                                    if let Some(t) = &siprec.track {
                                        xml.push_str(&format!(" track=\"{}\"", escape_xml_attr(t)));
                                    }
                                    xml.push_str(" />");
                                }
                                StartNoun::Transcription(trans) => {
                                    xml.push_str("\n    <Transcription");
                                    if let Some(n) = &trans.attributes.name {
                                        xml.push_str(&format!(" name=\"{}\"", escape_xml_attr(n)));
                                    }
                                    if let Some(t) = &trans.attributes.track {
                                        xml.push_str(&format!(" track=\"{}\"", escape_xml_attr(t)));
                                    }
                                    if let Some(lc) = &trans.attributes.language_code {
                                        xml.push_str(&format!(
                                            " languageCode=\"{}\"",
                                            escape_xml_attr(lc)
                                        ));
                                    }
                                    if let Some(enable) =
                                        trans.attributes.enable_automatic_punctuation
                                    {
                                        xml.push_str(&format!(
                                            " enableAutomaticPunctuation=\"{}\"",
                                            enable
                                        ));
                                    }
                                    if let Some(h) = &trans.attributes.hints {
                                        xml.push_str(&format!(" hints=\"{}\"", escape_xml_attr(h)));
                                    }
                                    if let Some(label) = &trans.attributes.inbound_track_label {
                                        xml.push_str(&format!(
                                            " inboundTrackLabel=\"{}\"",
                                            escape_xml_attr(label)
                                        ));
                                    }
                                    if let Some(service) = &trans.attributes.intelligence_service {
                                        xml.push_str(&format!(
                                            " intelligenceService=\"{}\"",
                                            escape_xml_attr(service)
                                        ));
                                    }
                                    if let Some(label) = &trans.attributes.outbound_track_label {
                                        xml.push_str(&format!(
                                            " outboundTrackLabel=\"{}\"",
                                            escape_xml_attr(label)
                                        ));
                                    }
                                    if let Some(enable) = trans.attributes.partial_results {
                                        xml.push_str(&format!(" partialResults=\"{}\"", enable));
                                    }
                                    if let Some(enable) = trans.attributes.profanity_filter {
                                        xml.push_str(&format!(" profanityFilter=\"{}\"", enable));
                                    }
                                    if let Some(model) = &trans.attributes.speech_model {
                                        xml.push_str(&format!(
                                            " speechModel=\"{}\"",
                                            escape_xml_attr(model)
                                        ));
                                    }
                                    if let Some(method) = &trans.attributes.status_callback_method {
                                        xml.push_str(&format!(
                                            " statusCallbackMethod=\"{}\"",
                                            escape_xml_attr(method)
                                        ));
                                    }
                                    if let Some(url) = &trans.attributes.status_callback_url {
                                        xml.push_str(&format!(
                                            " statusCallbackUrl=\"{}\"",
                                            escape_xml_attr(url)
                                        ));
                                    }
                                    if let Some(engine) = &trans.attributes.transcription_engine {
                                        xml.push_str(&format!(
                                            " transcriptionEngine=\"{}\"",
                                            escape_xml_attr(engine)
                                        ));
                                    }
                                    xml.push_str(" />");
                                }
                                StartNoun::Recording(rec) => {
                                    xml.push_str("\n    <Recording");
                                    if let Some(cb) = &rec.recording_status_callback {
                                        xml.push_str(&format!(
                                            " recordingStatusCallback=\"{}\"",
                                            escape_xml_attr(cb)
                                        ));
                                    }
                                    if let Some(m) = &rec.recording_status_callback_method {
                                        xml.push_str(&format!(
                                            " recordingStatusCallbackMethod=\"{}\"",
                                            escape_xml_attr(m)
                                        ));
                                    }
                                    if let Some(e) = &rec.recording_status_callback_event {
                                        xml.push_str(&format!(
                                            " recordingStatusCallbackEvent=\"{}\"",
                                            escape_xml_attr(e)
                                        ));
                                    }
                                    if let Some(tr) = &rec.trim {
                                        xml.push_str(&format!(" trim=\"{}\"", escape_xml_attr(tr)));
                                    }
                                    if let Some(t) = &rec.track {
                                        xml.push_str(&format!(" track=\"{}\"", escape_xml_attr(t)));
                                    }
                                    if let Some(ch) = &rec.channels {
                                        xml.push_str(&format!(
                                            " channels=\"{}\"",
                                            escape_xml_attr(ch)
                                        ));
                                    }
                                    xml.push_str(" />");
                                }
                            }
                        }

                        xml.push_str("\n  </Start>\n");
                    }
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
