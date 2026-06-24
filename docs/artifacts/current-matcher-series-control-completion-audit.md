# Current matcher-heavy series control completion audit

Goal ID: `06cc68b4-799f-4232-bb06-b1ac481edcd6`

This audit maps the active objective to current evidence. It is a control/readout document, not a performance claim.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Use the new repeated/interleaved wrapper before another matcher-heavy claim | [`matcher-heavy-series-control/worktree-8720eb8/summary.md`](matcher-heavy-series-control/worktree-8720eb8/summary.md) records two `scripts/bench-screenfs-series.py` runs with `--pairs 4 --order alternate`: one for `policy-heavy-matrix`, one for `matcher-descendant-directory`. | Satisfied |
| Avoid repeating previous rejected optimizations | No matcher optimization was implemented. The new artifact is same-binary control evidence only and does not retry no-writable mutability, visible-default short-circuit, subtree-only best-candidate, rank-zero early-stop, or empty-family lookup skip. | Satisfied |
| Preserve ScreenFS guardrails | Same binary was used for before/after (`binary-sha256.txt` records identical hashes), so no policy/matcher semantics changed in the measured control. The wrapper only orchestrates benchmark subprocesses. | Satisfied |
| Keep matcher-heavy status scoped until stable evidence exists | [`../benchmarks.md`](../benchmarks.md), [`../performance-roadmap.md`](../performance-roadmap.md), and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) reference the new control as control/no-claim evidence and keep matcher-heavy at `smoke` / no kept speedup. | Satisfied |
| Decide whether current evidence supports a claim | It does not. The same-binary series still shows large apparent movement: policy-heavy `matcher_hidden_stat_miss` p99 ranges `0.836x`-`1.564x`; descendant `matcher_descendant_readdirplus` p99 ranges `0.634x`-`2.210x`. This is not an implementation speedup and strengthens the requirement for repeated/interleaved evidence outside the control envelope. | Satisfied |
| Validate and review | [`current-matcher-series-control-validation.log`](current-matcher-series-control-validation.log) records observed validation commands and results after the series-control update. [`current-matcher-series-control-review-closure.md`](current-matcher-series-control-review-closure.md) records the read-only review findings, fixes, and final no-claim review closure for the checked scope. | Satisfied |

## Conclusion

The repeated/interleaved wrapper was used successfully for the next matcher-heavy measure-first step. The result is **no performance claim**: same-binary apparent movement is still too large, so a future matcher optimization must either isolate a smaller micro/trace signal or move well outside this control envelope across repeated/interleaved policy-heavy and descendant series.
