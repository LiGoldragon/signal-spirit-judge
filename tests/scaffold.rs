use signal_frame::{
    ExchangeFrameBody, ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, Request,
    ShortHeader, SubReply,
};
use signal_spirit::schema::signal::{
    Aliases, Certainty, CommitSequence, DatabaseMarker, Description, Domains, Entry, Importance,
    Justification, Kind, Magnitude, Privacy, Reasoning, RecordRequest, RecordSet, Referent,
    ReferentRegistration, Referents, RegisteredReferents, StateDigest, Testimony,
};
use signal_spirit_judge::{
    AdmissionJudgeOperation, AdmissionJudgePacket, AdmissionJudgeResponse, ContentHash,
    JudgeDiagnostic, JudgmentScope, RedactedText, ReferentRegistrationJudgePacket,
    ReferentRegistrationJudgeResponse, ReferentRegistrationJudgeVerdict,
    ReferentRegistrationRejectionReason, SpiritJudgeFrame, SpiritJudgeReply, SpiritJudgeRequest,
};

fn exchange_identifier() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(7),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn database_marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: CommitSequence::new(1),
        state_digest: StateDigest::new(99),
    }
}

fn redacted_diagnostic() -> JudgeDiagnostic {
    JudgeDiagnostic::new(
        RedactedText::new("private details redacted").unwrap(),
        vec![ContentHash::new("sha256:fixture-content-hash").unwrap()],
    )
}

fn justification() -> Justification {
    Justification {
        testimony: Testimony::new(Vec::new()),
        reasoning: Reasoning::new("workspace artifact name"),
    }
}

fn entry() -> Entry {
    Entry {
        domains: Domains::new(Vec::new()),
        kind: Kind::Principle,
        description: Description::new("Contracts carry typed data."),
        certainty: Certainty::new(Magnitude::High),
        importance: Importance::new(Magnitude::Medium),
        privacy: Privacy::new(Magnitude::Zero),
        referents: Referents::new(Vec::new()),
    }
}

fn record_request() -> RecordRequest {
    RecordRequest {
        entry: entry(),
        justification: justification(),
    }
}

fn referent_registration() -> ReferentRegistration {
    ReferentRegistration {
        referent: Referent::new("signal-spirit-judge"),
        aliases: Aliases::new(Referents::new(Vec::new())),
        justification: justification(),
    }
}

#[test]
fn private_scope_names_hashes_and_redaction_policy() {
    let scope = JudgmentScope::private_hashes_and_redaction();

    assert!(matches!(scope, JudgmentScope::Private(_)));
}

#[test]
fn admission_request_round_trips_through_binary_frame() {
    let request = SpiritJudgeRequest::JudgeAdmission(AdmissionJudgePacket::new(
        JudgmentScope::public(),
        AdmissionJudgeOperation::Record(record_request()),
        RecordSet::new(Vec::new()),
        database_marker(),
    ));
    let frame = SpiritJudgeFrame::with_short_header(
        ShortHeader::new(1),
        ExchangeFrameBody::Request {
            exchange: exchange_identifier(),
            request: Request::from_payload(request.clone()),
        },
    );

    let encoded = frame.encode_length_prefixed().unwrap();
    let decoded = SpiritJudgeFrame::decode_length_prefixed(&encoded).unwrap();

    assert_eq!(decoded.body(), frame.body());
}

#[test]
fn referent_registration_request_round_trips_through_binary_frame() {
    let request =
        SpiritJudgeRequest::JudgeReferentRegistration(ReferentRegistrationJudgePacket::new(
            JudgmentScope::private_hashes_and_redaction(),
            referent_registration(),
            RegisteredReferents::new(Vec::new()),
            database_marker(),
        ));
    let frame = SpiritJudgeFrame::with_short_header(
        ShortHeader::new(1),
        ExchangeFrameBody::Request {
            exchange: exchange_identifier(),
            request: Request::from_payload(request.clone()),
        },
    );

    let encoded = frame.encode_length_prefixed().unwrap();
    let decoded = SpiritJudgeFrame::decode_length_prefixed(&encoded).unwrap();

    assert_eq!(decoded.body(), frame.body());
}

#[test]
fn referent_registration_reply_round_trips_through_binary_frame() {
    let reply_payload =
        SpiritJudgeReply::ReferentRegistrationJudged(ReferentRegistrationJudgeResponse::new(
            ReferentRegistrationJudgeVerdict::RejectReferent(
                ReferentRegistrationRejectionReason::UnclearJustification,
            ),
            redacted_diagnostic(),
        ));
    let frame = SpiritJudgeFrame::with_short_header(
        ShortHeader::new(1),
        ExchangeFrameBody::Reply {
            exchange: exchange_identifier(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply_payload))),
        },
    );

    let encoded = frame.encode_length_prefixed().unwrap();
    let decoded = SpiritJudgeFrame::decode_length_prefixed(&encoded).unwrap();

    assert_eq!(decoded.body(), frame.body());
}

#[test]
fn admission_response_defaults_to_conservative_rejection() {
    let response = AdmissionJudgeResponse::conservative_rejection(redacted_diagnostic());

    assert!(matches!(
        response.verdict,
        signal_spirit_judge::AdmissionJudgeVerdict::Reject(
            signal_spirit_judge::AdmissionRejectionReason::JudgeUnavailable
        )
    ));
}

#[cfg(feature = "nota-text")]
#[test]
fn nota_projection_names_referent_response_shape() {
    use nota::NotaEncode;

    let reply =
        SpiritJudgeReply::ReferentRegistrationJudged(ReferentRegistrationJudgeResponse::new(
            ReferentRegistrationJudgeVerdict::Accept,
            JudgeDiagnostic::redacted(RedactedText::new("accepted").unwrap()),
        ));

    let text = reply.to_nota();

    assert!(text.contains("ReferentRegistrationJudged"));
    assert!(text.contains("Accept"));
    assert!(text.contains("accepted"));
}
