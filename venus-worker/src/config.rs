//! config for venus-worker

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml::from_slice;

use crate::sealing::seal::external::config::Ext;

/// configurations for sealing sectors
#[derive(Debug, Clone)]
pub struct Sealing {
    /// specified miner actors
    pub allowed_miners: Option<Vec<u64>>,

    /// specified sector sizes
    pub allowed_sizes: Option<Vec<String>>,

    /// enable sealing sectors with deal pieces
    pub enable_deals: bool,

    /// max retry times for tempoary failed sector
    pub max_retries: u32,

    /// interval between sectors
    pub seal_interval: Duration,

    /// interval between retry attempts
    pub recover_interval: Duration,

    /// interval between polling requests
    pub rpc_polling_interval: Duration,
}

impl Default for Sealing {
    fn default() -> Self {
        Sealing {
            allowed_miners: None,
            allowed_sizes: None,
            enable_deals: false,
            max_retries: 5,
            seal_interval: Duration::from_secs(30),
            recover_interval: Duration::from_secs(30),
            rpc_polling_interval: Duration::from_secs(30),
        }
    }
}

/// configurations for sealing sectors
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SealingOptional {
    /// specified miner actors
    pub allowed_miners: Option<Vec<u64>>,

    /// specified sector sizes
    pub allowed_sizes: Option<Vec<String>>,

    /// enable sealing sectors with deal pieces
    pub enable_deals: Option<bool>,

    /// max retry times for tempoary failed sector
    pub max_retries: Option<u32>,

    /// interval between sectors
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub seal_interval: Option<Duration>,

    /// interval between retry attempts
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub recover_interval: Option<Duration>,

    /// interval between polling requests
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub rpc_polling_interval: Option<Duration>,
}

/// configuration for remote store
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Remote {
    /// store path, if we are using fs based store
    pub path: Option<String>,
}

/// configurations for sector store
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Store {
    /// store location
    pub location: String,

    /// special sealing configuration
    pub sealing: Option<SealingOptional>,
}

/// configurations for rpc
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RPC {
    /// jsonrpc endpoint
    pub endpoint: String,
}

/// configurations for processors
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Processors {
    /// section for pc2 processor
    pub pc2: Option<Ext>,

    /// section for c2 processor
    pub c2: Option<Ext>,
}

/// global configuration
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// section for rpc
    pub rpc: RPC,

    /// section for common sealing
    pub sealing: SealingOptional,

    /// section for list of sector stores
    pub store: Vec<Store>,

    /// section for concurrent limit
    pub limit: HashMap<String, usize>,

    /// section for remote store
    pub remote: Remote,

    /// section for processors
    pub processors: Option<Processors>,
}

impl Config {
    /// load config from the reader
    pub fn from_reader<R: Read>(mut r: R) -> Result<Self> {
        let mut content = Vec::with_capacity(1 << 10);
        r.read_to_end(&mut content)?;

        let cfg = from_slice(&content)?;

        Ok(cfg)
    }

    /// load from config file
    pub fn load<P: AsRef<Path>>(p: P) -> Result<Self> {
        let f = File::open(p)?;
        Self::from_reader(f)
    }
}
