use fuels::prelude::*;
use fuels::signers::wallet::Wallet;
use fuels::tx::{default_parameters::MAX_GAS_PER_TX, ContractId};
use fuels_abigen_macro::abigen;

abigen!(
    AttackerContract,
    "test_artifacts/reentrancy_attacker_contract/out/debug/reentrancy_attacker_contract-abi.json",
);

abigen!(
    TargetContract,
    "test_artifacts/reentrancy_target_contract/out/debug/reentrancy_target_contract-abi.json",
);

#[tokio::test]
async fn can_detect_reentrancy() {
    let wallet = launch_provider_and_get_single_wallet().await;
    let (attacker_instance, _) = get_attacker_instance(wallet.clone()).await;
    let (_, target_id) = get_target_instance(wallet).await;

    let result = attacker_instance
        .launch_attack(target_id)
        .set_contracts(&[target_id])
        .tx_params(TxParameters::new(Some(0), Some(MAX_GAS_PER_TX), None, None))
        .call()
        .await
        .unwrap();

    assert_eq!(result.value, true);
}

#[tokio::test]
#[should_panic(expected = "Revert(0)")]
async fn can_block_reentrancy() {
    let wallet = launch_provider_and_get_single_wallet().await;
    let (attacker_instance, _) = get_attacker_instance(wallet.clone()).await;
    let (_, target_id) = get_target_instance(wallet).await;

    attacker_instance
        .launch_thwarted_attack_1(target_id)
        .set_contracts(&[target_id])
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "Revert(0)")]
async fn can_block_cross_function_reentrancy() {
    let wallet = launch_provider_and_get_single_wallet().await;
    let (attacker_instance, _) = get_attacker_instance(wallet.clone()).await;
    let (_, target_id) = get_target_instance(wallet).await;

    attacker_instance
        .launch_thwarted_attack_2(target_id)
        .set_contracts(&[target_id])
        .call()
        .await
        .unwrap();
}

#[tokio::test]
async fn can_call_guarded_function() {
    let wallet = launch_provider_and_get_single_wallet().await;
    let (attacker_instance, _) = get_attacker_instance(wallet.clone()).await;
    let (_, target_id) = get_target_instance(wallet).await;

    let result = attacker_instance
        .innocent_call(target_id)
        .set_contracts(&[target_id])
        .call()
        .await
        .unwrap();

    assert_eq!(result.value, true)
}

async fn get_attacker_instance(wallet: Wallet) -> (AttackerContract, ContractId) {
    let id = Contract::deploy(
        "test_artifacts/reentrancy_attacker_contract/out/debug/reentrancy_attacker_contract.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let instance = AttackerContract::new(id.to_string(), wallet);

    (instance, id)
}

async fn get_target_instance(wallet: Wallet) -> (TargetContract, ContractId) {
    let id = Contract::deploy(
        "test_artifacts/reentrancy_target_contract/out/debug/reentrancy_target_contract.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let instance = TargetContract::new(id.to_string(), wallet);

    (instance, id)
}
