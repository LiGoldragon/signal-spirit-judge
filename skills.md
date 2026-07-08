# skills — signal-spirit-judge

- Contract-local operation roots: `JudgeAdmission` and `JudgeReferentRegistration`.
- Binary component traffic uses typed rkyv records. NOTA is only projection for
  clients, tests, and tools.
- Every request carries an explicit `JudgmentScope`.
- Diagnostics default to redacted text and content hashes; do not add raw private
  content fields to reply diagnostics.
- Keep request and reply records in this repo together.
- Run `cargo fmt`, `cargo test`, and `nix flake check` after Rust changes.
