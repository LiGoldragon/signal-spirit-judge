# 3.1.0 — first protos 0.33 text-cohort adoption

The authored Ethos and the contract's Signal implementation are unchanged.
Generated aliases become tuple newtypes, with additional comparison/hash
traits; callers construct the wrappers explicitly. Tests now construct the
new signal-spirit 4.1.0 wrappers and use checked `ReaderBudget::try_from`.
This repository defines wire contracts, not a storage format.

Selected graph: datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0),
signal-spirit 3a0b62f (4.1.0), ethos-zero 5a5daca (19.2.0).
The lock retains two ethos-zero source spellings for that same exact commit,
required by upstream manifests; this is not two generator revisions.

Historical baseline: e41013c5380a3e3e3e74a314a66adbe5d4f005c7, version 3.0.0,
using its own manifest, lock, generated source and codec (6dccc76b).
Independent scratch copies add only a test witness to the baseline and to
the candidate source. They run with each copy's own tools on the remote Nix
builder. Evidence: `/tmp/01e4a4-runtime/judge-{old,new}-witness.log`,
`judge-{old,new}.tsv`, `judge-{old,new}-cross.log`, and the matching `.exit`
files; witness source is in `judge-{old,new}/tests/scaffold.rs` there.

The 26 fixtures cover Record and Propose requests (empty RecordSet), Accept,
all 18 admission rejection reasons and all five request-rejection reasons.
Their raw rkyv archives and the existing Signal frame bytes are identical
between versions, and every frame restores locally. Each side also restores
the other side's archives and parses the other side's printed text to the
same typed value. Raw printed text differs on all 26 fixtures; a separate token-sequence inspection preserves every token and exact
balanced string content, establishing whitespace-layout-only differences
for these fixtures. Typed cross-reading is separate from that inspection. This is representative evidence, not exhaustive coverage of all
seven admission operations, all payload values, or all rejection inputs.
The existing malformed-frame refusal test remains in the check suite.

No runtime/deployment compatibility claim follows from these contract tests.
