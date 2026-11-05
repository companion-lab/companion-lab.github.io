# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can can install the Tailwind CLI and use it directly.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

Plain `dx serve` is client-only (the static site). To run the build-upload
API locally, build the native server instead:

```bash
cargo run --features server
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

## Documentation pages

`/docs` renders the markdown articles in `docs/content/` as Dioxus
components. The articles are embedded at build time (`include_str!` in
`src/pages/docs.rs`) and parsed with `pulldown-cmark` — no server round-trip,
so the static Pages deployment serves them too.

To add an article: create `docs/content/<slug>.md`, then register it in the
`DOC_ARTICLES` array in `src/pages/docs.rs`. To add new markdown syntax
support, extend the renderer in
`src/components/organisms/docs/markdown.rs`.

## Deployment

### GitHub Pages (static landing page)

Pushes to `main` automatically build the client-only WASM bundle and deploy it
to GitHub Pages via [`.github/workflows/deploy-pages.yml`](.github/workflows/deploy-pages.yml).
One-time setup: enable **Settings → Pages → Source: GitHub Actions** (the
workflow also attempts to enable it itself). The site lives at
`https://<owner>.github.io/companion-landing-page/`; the asset base path is
derived from the repo name in CI, so local `dx serve` stays unprefixed.

### Build-upload API (fullstack server)

`POST /api/builds` and `GET /api/builds/latest/:platform` run in the `server`
build and need a real host (with Postgres + S3) — see
[`docs/upload-api.md`](docs/upload-api.md). They are not part of the Pages
deployment.

Feature wiring matters here: `fullstack` is only enabled by the `server`
feature, never by default. A fullstack client expects server-rendered
hydration data and crashes with `atob(undefined)` on a static page, so the
client-only build (default features) must stay free of it. Server-only
dependencies are additionally target-gated to non-wasm targets.


