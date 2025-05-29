# Omicron changelog

## v0.1.0

### Site structure

#### HTTP method handlers
* The following routes have been added:
    * `/`, `/w`: redirects to `/w/page?title=main`.
    * `/w/page`: primary interface for encyclopedia pages.
        * Interface via query parameters, via one of the following formats:
            * If query parameter `title` is present, then it presents the content of that page.
            * If none of the above options are applicable, it redirects to `/w/page?title=main`.

### API changes

#### Crate `omicron`
* Created library crate.
* Added re-exports of the following:
    * Private types `App`, `AppState`, and `Config` from `mod omicron::app`.
    * Private type `Error` from `mod omicron::error`.
    * Private fn `build_router` from `mod omicron::router`.

##### `mod omicron::controllers`
* Created module.
* Contains the method handlers for all of the differing URLs on a given website.
* The hierarchy roughly corresponds to site structure; see there for the corresponding info.

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
    * Used as an extractor for axum method handlers.
        * To use, add `AppState(app): AppState` to the first argument of the respective method handler.
        * For using `debug_handler` macro, replace the macro with `debug_handler(state = AppState)` to let the debugger know of the associated state type.
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

##### `mod omicron::router`
* Created private module.
* Provides the `build_router` function for building an axum router.
* Added function `pub fn build_router(app: Arc<App>) -> Router`.
    * Takes an `app` instance as input and constructs an axum `Router` instance with the provided app as the global state.
    * For technical reasons, the app must be extracted in method handlers via the `AppState` extractor.

### Dependencies

* Added the following crates as external dependencies to the workspace:
    * [`serde`](https://docs.rs/crate/serde/1.0.219) v1.0.219
        * enabled features: `derive`
    * [`thiserror`](https://docs.rs/crate/thiserror/2.0.12) v2.0.12
* Added the following crates as external dependencies to the crate `omicron`:
    * inherited workspace dependencies: `serde`, `thiserror`
    * [`dotenvy`](https://docs.rs/crate/dotenvy/0.15.7) v0.15.7
    * [`tokio`](https://docs.rs/crate/tokio/1.45.1) v1.45.1
        * enabled features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/crate/axum/0.8.4) v0.8.4
        * disabled default features
        * enabled features: `macros`, `tokio`, `http1`, `query`
    * [`tower`](https://docs.rs/crate/tower/0.5.2) v0.5.2
    * [`tower-http`](https://docs.rs/crate/tower-http/0.6.4) v0.6.4
        * enabled features: `normalize-path`
