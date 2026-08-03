# signal-spirit-judge editing contract

- The only request root is `JudgeAdmission`.
- Its packet is exactly operation, record set, and database marker.
- Binary component traffic uses rkyv; text is edge projection only.
- Diagnostics are uniformly redacted text plus content hashes.
- Keep request and reply records together; runtime and prompt prose live out of
  this repository.
- Do not add compatibility shapes or defaults.
- Run formatting, all-feature tests, and the Nix flake checks.
