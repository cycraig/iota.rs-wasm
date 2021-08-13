// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{client::Client, error::wasm_error};
use futures::executor;
use iota_client::{Api, ClientBuilder as RustClientBuilder};
use std::{rc::Rc, str::FromStr, time::Duration};
use wasm_bindgen::prelude::*;

fn to_duration(seconds: u32) -> Duration {
  Duration::from_secs(u64::from(seconds))
}

fn to_basic_auth<'a>(username: &'a Option<String>, password: &'a Option<String>) -> Option<(&'a str, &'a str)> {
  username.as_deref().zip(password.as_deref())
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ClientBuilder {
  pub(crate) builder: Option<RustClientBuilder>,
}

#[wasm_bindgen]
impl ClientBuilder {
  /// Creates an IOTA client builder.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      builder: Some(RustClientBuilder::new()),
    }
  }

  /// Adds an IOTA node by its URL.
  #[wasm_bindgen]
  pub fn node(&mut self, url: &str) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder: RustClientBuilder| builder.with_node(url).map_err(wasm_error))?;
    // is there a way we can do it without the clone?
    Ok(self.clone())
  }

  /// Adds an IOTA node by its URL to be used as primary node, with optional jwt and or basic authentication
  #[wasm_bindgen(js_name = primaryNode)]
  pub fn primary_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_primary_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })?;
    Ok(self.clone())
  }

  /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW), with optional jwt and or basic
  /// authentication
  #[wasm_bindgen(js_name = primaryPowNode)]
  pub fn primary_pow_node(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_primary_pow_node(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })?;
    Ok(self.clone())
  }

  /// Adds a permanode by its URL, with optional jwt and or basic authentication
  #[wasm_bindgen]
  pub fn permanode(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_permanode(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })?;
    Ok(self.clone())
  }

  /// Adds an IOTA node by its URL with optional jwt and or basic authentication
  #[wasm_bindgen(js_name = nodeAuth)]
  pub fn node_auth(
    &mut self,
    url: &str,
    jwt: Option<String>,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<ClientBuilder, JsValue> {
    self.try_with_mut(|builder| {
      builder
        .with_node_auth(url, jwt.clone(), to_basic_auth(&username, &password))
        .map_err(wasm_error)
    })?;
    Ok(self.clone())
  }
  /// Adds a list of IOTA nodes by their URLs.
  #[wasm_bindgen(js_name = nodes)]
  pub fn nodes(&mut self, urls: JsValue) -> Result<ClientBuilder, JsValue> {
    let urls: Vec<String> = urls.into_serde().map_err(wasm_error)?;
    self.try_with_mut(|builder| {
      builder
        .with_nodes(&urls.iter().map(std::ops::Deref::deref).collect::<Vec<&str>>())
        .map_err(wasm_error)
    })?;
    Ok(self.clone())
  }

  /// Set the node sync interval (has no effect because we can't spawn another thread in wasm to sync the nodes)
  #[wasm_bindgen(js_name = nodeSyncInterval)]
  pub fn node_sync_interval(&mut self, value: u32) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_node_sync_interval(to_duration(value)))?;
    Ok(self.clone())
  }

  /// Disables the node syncing process.
  /// Every node will be considered healthy and ready to use.
  #[wasm_bindgen(js_name = nodeSyncDisabled)]
  pub fn node_sync_disabled(&mut self) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_node_sync_disabled())?;
    Ok(self.clone())
  }

  /// Allows creating the client without nodes for offline address generation or signing
  #[wasm_bindgen(js_name = offlineMode)]
  pub fn offline_mode(&mut self) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_offline_mode())?;
    Ok(self.clone())
  }

  /// Get node list from the node_pool_urls
  #[wasm_bindgen(js_name = nodePoolUrls)]
  pub fn node_pool_urls(&mut self, node_pool_urls: JsValue) -> Result<ClientBuilder, JsValue> {
    let node_pool_urls: Vec<String> = node_pool_urls.into_serde().map_err(wasm_error)?;
    self
      .try_with_mut(|builder| executor::block_on(builder.with_node_pool_urls(&node_pool_urls)).map_err(wasm_error))?;
    Ok(self.clone())
  }

  /// Set if quroum should be used or not
  #[wasm_bindgen(js_name = quorum)]
  pub fn quorum(&mut self, value: bool) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_quorum(value))?;
    Ok(self.clone())
  }

  /// Set amount of nodes which should be used for quorum
  #[wasm_bindgen(js_name = quorumSize)]
  pub fn quorum_size(&mut self, value: usize) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_quorum_size(value))?;
    Ok(self.clone())
  }

  /// Set quorum_threshold
  #[wasm_bindgen(js_name = quorumThreshold)]
  pub fn quorum_threshold(&mut self, value: usize) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_quorum_threshold(value))?;
    Ok(self.clone())
  }

  /// Selects the type of network to get default nodes for it, only "testnet" is supported at the moment.
  /// Nodes that don't belong to this network are ignored. Default nodes are only used when no other nodes are
  /// provided.
  #[wasm_bindgen]
  pub fn network(mut self, network: &str) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_network(network))?;
    Ok(self)
  }

  /// Since we can only have a single thread in wasm, local PoW is much slower
  #[wasm_bindgen(js_name = localPow)]
  pub fn local_pow(&mut self, value: bool) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_local_pow(value))?;
    Ok(self.clone())
  }

  /// Sets after how many seconds new tips will be requested during PoW
  #[wasm_bindgen(js_name = tipsInterval)]
  pub fn tips_interval(&mut self, value: u32) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_tips_interval(u64::from(value)))?;
    Ok(self.clone())
  }

  /// Sets the default request timeout.
  #[wasm_bindgen(js_name = requestTimeout)]
  pub fn request_timeout(&mut self, value: u32) -> Result<ClientBuilder, JsValue> {
    self.with_mut(|builder| builder.with_request_timeout(to_duration(value)))?;
    Ok(self.clone())
  }

  /// Sets the request timeout for a specific API usage.
  #[wasm_bindgen(js_name = apiTimeout)]
  pub fn api_timeout(&mut self, api: String, timeout: u32) -> Result<ClientBuilder, JsValue> {
    let api = Api::from_str(&api)
      .map_err(|_| iota_client::Error::ApiError)
      .map_err(wasm_error)?;
    self.with_mut(|builder| builder.with_api_timeout(api.clone(), to_duration(timeout)))?;
    Ok(self.clone())
  }

  pub(crate) fn take_builder(&mut self) -> Result<RustClientBuilder, JsValue> {
    self.builder.take().ok_or_else(|| "Client Builder Consumed".into())
  }

  fn with_mut(&mut self, f: impl Fn(RustClientBuilder) -> RustClientBuilder) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?));
    Ok(())
  }

  fn try_with_mut(
    &mut self,
    f: impl Fn(RustClientBuilder) -> Result<RustClientBuilder, JsValue>,
  ) -> Result<(), JsValue> {
    self.builder = Some(f(self.take_builder()?)?);
    Ok(())
  }

  /// Build the client.
  #[wasm_bindgen]
  pub fn build(&mut self) -> Result<Client, JsValue> {
    let future = self.take_builder()?.finish();
    let output = executor::block_on(future).map_err(wasm_error)?;

    Ok(Client {
      client: Rc::new(output),
    })
  }
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self::new()
  }
}