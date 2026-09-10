use signal_spirit::{
    DatabaseMarker, Entry, Justification, Kind, Magnitude, RecordRequest, RecordSet,
};
use signal_spirit_judge::{
    AdmissionJudgeOperation, AdmissionJudgePacket, AdmissionJudgeResponse, AdmissionJudgeVerdict,
    AdmissionRejectionReason, ByteViewable, JudgeDiagnostic, Query, Response, Restorable, Signal,
    Signalizable, SpiritJudgeRequestRejection, SpiritJudgeRequestRejectionReason,
};

fn record_request() -> RecordRequest {
    RecordRequest {
        entry: Entry {
            domains: vec![],
            kind: Kind::Principle,
            description: "Contracts carry typed data.".into(),
            importance: Magnitude::Medium,
        },
        justification: Justification::Reasoning("guardian witness".into()),
    }
}

fn diagnostic() -> JudgeDiagnostic {
    JudgeDiagnostic {
        redacted_text: "sensitive details redacted".into(),
        content_hashes: vec!["sha256:fixture-content-hash".into()],
    }
}

fn admission() -> Query {
    Query::JudgeAdmission(AdmissionJudgePacket {
        admission_judge_operation: AdmissionJudgeOperation::Record(record_request()),
        record_set: RecordSet::new(),
        database_marker: DatabaseMarker {
            commit_sequence: 1,
            state_digest: 99,
        },
    })
}

#[test]
fn guardian_admission_round_trips_through_fresh_received_signal_bytes() {
    let query = admission();
    let received = Signal::<Query>::from(
        query
            .signalize()
            .expect("signalize admission")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore admission"), query);
}

#[test]
fn guardian_verdict_and_rejection_round_trip_through_fresh_received_signal_bytes() {
    let verdict = Response::AdmissionJudged(AdmissionJudgeResponse {
        admission_judge_verdict: AdmissionJudgeVerdict::Reject(
            AdmissionRejectionReason::ImportanceUnsupported,
        ),
        judge_diagnostic: diagnostic(),
    });
    let received = Signal::<Response>::from(
        verdict
            .signalize()
            .expect("signalize verdict")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore verdict"), verdict);

    let rejection = Response::RequestRejected(SpiritJudgeRequestRejection {
        spirit_judge_request_rejection_reason: SpiritJudgeRequestRejectionReason::InvalidRequest,
        judge_diagnostic: diagnostic(),
    });
    let received = Signal::<Response>::from(
        rejection
            .signalize()
            .expect("signalize rejection")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore rejection"), rejection);
}

#[test]
fn malformed_guardian_signal_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn guardian_admission_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = admission();
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(text);
    let decoded = pending
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("actualize guardian admission");
    assert_eq!(decoded, query);
}
