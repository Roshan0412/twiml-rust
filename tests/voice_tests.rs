use twiml_rust::voice::*;
use twiml_rust::TwiML;

#[test]
fn test_simple_say() {
    let response = VoiceResponse::new().say("Hello World");
    let xml = response.to_xml();
    assert!(xml.contains("<Say>Hello World</Say>"));
}

#[test]
fn test_say_with_attributes() {
    let response = VoiceResponse::new().say_with(
        Say::new("Hello")
            .voice("alice")
            .language("en-US")
            .loop_count(2),
    );
    let xml = response.to_xml();
    assert!(xml.contains("voice=\"alice\""));
    assert!(xml.contains("language=\"en-US\""));
    assert!(xml.contains("loop=\"2\""));
}

#[test]
fn test_say_with_ssml() {
    let response = VoiceResponse::new().say_with(
        Say::new("Hello")
            .add_break(Some("medium".to_string()), None)
            .add_emphasis(Some("strong".to_string()), "World"),
    );
    let xml = response.to_xml();
    assert!(xml.contains("<break strength=\"medium\""));
    assert!(xml.contains("<emphasis level=\"strong\">World</emphasis>"));
}

#[test]
fn test_ssml_break() {
    let response = VoiceResponse::new().say_with(
        Say::new("Please wait").add_break(Some("strong".to_string()), Some("2s".to_string())),
    );
    let xml = response.to_xml();
    assert!(xml.contains("<break strength=\"strong\" time=\"2s\""));
}

#[test]
fn test_ssml_emphasis() {
    let response = VoiceResponse::new()
        .say_with(Say::new("This is").add_emphasis(Some("moderate".to_string()), "important"));
    let xml = response.to_xml();
    assert!(xml.contains("<emphasis level=\"moderate\">important</emphasis>"));
}

#[test]
fn test_ssml_prosody() {
    let response = VoiceResponse::new().say_with(Say::new("Hello").add_prosody(
        Some("high".to_string()),
        Some("fast".to_string()),
        Some("loud".to_string()),
        "World",
    ));
    let xml = response.to_xml();
    assert!(xml.contains("<prosody pitch=\"high\" rate=\"fast\" volume=\"loud\">World</prosody>"));
}

#[test]
fn test_ssml_lang() {
    let response = VoiceResponse::new().say_with(Say::new("Hello").add_lang("fr-FR", "Bonjour"));
    let xml = response.to_xml();
    assert!(xml.contains("<lang xml:lang=\"fr-FR\">Bonjour</lang>"));
}

#[test]
fn test_ssml_p_and_s() {
    let response = VoiceResponse::new().say_with(
        Say::new("")
            .add_p("This is a paragraph.")
            .add_s("This is a sentence."),
    );
    let xml = response.to_xml();
    assert!(xml.contains("<p>This is a paragraph.</p>"));
    assert!(xml.contains("<s>This is a sentence.</s>"));
}

#[test]
fn test_ssml_phoneme() {
    let response = VoiceResponse::new().say_with(Say::new("You say").add_phoneme(
        "pɪˈkɑːn",
        "pecan",
        Some("ipa".to_string()),
    ));
    let xml = response.to_xml();
    assert!(xml.contains("<phoneme alphabet=\"ipa\" ph=\"pɪˈkɑːn\">pecan</phoneme>"));
}

#[test]
fn test_ssml_say_as() {
    let response = VoiceResponse::new().say_with(Say::new("Your number is").add_say_as(
        "telephone",
        "555-1234",
        None,
    ));
    let xml = response.to_xml();
    assert!(xml.contains("<say-as interpret-as=\"telephone\">555-1234</say-as>"));
}

#[test]
fn test_ssml_say_as_with_format() {
    let response = VoiceResponse::new().say_with(Say::new("The date is").add_say_as(
        "date",
        "20260101",
        Some("yyyymmdd".to_string()),
    ));
    let xml = response.to_xml();
    assert!(xml.contains("<say-as interpret-as=\"date\" format=\"yyyymmdd\">20260101</say-as>"));
}

#[test]
fn test_ssml_sub() {
    let response = VoiceResponse::new()
        .say_with(Say::new("Welcome to").add_sub("World Wide Web Consortium", "W3C"));
    let xml = response.to_xml();
    assert!(xml.contains("<sub alias=\"World Wide Web Consortium\">W3C</sub>"));
}

#[test]
fn test_ssml_w() {
    let response =
        VoiceResponse::new().say_with(Say::new("I").add_w("read", Some("amazon:VBD".to_string())));
    let xml = response.to_xml();
    assert!(xml.contains("<w role=\"amazon:VBD\">read</w>"));
}

#[test]
fn test_ssml_amazon_effect() {
    let response = VoiceResponse::new().say_with(
        Say::new("I want to tell you a secret").add_amazon_effect("whispered", "this is a secret"),
    );
    let xml = response.to_xml();
    assert!(xml.contains("<amazon:effect name=\"whispered\">this is a secret</amazon:effect>"));
}

#[test]
fn test_ssml_amazon_domain() {
    let response = VoiceResponse::new().say_with(
        Say::new("Welcome").add_amazon_domain("conversational", "How are you doing today?"),
    );
    let xml = response.to_xml();
    assert!(xml.contains(
        "<amazon:domain name=\"conversational\">How are you doing today?</amazon:domain>"
    ));
}

#[test]
fn test_ssml_complex_combination() {
    let response = VoiceResponse::new().say_with(
        Say::new("Welcome to our service")
            .add_break(Some("medium".to_string()), None)
            .add_emphasis(Some("strong".to_string()), "Please listen carefully")
            .add_break(None, Some("1s".to_string()))
            .add_prosody(
                Some("high".to_string()),
                Some("slow".to_string()),
                None,
                "This is important",
            )
            .add_lang("es-ES", "Hola")
            .add_say_as("telephone", "555-1234", None),
    );
    let xml = response.to_xml();
    assert!(xml.contains("<break strength=\"medium\""));
    assert!(xml.contains("<emphasis level=\"strong\">Please listen carefully</emphasis>"));
    assert!(xml.contains("<break time=\"1s\""));
    assert!(xml.contains("<prosody pitch=\"high\" rate=\"slow\">This is important</prosody>"));
    assert!(xml.contains("<lang xml:lang=\"es-ES\">Hola</lang>"));
    assert!(xml.contains("<say-as interpret-as=\"telephone\">555-1234</say-as>"));
}

