# signal-spirit-judge

Typed Signal contract between `spirit` and the Spirit judge adapter. Version
0.2.0 defines the admission-only wire revision 2 surface.

The contract owns both request and reply records for Spirit judgment calls. The
binary wire is rkyv-backed; NOTA projection is only for clients, tests, and
tools.

It has one request root, `JudgeAdmission`, over the four-field ordinary
`Entry` dependency. Admission diagnostics are redacted operational material,
not record privacy metadata. Provider selection and prompt/session material are
outside this pure contract.
