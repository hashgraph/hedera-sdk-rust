/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use time::Duration;

use crate::signer::AnySigner;
use crate::staked_id::StakedId;
use crate::{
    AccountId,
    Client,
    ContractCreateTransaction,
    Error,
    FileAppendTransaction,
    FileCreateTransaction,
    FileDeleteTransaction,
    FileId,
    Hbar,
    Key,
    PrivateKey,
    PublicKey,
    TransactionResponse,
};

/// Create a new smart contract
///
/// The operation of this flow is as follows:
/// 1. Create a file for the contract's bytecode (via a [`FileCreateTransaction`] and zero or more [`FileAppendTransaction`]s)
/// 2. Execute a [`ContractCreateTransaction`] using the provided information and the newly created file.
/// 3. Delete the file created in step 1.
#[derive(Default, Debug)]
pub struct ContractCreateFlow {
    bytecode: Vec<u8>,
    file_append_max_chunks: Option<usize>,
    node_account_ids: Option<Vec<AccountId>>,
    contract_data: ContractData,
}

impl ContractCreateFlow {
    /// Create a new `ContractCreateFlow`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns bytes of the smart contract.
    #[must_use]
    pub fn get_bytecode(&self) -> &[u8] {
        &self.bytecode
    }

    /// Sets the raw bytes of the smart contract.
    pub fn bytecode(&mut self, bytecode: Vec<u8>) -> &mut Self {
        self.bytecode = bytecode;

        self
    }

    /// Sets the bytecode of the smart contract in hex.
    ///
    /// # Errors
    /// - [`Error::BasicParse`](Error::BasicParse) if `bytecode` is invalid hex.
    pub fn bytecode_hex(&mut self, bytecode: &str) -> crate::Result<&mut Self> {
        self.bytecode = hex::decode(bytecode).map_err(Error::basic_parse)?;

        Ok(self)
    }

    /// Returns the account IDs of the nodes the transactions may be submitted to.
    #[must_use]
    pub fn get_node_account_ids(&self) -> Option<&[AccountId]> {
        self.node_account_ids.as_deref()
    }

    /// Sets the account IDs of the nodes the transactions may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client.
    pub fn node_account_ids(
        &mut self,
        node_account_ids: impl IntoIterator<Item = AccountId>,
    ) -> &mut Self {
        self.node_account_ids = Some(node_account_ids.into_iter().collect());

        self
    }

    /// Returns maximum number of chunks the `FileAppendTransaction` can be split into.
    ///
    /// If null, the default value for a [`FileAppendTransaction`] will be used.
    #[must_use]
    pub fn get_max_chunks(&self) -> Option<usize> {
        self.file_append_max_chunks
    }

    /// Sets the maximum number of chunks the [`FileAppendTransaction`] can be split into.
    pub fn max_chunks(&mut self, max_chunks: usize) -> &mut Self {
        self.file_append_max_chunks = Some(max_chunks);

        self
    }

    /// Returns the parameters to pass to the constructor.
    #[must_use]
    pub fn get_constructor_parameters(&self) -> &[u8] {
        &self.contract_data.constructor_parameters
    }

    /// Sets the parameters to pass to the constructor.
    pub fn constructor_parameters(
        &mut self,
        constructor_parameters: impl Into<Vec<u8>>,
    ) -> &mut Self {
        self.contract_data.constructor_parameters = constructor_parameters.into();

        self
    }

    // /// Sets the parameters to pass to the constructor.
    // ///
    // /// This is equivalent to calling `constructorParameters(parameters.toBytes())`
    // pub fn constructorParameters(&mut self , constructorParameters: ContractFunctionParameters) -> &mut Self {
    //     self.constructorParameters = constructorParameters.toBytes()

    //     self    // }

    /// Returns the gas limit to deploy the smart contract.
    #[must_use]
    pub fn get_gas(&self) -> u64 {
        self.contract_data.gas
    }

