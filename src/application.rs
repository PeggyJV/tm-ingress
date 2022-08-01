//! CosmosTxEndpoint Abscissa Application

use crate::{commands::EntryPoint, config::CosmosTxEndpointConfig};
use abscissa_core::{
    application::{self, AppCell},
    config::{self, CfgCell},
    trace, Application, FrameworkError, StandardPaths,
};
use abscissa_tokio::TokioComponent;
use reqwest::Url;

/// Application state
pub static APP: AppCell<CosmosTxEndpointApp> = AppCell::new();

/// CosmosTxEndpoint Application
#[derive(Debug)]
pub struct CosmosTxEndpointApp {
    /// Application configuration.
    config: CfgCell<CosmosTxEndpointConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for CosmosTxEndpointApp {
    fn default() -> Self {
        Self {
            config: CfgCell::default(),
            state: application::State::default(),
        }
    }
}

impl Application for CosmosTxEndpointApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint;

    /// Application configuration.
    type Cfg = CosmosTxEndpointConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<CosmosTxEndpointConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    ///
    /// If you would like to add additional components to your application
    /// beyond the default ones provided by the framework, this is the place
    /// to do so.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let mut framework_components = self.framework_components(command)?;
        framework_components.push(Box::new(TokioComponent::new()?));
        let mut app_components = self.state.components_mut();
        app_components.register(framework_components)
    }

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        let mut components = self.state.components_mut();
        components.after_config(&config)?;
        self.config.set_once(config);

        // Validate node RPC url
        let url = &self.config.read().node.rpc;
        let url_parts =
            Url::parse(url).unwrap_or_else(|_| panic!("failed to parse node RPC url: {}", url));
        url_parts
            .host()
            .unwrap_or_else(|| panic!("unable to determine node RPC port from url '{}'", url));
        url_parts
            .port_or_known_default()
            .unwrap_or_else(|| panic!("unable to parse node RPC host from url '{}'", url));

        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
