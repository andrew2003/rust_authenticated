use serde::Deserialize;

use rust_authenticated_adapter::repositories::postgres::config::DBConfig;
use rust_authenticated_common::options::{default_log, Log};
#[readonly::make]
#[derive(Deserialize, Debug)]
pub struct Options {
    /// Configuration for the server.
    pub server: Server,
    // /// gRPC.
    // pub gpt_answer_service_url: String,
    /// Specifies the configuration of database will be connected.
    pub pg: DBConfig,
    /// The endpoint for the exporter.
    pub exporter_endpoint: String,
    /// The name of the service.
    pub service_name: String,
    /// Configuration for logging, including log level.
    #[serde(default = "default_log")]
    pub log: Log,
}

/// Represents server configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    /// Port number for the server.
    pub port: u16,
    /// URL for the server.
    pub url: String,
}