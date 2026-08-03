//! Typed request and reply contract between `spirit` and the Spirit judge adapter.
//!
//! The binary wire is rkyv-backed through `signal-frame`. NOTA projection is an
//! edge feature for clients, tests, and tools; it is not the component-to-
//! component transport.

#![forbid(unsafe_code)]

use thiserror::Error;

pub use signal_spirit::schema::signal::{
    Clarification, ClarificationResolution, DatabaseMarker, Proposal, RecordChange, RecordRequest,
    RecordSet, Retirement, Supersession,
};

pub const SIGNAL_SCHEMA_SOURCE: &str = include_str!("../schema/signal.schema");

pub type SpiritJudgeFrame = signal_frame::ExchangeFrame<SpiritJudgeRequest, SpiritJudgeReply>;
pub type SpiritJudgeFrameBody =
    signal_frame::ExchangeFrameBody<SpiritJudgeRequest, SpiritJudgeReply>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("spirit judge contract value is empty")]
    EmptyValue,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SpiritJudgeRequest {
    JudgeAdmission(AdmissionJudgePacket),
}

impl signal_frame::RequestPayload for SpiritJudgeRequest {}

impl signal_frame::LogVariant for SpiritJudgeRequest {
    fn log_variant(&self) -> u64 {
        1
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SpiritJudgeReply {
    AdmissionJudged(AdmissionJudgeResponse),
    RequestRejected(SpiritJudgeRequestRejection),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AdmissionJudgePacket {
    pub operation: AdmissionJudgeOperation,
    pub records: RecordSet,
    pub database_marker: DatabaseMarker,
}

impl AdmissionJudgePacket {
    pub fn new(
        operation: AdmissionJudgeOperation,
        records: RecordSet,
        database_marker: DatabaseMarker,
    ) -> Self {
        Self {
            operation,
            records,
            database_marker,
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AdmissionJudgeOperation {
    Record(RecordRequest),
    Propose(Proposal),
    Clarify(Clarification),
    ResolveClarification(ClarificationResolution),
    Supersede(Supersession),
    Retire(Retirement),
    ChangeRecord(RecordChange),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AdmissionJudgeResponse {
    pub verdict: AdmissionJudgeVerdict,
    pub diagnostic: JudgeDiagnostic,
}

impl AdmissionJudgeResponse {
    pub fn new(verdict: AdmissionJudgeVerdict, diagnostic: JudgeDiagnostic) -> Self {
        Self {
            verdict,
            diagnostic,
        }
    }

    pub fn conservative_rejection(diagnostic: JudgeDiagnostic) -> Self {
        Self::new(
            AdmissionJudgeVerdict::Reject(AdmissionRejectionReason::JudgeUnavailable),
            diagnostic,
        )
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AdmissionJudgeVerdict {
    Accept,
    Reject(AdmissionRejectionReason),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
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

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct JudgeDiagnostic {
    pub redacted_text: RedactedText,
    pub content_hashes: Vec<ContentHash>,
}

impl JudgeDiagnostic {
    pub fn new(redacted_text: RedactedText, content_hashes: Vec<ContentHash>) -> Self {
        Self {
            redacted_text,
            content_hashes,
        }
    }

    pub fn redacted(redacted_text: RedactedText) -> Self {
        Self::new(redacted_text, Vec::new())
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SpiritJudgeRequestRejection {
    pub reason: SpiritJudgeRequestRejectionReason,
    pub diagnostic: JudgeDiagnostic,
}

impl SpiritJudgeRequestRejection {
    pub fn new(reason: SpiritJudgeRequestRejectionReason, diagnostic: JudgeDiagnostic) -> Self {
        Self { reason, diagnostic }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SpiritJudgeRequestRejectionReason {
    InvalidRequest,
    ConfigurationUnavailable,
    ProviderUnavailable,
    ProviderRejected,
    ResponseFormatFailure,
}

macro_rules! non_empty_text_type {
    ($name:ident) => {
        #[cfg_attr(
            feature = "nota-text",
            derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
        )]
        #[derive(
            rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq,
        )]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if value.is_empty() {
                    return Err(Error::EmptyValue);
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
    };
}

non_empty_text_type!(ContentHash);
non_empty_text_type!(RedactedText);