    /// Sets the gas limit to deploy the smart contract.
    pub fn gas(&mut self, gas: u64) -> &mut Self {
        self.contract_data.gas = gas;

        self
    }

    /// Returns the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    #[must_use]
    pub fn get_initial_balance(&self) -> Hbar {
        self.contract_data.initial_balance
    }

    /// Sets the initial balance to put into the cryptocurrency account associated with the new
    /// smart contract.
    pub fn initial_balance(&mut self, initial_balance: Hbar) -> &mut Self {
        self.contract_data.initial_balance = initial_balance;

        self
    }

    /// Retunrs the maximum number of tokens that the contract can be automatically associated with.
    #[must_use]
    pub fn get_max_automatic_token_associations(&self) -> u32 {
        self.contract_data.max_automatic_token_associations
    }

    /// Sets the maximum number of tokens that the contract can be automatically associated with.
    pub fn max_automatic_token_associations(
        &mut self,
        max_automatic_token_associations: u32,
    ) -> &mut Self {
        self.contract_data.max_automatic_token_associations = max_automatic_token_associations;

        self
    }

    /// If `true`, the contract will decline receiving a staking reward.
    ///
    /// The default value is false.
    #[must_use]
    pub fn get_decline_staking_reward(&self) -> bool {
        self.contract_data.decline_staking_reward
    }

    /// If set to `true`, the contract will decline receiving a staking reward.
    pub fn decline_staking_reward(&mut self, decline_staking_reward: bool) -> &mut Self {
        self.contract_data.decline_staking_reward = decline_staking_reward;

        self
    }

