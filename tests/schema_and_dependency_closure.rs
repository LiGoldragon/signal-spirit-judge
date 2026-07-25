use schema_language::{SchemaEngine, SchemaIdentity, SchemaSource, TypeDeclaration, TypeReference};
use std::process::Command;

const NOTA_REVISION: &str = "89dc3c85a9ff96d4e4d53accfd867df672cae5a8";
const SCHEMA_LANGUAGE_REVISION: &str = "9c217610c4b8d3bdaa9f95542e28c04424a593e3";
const SCHEMA_RUST_REVISION: &str = "3721656b0a654d47d9abde31f14d89d01f9305cf";
const SIGNAL_DOMAIN_REVISION: &str = "f5e79ffd1f6985cb12925ddd43addb66d6755b54";
const SIGNAL_FRAME_REVISION: &str = "e27bbb5752f133589ba3200c3aace1b350c5123e";
const SIGNAL_SPIRIT_REVISION: &str = "d2589210c87bcae08386c4b645fc1d7eb49a139c";
const VERSION_PROJECTION_REVISION: &str = "485be1c609e5f2038fdf54ed0de04cd29d884b06";

#[test]
fn authored_schema_is_the_complete_public_wire_inventory() {
    assert_exact_inventory(&authored_schema());
}

#[test]
fn schema_inventory_rejects_semantic_mutations() {
    for (name, from, to) in [
        (
            "wrong input root payload",
            "JudgeAdmission.AdmissionJudgePacket",
            "JudgeAdmission.ReferentRegistrationJudgePacket",
        ),
        ("missing operation variant", "    Retire.Retirement\n", ""),
        (
            "changed operation variant payload",
            "    Record.RecordRequest",
            "    Record.Proposal",
        ),
        (
            "changed record field type",
            "Vector.ContentHash",
            "Vector.RedactedText",
        ),
        (
            "changed record field order",
            "JudgeDiagnostic.{ RedactedText Vector.ContentHash }",
            "JudgeDiagnostic.{ Vector.ContentHash RedactedText }",
        ),
        (
            "extra parallel declaration",
            "  ContentHash.String",
            "  ContentHashes.Vector.ContentHash\n  ContentHash.String",
        ),
    ] {
        let mutated = signal_spirit_judge::SIGNAL_SCHEMA_SOURCE.replace(from, to);
        let source = SchemaSource::from_schema_text(&mutated)
            .unwrap_or_else(|error| panic!("{name} must remain parseable: {error}"));
        assert!(
            std::panic::catch_unwind(|| assert_exact_inventory(&source)).is_err(),
            "inventory must reject {name}"
        );
    }
}

#[test]
fn modern_vector_field_lowers_to_the_existing_public_vec_identity() {
    let source = SchemaSource::from_schema_text(
        "{}\n[]\n[]\n{\n  ContentHash.String\n  RedactedText.String\n  JudgeDiagnostic.{ RedactedText Vector.ContentHash }\n}\n{}\n{}",
    )
    .expect("parse direct vector field witness");
    let schema = source
        .lower(
            &SchemaEngine::default(),
            SchemaIdentity::new("signal-spirit-judge:wire-witness", "0.1.0"),
        )
        .expect("lower direct vector field witness");
    let TypeDeclaration::Struct(diagnostic) = schema
        .type_named("JudgeDiagnostic")
        .expect("JudgeDiagnostic declaration")
    else {
        panic!("JudgeDiagnostic must lower to a record");
    };
    let fields = diagnostic.fields.entries();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name.as_str(), "redacted_text");
    assert_eq!(fields[0].reference, TypeReference::new("RedactedText"));
    assert_eq!(fields[1].name.as_str(), "content_hash_vector");
    assert_eq!(
        fields[1].reference,
        TypeReference::vector(TypeReference::new("ContentHash"))
    );
    assert!(schema.type_named("ContentHashes").is_none());

    let diagnostic = signal_spirit_judge::JudgeDiagnostic::new(
        signal_spirit_judge::RedactedText::new("redacted").expect("non-empty redaction"),
        vec![signal_spirit_judge::ContentHash::new("hash").expect("non-empty hash")],
    );
    let _: Vec<signal_spirit_judge::ContentHash> = diagnostic.content_hashes.clone();
}

