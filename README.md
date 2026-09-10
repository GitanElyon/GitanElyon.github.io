# GitanElyon.github.io — Portfolio Site

Dioxus 0.7 (Rust + WASM) portfolio for Gitan Elyon Mandell-Balogh.
Deployed to GitHub Pages (custom domain `gitanelyon.dev`).

## Routes

| Path        | Page    | In navbar? |
|-------------|---------|------------|
| `/`         | Home (hero, about, skills, featured projects, career, contact) | yes |
| `/about`    | About   | yes |
| `/projects` | Projects showcase | yes |
| `/profile`  | Profile — experience, projects, publications, education, skills, contact | yes |
| `/contact`  | Contact | yes |
| `/links`    | Standalone link-in-bio page (no navbar) | no — direct URL only |
| anything else | 404 page | — |

## Profile data (source of truth)

The profile page, home career timeline, and contact details all render from
`assets/profile.toml`, a vendored copy of
[`GitanElyon/resume`](https://github.com/GitanElyon/resume) `resume.toml`.

- **Local auto-sync:** `build.rs` copies `../resume/resume.toml` over
  `assets/profile.toml` on every build when a sibling checkout exists
  (i.e. `~/Projects/resume` next to this repo). Just pull the resume repo
  and rebuild — no manual step.
- **Manual sync:** `./scripts/sync-profile.sh` (uses the sibling checkout,
  falling back to the GitHub raw URL).
- **CI:** uses the checked-in vendored copy, so always commit the synced
  `assets/profile.toml` after updating resume data.

`toml` parsing is compile-time-bundled (`include_str!`); the site makes no
runtime fetch for profile data.

## Develop

Prerequisites: [Nix](https://nixos.org/download/) (provides Rust + wasm
target + Dioxus CLI via `flake.nix`).

```sh
nix develop          # enter dev shell
dx serve --platform web
```

## Build

```sh
dx build --platform web --release
```

Output lands in `target/dx/<name>/release/web/public/`.
`public/robots.txt` is copied to the output root automatically by `dx`.

GitHub Pages deployment (including an `index.html` → `404.html` copy so
client-side routes resolve) is handled by `.github/workflows/deploy.yml`.

## Checks

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
```

## Structure

- `src/main.rs` — router (`/`, `/about`, `/projects`, `/profile`,
  `/contact`, `/links`, 404) + site-wide SEO heads
- `src/views/` — one file per route
- `src/components/` — `Background` (hex grid), `ContactForm` (shared
  Formspree form)
- `src/data/` — `profile` (TOML-backed), `projects`, `skills`
- `assets/styling/` — `base.css`, `components.css`, `responsive.css`,
  `background.css`
- `assets/profile.toml` — vendored profile data (see above)
- `public/` — files copied verbatim to the site root (`robots.txt`)
