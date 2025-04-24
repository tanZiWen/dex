use dex::Token;
use dex::Dex;
use dex::Token1;

use placeholder_project_name_placeholder_sdk::Contract;
use placeholder_project_name_placeholder_sdk::Error;
use placeholder_project_name_placeholder_sdk::Io;
use placeholder_project_name_placeholder_sdk::Sdk;
use placeholder_project_name_placeholder_sdk::stdlib::Wallet;
use placeholder_project_name_placeholder_zk::field::goldilocks_field::GoldilocksField;
use placeholder_project_name_placeholder_zk::hash::poseidon::PoseidonHash;
use placeholder_project_name_placeholder_zk::plonk::config::Hasher;

fn main() -> Result<(), Error> {
    let password_a = "1234";
    let password_b = "2345";
    let wallet_a = Wallet::new_by_str_with_rand_salt(password_a);
    let wallet_b = Wallet::new_by_str_with_rand_salt(password_b);
    let token = Token::new();
    let token_contract = token.contract;

    let token_1 = Token1::new();
    let token_contract_1 = token_1.contract;

    println!("token_contract {:?}", token_contract.address());
    println!("token_contract_1 {:?}", token_contract_1.address());

    let dex = Dex::new();
    let dex_contract = dex.contract;

    let total_supply = 10000000000 * 10_i32.pow(9) as u128;
    let amount = Sdk::u128_to_field(total_supply);

    // Initialize token contract
    let (nonce, proof) = wallet_a.approve_by_str(token_contract.address(), wallet_a.address(), password_a)?;
    let tx = token_contract.tx(Default::default(), |args| {
        args.set_item("entry", token.init_address);
        args.set_vk("sender", wallet_a.vk().into());
        args.set_proof("proof", proof.into());
        args.set_item("amount", amount);
        args.set_item("nonce", nonce);
    })?;
    Sdk::transit(tx)?;

    let value = 1000 * 10_i32.pow(9) as u128;
    let amount = Sdk::u128_to_field(value);
    let (nonce_inbox, proof) = wallet_a.approve_by_str(token_contract.address(), wallet_a.address(), password_a)?;
    let tx = token_contract.tx(Default::default(), |args| {
        args.set_item("entry", token.transfer_address);
        args.set_vk("sender", wallet_a.vk().into());
        args.set_proof("proof", proof.into());
        args.set_item("receiver", wallet_b.address());
        args.set_item("amount", amount);
        args.set_item("nonce", nonce_inbox);
    })?;
    Sdk::transit(tx)?;
    let (nonce, proof) = wallet_b.approve_by_str(token_contract.address(), wallet_b.address(), password_b)?;
    let tx = token_contract.tx(Default::default(), |args| {
        args.set_item("entry", token.collect_address);
        args.set_item("sender", wallet_a.address());
        args.set_proof("proof", proof.into());
        args.set_vk("receiver", wallet_b.vk().into());
        args.set_item("inbox_nonce", nonce_inbox);
        args.set_item("nonce", nonce);
    })?;
    Sdk::transit(tx)?;
    Sdk::localstate(|state| {
        eprintln!("{:?} {:?}", state.lastsnap(), state.lastsnaproot());
        let hkey_prefix_balance = Sdk::hash_of_str("balance");
        let hkey_balance = Sdk::hash2to1(hkey_prefix_balance, wallet_a.address());
        eprintln!("wallet_a balance {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract.address(), hkey_balance).unwrap()));
        let hkey_prefix_balance = Sdk::hash_of_str("balance");
        let hkey_balance = Sdk::hash2to1(hkey_prefix_balance, wallet_b.address());
        eprintln!("wallet_b balance {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract.address(), hkey_balance).unwrap()));
        let hkey_total_supply = Sdk::hash_of_str("total_supply");
        eprintln!("total_supply {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract.address(), hkey_total_supply).unwrap()));
        let hkey_name = Sdk::hash_of_str("name");
        eprintln!("name {:?}", Sdk::field_to_string(state.lastslotitem(token_contract.address(), hkey_name).unwrap()));
        Ok(())
    })?;

    // Initialize token_1 contract
    let total_supply = 10000000000 * 10_i32.pow(9) as u128;
    let amount = Sdk::u128_to_field(total_supply);

    let (nonce, proof) = wallet_a.approve_by_str(token_contract_1.address(), wallet_a.address(), password_a)?;
    let tx = token_contract_1.tx(Default::default(), |args| {
        args.set_item("entry", token_1.init_address);
        args.set_vk("sender", wallet_a.vk().into());
        args.set_proof("proof", proof.into());
        args.set_item("amount", amount);
        args.set_item("nonce", nonce);
    })?;
    Sdk::transit(tx)?;

    let value = 1000 * 10_i32.pow(9) as u128;
    let amount = Sdk::u128_to_field(value);
    let (nonce_inbox, proof) = wallet_a.approve_by_str(token_contract_1.address(), wallet_a.address(), password_a)?;
    let tx = token_contract_1.tx(Default::default(), |args| {
        args.set_item("entry", token_1.transfer_address);
        args.set_vk("sender", wallet_a.vk().into());
        args.set_proof("proof", proof.into());
        args.set_item("receiver", wallet_b.address());
        args.set_item("amount", amount);
        args.set_item("nonce", nonce_inbox);
    })?;
    Sdk::transit(tx)?;

    let (nonce, proof) = wallet_b.approve_by_str(token_contract_1.address(), wallet_b.address(), password_b)?;
    let tx = token_contract_1.tx(Default::default(), |args| {
        args.set_item("entry", token_1.collect_address);
        args.set_item("sender", wallet_a.address());
        args.set_proof("proof", proof.into());
        args.set_vk("receiver", wallet_b.vk().into());
        args.set_item("inbox_nonce", nonce_inbox);
        args.set_item("nonce", nonce);
    })?;
    Sdk::transit(tx)?;
    Sdk::localstate(|state| {
        eprintln!("{:?} {:?}", state.lastsnap(), state.lastsnaproot());
        let hkey_prefix_balance = Sdk::hash_of_str("balance");
        let hkey_balance = Sdk::hash2to1(hkey_prefix_balance, wallet_a.address());
        eprintln!("token_contract_1 wallet_a balance {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract_1.address(), hkey_balance).unwrap()));
        let hkey_prefix_balance = Sdk::hash_of_str("balance");
        let hkey_balance = Sdk::hash2to1(hkey_prefix_balance, wallet_b.address());
        eprintln!("token_contract_1 wallet_b balance {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract_1.address(), hkey_balance).unwrap()));
        let hkey_total_supply = Sdk::hash_of_str("total_supply");
        eprintln!("token_contract_1 total_supply {:?}", Sdk::field_to_u128(state.lastslotitem(token_contract_1.address(), hkey_total_supply).unwrap()));
        let hkey_name = Sdk::hash_of_str("name");
        eprintln!("token_contract_1 name {:?}", Sdk::field_to_string(state.lastslotitem(token_contract_1.address(), hkey_name).unwrap()));
        Ok(())
    })?;

    let tx = dex_contract.tx(Default::default(), |args| {
        args.set_item("entry", dex.init_address);
        args.set_item("token_0", token_contract.address());
        args.set_item("token_1", token_contract_1.address());
    })?;
    Sdk::transit(tx)?;

    let value = 100 * 10_i32.pow(9) as u128;
    let amount = Sdk::u128_to_field(value);
    let (nonce_inbox, proof) = wallet_b.approve_by_str(token_contract.address(), wallet_b.address(), password_b)?;
    let tx = token_contract.tx(Default::default(), |args| {
        args.set_item("entry", token.transfer_address);
        args.set_vk("sender", wallet_b.vk().into());
        args.set_proof("proof", proof.into());
        args.set_item("receiver", dex_contract.address());
        args.set_item("amount", amount);
        args.set_item("nonce", nonce_inbox);
    })?;
    Sdk::transit(tx)?;

    Ok(())

}
