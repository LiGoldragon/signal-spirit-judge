# signal-spirit-judge architecture

`signal-spirit-judge` is the typed exchange contract between `spirit` and the
Spirit judge adapter. Version 0.2.0 defines wire revision 2 and depends exactly
on `signal-spirit` 0.14.0.

The request surface has one root:

```text
JudgeAdmission(
  AdmissionJudgePacket {
    AdmissionJudgeOperation
    RecordSet
    DatabaseMarker
  }
)
```

Replies are `AdmissionJudged` or `RequestRejected`. Diagnostics contain only
redacted text and content hashes. This conservative form applies uniformly as
an operational security rule.

This crate owns typed request/reply records, the rkyv exchange shape, and the
optional text projection. Provider calls belong in `judge`, prompt prose in
`spirit-judge-config`, adapter lifecycle in `spirit-judge`, and storage or
admission execution in `spirit`.

Revision 1 frames are not accepted. Tests prove the one admission request and
reply round trip, four-field record dependency, uniform diagnostics, rejected
old request shapes, and absence of retired vocabulary from active artifacts.
