# Loporia and Koretoretos

This is a separate worldbuilding project in the repository. Its setting and
software are independent of the animatronic-robot project documented elsewhere.

## Core premise

Spirits such as Metatron and Azazel exert an influence that behaves like a
memetic infection: sustained exposure can reshape a human's identity until
they become Metatronian or Azazellian. In this setting, the change is both
personal and cultural; each spirit's way of seeing the world propagates through
its followers.

## The realms

### Loporia

Loporia is ruled by Metatron. Its influence favors order, symmetry, sacred
geometry, hierarchy, records, and immutable law. A Metatronian identity
prioritizes the blueprint and the larger design, often at the cost of
individual exception or improvisation.

### Koretoretos

Koretoretos is ruled by Azazel. Its influence favors craft, forbidden
knowledge, self-determination, and defiance of imposed authority. An Azazellian
identity embraces the risks of making and mastering tools outside an
established order, including the isolation that may follow.

## Central tension

The realms embody competing creative and social forces. Loporia can turn
structure into bureaucracy and constraint; Koretoretos can turn freedom into
volatility and fragmentation. Their opposition creates a tension between
reliable form and the disruptive capacity to invent.

The setting's central question remains open: are the realms in a cold war for
human allegiance, or does each depend on the other to keep the wider cosmos
from collapsing?

## Mythos toolkit

This directory is a Rust workspace for authoring and inspecting the setting.
It is organized as a reusable domain library (`mythos-core`) and a command-line
application (`mythos`).

The toolkit currently provides:

- A TOML project format for realms, spirits, people, myths, events, relations,
  and influence encounters.
- Validation for malformed and duplicate IDs, broken references, event
  chronology, predecessor cycles, and influence-rule values.
- A deterministic, fictional influence model for exploring resonance,
  transformation, and conflict between realms.
- A compact project summary and a Markdown setting-guide export.
- A starter project based on the established Loporia and Koretoretos premise.

The influence simulator is a narrative design tool; its scores are not claims
about real people, spirits, or psychological processes.

### Build and use

Requires a stable Rust toolchain.

```sh
cargo test --workspace
cargo run --bin mythos -- init mythos.toml
cargo run --bin mythos -- validate mythos.toml
cargo run --bin mythos -- summary mythos.toml
cargo run --bin mythos -- simulate the-artisan mythos.toml
cargo run --bin mythos -- export-markdown mythos.toml setting-guide.md
```

Edit the generated TOML to add people, encounters, myths, events, and
relationships. Run `validate` before using the influence simulator or exporting
a guide.
