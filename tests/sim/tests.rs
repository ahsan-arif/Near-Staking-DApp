use near_sdk::{json_types::U128};
use near_sdk_sim::{call, to_yocto, transaction::ExecutionStatus, view};
use std::{thread, time};

use crate::utils::{init, register_user};

#[test]
fn simulate_total_supply() {
    let initial_balance = to_yocto("100");

    let (_, ftt, _, _) = init(initial_balance);

    let total_supply: U128 = view!(ftt.ft_total_supply()).unwrap_json();
    assert_eq!(initial_balance, total_supply.0);
}
#[test]
fn simulate_token_transfer() {
    let amount = to_yocto("2000");
    let initial_balance = to_yocto("1000000");
    let (root, ft, _, alice) = init(initial_balance);
    //===> With Macro<========//
    call!(
        root,
        ft.ft_transfer(alice.account_id(), amount.into(), None),
        deposit = 1
    )
    .assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("root balance {:?}", root_balance);
    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("alice balance {:?}", _alice_balance);
    assert_eq!(initial_balance - amount, root_balance.0);
}

#[test]
#[should_panic(expected = "Reward can be claimed after staking for 30 days")]
pub fn stimulate_claim_reward() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    //Adding users to Whitelist
    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();

    let _root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", root_balance);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance from root = {:?}", _alice_balance);
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance after stake = {:?}", _alice_balance);

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("root balance  {:?}", root_balance);
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    // println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    // let num: U128 = "1".to_string();
    thread::sleep(ten_millis);
    // call!(alice, staking.ft_unstake(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance After Unstake = {:?}", _alice_balance);

    // let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();

    // assert_eq!(amount, _alice_balance.0);
    let id: U128 = U128::from(1);
    call!(alice, staking.claim_reward(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance After Unstake = {:?}", _alice_balance);
}
//<=======================>//
//    STAKING TEST CASES   //
//<=======================>//
#[test]
pub fn stimulate_staking_fungible_tokens() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    assert!(res.is_ok());

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    // println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);
}
#[test]
pub fn check_minimum_limit_staking() {
    let amount = to_yocto("3000");
    let initial_balance = to_yocto("3000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    assert!(res.is_ok());

    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("Cannot stake less than 5000000000000000000000000000 tokens"));
    } else {
        unreachable!();
    }
    // println!("promise error starts{:#?}", res.promise_errors());
}

#[test]
pub fn check_min_staking_duration() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":1577880,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    assert!(res.is_ok());

    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("Invalid Duration"));
    } else {
        unreachable!();
    }
    // println!("promise error starts{:#?}", res.promise_errors());
}

#[test]
pub fn check_invalid_staking_arguments() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_\":\"ft\",\"decimal\":24,\"duration\":1577880,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    assert!(res.is_ok());

    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("Invalid Staking Argument"));
    } else {
        unreachable!();
    }
    // println!("promise error starts{:#?}", res.promise_errors());
}

#[test]
#[ignore]
pub fn check_approved_ft_tokens() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_\":\"ftt\",\"decimal\":24,\"duration\":1577880,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    assert!(res.is_ok());

    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("Only approved FT can be staked"));
    } else {
        unreachable!();
    }
    // println!("promise error starts{:#?}", res.promise_errors());
}

#[test]
pub fn check_staking_plan_invalid() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);

    //===> With Macro<========//
    let res=call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium\"}".to_string()),
    deposit =1);
    println!("alice transaction receipt{:#?}", res.promise_results());
    assert!(res.is_ok());

    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        //Because with wrong plan there is no Apy exists.
        assert!(execution_error.to_string().contains("None"));
    } else {
        unreachable!();
    }
    // println!("promise error starts{:#?}", res.promise_errors());
}
//<=============================>//
//    CLAIM REWARD TEST CASES   //
//<============================>//
#[test]
#[should_panic(expected = "No staking data with this id found for caller")]
pub fn check_stake_id_for_claim_reward() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);
    let id: U128 = U128::from(2);
    call!(alice, staking.claim_reward(id)).assert_success();
}

#[test]
#[should_panic(expected = "This user has not staked yet.")]
pub fn check_user_staked_for_claim_reward() {
    let initial_balance = to_yocto("6000");
    let (_, _, staking, alice) = init(initial_balance);

    let id: U128 = U128::from(2);
    call!(alice, staking.claim_reward(id)).assert_success();
}

#[test]
#[should_panic(expected = "Reward can be claimed after staking for 30 days")]
pub fn check_claim_reward_duration() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);
    //adding users to whitelist
    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();

    register_user(&staking.user_account);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);
    let id: U128 = U128::from(1);

    call!(alice, staking.claim_reward(id)).assert_success();
}
#[test]
#[ignore = "Time Duration of Staked tokens is less than expected to claim reward"]
pub fn check_claim_reward_() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    thread::sleep(ten_millis);
    let id: U128 = U128::from(1);

    call!(alice, staking.claim_reward(id)).assert_success();
}

