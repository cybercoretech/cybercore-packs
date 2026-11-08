# 📦 Cybercore Packs

The official registry and tooling contracts for installable Cybercore resources.

Packs are namespaced, versioned, reviewable collections of themes, effects, components, snippets, wallpapers, fonts, SVG assets, and web templates. Official resources use the `cybercore.*` namespace. Community resources use the creator namespace, such as `creatorname.neon-terminal`.

## ✨ Repository layout

- `registry/` — searchable stable, beta, and nightly catalog indexes
- `packs/` — official pack manifests and small source assets
- `schemas/` — machine-readable manifest and lockfile contracts
- `crates/cybercore-packs/` — Rust manifest, registry, lockfile, and verification library
- `crates/cybercore-pack/` — companion CLI

## 📦 Official packs

The `cybercore.*` packs are **generated** from the
[Cybercore framework](https://github.com/cybercoretech/cybercore), never
edited here. `.github/workflows/sync-cybercore.yml` runs
`tools/export-distribution.py packs` from cybercore every 6 hours (or on
demand from the Actions tab) and opens a pull request when anything changed.
A pack whose payload changed gets the next patch version; unchanged packs
keep theirs.

| Pack | Contents | Source in cybercore |
| --- | --- | --- |
| `cybercore.themes` | All 600 palettes in 50 families (`catalog.json`), 12 signature mashups, classic terminal palettes | `schema/themes`, `schema/mashups.json`, `schema/kitty-themes.json` |
| `cybercore.effects` | Web Kit effect styles and recipes, motion profiles, HUD scanline with fallbacks | `web/effects.css`, `schema/webkit.json`, `packs/cybercore-effects` |
| `cybercore.components` | Design tokens, component stylesheet, universal theme chooser | `css/`, `web/chooser.*` |
| `cybercore.cyberdark-wallpapers` | 4K SVG wallpapers, index, theme assignments | `packs/cybercore-images` |
| `cybercore.snippets` | Chooser embed, site variable mapping, Web Kit runtime, themed CSS, Rust, CI, shell | `packs/cybercore-snippets` |
| `cybercore.fonts` | Oxanium, Orbitron, Audiowide, Share Tech Mono (OFL) with `fonts.css` | `packs/cybercore-fonts` |
| `cybercore.svg-assets` | Logos, wordmarks, headers, social card, icons at every size | `assets/` |
| `cybercore.web-templates` | Landing page template wired to the live kit and chooser | `packs/cybercore-web-templates` |

To change an official pack, open a PR against cybercore. Community packs
(`creatorname.*`) are added here directly and kept by the sync.

`cargo run --example verify_registry` checks every manifest against the
registry and every resource against its SHA-256.

## ✨ CLI preview

```bash
cargo install --git https://github.com/cybercoretech/cybercore-packs --package cybercore-pack
cybercore-pack search theme
cybercore-pack install cybercore.themes@1.1.0
cybercore-pack list
cybercore-pack verify

# Control Plane spelling
cybercore-control pack search cyberdark
cybercore-control pack install cybercore.cyberdark-wallpapers
```

Installation is project-local: manifests are written under `.cybercore/packs/` and exact selections are recorded in `.cybercore/packs.lock.json`. Set `CYBERCORE_PACK_REGISTRY` to use a private or mirrored registry.

Once the crate is published to crates.io, the equivalent install is:

```bash
cargo install cybercore-pack
```

## 🚢 Release contract

Every release should include a Git tag, downloadable archive, manifest, SHA-256 checksums, generated previews, compatibility metadata, and a channel update. Large binary collections belong in release assets or object storage; Git should remain focused on manifests and reviewable source.

## ✨ Related projects

- [Cybercore Control Plane](https://github.com/cybercoretech/cybercore-control-plane)
- [Cybercore Framework](https://github.com/cybercoretech/cybercore)
- [Cybercore Web Kit](https://github.com/cybercoretech/cybercore-web-kit)
