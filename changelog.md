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
    * `/asset`: interface for loading web assets.
        * Query parameters determine the functionality.
            * If query parameter `file` is present, then the corresponding server-side file is loaded.
            * Otherwise, status code `NOT_FOUND` is returned.

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
    * Added field `pub asset_manager: omicron::asset::AssetManager`.
        * The asset manager instance for this app.
    * Added field `pub render_manager: omicron::asset::RenderManager`.
        * The render manager instance for this app.
    * Added constructor `pub fn new(config: Config) -> Result<Self, Error>`.
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
    * Added constructor `pub fn from_env() -> Result<Self, Error>`.
        * Reads the content of the configuration file from the environment variables, as described by the respective field above.
        * Currently the most robust method of constructing this type.

##### `mod omicron::asset`
* Created public module.
* Added type `struct Asset`.
    * Added field `pub data: Box<[u8]>`.
        * Contains the raw data of the asset.
    * Added field `pub meta: AssetMeta`.
        * The metadata of the asset, such as expected format and content type.
* Added type `struct AssetMeta`.
    * Added field `pub content_type: ContentType`.
* Added type `enum ContentType`.
    * Represents different HTML content types.
* Added type `struct AssetManager`.
    * Contains application state for assets.
    * Added field `pub root_dir: PathBuf`.
        * The root directory of `assets` folder, read from the app configuration.
    * Added private field `cache: Cache`.
        * The underlying asset cache.
    * Added constructor `pub fn new(config: &Config) -> Result<Self, Error>`.
    * Added method `pub fn load_transient(&self, key: &AssetKey) -> Result<Arc<Asset>, Error>`.
        * Loads asset without putting it in cache.
    * Added method `pub fn load(&self, key: AssetKey) -> Result<Arc<Asset>, Error>`.
        * Loads asset from cache, or loads it from the server hard drive.
    * Added method `pub fn reload(&mut self, key: AssetKey) -> Result<Arc<Asset>, Error>`.
        * Loads asset from hard drive and replaces it in cache, even if it was already loaded.
    * Added method `pub fn clear_cache(&mut self)`.
        * Clears the internal cache from all loaded assets.
* Added type `struct AssetKey`.
    * Represents a key for indexing assets in the asset registry.
    * Added field `pub path: String`.
        * The name of the asset in the registry.
    * Added constructor `pub fn new(path: String) -> Self`.

#### `mod omicron::render`
* Created public module.
* Added type `struct RenderManager`.
    * Contains application state for rendering HTML templates.
    * Current backend used is the `tera` crate and template language.
    * Added constructor `pub fn new(asset_manager: &AssetManager) -> Result<Self, Error>`.
        * Main constructor, taking in the `asset_manager` instance of the current application.
        * Loads templates in `${ASSET_DIR}/templates`.

##### `mod omicron::error`
* Created private module.
* Contains API related to the error type provided by `omicron` crate.
* Added type `enum Error`.
    * A generic error type for the whole crate.
    * Added the following variants to the enum:
        * `AssetName(PathBuf)`: error occurred during loading an asset pointed by the given path.
        * Wrappers `Io(std::io::Error)`, `Env(std::env::VarError)`, `Dotenvy(dotenvy::Error)`, `Tera(tera::Error)`, and `Http(axum::http::Error)` of the respective error types.
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
    * [`serde_json`](https://docs.rs/crate/serde_json/1.0.140) v1.0.140
    * [`thiserror`](https://docs.rs/crate/thiserror/2.0.12) v2.0.12
* Added the following crates as external dependencies to the crate `omicron`:
    * inherited workspace dependencies: `serde`, `serde_json`, `thiserror`
    * [`dotenvy`](https://docs.rs/crate/dotenvy/0.15.7) v0.15.7
    * [`tokio`](https://docs.rs/crate/tokio/1.45.1) v1.45.1
        * enabled features: `rt-multi-thread`, `macros`, `net`, `fs`
    * [`axum`](https://docs.rs/crate/axum/0.8.4) v0.8.4
        * disabled default features
        * enabled features: `macros`, `tokio`, `http1`, `query`
    * [`tower`](https://docs.rs/crate/tower/0.5.2) v0.5.2
    * [`tower-http`](https://docs.rs/crate/tower-http/0.6.4) v0.6.4
        * enabled features: `normalize-path`
    * [`tera`](https://docs.rs/crate/tera/1.20.0) 1.20.0