#[test]
fn test_play() {
    let response = VoiceResponse::new().play("https://example.com/audio.mp3");
    let xml = response.to_xml();
    assert!(xml.contains("<Play>https://example.com/audio.mp3</Play>"));
}

#[test]
fn test_play_with_loop() {
    let response = VoiceResponse::new().play_with(
        Play::new()
            .url("https://example.com/audio.mp3")
            .loop_count(3),
    );
    let xml = response.to_xml();
    assert!(xml.contains("loop=\"3\""));
}

#[test]
fn test_pause() {
    let response = VoiceResponse::new().pause(Some(5));
    let xml = response.to_xml();
    assert!(xml.contains("<Pause length=\"5\""));
}

#[test]
fn test_dial_simple() {
    let response = VoiceResponse::new().dial("+15551234567");
    let xml = response.to_xml();
    assert!(xml.contains("<Dial>+15551234567</Dial>"));
}

#[test]
fn test_dial_with_conference() {
    let conference = DialConference {
        name: "MyRoom".to_string(),
        muted: Some(false),
        beep: Some("true".to_string()),
        start_conference_on_enter: Some(true),
        end_conference_on_exit: Some(false),
        wait_url: None,
        wait_method: None,
        max_participants: Some(10),
        record: Some("record-from-start".to_string()),
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
    };

    let response = VoiceResponse::new().dial_with(Dial::new().add_conference(conference));
    let xml = response.to_xml();
    assert!(xml.contains("<Conference"));
    assert!(xml.contains("muted=\"false\""));
    assert!(xml.contains("maxParticipants=\"10\""));
    assert!(xml.contains(">MyRoom</Conference>"));
}

#[test]
fn test_gather() {
    let gather = Gather::new()
        .input(vec!["dtmf".to_string(), "speech".to_string()])
        .action("https://example.com/gather")
        .timeout(10)
        .add_say(Say::new("Please enter your account number"));

    let response = VoiceResponse::new().gather(gather);
    let xml = response.to_xml();
    assert!(xml.contains("<Gather"));
    assert!(xml.contains("input=\"dtmf speech\""));
    assert!(xml.contains("timeout=\"10\""));
    assert!(xml.contains("<Say>Please enter your account number</Say>"));
    assert!(xml.contains("</Gather>"));
}

#[test]
fn test_record() {
    let record = Record::new().max_length(30).timeout(5);

    let response = VoiceResponse::new().record(record);
    let xml = response.to_xml();
    assert!(xml.contains("<Record"));
    assert!(xml.contains("maxLength=\"30\""));
    assert!(xml.contains("timeout=\"5\""));
}

#[test]
fn test_hangup() {
    let response = VoiceResponse::new().hangup();
    let xml = response.to_xml();
    assert!(xml.contains("<Hangup />"));
}

#[test]
fn test_redirect() {
    let response = VoiceResponse::new().redirect("https://example.com/next");
    let xml = response.to_xml();
    assert!(xml.contains("<Redirect>https://example.com/next</Redirect>"));
}

#[test]
fn test_reject() {
    let response = VoiceResponse::new().reject(Reject::new().reason("busy"));
    let xml = response.to_xml();
    assert!(xml.contains("<Reject"));
    assert!(xml.contains("reason=\"busy\""));
}

#[test]
fn test_complex_response() {
    let response = VoiceResponse::new()
        .say("Welcome to our service")
        .pause(Some(1))
        .gather(
            Gather::new()
                .input(vec!["dtmf".to_string()])
                .num_digits(1)
                .add_say(Say::new("Press 1 for sales, 2 for support")),
        )
        .say("We didn't receive any input")
        .hangup();

    let xml = response.to_xml();
    assert!(xml.contains("<Say>Welcome to our service</Say>"));
    assert!(xml.contains("<Pause length=\"1\""));
    assert!(xml.contains("<Gather"));
    assert!(xml.contains("numDigits=\"1\""));
    assert!(xml.contains("<Say>We didn't receive any input</Say>"));
    assert!(xml.contains("<Hangup />"));
}

#[test]
fn test_connect_with_stream() {
    let stream = Stream::new()
        .name("my-stream")
        .url("wss://example.com/stream")
        .track("both_tracks");

    let connect = Connect::new()
        .action("https://example.com/connect-status")
        .add_stream(stream);

    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<Connect"));
    assert!(xml.contains("action=\"https://example.com/connect-status\""));
    assert!(xml.contains("<Stream"));
    assert!(xml.contains("name=\"my-stream\""));
    assert!(xml.contains("url=\"wss://example.com/stream\""));
    assert!(xml.contains("track=\"both_tracks\""));
}

#[test]
fn test_connect_with_room() {
    let room = Room::new("my-video-room").participant_identity("user123");

    let connect = Connect::new().add_room(room);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<Room"));
    assert!(xml.contains("participantIdentity=\"user123\""));
    assert!(xml.contains(">my-video-room</Room>"));
}

