# Security policy

## Reporting a vulnerability

Report suspected vulnerabilities privately to the maintainers. Do not open a
public issue for an unpatched problem. Include the affected version, the
component involved, a minimal reproduction, and the expected impact.

## Secrets

- Never commit credentials, tokens, keys, or certificates.
- `.env.example` contains placeholders only; a real `.env` stays on the
  workstation and is ignored by git.
- Keep local configuration out of synced or shared folders.
- Rotate any credential that was accidentally exposed and report it privately.

## Supply chain

- Pin dependency versions. Do not use floating versions for build tools.
- Do not add dependencies on code studied for inspiration; reimplement the
  behavior instead.
- Do not run untrusted code from component manifests or registry entries.
- Third-party license notices are generated from the actual dependency graph.

## Scope

This policy applies to the component crates, the CLI, the linter, and the
showcase application.
