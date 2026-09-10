# Assignment and fulfillment gameability

The seeded simulation shows that prescribed peers constrain requester choice, while a 90% fulfillment gate still permits selective nonresponse and cannot detect on-time dishonest answers. Predictable majority-controlled panels remain a direct threat even when every modeled submission receives all required reviews. This assignment and accounting stress simulation leaves economic equilibrium, operator identity and language quality unmeasured.

[Rust source](../experiments/assignment-gameability/src/main.rs), [full results](../experiments/assignment-gameability/results.json), and [aggregate results](../experiments/assignment-gameability/summary.json) are retained. No miners, chain, models, detector APIs, or private data were called.

The fixed synthetic roster has 100 UIDs qualified for both logical service interfaces. Cartel sizes are 0, 10, 20, 30 and 50 UIDs; membership is fixed before assignments. Seeds 0, 17 and 29 identify three synthetic chains, each with 10,000 scheduled slots. SHA-256 ranks canonically length-prefixed domain, chain, netuid, requester, lifetime-index and peer fields, breaking ties by UID. Roster membership does not enter each hash. Each requester receives 100 issued slots, 50 per interface. The same draws are reused across strategies and cartel shares.

There are three permitted peers total: the mandatory first provider and two ordered failure fallbacks. This synthetic roster does not validate deployed UID/mechanism capacity. All possible fallback windows are assumed reserved; latency, prices and capacity contention are not simulated. Provider-selective failures and requester abandonment are separate strategies.

The following percentages concern cartel requesters obtaining a cartel provider. Self-UID exclusion means the first-peer probability is `(C-1)/(N-1)`. A choose-any-three menu instead gives `1 - choose(N-C,3)/choose(N-1,3)` under uniform hash ranks.

| Cartel UID share | Mandatory first | Choose any of three | Mandatory, after counted capped skips |
| --- | ---: | ---: | ---: |
| 10% | 9.37% | 25.17% | 10.41% |
| 20% | 18.80% | 47.92% | 20.89% |
| 30% | 29.74% | 66.29% | 33.05% |
| 50% | 49.82% | 87.51% | 55.36% |

The final column uses delivered requests as its denominator; the first two use all issued requests. Capped requesters abandon at most 10% of issued slots per interface and in aggregate, retain every consumed index, and obtain no extra draws. At 20% cartel share, the favorable count remains 1,128: 600 counted skips reduce delivered requests from 6,000 to 5,400. Skips before valid payload delivery count against the requester, never the provider. Abandoning every unfavorable slot produces 100% favorable delivered traffic, but all 60 cartel UID-epochs fail the correct aggregate gate. A delivered-only denominator incorrectly passes all 60. Even aggregate-capped skipping violates an interface gate in 34 of those 60 UID-epochs; the implemented hardened cap checks both interfaces.

Cartel providers also selectively refuse noncartel requesters, using at most `floor(initial assigned requests/10)` failures per provider/interface. Extra fallback attempts do not increase this attack budget. Every provider still passes the per-interface and aggregate gates. At 20% share, 553 failures occur in 30,000 tickets, 472 tickets recover through fallback, and 8 exhaust their three peers. Erasing failures after successful fallback would incorrectly retain only 24 of the 553 failures. At 50% share there are 1,365 failures and 132 exhausted tickets. This demonstrates one feasible strategy, not the maximum possible disruption.

For common detector panels, UID 0 is always the strongest A service; the other two are hash-ranked. Its honest or cartel status is an exogenous scenario, not a verified qualification claim. A ten-UID B batch excludes all three exact panel UIDs. Other UIDs in that batch can still belong to the cartel. Shared detector-matrix replacement is outside this simulation; actual reserve selection must exclude every frozen B UID as well as the initial A panel.

Each reviewer assignment uses a separate, already-filtered roster of UIDs 1000-1099, disjoint from every A/B UID, with the stated conditional cartel share. Its hash binds the actual first B submitter UID. One submission per comparison is modeled, receiving all three reviews; the rest of each B batch is not reviewed in this simulation. Publicly predictable reviewer assignments let coordinated submitters reserve bad candidates for known captured review sets. The attack assumes the cartel can coordinate and recognize its relevant submissions; the effect of grading-interface blinding is not measured.

Each panel scenario has 30,000 draws. Values below are observed percentages, with exact without-replacement probabilities in parentheses.

| Cartel share | Three A UIDs, strongest honest | Three A UIDs, strongest cartel | Three reviewers |
| --- | ---: | ---: | ---: |
| 10% | 0.947% (0.928%) | 16.933% (17.440%) | 2.513% (2.579%) |
| 20% | 3.930% (3.917%) | 34.167% (34.859%) | 10.200% (10.105%) |
| 30% | 8.913% (8.967%) | 49.763% (50.216%) | 21.410% (21.342%) |
| 50% | 25.003% (25.253%) | 74.530% (74.747%) | 50.270% (50.000%) |