#[test]
fn test_connect_with_conversation_relay_session_basic() {
    let session = ConversationRelaySession::new().connector("my-connector");

    let connect = Connect::new().add_conversation_relay_session(session);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelaySession"));
    assert!(xml.contains("connector=\"my-connector\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_connect_with_conversation_relay_session_full() {
    let session = ConversationRelaySession::new()
        .connector("my-connector")
        .session_configuration("my-config");

    let connect = Connect::new().add_conversation_relay_session(session);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelaySession"));
    assert!(xml.contains("connector=\"my-connector\""));
    assert!(xml.contains("sessionConfiguration=\"my-config\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_connect_with_conversation_relay_session_empty() {
    let session = ConversationRelaySession::new();

    let connect = Connect::new().add_conversation_relay_session(session);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelaySession"));
    assert!(xml.contains(" />"));
}

#[test]
fn test_enqueue() {
    let enqueue = Enqueue::new()
        .name("support-queue")
        .action("https://example.com/enqueue-status")
        .wait_url("https://example.com/wait-music");

    let response = VoiceResponse::new().enqueue(enqueue);
    let xml = response.to_xml();

    assert!(xml.contains("<Enqueue"));
    assert!(xml.contains("action=\"https://example.com/enqueue-status\""));
    assert!(xml.contains("waitUrl=\"https://example.com/wait-music\""));
    assert!(xml.contains(">support-queue"));
    assert!(xml.contains("</Enqueue>"));
}

#[test]
fn test_pay() {
    let pay = Pay::new()
        .input("dtmf")
        .action("https://example.com/payment-complete")
        .charge_amount("10.00")
        .currency("USD");

    let response = VoiceResponse::new().pay(pay);
    let xml = response.to_xml();

    assert!(xml.contains("<Pay"));
    assert!(xml.contains("input=\"dtmf\""));
    assert!(xml.contains("chargeAmount=\"10.00\""));
    assert!(xml.contains("currency=\"USD\""));
}

#[test]
fn test_refer_with_sip() {
    let refer_sip = ReferSip::new("sip:alice@example.com");
    let refer = Refer::new()
        .action("https://example.com/refer-status")
        .add_refer_sip(refer_sip);

    let response = VoiceResponse::new().refer(refer);
    let xml = response.to_xml();

    assert!(xml.contains("<Refer"));
    assert!(xml.contains("action=\"https://example.com/refer-status\""));
    assert!(xml.contains("<Sip>sip:alice@example.com</Sip>"));
}

#[test]
fn test_start_with_stream() {
    let stream = Stream::new()
        .name("recording-stream")
        .url("wss://example.com/recording");

    let start = Start::new().add_stream(stream);
    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("<Start"));
    assert!(xml.contains("<Stream"));
    assert!(xml.contains("name=\"recording-stream\""));
}

#[test]
fn test_start_with_transcription() {
    let transcription = Transcription::new()
        .name("live-transcription")
        .track("inbound_track")
        .language_code("en-US");

    let start = Start::new().add_transcription(transcription);
    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("<Start"));
    assert!(xml.contains("<Transcription"));
    assert!(xml.contains("name=\"live-transcription\""));
    assert!(xml.contains("languageCode=\"en-US\""));
}

#[test]
fn test_start_with_recording_basic() {
    let recording = Recording::new().track("both");

    let start = Start::new().add_recording(recording);
    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("<Start"));
    assert!(xml.contains("<Recording"));
    assert!(xml.contains("track=\"both\""));
    assert!(xml.contains("</Start>"));
}

#[test]
fn test_start_with_recording_all_attributes() {
    let recording = Recording::new()
        .recording_status_callback("https://example.com/recording-status")
        .recording_status_callback_method("POST")
        .recording_status_callback_event("in-progress completed")
        .trim("trim-silence")
        .track("both")
        .channels("dual");

    let start = Start::new()
        .action("https://example.com/start-action")
        .method("POST")
        .add_recording(recording);

    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("<Start"));
    assert!(xml.contains("action=\"https://example.com/start-action\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("<Recording"));
    assert!(xml.contains("recordingStatusCallback=\"https://example.com/recording-status\""));
    assert!(xml.contains("recordingStatusCallbackMethod=\"POST\""));
    assert!(xml.contains("recordingStatusCallbackEvent=\"in-progress completed\""));
    assert!(xml.contains("trim=\"trim-silence\""));
    assert!(xml.contains("track=\"both\""));
    assert!(xml.contains("channels=\"dual\""));
    assert!(xml.contains("</Start>"));
}

#[test]
fn test_start_with_recording_inbound_track() {
    let recording = Recording::new().track("inbound").channels("mono");

    let start = Start::new().add_recording(recording);
    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("track=\"inbound\""));
    assert!(xml.contains("channels=\"mono\""));
}

#[test]
fn test_start_with_recording_outbound_track() {
    let recording = Recording::new().track("outbound").trim("do-not-trim");

    let start = Start::new().add_recording(recording);
    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("track=\"outbound\""));
    assert!(xml.contains("trim=\"do-not-trim\""));
}

#[test]
fn test_start_with_multiple_nouns() {
    let stream = Stream::new()
        .name("my-stream")
        .url("wss://example.com/stream");

    let recording = Recording::new().track("both").channels("dual");

    let transcription = Transcription::new()
        .name("my-transcription")
        .track("inbound_track");

    let start = Start::new()
        .add_stream(stream)
        .add_recording(recording)
        .add_transcription(transcription);

    let response = VoiceResponse::new().start(start);
    let xml = response.to_xml();

    assert!(xml.contains("<Stream"));
    assert!(xml.contains("name=\"my-stream\""));
    assert!(xml.contains("<Recording"));
    assert!(xml.contains("track=\"both\""));
    assert!(xml.contains("<Transcription"));
    assert!(xml.contains("name=\"my-transcription\""));
}

#[test]
fn test_dial_with_number() {
    let number = DialNumber::new("+15551234567")
        .send_digits("1234")
        .url("https://example.com/dial-status");

    let dial = Dial::new().add_number(number);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Number"));
    assert!(xml.contains("sendDigits=\"1234\""));
    assert!(xml.contains("url=\"https://example.com/dial-status\""));
    assert!(xml.contains(">+15551234567</Number>"));
}

#[test]
fn test_dial_with_client() {
    let client = DialClient::new("alice").url("https://example.com/client-status");

    let dial = Dial::new().add_client(client);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Client"));
    assert!(xml.contains("url=\"https://example.com/client-status\""));
    assert!(xml.contains(">alice</Client>"));
}

#[test]
fn test_dial_with_sip() {
    let sip = DialSip::new("sip:alice@example.com")
        .username("user")
        .password("pass");

    let dial = Dial::new().add_sip(sip);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Sip"));
    assert!(xml.contains("username=\"user\""));
    assert!(xml.contains("password=\"pass\""));
    assert!(xml.contains(">sip:alice@example.com</Sip>"));
}

