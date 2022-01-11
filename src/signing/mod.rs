// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Signing module to allow using different signer types for address generation and transaction essence signing

use bee_message::{address::Address, payload::transaction::TransactionEssence, unlock_block::UnlockBlock};
use tokio::sync::Mutex;

use core::ops::Deref;
use std::{path::Path, sync::Arc};

#[cfg(any(feature = "ledger-nano", feature = "ledger-nano-simulator"))]
pub mod ledger;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Signing related types
pub mod types;
pub use types::{GenerateAddressMetadata, LedgerStatus, Network, SignMessageMetadata, SignerType, TransactionInput};

/// SignerHandle, possible signers are mnemonic, Stronghold and Ledger
#[derive(Clone)]
pub struct SignerHandle(pub(crate) Arc<Mutex<Box<dyn Signer + Sync + Send>>>);

impl SignerHandle {
    /// Create a new SignerHandle
    pub fn new(signer: Box<dyn Signer + Sync + Send>) -> Self {
        Self(Arc::new(Mutex::new(signer)))
    }
}
impl Deref for SignerHandle {
    type Target = Arc<Mutex<Box<dyn Signer + Sync + Send>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Signer interface.
#[async_trait::async_trait]
pub trait Signer {
    /// Get the ledger status.
    async fn get_ledger_status(&self, is_simulator: bool) -> LedgerStatus;
    /// Initialises a mnemonic.
    async fn store_mnemonic(&mut self, storage_path: &Path, mnemonic: String) -> crate::Result<()>;
    /// Generates an address.
    async fn generate_address(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        index: u32,
        internal: bool,
        metadata: GenerateAddressMetadata,
    ) -> crate::Result<Address>;
    /// Signs transaction essence.
    async fn sign_transaction_essence<'a>(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        essence: &TransactionEssence,
        inputs: &mut Vec<TransactionInput>,
        metadata: SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<UnlockBlock>>;
}