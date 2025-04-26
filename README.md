<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>


# Camino-Time
## Description
This project is an experiment/first foray into the world of rust and leptos. It builds a simple site that shows an openstreetmap map that renders gpx files. These gpx files are stored on the SERVER and can be appended through a secured endpoint for custom uploading. (not yet built)

The intention is to use this app as a way for friends & families to track our camino progress. By uploading a gpx track of the hike daily, they get to see the progress we've made over the course of time.

## Features (todo)
- [ ] Map the renders all GPX files
  - [ ] color graded per day (more gray as they get older)
  - [ ] nobs at the end to indicate endpoints. (Gives indiana jones vibes)
  - [ ] Sample track (the estimated way)
  - [ ] Resolution filter on gpx endpoint, to reduce the amount of points sent.
- [ ] Upload endpoint to be able to push gpx files from mobile device into `./uploads`
  - [ ] Secure the endpoint to make sure only owner can upload (how to do)
    - [ ] Has to work on android and be easy to use
- [ ] Secure site with basic password to allow only friends to follow. (for basic privacy)

## Running the project

Interatively with hot reload:
```bash
cargo leptos watch
```
Blocking:
```bash
cargo leptos serve
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
ssr-test
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="ssr-test"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