#[test]
fn test_leave() {
    let response = VoiceResponse::new().leave();
    let xml = response.to_xml();
    assert!(xml.contains("<Leave />"));
}

#[test]
fn test_stop() {
    let response = VoiceResponse::new().stop();
    let xml = response.to_xml();
    assert!(xml.contains("<Stop />"));
}

#[test]
fn test_echo() {
    let response = VoiceResponse::new().echo();
    let xml = response.to_xml();
    assert!(xml.contains("<Echo />"));
}

#[test]
fn test_comment_before() {
    let response = VoiceResponse::new()
        .comment_before("Generated by TwiML Rust")
        .say("Hello!");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Generated by TwiML Rust -->"));
    assert!(xml.contains("<Say>Hello!</Say>"));

    // Verify comment comes before Response
    let comment_pos = xml.find("<!-- Generated by TwiML Rust -->").unwrap();
    let response_pos = xml.find("<Response>").unwrap();
    assert!(comment_pos < response_pos);
}

#[test]
fn test_comment_inside() {
    let response = VoiceResponse::new()
        .say("Hello!")
        .comment("This is a comment inside the response")
        .say("Goodbye!");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- This is a comment inside the response -->"));
    assert!(xml.contains("<Say>Hello!</Say>"));
    assert!(xml.contains("<Say>Goodbye!</Say>"));

    // Verify comment is inside Response
    let response_start = xml.find("<Response>").unwrap();
    let response_end = xml.find("</Response>").unwrap();
    let comment_pos = xml
        .find("<!-- This is a comment inside the response -->")
        .unwrap();
    assert!(comment_pos > response_start && comment_pos < response_end);
}

#[test]
fn test_comment_after() {
    let response = VoiceResponse::new()
        .say("Hello!")
        .comment_after("End of TwiML");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- End of TwiML -->"));
    assert!(xml.contains("<Say>Hello!</Say>"));

    // Verify comment comes after Response
    let response_pos = xml.find("</Response>").unwrap();
    let comment_pos = xml.find("<!-- End of TwiML -->").unwrap();
    assert!(comment_pos > response_pos);
}

#[test]
fn test_multiple_comments_before() {
    let response = VoiceResponse::new()
        .comment_before("First comment")
        .comment_before("Second comment")
        .say("Hello!");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- First comment -->"));
    assert!(xml.contains("<!-- Second comment -->"));

    // Verify order
    let first_pos = xml.find("<!-- First comment -->").unwrap();
    let second_pos = xml.find("<!-- Second comment -->").unwrap();
    assert!(first_pos < second_pos);
}

#[test]
fn test_multiple_comments_inside() {
    let response = VoiceResponse::new()
        .comment("First inside comment")
        .say("Hello!")
        .comment("Second inside comment")
        .say("Goodbye!");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- First inside comment -->"));
    assert!(xml.contains("<!-- Second inside comment -->"));
}

#[test]
fn test_multiple_comments_after() {
    let response = VoiceResponse::new()
        .say("Hello!")
        .comment_after("First after comment")
        .comment_after("Second after comment");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- First after comment -->"));
    assert!(xml.contains("<!-- Second after comment -->"));

    // Verify order
    let first_pos = xml.find("<!-- First after comment -->").unwrap();
    let second_pos = xml.find("<!-- Second after comment -->").unwrap();
    assert!(first_pos < second_pos);
}

#[test]
fn test_comments_with_all_features() {
    let response = VoiceResponse::new()
        .comment_before("Generated by TwiML Rust")
        .comment_before("Version 1.0")
        .comment("Main call flow")
        .say("Welcome to our service")
        .comment("Gather input section")
        .gather(
            Gather::new()
                .input(vec!["dtmf".to_string()])
                .num_digits(1)
                .add_say(Say::new("Press 1 for sales")),
        )
        .comment("Fallback section")
        .say("We didn't receive any input")
        .hangup()
        .comment_after("End of TwiML")
        .comment_after("Generated at 2026-01-01");

    let xml = response.to_xml();

    // Verify all comments are present
    assert!(xml.contains("<!-- Generated by TwiML Rust -->"));
    assert!(xml.contains("<!-- Version 1.0 -->"));
    assert!(xml.contains("<!-- Main call flow -->"));
    assert!(xml.contains("<!-- Gather input section -->"));
    assert!(xml.contains("<!-- Fallback section -->"));
    assert!(xml.contains("<!-- End of TwiML -->"));
    assert!(xml.contains("<!-- Generated at 2026-01-01 -->"));

    // Verify verbs are present
    assert!(xml.contains("<Say>Welcome to our service</Say>"));
    assert!(xml.contains("<Gather"));
    assert!(xml.contains("<Hangup />"));
}

#[test]
fn test_comment_with_special_characters() {
    let response = VoiceResponse::new()
        .comment_before("Comment with <special> & \"characters\"")
        .say("Hello!");

    let xml = response.to_xml();
    // Comments are now escaped for security
    assert!(xml.contains("<!-- Comment with &lt;special&gt; &amp; \"characters\" -->"));
}

#[test]
fn test_empty_response_with_comments() {
    let response = VoiceResponse::new()
        .comment_before("Before")
        .comment("Inside")
        .comment_after("After");

    let xml = response.to_xml();
    assert!(xml.contains("<!-- Before -->"));
    assert!(xml.contains("<!-- Inside -->"));
    assert!(xml.contains("<!-- After -->"));
    assert!(xml.contains("<Response>"));
    assert!(xml.contains("</Response>"));
}

// Tests for new Record attributes
#[test]
fn test_record_with_recording_channels() {
    let response =
        VoiceResponse::new().record_with(Record::new().recording_channels("dual").max_length(60));

    let xml = response.to_xml();
    assert!(xml.contains("recordingChannels=\"dual\""));
    assert!(xml.contains("maxLength=\"60\""));
}

#[test]
fn test_record_with_recording_track() {
    let response =
        VoiceResponse::new().record_with(Record::new().recording_track("both").timeout(10));

    let xml = response.to_xml();
    assert!(xml.contains("recordingTrack=\"both\""));
    assert!(xml.contains("timeout=\"10\""));
}

