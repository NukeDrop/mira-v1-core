use fuels::{
    accounts::wallet::WalletUnlocked,
    types::{AssetId, Bits256, ContractId},
};
use test_harness::{
    data_structures::WalletAssetConfiguration,
    interface::{
        mock::{add_token, deploy_mock_token_contract, get_sub_id},
        MiraAMM, MockToken,
    },
    setup::common::{deploy_amm, setup_wallet_and_provider},
    utils::common::{order_sub_ids, order_token_ids},
};

pub async fn setup() -> (
    MiraAMM<WalletUnlocked>,
    WalletUnlocked,
    ContractId,
    MockToken<WalletUnlocked>,
    (AssetId, AssetId),
    (Bits256, Bits256),
) {
    let (wallet, _asset_ids, _provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;
    let amm = deploy_amm(&wallet).await;
    let (token_contract_id, token_contract) = deploy_mock_token_contract(&wallet).await;

    let token_a_id =
        add_token(&token_contract, "TOKEN_A".to_string(), "TKA".to_string(), 9).await.value;
    let token_b_id =
        add_token(&token_contract, "TOKEN_B".to_string(), "TKB".to_string(), 9).await.value;

    let token_a_sub_id = get_sub_id(&token_contract, token_a_id).await.value.unwrap();
    let token_b_sub_id = get_sub_id(&token_contract, token_b_id).await.value.unwrap();

    let (token_a_sub_id, token_b_sub_id) =
        order_sub_ids((token_a_id, token_b_id), (token_a_sub_id, token_b_sub_id));
    let (token_a_id, token_b_id) = order_token_ids((token_a_id, token_b_id));

    (
        amm.instance,
        wallet,
        token_contract_id,
        token_contract,
        (token_a_id, token_b_id),
        (token_a_sub_id, token_b_sub_id),
    )
}