#[test]
pub fn check_claim_authentication() {
    let initial_balance = to_yocto("6000");
    let (_root, _ft, staking, alice) = init(initial_balance);
    let id: U128 = U128::from(1);

    let res = call!(alice, staking.claim_reward(id));
    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("This user has not staked yet."));
    } else {
        unreachable!();
    }
}
//<=========================>//
//    UNSTAKING TEST CASES   //
//<=========================>//
#[test]
#[should_panic(expected = "No staking data with this id found for caller")]
pub fn check_stake_id_for_un_staking() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    //Adding users to whitelist
    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();

    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);
    let id: U128 = U128::from(2);

    call!(alice, staking.ft_unstake(id)).assert_success();
}
#[ignore = "Cannot reached at that assert"]
#[test]
pub fn check_who_can_unstake() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("12000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();
    call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(0, root_balance.0);
    assert_eq!(amount + amount, staking_balance.0);
    let id: U128 = U128::from(1);

    call!(root, staking.ft_unstake(id)).assert_success();
}

#[test]
#[should_panic(expected = "Cannot withdraw before locked time")]
pub fn check_duration_of_unstaking() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);
    //adding users to whitelist
    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();
    register_user(&staking.user_account);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);
    let id: U128 = U128::from(1);
    //Time duration will no meet
    call!(alice, staking.ft_unstake(id)).assert_success();
}

#[test]
#[should_panic(expected = "None")]
pub fn check_user_who_not_staker_but_unstaking() {
    let initial_balance = to_yocto("6000");
    let (root, _, staking, alice) = init(initial_balance);
    //Adding Caller to Whitelist
    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();
    let id: U128 = U128::from(1);
    call!(alice, staking.ft_unstake(id)).assert_success();
}
#[test]
#[ignore = "Time Duration of Unstaking is less than expected"]
pub fn stimulate_unstake_fungible_token() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    let _root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", root_balance);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance from root = {:?}", _alice_balance);
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance after stake = {:?}", _alice_balance);

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("root balance  {:?}", root_balance);
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    // println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    // let num: U128 = "1".to_string();
    thread::sleep(ten_millis);
    let id: U128 = U128::from(1);
    call!(alice, staking.ft_unstake(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance After Unstake = {:?}", _alice_balance);

    // let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();

    assert_eq!(amount, _alice_balance.0);
}

#[test]
pub fn stimulate_get_staking_history() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("50000");
    let (root, ft, staking, _) = init(initial_balance);

    register_user(&staking.user_account);
    let _root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", _root_balance);
    //===>With Macro<========//
    // call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()), deposit=1).assert_success();
    call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()), deposit=1);
    call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()), deposit=1);

    let _id = root.account_id();
    // println!("Id : {}", _id);
    let index = U128::from(0);
    let _staking_history =
        view!(staking.get_staking_history(root.account_id(), Some(index), Some(1)))
            .unwrap_json_value()
            .to_string();
    println!("stake history = {:#?}", _staking_history);
    assert!(_staking_history.contains("{\"amount\":\"6000000000000000000000000000\",\"decimal\":24,\"duration\":15778800,\"ft_account_id\":\"ft\",\"ft_symbol\":\"BKRT\",\"stake_id\":\"1\",\"staked_at\":17,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"},{\"amount\":\"6000000000000000000000000000\",\"decimal\":24,\"duration\":15778800,\"ft_account_id\":\"ft\",\"ft_symbol\":\"BKRT\",\"stake_id\":\"2\",\"staked_at\":22,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}"));
}

#[test]
pub fn check_whitelist_insertion_by_user() {
    let initial_balance = to_yocto("10000");
    let (root, _ft, staking, alice) = init(initial_balance);

    let whitelist_account_address = root.account_id();
    //method called by a user who is not the owner of the contract.
    let res = call!(
        alice,
        staking.whitelist_address_insert(whitelist_account_address)
    );
    println!("alice transaction receipt{:#?}", res.get_receipt_results());
    //contract should panic because caller is not an onwer.
    if let ExecutionStatus::Failure(execution_error) =
        &res.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error.to_string().contains("Owner's method"));
    } else {
        unreachable!();
    }
}

#[test]
pub fn check_whitelist_insertion_by_owner() {
    let initial_balance = to_yocto("10000");
    let (root, _ft, staking, _alice) = init(initial_balance);

    let whitelist_account_address = root.account_id();
    call!(
        root,
        staking.whitelist_address_insert(whitelist_account_address)
    )
    .assert_success();
    let _from_index = U128::from(0);
    let x = view!(staking.whitelist_addresses_get(Some(_from_index), Some(2))).unwrap_json_value();

    //assertion to check that if it contains our expected result
    let _getting_whitelist_addresses = x.to_string();
    assert!(_getting_whitelist_addresses.contains("root"));
}

#[test]
pub fn check_whitelist_view_function_with_pagination() {
    let initial_balance = to_yocto("1000");
    let (root, _ft, staking, alice) = init(initial_balance);

    call!(root, staking.whitelist_address_insert(alice.account_id())).assert_success();
    call!(root, staking.whitelist_address_insert(root.account_id())).assert_success();
    let _from_index = U128::from(0);
    let x = view!(staking.whitelist_addresses_get(Some(_from_index), Some(3))).unwrap_json_value();
    let _getting_whitelist_addresses = x.to_string();
    // /assertion to check that if it contains our expected result
    assert!(_getting_whitelist_addresses.contains("root"));
    assert!(_getting_whitelist_addresses.contains("alice"));
}