    /// Reutrns the admin key for the new contract.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.contract_data.admin_key.as_ref()
    }

    /// Sets the admin key for the new contract.
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.contract_data.admin_key = Some(admin_key.into());

        self
    }

    /// Returns the account to be used at the contract's expiration time to extend the life of the contract.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.contract_data.auto_renew_account_id
    }

    /// Sets the account to be used at the contract's expiration time to extend the life of the contract.
    pub fn auto_renew_account_id(&mut self, auto_renew_account_id: AccountId) -> &mut Self {
        self.contract_data.auto_renew_account_id = Some(auto_renew_account_id);

        self
    }

    /// Returns the auto renew period for the smart contract.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.contract_data.auto_renew_period
    }

    /// Sets the auto renew period for the smart contract.
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.contract_data.auto_renew_period = Some(auto_renew_period);

        self
    }

    /// Returns the memo for the new smart contract.
    #[must_use]
    pub fn get_contract_memo(&self) -> Option<&str> {
        self.contract_data.contract_memo.as_deref()
    }

    /// Sets the memo for the new smart contract.
    pub fn contract_memo(&mut self, contract_memo: String) -> &mut Self {
        self.contract_data.contract_memo = Some(contract_memo);

        self
    }

    /// Returns the ID of the account to which the contract is staking.
    pub fn get_staked_account_id(&self) -> Option<AccountId> {
        self.contract_data.staked_id.and_then(StakedId::to_account_id)
    }

    /// Sets the ID of the account to which the contract is staking.
    pub fn staked_account_id(&mut self, staked_account_id: AccountId) -> &mut Self {
        self.contract_data.staked_id = Some(StakedId::AccountId(staked_account_id));

        self
    }

    /// Returns ID of the node to which the contract is staking.
    pub fn get_staked_node_id(&self) -> Option<u64> {
        self.contract_data.staked_id.and_then(StakedId::to_node_id)
    }

    /// Sets the ID of the node to which the contract is staking.
    pub fn staked_node_id(&mut self, staked_node_id: u64) -> &mut Self {
        self.contract_data.staked_id = Some(StakedId::NodeId(staked_node_id));

        self
    }

    /// Sets the client to use for freezing the generated *``ContractCreateTransaction``*.
    ///
    /// By default freezing will use the client provided to ``execute``.
    ///
    /// Note: This *only* affects the ``ContractCreateTransaction`` currently, that is not guaranteed to always be the case.
    pub fn freeze_with(&mut self, client: Client) -> &mut Self {
        self.contract_data.freeze_with_client = Some(client);

        self
    }

    /// Sets the signer for use in the ``ContractCreateTransaction``
    ///
    /// Important: Only *one* signer is allowed.
    pub fn sign(&mut self, key: PrivateKey) -> &mut Self {
        self.contract_data.signer = Some(AnySigner::PrivateKey(key));

        self
    }

    /// Sets the signer for use in the ``ContractCreateTransaction``
    ///
    /// Important: Only *one* signer is allowed.
    pub fn sign_with<F: Fn(&[u8]) -> Vec<u8> + Send + Sync + 'static>(
        &mut self,
        public_key: PublicKey,
        signer: F,
    ) -> &mut Self {
        self.contract_data.signer = Some(AnySigner::arbitrary(Box::new(public_key), signer));

        self
    }

    /// Generates the required transactions and executes them all.
    pub async fn execute(&self, client: &Client) -> crate::Result<TransactionResponse> {
        self.execute_with_optional_timeout(client, None).await
    }

    /// Generates the required transactions and executes them all.
    pub async fn execute_with_timeout(
        &self,
        client: &Client,
        timeout_per_transaction: std::time::Duration,
    ) -> crate::Result<TransactionResponse> {
        self.execute_with_optional_timeout(client, Some(timeout_per_transaction)).await
    }

    async fn execute_with_optional_timeout(
        &self,
        client: &Client,
        timeout_per_transaction: Option<std::time::Duration>,
    ) -> crate::Result<TransactionResponse> {
        // todo: proper error
        let operator_public_key = client
            .load_operator()
            .as_deref()
            .map(|it| it.signer.public_key())
            .expect("Must call `Client.set_operator` to use contract create flow");

        let bytecode = split_bytecode(&self.bytecode);
        let file_id = make_file_create_transaction(
            bytecode.0,
            operator_public_key,
            self.node_account_ids.clone(),
        )
        .execute_with_optional_timeout(client, timeout_per_transaction)
        .await?
        .get_receipt_query()
        .execute_with_optional_timeout(client, timeout_per_transaction)
        .await?
        .file_id
        .expect("Creating a file means there's a file ID");

        if let Some(file_append_bytecode) = bytecode.1 {
            // note: FileAppendTransaction already waits for receipts, so we don't need to wait for one before executing the ContractCreateTransaction.
            make_file_append_transaction(
                file_id,
                file_append_bytecode,
                self.file_append_max_chunks,
                self.node_account_ids.clone(),
            )
            .execute_all_with_optional_timeout(client, timeout_per_transaction)
            .await?;
        }

        let response = make_contract_create_transaction(
            file_id,
            &self.contract_data,
            self.node_account_ids.clone(),
        )?
        .execute_with_optional_timeout(client, timeout_per_transaction)
        .await?;

        response
            .get_receipt_query()
            .execute_with_optional_timeout(client, timeout_per_transaction)
            .await?;

        // todo: Should this return `response` even if this fails?
        make_file_delete_transaction(file_id, self.node_account_ids.clone())
            .execute_with_optional_timeout(client, timeout_per_transaction)
            .await?
            .get_receipt_query()
            .execute_with_optional_timeout(client, timeout_per_transaction)
            .await?;

        Ok(response)
    }
}

// Not to be confused with ContractCreateTrasnactionData which is missing a couple fields.
#[derive(Default, Debug)]
struct ContractData {
    constructor_parameters: Vec<u8>,
    gas: u64,
    initial_balance: Hbar,
    max_automatic_token_associations: u32,
    decline_staking_reward: bool,
    admin_key: Option<Key>,
    //  proxy_account_id: Option<AccountId>
    auto_renew_account_id: Option<AccountId>,
    auto_renew_period: Option<time::Duration>,
    contract_memo: Option<String>,
    staked_id: Option<StakedId>,
    freeze_with_client: Option<Client>,
    signer: Option<AnySigner>,
}

