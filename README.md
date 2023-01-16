<div align="center">
    <h1><code>sentor</code></h1>
    A node editor for AI models.
</div>

## Toolchain Dependencies
    - node
    - yarn(`brew install yarn`)
    - cmake(`brew install cmake`)
    - cargo([rustup.rs](https://rustup.rs))
        + `wasm32-unknown-unknown` target(`rustup target add wasm32-unknown-unknown`)

## Troubleshooting
    - darwin-arm64: Missing OpenSSL
        - You may need to force brew to link it. For me it was `brew link --force openssl@1.1`
        - Source: https://github.com/murat-dogan/node-datachannel/issues/63#issuecomment-1076034512

## Captain's Logs
### Week 1
- Setup project template using `WebAssembly` + `Rust` + `Preact`.
- Got sick
### Week 2
- 