# Omicron changelog

## v0.1.0

### API changes

### Crate `omicron`
* Created library crate.
* Added re-exports of the following:
    * Private type `enum omicron::error::Error`.

#### `mod omicron::error`
* Created private module.
* Contains API related to the error type provided by `omicron` crate.
* Added type `enum omicron::error::Error`.
    * A generic error type for the whole crate.
    * Added the following variants to the enum:
        * Wrappers `Io(std::io::Error)`, `Env(std::env::VarError)`, and `Dotenvy(dotenvy::Error)`, of the respective error types.
    * Added `impl IntoResponse` trait implementation, so that it can be returned via a `Result` by axum method handlers.

### Dependencies

* Added the following crates as external dependencies to the workspace:
    * [`thiserror`](https://docs.rs/crate/thiserror/2.0.12) v2.0.12
* Added the following crates as external dependencies to the crate `omicron`:
    * `thiserror` (workspace dependency)
    * [`dotenvy`](https://docs.rs/crate/dotenvy/0.15.7) v0.15.7
    * [`tokio`](https://docs.rs/crate/tokio/1.45.1) v1.45.1
        * enabled features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/crate/axum/0.8.4) v0.8.4
        * disabled default features
        * enabled features: `macros`, `tokio`, `http1`