fn split_bytecode(bytecode: &[u8]) -> (Vec<u8>, Option<Vec<u8>>) {
    const MAX_FILE_CREATE_DATA_BYTES: usize = 2048;

    let bytecode = hex::encode(bytecode).into_bytes();

    if bytecode.len() <= MAX_FILE_CREATE_DATA_BYTES {
        return (bytecode, None);
    }

    let mut file_create_bytecode = bytecode;
    let file_append_bytecode = file_create_bytecode.split_off(MAX_FILE_CREATE_DATA_BYTES);

    // note: this uses `subdata` because `Data` is it's own subsequence...
    // It's weirdly written such that the `fileAppendData` wouldn't start at index 0
    // even though that's literally what you'd expect.
    (file_create_bytecode, Some(file_append_bytecode))
}

fn make_file_create_transaction(
    bytecode: Vec<u8>,
    key: PublicKey,
    node_account_ids: Option<Vec<AccountId>>,
) -> FileCreateTransaction {
    let mut tmp = FileCreateTransaction::new();

    tmp.contents(bytecode).keys([key]);

    if let Some(node_account_ids) = node_account_ids {
        tmp.node_account_ids(node_account_ids);
    }

    tmp
}

fn make_file_append_transaction(
    file_id: FileId,
    bytecode: Vec<u8>,
    max_chunks: Option<usize>,
    node_account_ids: Option<Vec<AccountId>>,
) -> FileAppendTransaction {
    let mut tmp = FileAppendTransaction::new();

    tmp.file_id(file_id).contents(bytecode);

    if let Some(max_chunks) = max_chunks {
        tmp.max_chunks(max_chunks);
    }

    if let Some(node_account_ids) = node_account_ids {
        tmp.node_account_ids(node_account_ids);
    }

    tmp
}

fn make_contract_create_transaction(
    file_id: FileId,
    data: &ContractData,
    node_account_ids: Option<Vec<AccountId>>,
) -> crate::Result<ContractCreateTransaction> {
    let mut tmp = ContractCreateTransaction::new();

    tmp.bytecode_file_id(file_id)
        .constructor_parameters(data.constructor_parameters.clone())
        .gas(data.gas)
        .initial_balance(data.initial_balance)
        .max_automatic_token_associations(data.max_automatic_token_associations)
        .decline_staking_reward(data.decline_staking_reward);

    if let Some(admin_key) = &data.admin_key {
        tmp.admin_key(admin_key.clone());
    }

    // if let Some(proxy_account_id) = data.proxy_account_id {
    //     tmp.proxy_account_id(proxy_account_id);
    // }

    if let Some(auto_renew_account_id) = data.auto_renew_account_id {
        tmp.auto_renew_account_id(auto_renew_account_id);
    }

    if let Some(auto_renew_period) = data.auto_renew_period {
        tmp.auto_renew_period(auto_renew_period);
    }

    if let Some(contract_memo) = &data.contract_memo {
        tmp.contract_memo(contract_memo.clone());
    }

    match data.staked_id {
        Some(StakedId::AccountId(account_id)) => {
            tmp.staked_account_id(account_id);
        }
        Some(StakedId::NodeId(node_id)) => {
            tmp.staked_node_id(node_id);
        }
        None => {}
    }

    if let Some(node_account_ids) = node_account_ids {
        tmp.node_account_ids(node_account_ids);
    }

    if let Some(client) = &data.freeze_with_client {
        tmp.freeze_with(client)?;
    }

    if let Some(signer) = &data.signer {
        tmp.sign_signer(signer.clone());
    }

    Ok(tmp)
}

fn make_file_delete_transaction(
    file_id: FileId,
    node_account_ids: Option<Vec<AccountId>>,
) -> FileDeleteTransaction {
    let mut tmp = FileDeleteTransaction::new();

    tmp.file_id(file_id);

    if let Some(node_account_ids) = node_account_ids {
        tmp.node_account_ids(node_account_ids);
    }

    tmp
}
