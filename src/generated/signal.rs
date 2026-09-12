#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdmissionJudgePacket {
    pub admission_judge_operation: AdmissionJudgeOperation,
    pub record_set: signal_spirit::RecordSet,
    pub database_marker: signal_spirit::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AdmissionJudgeOperation {
    Record(signal_spirit::RecordRequest),
    Propose(signal_spirit::RecordRequest),
    Clarify(signal_spirit::ClarificationRequest),
    ResolveClarification(signal_spirit::ClarificationResolution),
    Supersede(signal_spirit::Supersession),
    Retire(signal_spirit::Retirement),
    ChangeRecord(signal_spirit::RecordChange),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdmissionJudgeResponse {
    pub admission_judge_verdict: AdmissionJudgeVerdict,
    pub judge_diagnostic: JudgeDiagnostic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AdmissionJudgeVerdict {
    Accept,
    Reject(AdmissionRejectionReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct JudgeDiagnostic {
    pub redacted_text: RedactedText,
    pub content_hashes: ContentHashes,
}
#[rustfmt::skip]
pub type ContentHashes = std::vec::Vec<ContentHash>;
#[rustfmt::skip]
pub type ContentHash = String;
#[rustfmt::skip]
pub type RedactedText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SpiritJudgeRequestRejection {
    pub spirit_judge_request_rejection_reason: SpiritJudgeRequestRejectionReason,
    pub judge_diagnostic: JudgeDiagnostic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SpiritJudgeRequestRejectionReason {
    InvalidRequest,
    ConfigurationUnavailable,
    ProviderUnavailable,
    ProviderRejected,
    ResponseFormatFailure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    JudgeAdmission(AdmissionJudgePacket),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    AdmissionJudged(AdmissionJudgeResponse),
    RequestRejected(SpiritJudgeRequestRejection),
}
