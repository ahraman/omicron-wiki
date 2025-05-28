# Omicron changelog

## v0.1.0

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