For `N` eligible UIDs and `C` cartel UIDs, the honest-strongest probability is `choose(C,2)/choose(N-1,2)`. The cartel-strongest probability is `1 - choose(N-C,2)/choose(N-1,2)`. Reviewer majority probability is `(choose(C,2)*(N-C) + choose(C,3))/choose(N,3)`. These formulas assume uniform digest rankings; agreement with seeded frequencies is a simulation check, not a hash-security proof.

At 20% share with an honest strongest detector, 1,035 of 30,000 comparisons have both a cartel-majority A panel and another cartel UID in the B batch. With a cartel strongest detector, this occurs in 9,053 comparisons. Exact UID exclusion holds in every draw. In the honest-strongest scenario, 576 of 6,033 modeled cartel submissions fall on known cartel-majority reviewer sets. The cartel can make only those submissions bad and return all assigned answers on time. All three reviews still occur. In the stylized vote model, the majority approves those bad cases while every cartel service fulfills 100%; availability supplies no evidence against fabrication. These are capture opportunities under the modeled majority rule, not measured fraudulent payouts. The paper's separate dispute resolution and quality requirements remain necessary; missing or disputed reviews are not assumed approved.

The size sensitivity is analytic above size three. At 20% cartel share:

| Panel/reviewer size | A majority, strongest honest | A majority, strongest cartel | Reviewer majority |
| --- | ---: | ---: | ---: |
| 3 | 3.917% | 34.859% | 10.105% |
| 5 | 2.521% | 16.517% | 5.320% |
| 7 | 1.445% | 8.283% | 2.813% |
| 9 | 0.790% | 4.191% | 1.470% |

Calls grow linearly with size under the fixed workload assumption. Larger panels reduce these fixed-share probabilities; they do not establish independent ownership or prevent adaptive UID acquisition.

The exact gate is `N>0 && 10*F<=N`. One failure fails at nine requests and passes at ten; two failures fail at nineteen and pass at twenty. Zero assigned work supplies no automatic pass. Mixing paid successes into mandatory accounting is unsafe: two failures among ten mandatory requests become exactly 10% after ten cheap successful customer requests. An aggregate-only mandatory gate also passes those two A failures when B supplies 100 successes. Separate mandatory denominators per committed interface and in aggregate block both substitutions.

The zero-emission cliff also has a continuation problem. With a fixed final total of 100 obligations and 11 irreversible failures, completing the remaining 89 earns the same current-epoch emission credit as abandoning them: zero. At unit service cost and with no future benefits, their remaining-work payoffs are -89 and 0. The gate alone does not reward continued service after an irrecoverable failure.

The proposed recovery rule is `D_e = max(0, D_previous + 10*F_e - N_e)`, separately for each mandatory class, starting at zero. Completing all remaining work leaves deficit 10; abandoning it leaves 900. Ten or 900 later scheduler-assigned successes in that same class retire those deficits if there are no new failures and sufficient probation capacity. An epoch with no assigned work cannot erase debt; healthy epochs cannot bank negative debt. Paid traffic and another interface's work do not reduce it, and the failed epoch's credit stays zero. This makes continued service useful for future recovery when future eligibility has value; it does not prove participation is profitable or prevent fresh-identity evasion.

The minimum guardrails supported by these examples are prescribed ordered fallbacks with permanent failure records; requester accounting for abandoned issued slots; separate mandatory interface and aggregate gates; retained recovery deficits with bounded assigned probation; and quality review that explicitly assumes some reviewers may collude despite full attendance. Keep strongest-detector evidence and disagreements visible. Hash assignment establishes reproducible allocation, not operator independence, truthful private execution or preservation quality. Fixed membership, qualification outcomes, public assignment predictability and an unoptimized attack strategy limit this experiment; adaptive slot acquisition and economic incentives need separate analysis.

Reproduce from the repository root with `sh experiments/assignment-gameability/run.sh`. The isolated Cargo manifest and lock pin dependencies; on a machine without the cached crates, run `cargo fetch --locked --manifest-path experiments/assignment-gameability/Cargo.toml` first. The script checks formatting, runs three focused tests and Clippy with warnings denied, executes the release simulation, and writes SHA-256 artifact bindings. The completed checks cover the known SHA-256 vector, stable UID ordering/exclusion, threshold/recovery arithmetic and deterministic service accounting. No full repository suite or live service test was required for this isolated simulation.