#[test]
fn test_record_with_all_new_attributes() {
    let response = VoiceResponse::new().record_with(
        Record::new()
            .recording_channels("dual")
            .recording_track("both")
            .finish_on_key("#")
            .trim("trim-silence")
            .recording_status_callback("https://example.com/status")
            .recording_status_callback_method("POST")
            .recording_status_callback_event(vec!["completed".to_string(), "absent".to_string()]),
    );

    let xml = response.to_xml();
    assert!(xml.contains("recordingChannels=\"dual\""));
    assert!(xml.contains("recordingTrack=\"both\""));
    assert!(xml.contains("finishOnKey=\"#\""));
    assert!(xml.contains("trim=\"trim-silence\""));
    assert!(xml.contains("recordingStatusCallback=\"https://example.com/status\""));
    assert!(xml.contains("recordingStatusCallbackMethod=\"POST\""));
}

// Tests for new Transcription attributes
#[test]
fn test_transcription_with_automatic_punctuation() {
    let response = VoiceResponse::new().start_with(
        Start::new().add_transcription(
            Transcription::new()
                .name("my_transcription")
                .enable_automatic_punctuation(true),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<Transcription"));
    assert!(xml.contains("name=\"my_transcription\""));
    assert!(xml.contains("enableAutomaticPunctuation=\"true\""));
}

#[test]
fn test_transcription_with_hints() {
    let response = VoiceResponse::new().start_with(
        Start::new().add_transcription(
            Transcription::new()
                .hints("Twilio,TwiML,API")
                .partial_results(true),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("hints=\"Twilio,TwiML,API\""));
    assert!(xml.contains("partialResults=\"true\""));
}

#[test]
fn test_transcription_with_all_new_attributes() {
    let response = VoiceResponse::new().start_with(
        Start::new().add_transcription(
            Transcription::new()
                .name("full_transcription")
                .track("both")
                .language_code("en-US")
                .enable_automatic_punctuation(true)
                .hints("Twilio,API")
                .inbound_track_label("customer")
                .outbound_track_label("agent")
                .partial_results(true)
                .profanity_filter(false)
                .speech_model("phone_call")
                .status_callback_url("https://example.com/transcription")
                .status_callback_method("POST")
                .transcription_engine("google"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("name=\"full_transcription\""));
    assert!(xml.contains("track=\"both\""));
    assert!(xml.contains("languageCode=\"en-US\""));
    assert!(xml.contains("enableAutomaticPunctuation=\"true\""));
    assert!(xml.contains("hints=\"Twilio,API\""));
    assert!(xml.contains("inboundTrackLabel=\"customer\""));
    assert!(xml.contains("outboundTrackLabel=\"agent\""));
    assert!(xml.contains("partialResults=\"true\""));
    assert!(xml.contains("profanityFilter=\"false\""));
    assert!(xml.contains("speechModel=\"phone_call\""));
    assert!(xml.contains("statusCallbackUrl=\"https://example.com/transcription\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("transcriptionEngine=\"google\""));
}

// Tests for new Conference attributes
#[test]
fn test_conference_with_call_sid_to_coach() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_conference(
            DialConference::new("MyRoom")
                .call_sid_to_coach("CA1234567890abcdef1234567890abcdef")
                .coaching(true),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<Conference"));
    assert!(xml.contains("callSidToCoach=\"CA1234567890abcdef1234567890abcdef\""));
    assert!(xml.contains("coaching=\"true\""));
}

#[test]
fn test_conference_with_beep_on_customer_entrance() {
    let response =
        VoiceResponse::new()
            .dial_with(Dial::new().add_conference(
                DialConference::new("SupportRoom").beep_on_customer_entrance(true),
            ));

    let xml = response.to_xml();
    assert!(xml.contains("beepOnCustomerEntrance=\"true\""));
}

#[test]
fn test_conference_with_all_new_attributes() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_conference(
            DialConference::new("FullFeaturedRoom")
                .muted(false)
                .beep("true")
                .start_conference_on_enter(true)
                .end_conference_on_exit(false)
                .wait_url("https://example.com/wait")
                .wait_method("GET")
                .max_participants(10)
                .record("record-from-start")
                .region("us1")
                .coach("CA1234567890abcdef1234567890abcdef")
                .trim("trim-silence")
                .status_callback_event(vec!["start".to_string(), "end".to_string()])
                .status_callback("https://example.com/status")
                .status_callback_method("POST")
                .recording_status_callback("https://example.com/recording")
                .recording_status_callback_method("POST")
                .recording_status_callback_event(vec!["completed".to_string()])
                .event_callback_url("https://example.com/events")
                .jitter_buffer_size("large")
                .participant_label("Agent")
                .call_sid_to_coach("CA9876543210fedcba9876543210fedcba")
                .beep_on_customer_entrance(true)
                .coaching(true),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("muted=\"false\""));
    assert!(xml.contains("beep=\"true\""));
    assert!(xml.contains("startConferenceOnEnter=\"true\""));
    assert!(xml.contains("endConferenceOnExit=\"false\""));
    assert!(xml.contains("waitUrl=\"https://example.com/wait\""));
    assert!(xml.contains("waitMethod=\"GET\""));
    assert!(xml.contains("maxParticipants=\"10\""));
    assert!(xml.contains("record=\"record-from-start\""));
    assert!(xml.contains("region=\"us1\""));
    assert!(xml.contains("coach=\"CA1234567890abcdef1234567890abcdef\""));
    assert!(xml.contains("trim=\"trim-silence\""));
    assert!(xml.contains("statusCallbackEvent=\"start end\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("recordingStatusCallback=\"https://example.com/recording\""));
    assert!(xml.contains("recordingStatusCallbackMethod=\"POST\""));
    assert!(xml.contains("recordingStatusCallbackEvent=\"completed\""));
    assert!(xml.contains("eventCallbackUrl=\"https://example.com/events\""));
    assert!(xml.contains("jitterBufferSize=\"large\""));
    assert!(xml.contains("participantLabel=\"Agent\""));
    assert!(xml.contains("callSidToCoach=\"CA9876543210fedcba9876543210fedcba\""));
    assert!(xml.contains("beepOnCustomerEntrance=\"true\""));
    assert!(xml.contains("coaching=\"true\""));
}

// Tests for new SIP attributes
#[test]
fn test_sip_with_codecs() {
    let response = VoiceResponse::new().dial_with(Dial::new().add_sip(
        DialSip::new("sip:alice@example.com").codecs(vec!["PCMU".to_string(), "PCMA".to_string()]),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("<Sip"));
    assert!(xml.contains("codecs=\"PCMU,PCMA\""));
    assert!(xml.contains("sip:alice@example.com"));
}

#[test]
fn test_sip_with_add_codec() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_sip(
            DialSip::new("sip:bob@example.com")
                .add_codec("PCMU")
                .add_codec("PCMA")
                .add_codec("G722"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("codecs=\"PCMU,PCMA,G722\""));
}

#[test]
fn test_sip_with_custom_headers() {
    let response = VoiceResponse::new().dial_with(Dial::new().add_sip(
        DialSip::new("sip:charlie@example.com").custom_headers(vec![
            ("X-Custom-Header".to_string(), "value1".to_string()),
            ("X-Another-Header".to_string(), "value2".to_string()),
        ]),
    ));

    let xml = response.to_xml();
    assert!(xml.contains("<SipHeader name=\"X-Custom-Header\" value=\"value1\"/>"));
    assert!(xml.contains("<SipHeader name=\"X-Another-Header\" value=\"value2\"/>"));
}

#[test]
fn test_sip_with_add_custom_header() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_sip(
            DialSip::new("sip:dave@example.com")
                .add_custom_header("X-First", "first-value")
                .add_custom_header("X-Second", "second-value"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<SipHeader name=\"X-First\" value=\"first-value\"/>"));
    assert!(xml.contains("<SipHeader name=\"X-Second\" value=\"second-value\"/>"));
}

#[test]
fn test_sip_with_all_new_attributes() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_sip(
            DialSip::new("sip:eve@example.com")
                .url("https://example.com/sip")
                .method("POST")
                .username("user123")
                .password("pass456")
                .status_callback_event(vec!["initiated".to_string(), "ringing".to_string()])
                .status_callback("https://example.com/status")
                .status_callback_method("POST")
                .codecs(vec![
                    "PCMU".to_string(),
                    "PCMA".to_string(),
                    "G722".to_string(),
                ])
                .custom_headers(vec![
                    ("X-Account-ID".to_string(), "12345".to_string()),
                    ("X-Session-ID".to_string(), "abc-def-ghi".to_string()),
                ]),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("url=\"https://example.com/sip\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("username=\"user123\""));
    assert!(xml.contains("password=\"pass456\""));
    assert!(xml.contains("statusCallbackEvent=\"initiated ringing\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("codecs=\"PCMU,PCMA,G722\""));
    assert!(xml.contains("<SipHeader name=\"X-Account-ID\" value=\"12345\"/>"));
    assert!(xml.contains("<SipHeader name=\"X-Session-ID\" value=\"abc-def-ghi\"/>"));
}

#[test]
fn test_sip_with_codecs_and_headers_combined() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_sip(
            DialSip::new("sip:frank@example.com")
                .add_codec("PCMU")
                .add_custom_header("X-Priority", "high")
                .add_codec("PCMA")
                .add_custom_header("X-Region", "us-east"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("codecs=\"PCMU,PCMA\""));
    assert!(xml.contains("<SipHeader name=\"X-Priority\" value=\"high\"/>"));
    assert!(xml.contains("<SipHeader name=\"X-Region\" value=\"us-east\"/>"));
}

// Tests for Dial callReason attribute
#[test]
fn test_dial_with_call_reason() {
    let response = VoiceResponse::new().dial_with(
        Dial::new()
            .call_reason("Appointment Reminder")
            .add_number(DialNumber::new("+15551234567")),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<Dial"));
    assert!(xml.contains("callReason=\"Appointment Reminder\""));
}

// Tests for Number attributes
#[test]
fn test_number_with_call_reason() {
    let response = VoiceResponse::new().dial_with(
        Dial::new()
            .add_number(DialNumber::new("+15551234567").call_reason("Customer Support Call")),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<Number"));
    assert!(xml.contains("callReason=\"Customer Support Call\""));
}

#[test]
fn test_number_with_machine_detection() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_number(
            DialNumber::new("+15551234567")
                .machine_detection("Enable")
                .machine_detection_timeout(5000)
                .machine_detection_speech_threshold(2400)
                .machine_detection_speech_end_threshold(1200)
                .machine_detection_silence_timeout(5000)
                .amd_status_callback("https://example.com/amd")
                .amd_status_callback_method("POST"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("machineDetection=\"Enable\""));
    assert!(xml.contains("machineDetectionTimeout=\"5000\""));
    assert!(xml.contains("machineDetectionSpeechThreshold=\"2400\""));
    assert!(xml.contains("machineDetectionSpeechEndThreshold=\"1200\""));
    assert!(xml.contains("machineDetectionSilenceTimeout=\"5000\""));
    assert!(xml.contains("amdStatusCallback=\"https://example.com/amd\""));
    assert!(xml.contains("amdStatusCallbackMethod=\"POST\""));
}

#[test]
fn test_number_with_all_attributes() {
    let response = VoiceResponse::new().dial_with(
        Dial::new().add_number(
            DialNumber::new("+15551234567")
                .send_digits("1234")
                .url("https://example.com/number")
                .method("POST")
                .status_callback("https://example.com/status")
                .status_callback_event(vec!["initiated".to_string(), "answered".to_string()])
                .status_callback_method("POST")
                .call_reason("Sales Call")
                .byoc("byoc-trunk-sid")
                .machine_detection("DetectMessageEnd"),
        ),
    );

    let xml = response.to_xml();
    assert!(xml.contains("sendDigits=\"1234\""));
    assert!(xml.contains("url=\"https://example.com/number\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("statusCallbackEvent=\"initiated answered\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("callReason=\"Sales Call\""));
    assert!(xml.contains("byoc=\"byoc-trunk-sid\""));
    assert!(xml.contains("machineDetection=\"DetectMessageEnd\""));
}

// Tests for Gather attributes
#[test]
fn test_gather_with_speech_attributes() {
    let response = VoiceResponse::new().gather_with(
        Gather::new()
            .input(vec!["speech".to_string()])
            .language("en-US")
            .hints("Twilio,TwiML")
            .enhanced(true)
            .speech_model("phone_call")
            .speech_timeout("auto")
            .profanity_filter(true),
    );

    let xml = response.to_xml();
    assert!(xml.contains("<Gather"));
    assert!(xml.contains("input=\"speech\""));
    assert!(xml.contains("language=\"en-US\""));
    assert!(xml.contains("hints=\"Twilio,TwiML\""));
    assert!(xml.contains("enhanced=\"true\""));
    assert!(xml.contains("speechModel=\"phone_call\""));
    assert!(xml.contains("speechTimeout=\"auto\""));
    assert!(xml.contains("profanityFilter=\"true\""));
}

#[test]
fn test_gather_with_partial_results() {
    let response = VoiceResponse::new().gather_with(
        Gather::new()
            .input(vec!["speech".to_string()])
            .partial_result_callback("https://example.com/partial")
            .partial_result_callback_method("POST"),
    );

    let xml = response.to_xml();
    assert!(xml.contains("partialResultCallback=\"https://example.com/partial\""));
    assert!(xml.contains("partialResultCallbackMethod=\"POST\""));
}

#[test]
fn test_gather_with_all_new_attributes() {
    let response = VoiceResponse::new().gather_with(
        Gather::new()
            .input(vec!["dtmf".to_string(), "speech".to_string()])
            .action("https://example.com/gather")
            .method("POST")
            .timeout(5)
            .finish_on_key("#")
            .num_digits(4)
            .language("en-US")
            .hints("account,balance,transfer")
            .action_on_empty_result(true)
            .barge_in(false)
            .debug(true)
            .enhanced(true)
            .max_speech_time(60)
            .partial_result_callback("https://example.com/partial")
            .partial_result_callback_method("POST")
            .profanity_filter(false)
            .speech_model("numbers_and_commands")
            .speech_timeout("5"),
    );

    let xml = response.to_xml();
    assert!(xml.contains("input=\"dtmf speech\""));
    assert!(xml.contains("action=\"https://example.com/gather\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("timeout=\"5\""));
    assert!(xml.contains("finishOnKey=\"#\""));
    assert!(xml.contains("numDigits=\"4\""));
    assert!(xml.contains("language=\"en-US\""));
    assert!(xml.contains("hints=\"account,balance,transfer\""));
    assert!(xml.contains("actionOnEmptyResult=\"true\""));
    assert!(xml.contains("bargeIn=\"false\""));
    assert!(xml.contains("debug=\"true\""));
    assert!(xml.contains("enhanced=\"true\""));
    assert!(xml.contains("maxSpeechTime=\"60\""));
    assert!(xml.contains("partialResultCallback=\"https://example.com/partial\""));
    assert!(xml.contains("partialResultCallbackMethod=\"POST\""));
    assert!(xml.contains("profanityFilter=\"false\""));
    assert!(xml.contains("speechModel=\"numbers_and_commands\""));
    assert!(xml.contains("speechTimeout=\"5\""));
}

#[test]
fn test_connect_with_conversation_relay_basic() {
    let relay = ConversationRelay::new("https://example.com/relay");
    let connect = Connect::new().add_conversation_relay(relay);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelay"));
    assert!(xml.contains("url=\"https://example.com/relay\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_connect_with_conversation_relay_full_attributes() {
    let relay = ConversationRelay::new("https://example.com/relay")
        .welcome_greeting("Hello! Welcome to our service.")
        .voice("Polly.Joanna")
        .language("en-US")
        .dtmf_detection(true)
        .interruptible(true)
        .interruption_sensitivity("high")
        .speech_model("phone_call")
        .profanity_filter(true)
        .transcription_enabled(true)
        .status_callback("https://example.com/status")
        .status_callback_method("POST")
        .max_duration(3600);

    let connect = Connect::new().add_conversation_relay(relay);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelay"));
    assert!(xml.contains("url=\"https://example.com/relay\""));
    assert!(xml.contains("welcomeGreeting=\"Hello! Welcome to our service.\""));
    assert!(xml.contains("voice=\"Polly.Joanna\""));
    assert!(xml.contains("language=\"en-US\""));
    assert!(xml.contains("dtmfDetection=\"true\""));
    assert!(xml.contains("interruptible=\"true\""));
    assert!(xml.contains("interruptionSensitivity=\"high\""));
    assert!(xml.contains("speechModel=\"phone_call\""));
    assert!(xml.contains("profanityFilter=\"true\""));
    assert!(xml.contains("transcriptionEnabled=\"true\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("maxDuration=\"3600\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_connect_with_conversation_relay_with_languages() {
    let lang1 = Language::new("en-US")
        .tts_provider("google")
        .stt_provider("google");
    let lang2 = Language::new("es-ES").tts_provider("amazon-polly");

    let relay = ConversationRelay::new("https://example.com/relay")
        .add_language(lang1)
        .add_language(lang2);

    let connect = Connect::new().add_conversation_relay(relay);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelay"));
    assert!(xml.contains("url=\"https://example.com/relay\""));
    assert!(xml.contains("<Language"));
    assert!(xml.contains("code=\"en-US\""));
    assert!(xml.contains("ttsProvider=\"google\""));
    assert!(xml.contains("sttProvider=\"google\""));
    assert!(xml.contains("code=\"es-ES\""));
    assert!(xml.contains("ttsProvider=\"amazon-polly\""));
    assert!(xml.contains("</ConversationRelay>"));
}

#[test]
fn test_connect_with_conversation_relay_with_parameters() {
    let param1 = Parameter::new().name("custom_field").value("custom_value");
    let param2 = Parameter::new().name("user_id").value("12345");

    let relay = ConversationRelay::new("https://example.com/relay")
        .add_parameter(param1)
        .add_parameter(param2);

    let connect = Connect::new().add_conversation_relay(relay);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelay"));
    assert!(xml.contains("url=\"https://example.com/relay\""));
    assert!(xml.contains("<Parameter"));
    assert!(xml.contains("name=\"custom_field\""));
    assert!(xml.contains("value=\"custom_value\""));
    assert!(xml.contains("name=\"user_id\""));
    assert!(xml.contains("value=\"12345\""));
    assert!(xml.contains("</ConversationRelay>"));
}

#[test]
fn test_connect_with_conversation_relay_complete() {
    let lang = Language::new("en-US").tts_provider("google");
    let param = Parameter::new().name("session_id").value("abc123");

    let relay = ConversationRelay::new("https://example.com/relay")
        .welcome_greeting("Welcome!")
        .voice("Polly.Joanna")
        .language("en-US")
        .dtmf_detection(true)
        .interruptible(true)
        .add_language(lang)
        .add_parameter(param);

    let connect = Connect::new().add_conversation_relay(relay);
    let response = VoiceResponse::new().connect(connect);
    let xml = response.to_xml();

    assert!(xml.contains("<ConversationRelay"));
    assert!(xml.contains("url=\"https://example.com/relay\""));
    assert!(xml.contains("welcomeGreeting=\"Welcome!\""));
    assert!(xml.contains("voice=\"Polly.Joanna\""));
    assert!(xml.contains("language=\"en-US\""));
    assert!(xml.contains("dtmfDetection=\"true\""));
    assert!(xml.contains("interruptible=\"true\""));
    assert!(xml.contains("<Language"));
    assert!(xml.contains("code=\"en-US\""));
    assert!(xml.contains("ttsProvider=\"google\""));
    assert!(xml.contains("<Parameter"));
    assert!(xml.contains("name=\"session_id\""));
    assert!(xml.contains("value=\"abc123\""));
    assert!(xml.contains("</ConversationRelay>"));
}

#[test]
fn test_dial_application_with_customer_id() {
    let app = DialApplication::new("APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").customer_id("CUST123");
    let dial = Dial::new().add_application(app);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Application"));
    assert!(xml.contains("sid=\"APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\""));
    assert!(xml.contains("customerId=\"CUST123\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_dial_application_with_copy_parent_to() {
    let app = DialApplication::new("APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
        .copy_parent_to("parent_call_sid");
    let dial = Dial::new().add_application(app);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Application"));
    assert!(xml.contains("sid=\"APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\""));
    assert!(xml.contains("copyParentTo=\"parent_call_sid\""));
    assert!(xml.contains(" />"));
}

#[test]
fn test_dial_application_with_parameters() {
    let param = Parameter::new().name("user_id").value("12345");
    let app = DialApplication::new("APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").add_parameter(param);
    let dial = Dial::new().add_application(app);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Application"));
    assert!(xml.contains("sid=\"APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\""));
    assert!(xml.contains("<Parameter"));
    assert!(xml.contains("name=\"user_id\""));
    assert!(xml.contains("value=\"12345\""));
    assert!(xml.contains("</Application>"));
}

#[test]
fn test_dial_application_complete() {
    let param = Parameter::new().name("session_id").value("abc123");
    let app = DialApplication::new("APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
        .customer_id("CUST123")
        .copy_parent_to("parent_call_sid")
        .add_parameter(param);
    let dial = Dial::new().add_application(app);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Application"));
    assert!(xml.contains("sid=\"APxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\""));
    assert!(xml.contains("customerId=\"CUST123\""));
    assert!(xml.contains("copyParentTo=\"parent_call_sid\""));
    assert!(xml.contains("<Parameter"));
    assert!(xml.contains("name=\"session_id\""));
    assert!(xml.contains("value=\"abc123\""));
    assert!(xml.contains("</Application>"));
}

#[test]
fn test_dial_client_with_client_notification_url() {
    let client = DialClient::new("alice").client_notification_url("https://example.com/notify");
    let dial = Dial::new().add_client(client);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Client"));
    assert!(xml.contains("clientNotificationUrl=\"https://example.com/notify\""));
    assert!(xml.contains(">alice</Client>"));
}

#[test]
fn test_dial_client_with_all_attributes() {
    let client = DialClient::new("alice")
        .url("https://example.com/client")
        .method("POST")
        .status_callback_event(vec!["initiated".to_string(), "ringing".to_string()])
        .status_callback("https://example.com/status")
        .status_callback_method("POST")
        .client_notification_url("https://example.com/notify");
    let dial = Dial::new().add_client(client);
    let response = VoiceResponse::new().dial_with(dial);
    let xml = response.to_xml();

    assert!(xml.contains("<Client"));
    assert!(xml.contains("url=\"https://example.com/client\""));
    assert!(xml.contains("method=\"POST\""));
    assert!(xml.contains("statusCallbackEvent=\"initiated ringing\""));
    assert!(xml.contains("statusCallback=\"https://example.com/status\""));
    assert!(xml.contains("statusCallbackMethod=\"POST\""));
    assert!(xml.contains("clientNotificationUrl=\"https://example.com/notify\""));
    assert!(xml.contains(">alice</Client>"));
}

#[test]
fn test_gather_with_dtmf_detection_disabled() {
    let gather = Gather::new()
        .input(vec!["speech".to_string()])
        .dtmf_detection(false);
    let response = VoiceResponse::new().gather_with(gather);
    let xml = response.to_xml();

    assert!(xml.contains("<Gather"));
    assert!(xml.contains("input=\"speech\""));
    assert!(xml.contains("dtmfDetection=\"false\""));
}

#[test]
fn test_gather_with_dtmf_detection_enabled() {
    let gather = Gather::new()
        .input(vec!["speech".to_string(), "dtmf".to_string()])
        .dtmf_detection(true);
    let response = VoiceResponse::new().gather_with(gather);
    let xml = response.to_xml();

    assert!(xml.contains("<Gather"));
    assert!(xml.contains("input=\"speech dtmf\""));
    assert!(xml.contains("dtmfDetection=\"true\""));
}
