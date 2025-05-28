# Omicron changelog

## v0.1.0

### API changes

#### Crate `omicron`
* Created library crate.
* Added re-exports of the following:
    * Private types `struct App`, `struct AppState`, `struct Config` from `mod omicron::app`.
    * Private type `enum Error` from `mod omicron::error`.

##### `mod omicron::app`
* Created private module.
* Added type `struct App`.
    * Contains global app state and data.
    * Added field `pub config: Config`.
        * Contains app configuration, that are loaded from environment variables.
    * Added method `pub fn new(config: Config) -> Result<Self, Error>`.
        * The main constructor, that uses a user-provided `config`.
* Added type `struct AppState`.
    * Transparent wrapper over an `Arc<App>` instance.
* Added type `struct Config`.
    * Contains server configuration that is loaded from environment variables.
    * The content of this type are all read by `dotenvy` crate from a `.env` file at the workspace root.
    * Added field `pub server_url: String`.
        * The URL of the server to listen on.
        * Set by the environment variable `SERVER_URL`.
    * Added method `pub fn from_env() -> Result<Self, Error>`.
        * Reads the content of the configuration file from the environment variables, as described by the respective field above.
        * Currently the most robust method of constructing this type.

##### `mod omicron::error`
* Created private module.
* Contains API related to the error type provided by `omicron` crate.
* Added type `enum Error`.
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
