# Slop Ninja: private editing for writers and media teams

Design proposal, September 11, 2026. Companion to [whitepaper draft 0.15](../whitepaper/slop_ninja.pdf).

B transformation is the primary commercial product we propose: editing
model-written drafts toward an authorized writer's voice while preserving
meaning, detail and intended tone. It also targets reduced detection by Pangram
and the subnet's own detectors. For a media customer, the commercial hypothesis
is fewer unwanted model-writing patterns and less corrective editing, with
the writer's characteristic voice and the source's substance intact. We have
not yet demonstrated that outcome reliably or established buyer demand.

Candidate uses for an editorial pilot include:

| Workflow | Proposed benefit to test |
| --- | --- |
| An author-led newsletter using model-assisted drafts | Restore that author's characteristic phrasing and structure with fewer manual corrections. |
| An editor reviewing a contributor's model-assisted article | Remove filler and formulaic phrasing while retaining claims, quotations, qualifications and source attribution. |
| A communications team editing an unpublished announcement | Revise approved material toward an authorized writer's voice without giving the compute operator access to its content. |

These are hypotheses for interviews and evaluation, not existing customers or
supported production workflows. Each job needs authorized reference writing and
an editorial brief. The customer retains editorial approval; a detector pass
cannot establish accuracy, preserved meaning or human authorship.

A is an open detector resource that strengthens B training and supplies
independent scoring. It can also support a separately priced hosted detection
service. We expect B to provide the stronger reason to buy inference; buyer
interviews may identify independent demand for A. The two services have separate
performance claims, including A's false-positive and calibration limits.

Every paid B job requires privacy for both the model weights and customer text.
The model owner uploads encrypted weights and releases its model key only to a
verified runtime. The customer independently verifies that runtime and its
loaded model version, then encrypts the draft, writer references and brief to a
job key generated inside it. These are separate authorizations; the model owner
cannot approve disclosure of the customer's text.

Only the accepted application processes both plaintext weights and text during
inference. It returns an encrypted revision to the customer, with no weight
download or inspection interface. The miner retains its original model but
receives no customer input key or output copy. A separate compute host, if used,
receives neither plaintext. The execution code must be public, reviewed and
verifiable; the weights remain private. A confidential VM alone is insufficient
if its application can log text, forward requests or use an unprotected GPU.
Ordinary hosting is ineligible for paid B. A's detector models remain public.

This is a practical direction to prototype. NVIDIA documents confidential
CPU/GPU configurations and encrypted model provisioning, and Privatemode
describes verification of measured workload policies before key exchange.
Those precedents do not qualify our
implementation or remove trust in hardware and audited software.
[NVIDIA deployment guide](https://docs.nvidia.com/cc-deployment-guide-snp.pdf),
[NVIDIA model-provisioning architecture](https://developer.nvidia.com/blog/?p=114591),
[Privatemode attestation](https://docs.privatemode.ai/security/attestation/overview/).

The intended compromise gives customers access to current private models
without granting the operator access to their drafts, while miners retain the
weights that distinguish their service. We no longer require monthly public
releases of the best B weights. Miners can earn both performance-based emissions
and customer inference fees while
retaining their models. A public version commitment and an execution receipt
associate an attested service with its benchmarked model. Validators still
check the output's meaning, tone, author fit and detector results independently.
Attestation earns no quality bonus. Private weights prevent direct downloads
by Pangram or competitors; they cannot prevent study of legitimately purchased
outputs or guarantee that an evasion advantage persists.

Fees are competitively quoted in subnet alpha and paid to the serving miner
under the settlement terms. Network emissions are separate. The protocol does
not automatically route a royalty to Slop Ninja Research; any later application
subscription or gateway fee would need its own disclosed terms. Customer demand,
repeat purchases and revenue after confidential-compute costs determine whether
the commercial model works.

For a media-facing application, the proposed offer would combine a team editing
workflow, customer-controlled author profiles, reviewable changes and usage of
qualified private B services. That application could charge a disclosed team
subscription or usage fee covering its service and purchased inference. This
is a commercial option to test, not an implemented billing model or an assumed
royalty on every subnet job. Profile storage, reviewer access and integrations
must preserve the customer's chosen privacy boundary.

The next commercial step is a narrowly scoped editorial pilot. Identify an
editor who owns a recurring workflow and a budget holder who could approve a
paid trial. Establish their current editing process, document volume, required
privacy boundary and acceptable purchase terms before selecting a pilot scope.
Start with published, synthetic or otherwise authorized non-sensitive drafts
while attested serving is under development; no unpublished announcement should
be needed to demonstrate basic editing value.

Compare the same source drafts under the existing process, a general-purpose
model and Slop Ninja, using authorized writer samples held separate from
evaluation drafts. Agree the briefs, review procedure and acceptance criteria
before comparing results. Record editor time including verification and rework,
author-fit judgments, acceptance rate, and material meaning or tone failures.
Measure both required detector outcomes separately on material authorized for
external evaluation. A low detector score cannot compensate for a failed
editorial requirement. Track cost per accepted revision and willingness to pay
for repeat use; emissions must not be counted as customer revenue when assessing
the commercial case.

Paid B launch requires a working, independently reviewed prototype:
client-side verification, protected GPU execution, private-model binding,
denied egress, isolated job state and no downgrade to ordinary hosting.
Hardware vulnerabilities, side channels, verifier compromise and denial of
service remain risks. Attestation does not prove editorial quality or resolve
payment disputes over secret text. Until those requirements are demonstrated,
this is a proposed product rather than an available confidentiality guarantee.