#[test]
fn canonical_cargo_closure_has_no_duplicate_packages() {
    let output = Command::new(env!("CARGO"))
        .args(["tree", "--locked", "--duplicates", "--target", "all"])
        .output()
        .expect("run Cargo duplicate-package gate");

    assert!(
        output.status.success(),
        "Cargo duplicate-package gate failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).trim().is_empty(),
        "canonical closure has duplicate packages: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

fn authored_schema() -> SchemaSource {
    SchemaSource::from_schema_text(signal_spirit_judge::SIGNAL_SCHEMA_SOURCE)
        .expect("decode authored judge schema")
}

fn assert_exact_inventory(actual: &SchemaSource) {
    assert_eq!(actual, &expected_schema());
}

fn expected_schema() -> SchemaSource {
    SchemaSource::from_schema_text(
        r#"
{
  signal-spirit.signal.[
    RecordRequest
    Proposal
    Clarification
    ClarificationResolution
    Supersession
    Retirement
    RecordChange
    RecordSet
    ReferentRegistration
    RegisteredReferents
    DatabaseMarker
  ]
}
[
  JudgeAdmission.AdmissionJudgePacket
  JudgeReferentRegistration.ReferentRegistrationJudgePacket
]
[
  AdmissionJudged.AdmissionJudgeResponse
  ReferentRegistrationJudged.ReferentRegistrationJudgeResponse
  RequestRejected.SpiritJudgeRequestRejection
]
{
  AdmissionJudgePacket.{ JudgmentScope AdmissionJudgeOperation RecordSet DatabaseMarker }
  ReferentRegistrationJudgePacket.{ JudgmentScope ReferentRegistration RegisteredReferents DatabaseMarker }
  JudgmentScope.[Public Private.PrivateJudgmentScope]
  PrivateJudgmentScope.PrivateDiagnosticPolicy
  PrivateDiagnosticPolicy.[HashesAndRedaction]
  AdmissionJudgeOperation.[
    Record.RecordRequest
    Propose.Proposal
    Clarify.Clarification
    ResolveClarification.ClarificationResolution
    Supersede.Supersession
    Retire.Retirement
    ChangeRecord.RecordChange
  ]

  AdmissionJudgeResponse.{ AdmissionJudgeVerdict JudgeDiagnostic }
  ReferentRegistrationJudgeResponse.{ ReferentRegistrationJudgeVerdict JudgeDiagnostic }
  AdmissionJudgeVerdict.[Accept Reject.AdmissionRejectionReason]
  ReferentRegistrationJudgeVerdict.[
    Accept
    RejectReferent.ReferentRegistrationRejectionReason
  ]
  AdmissionRejectionReason.[Duplicate Contradiction Compound NonIntent NegativeGuideline Matter UnclearPrivacy UnclearDomain ClarifyTramples ClarifyLosesMeaning SupersedeTargetMissing RetrievalInsufficient MissingTestimony TestimonyFabricated InsufficientWarrant Overstated ImportanceUnsupported JudgeUnavailable JudgeMalformed JudgeTimedOut]
  ReferentRegistrationRejectionReason.[Duplicate Ambiguous TooVague AliasCollision NonReferent UnclearJustification JudgeUnavailable JudgeMalformed JudgeTimedOut]
  JudgeDiagnostic.{ RedactedText Vector.ContentHash }
  SpiritJudgeRequestRejection.{ SpiritJudgeRequestRejectionReason JudgeDiagnostic }
  SpiritJudgeRequestRejectionReason.[
    InvalidRequest
    ConfigurationUnavailable
    ProviderUnavailable
    ProviderRejected
    ResponseFormatFailure
  ]
  ContentHash.String
  RedactedText.String
}
{}
{}
"#,
    )
    .expect("decode expected complete schema inventory")
}

#[test]
fn manifest_and_lock_close_on_the_canonical_micro_repository_family() {
    let manifest = include_str!("../Cargo.toml");
    let lock = include_str!("../Cargo.lock");

    for forbidden in [
        "branch =",
        "[patch.",
        "nota-next",
        "1cf7c010029de46369b742687da4fa1ca6def9a9",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "manifest retains forbidden mixed-closure evidence: {forbidden}"
        );
        assert!(
            !lock.contains(forbidden),
            "lock retains forbidden mixed-closure evidence: {forbidden}"
        );
    }

    for (repository, revision) in [
        ("nota.git", NOTA_REVISION),
        ("schema-language.git", SCHEMA_LANGUAGE_REVISION),
        ("schema-rust.git", SCHEMA_RUST_REVISION),
        ("signal-domain.git", SIGNAL_DOMAIN_REVISION),
        ("signal-frame.git", SIGNAL_FRAME_REVISION),
        ("signal-spirit.git", SIGNAL_SPIRIT_REVISION),
        ("version-projection.git", VERSION_PROJECTION_REVISION),
    ] {
        let exact_source = format!("{repository}?rev={revision}#{revision}");
        assert!(
            lock.contains(&exact_source),
            "lock does not contain exact canonical source {exact_source}"
        );
    }
}
