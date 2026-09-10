#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AdmissionJudgePacket {
    pub admission_judge_operation: AdmissionJudgeOperation,
    pub record_set: signal_spirit::RecordSet,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AdmissionJudgeOperation {
    Record(signal_spirit::RecordRequest),
    Propose(signal_spirit::RecordRequest),
    Clarify(signal_spirit::ClarificationRequest),
    ResolveClarification(signal_spirit::ClarificationResolution),
    Supersede(signal_spirit::Supersession),
    Retire(signal_spirit::Retirement),
    ChangeRecord(signal_spirit::RecordChange),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AdmissionJudgeResponse {
    pub admission_judge_verdict: AdmissionJudgeVerdict,
    pub judge_diagnostic: JudgeDiagnostic,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AdmissionJudgeVerdict {
    Accept,
    Reject(AdmissionRejectionReason),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum AdmissionRejectionReason {
    Duplicate,
    Contradiction,
    Compound,
    NonIntent,
    NegativeGuideline,
    Matter,
    UnclearDomain,
    ClarifyTramples,
    ClarifyLosesMeaning,
    SupersedeTargetMissing,
    RetrievalInsufficient,
    MissingTestimony,
    TestimonyFabricated,
    InsufficientWarrant,
    ImportanceUnsupported,
    JudgeUnavailable,
    JudgeMalformed,
    JudgeTimedOut,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct JudgeDiagnostic {
    pub redacted_text: RedactedText,
    pub content_hashes: ContentHashes,
}
pub type ContentHashes = std::vec::Vec<ContentHash>;
pub type ContentHash = String;
pub type RedactedText = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SpiritJudgeRequestRejection {
    pub spirit_judge_request_rejection_reason: SpiritJudgeRequestRejectionReason,
    pub judge_diagnostic: JudgeDiagnostic,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum SpiritJudgeRequestRejectionReason {
    InvalidRequest,
    ConfigurationUnavailable,
    ProviderUnavailable,
    ProviderRejected,
    ResponseFormatFailure,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    JudgeAdmission(AdmissionJudgePacket),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    AdmissionJudged(AdmissionJudgeResponse),
    RequestRejected(SpiritJudgeRequestRejection),
}
