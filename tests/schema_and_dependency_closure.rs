use schema_language::{SchemaSource, SourceDeclarationValue};

const NOTA_REVISION: &str = "89dc3c85a9ff96d4e4d53accfd867df672cae5a8";
const SCHEMA_LANGUAGE_REVISION: &str = "9c217610c4b8d3bdaa9f95542e28c04424a593e3";
const SCHEMA_RUST_REVISION: &str = "3721656b0a654d47d9abde31f14d89d01f9305cf";
const SIGNAL_DOMAIN_REVISION: &str = "f5e79ffd1f6985cb12925ddd43addb66d6755b54";
const SIGNAL_FRAME_REVISION: &str = "e27bbb5752f133589ba3200c3aace1b350c5123e";
const SIGNAL_SPIRIT_REVISION: &str = "d2589210c87bcae08386c4b645fc1d7eb49a139c";
const VERSION_PROJECTION_REVISION: &str = "485be1c609e5f2038fdf54ed0de04cd29d884b06";

#[test]
fn authored_schema_closes_the_public_wire_inventory() {
    let source = SchemaSource::from_schema_text(signal_spirit_judge::SIGNAL_SCHEMA_SOURCE)
        .expect("decode authored judge schema");
    let input = source
        .input()
        .body()
        .as_enum()
        .expect("Input root remains an enum");
    let output = source
        .output()
        .body()
        .as_enum()
        .expect("Output root remains an enum");

    assert_eq!(
        input
            .variants()
            .iter()
            .map(|variant| variant.name().as_str())
            .collect::<Vec<_>>(),
        ["JudgeAdmission", "JudgeReferentRegistration"]
    );
    assert_eq!(
        output
            .variants()
            .iter()
            .map(|variant| variant.name().as_str())
            .collect::<Vec<_>>(),
        [
            "AdmissionJudged",
            "ReferentRegistrationJudged",
            "RequestRejected",
        ]
    );

    for declaration in [
        "AdmissionJudgeOperation",
        "AdmissionJudgeVerdict",
        "ReferentRegistrationJudgeVerdict",
        "SpiritJudgeRequestRejectionReason",
    ] {
        let entry = source
            .types()
            .entries()
            .iter()
            .find(|entry| entry.name().as_str() == declaration)
            .unwrap_or_else(|| panic!("{declaration} declaration remains present"));
        assert!(
            matches!(entry.value(), SourceDeclarationValue::Enum(_)),
            "{declaration} remains an enum"
        );
    }
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
