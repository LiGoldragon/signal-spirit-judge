# signal-spirit-judge — architecture

`signal-spirit-judge` is the Spirit-specific contract between `spirit` and the
`spirit-judge` text/model edge adapter.

It owns both sides of the exchange: Spirit sends typed judge requests, and the
adapter returns typed judge replies. The initial surface covers Spirit's current
judge families: record/proposal admission judgment and referent registration
judgment.

## Boundary

Owned here:

- `SpiritJudgeRequest` and `SpiritJudgeReply`;
- typed request packet records for Spirit admission and referent judgment;
- explicit public/private request scope records;
- typed verdict, rejection, and privacy-safe diagnostic records;
- rkyv-compatible wire records;
- NOTA projection shape for clients, tests, and tools.

Not owned here:

- provider calls and retries, which belong in `judge`;
- prompt prose, which belongs in a Spirit judge configuration repo;
- adapter process lifecycle, which belongs in a future `spirit-judge` runtime;
- Spirit storage, admission, or referent registration logic, which belongs in
  `spirit`.

## Privacy

Requests name whether the adapter may receive public or private Spirit content.
Replies and diagnostics must stay privacy-safe by default: they carry redacted
messages and content hashes, not raw private content.
