# Cybercore Packs

The official registry and tooling contracts for installable Cybercore resources.

Packs are namespaced, versioned, reviewable collections of themes, effects, components, snippets, wallpapers, fonts, SVG assets, and web templates. Official resources use the `cybercore.*` namespace. Community resources use the creator namespace, such as `creatorname.neon-terminal`.

## Repository layout

- `registry/` — searchable stable, beta, and nightly catalog indexes
- `packs/` — official pack manifests and small source assets
- `schemas/` — machine-readable manifest and lockfile contracts
- `crates/cybercore-packs/` — Rust manifest, registry, lockfile, and verification library
- `crates/cybercore-pack/` — companion CLI

## CLI preview

```bash
cargo install --path crates/cybercore-pack
cybercore-pack search theme
cybercore-pack add cybercore.themes
cybercore-pack lock
cybercore-pack verify
```

The Control Plane will eventually expose the same workflow through `cybercore-control pack ...`, while this CLI remains useful for projects that only need pack management.

## Release contract

Every release should include a Git tag, downloadable archive, manifest, SHA-256 checksums, generated previews, compatibility metadata, and a channel update. Large binary collections belong in release assets or object storage; Git should remain focused on manifests and reviewable source.

## Related projects

- [Cybercore Control Plane](https://github.com/cybercoretech/cybercore-control-plane)
- [Cybercore Framework](https://github.com/cybercoretech/cybercore)
- [Cybercore Web Kit](https://github.com/cybercoretech/cybercore-web-kit)
