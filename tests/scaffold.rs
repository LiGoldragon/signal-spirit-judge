use signal_frame::{
    ExchangeFrameBody, ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, Request,
    ShortHeader, SubReply,
};
use signal_spirit::{
    CommitSequence, DatabaseMarker, Description, Domain, Domains, Entry, Importance, Justification,
    Kind, Magnitude, Reasoning, RecordRequest, RecordSet, StateDigest, Testimony,
};
use signal_spirit_judge::{
    AdmissionJudgeOperation, AdmissionJudgePacket, AdmissionJudgeResponse, AdmissionJudgeVerdict,
    AdmissionRejectionReason, ContentHash, JudgeDiagnostic, RedactedText, SpiritJudgeFrame,
    SpiritJudgeReply, SpiritJudgeRequest,
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
        RedactedText::new("sensitive details redacted").unwrap(),
        vec![ContentHash::new("sha256:fixture-content-hash").unwrap()],
    )
}

fn record_request() -> RecordRequest {
    RecordRequest {
        entry: Entry {
            domains: Domains::new(vec![Domain::All]),
            kind: Kind::Principle,
            description: Description::new("Contracts carry typed data."),
            importance: Importance::new(Magnitude::Medium),
        },
        justification: Justification {
            testimony: Testimony::new(Vec::new()),
            reasoning: Reasoning::new("workspace artifact name"),
        },
    }
}

fn admission_request() -> SpiritJudgeRequest {
    SpiritJudgeRequest::JudgeAdmission(AdmissionJudgePacket::new(
        AdmissionJudgeOperation::Record(record_request()),
        RecordSet::new(Vec::new()),
        database_marker(),
    ))
}

#[test]
fn admission_request_round_trips_through_binary_frame() {
    let request = admission_request();
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
fn admission_reply_round_trips_through_binary_frame() {
    let reply_payload = SpiritJudgeReply::AdmissionJudged(AdmissionJudgeResponse::new(
        AdmissionJudgeVerdict::Reject(AdmissionRejectionReason::ImportanceUnsupported),
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
        AdmissionJudgeVerdict::Reject(AdmissionRejectionReason::JudgeUnavailable)
    ));
    assert_eq!(response.diagnostic.content_hashes.len(), 1);
    assert_eq!(
        response.diagnostic.redacted_text.as_str(),
        "sensitive details redacted"
    );
}

#[test]
fn active_contract_excludes_revision_1_scope_and_registration_vocabulary() {
    let rust = include_str!("../src/lib.rs");
    for removed in [
        "JudgmentScope",
        "PrivateDiagnosticPolicy",
        "ReferentRegistration",
        "UnclearPrivacy",
        "Overstated",
    ] {
        assert!(!signal_spirit_judge::SIGNAL_SCHEMA_SOURCE.contains(removed));
        assert!(!rust.contains(removed));
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn revision_1_request_shapes_fail_to_decode() {
    use nota::NotaSource;

    for source in [
        "(JudgeReferentRegistration ignored)",
        "(JudgeAdmission (Public ignored [] (1 99)))",
    ] {
        assert!(
            NotaSource::new(source)
                .parse::<SpiritJudgeRequest>()
                .is_err()
        );
    }
}
