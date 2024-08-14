use fuels::{
    prelude::{abigen, AssetId, TxPolicies, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
};

abigen!(
    Contract(
        name = "MiraAMM",
        abi = "./contracts/mira_amm_contract/out/debug/mira_amm_contract-abi.json"
    ),
    Contract(
        name = "MockToken",
        abi = "./contracts/mocks/mock_token/out/debug/mock_token-abi.json"
    )
);

pub mod amm {
    use fuels::types::{Bits256, Bytes, ContractId, Identity};

    use crate::types::PoolId;

    use super::*;

    pub async fn create_pool(
        contract: &MiraAMM<WalletUnlocked>,
        token_contract: &MockToken<WalletUnlocked>,
        token_0_contract_id: ContractId,
        token_0_sub_id: Bits256,
        token_1_contract_id: ContractId,
        token_1_sub_id: Bits256,
        is_stable: bool,
    ) -> FuelCallResponse<PoolId> {
        contract
            .methods()
            .create_pool(
                token_0_contract_id,
                token_0_sub_id,
                token_1_contract_id,
                token_1_sub_id,
                is_stable,
            )
            .with_contracts(&[token_contract])
            .call()
            .await
            .unwrap()
    }

    pub async fn pool_metadata(
        contract: &MiraAMM<WalletUnlocked>,
        pool_id: PoolId,
    ) -> FuelCallResponse<Option<PoolMetadata>> {
        contract.methods().pool_metadata(pool_id).call().await.unwrap()
    }

    pub async fn pools(contract: &MiraAMM<WalletUnlocked>) -> FuelCallResponse<Vec<PoolId>> {
        contract.methods().pools().call().await.unwrap()
    }

    pub async fn fees(
        contract: &MiraAMM<WalletUnlocked>,
    ) -> FuelCallResponse<(u64, u64, u64, u64)> {
        contract.methods().fees().call().await.unwrap()
    }

    pub async fn mint(
        contract: &MiraAMM<WalletUnlocked>,
        pool_id: PoolId,
        to: Identity,
    ) -> FuelCallResponse<Asset> {
        contract.methods().mint(pool_id, to).call().await.unwrap()
    }

    pub async fn burn(
        contract: &MiraAMM<WalletUnlocked>,
        pool_id: PoolId,
        to: Identity,
    ) -> FuelCallResponse<(u64, u64)> {
        contract.methods().burn(pool_id, to).call().await.unwrap()
    }

    pub async fn swap(
        contract: &MiraAMM<WalletUnlocked>,
        pool_id: PoolId,
        amount_0_out: u64,
        amount_1_out: u64,
        to: Identity,
        data: Bytes,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .swap(pool_id, amount_0_out, amount_1_out, to, data)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }
}

pub mod mock {
    use fuels::{
        programs::contract::{Contract, LoadConfiguration},
        types::{Bits256, ContractId},
    };

    use crate::paths::MOCK_TOKEN_CONTRACT_BINARY_PATH;

    use super::*;

    pub async fn deploy_mock_token_contract(
        wallet: &WalletUnlocked,
    ) -> (ContractId, MockToken<WalletUnlocked>) {
        let contract_id =
            Contract::load_from(MOCK_TOKEN_CONTRACT_BINARY_PATH, LoadConfiguration::default())
                .unwrap()
                .deploy(wallet, TxPolicies::default())
                .await
                .unwrap();

        let id = ContractId::from(contract_id.clone());
        let instance = MockToken::new(contract_id, wallet.clone());

        (id, instance)
    }

    pub async fn add_token(
        contract: &MockToken<WalletUnlocked>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> FuelCallResponse<AssetId> {
        contract.methods().add_token(name, symbol, decimals).call().await.unwrap()
    }

    pub async fn mint_tokens(
        contract: &MockToken<WalletUnlocked>,
        asset_id: AssetId,
        amount: u64,
    ) -> FuelCallResponse<()> {
        contract
            .methods()
            .mint_tokens(asset_id, amount)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_sub_id(
        contract: &MockToken<WalletUnlocked>,
        asset_id: AssetId,
    ) -> FuelCallResponse<Option<Bits256>> {
        contract.methods().get_sub_id(asset_id).call().await.unwrap()
    }
}