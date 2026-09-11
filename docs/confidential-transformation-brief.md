# Slop Ninja: private transformation with attested inference

Design proposal, September 11, 2026. Companion to [whitepaper draft 0.14](../whitepaper/slop_ninja.pdf).

B transformation is the primary commercial product. Customers pay to improve
model-written text toward an authorized writer's voice while preserving its
meaning and intended tone and reducing detection by Pangram and the subnet's
own detectors. A remains an open detector resource that strengthens B training
and supplies independent scoring. Customers can also pay for convenient A
hosting, but we expect B to provide the stronger reason to buy inference.
That expectation needs customer and margin evidence.

For sensitive material, ordinary encryption to a miner is insufficient: the
miner can read the text after decryption. We propose attested inference for
private B models. The customer verifies fresh evidence for an approved
application and protected CPU/GPU configuration, then encrypts directly to a
job key generated inside that instance. Only the measured application receives
the plaintext and it encrypts the result back to the customer. The miner keeps
its weights and supplies compute, but receives no customer input key or output
copy. A confidential VM alone is insufficient if its application can log text,
forward requests or use an unprotected GPU.

This is a practical direction to prototype. NVIDIA documents confidential
CPU/GPU configurations, and Privatemode describes verification of measured
workload policies before key exchange. Those precedents do not qualify our
implementation or remove trust in hardware and audited software.
[NVIDIA deployment guide](https://docs.nvidia.com/cc-deployment-guide-snp.pdf),
[Privatemode attestation](https://docs.privatemode.ai/security/attestation/overview/).

We no longer require monthly public releases of the best B weights. Miners
can earn both performance-based emissions and customer inference fees while
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

Sensitive paid launch requires a working, independently reviewed prototype:
client-side verification, protected GPU execution, private-model binding,
denied egress, isolated job state and no downgrade to ordinary hosting.
Hardware vulnerabilities, side channels, verifier compromise and denial of
service remain risks. Attestation does not prove editorial quality or resolve
payment disputes over secret text. Until those requirements are demonstrated,
this is a proposed product rather than an available confidentiality guarantee.
