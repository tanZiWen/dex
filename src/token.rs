use placeholder_project_name_placeholder_sdk::Builder;
use placeholder_project_name_placeholder_sdk::Contract;
use placeholder_project_name_placeholder_sdk::stdlib::Compound;
use placeholder_project_name_placeholder_zk::field::goldilocks_field::GoldilocksField;
use placeholder_project_name_placeholder_sdk::stdlib::Approval;

use std::sync::LazyLock;

pub struct Token {
    pub contract: LazyLock<Contract>,
    pub init_address: [GoldilocksField; 4],
    pub transfer_address: [GoldilocksField; 4],
    pub collect_address: [GoldilocksField; 4],
    pub name_address: [GoldilocksField; 4],
    pub total_supply_address: [GoldilocksField; 4],
    pub decimal_address: [GoldilocksField; 4],
    pub balance_of_address: [GoldilocksField; 4],
}

impl Token {
    pub fn new() -> Self {
       Token {
            contract: LazyLock::new(|| Compound::new_copy_args([&INIT, &TRANSFER, &COLLECT, &NAME, &TOTAL_SUPPLY, &DECIMAL, &BALANCE_OF], [vec!["sender", "proof", "amount", "nonce"], vec!["sender", "proof", "receiver", "amount", "nonce"], vec!["sender", "proof", "receiver", "nonce", "inbox_nonce"], vec![], vec![], vec![], vec![]]).into()),
            init_address: INIT.address(),
            transfer_address: TRANSFER.address(),
            collect_address: COLLECT.address(),
            name_address: NAME.address(),
            total_supply_address: TOTAL_SUPPLY.address(),
            decimal_address: DECIMAL.address(),
            balance_of_address: BALANCE_OF.address(),
        }
    }
}

static INIT: LazyLock<Contract> = LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();
    let vk = builder.arg_vk("sender");
    let proof = builder.arg_proof("proof");
    let addr = builder.addr(vk);
    let amount = builder.arg_item("amount");
    let nonce = builder.arg_item("nonce");
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let hkey_prefix_name = builder.constant_hash_of_str("name");
    let hkey_total_supply = builder.constant_hash_of_str("total_supply");
    let hkey_decimal = builder.constant_hash_of_str("decimal");
    let hkey_balance = builder.hash2to1(hkey_prefix_balance, addr);
    let total_supply = builder.contslotitem(root, hkey_total_supply);
    let decial_default = builder.constant_u128(9);
    let name = builder.constant_str("Test");
    builder.enforce_zeros(total_supply);

    builder.add_diff(hkey_balance, amount);
    builder.add_diff(hkey_total_supply, amount);
    builder.add_diff(hkey_decimal, decial_default);
    builder.add_diff(hkey_prefix_name, name);
    (nonce, addr, vk, proof)
})));

static TRANSFER: LazyLock<Contract> = LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();
    let vk = builder.arg_vk("sender");
    let proof = builder.arg_proof("proof");
    let receiver = builder.arg_item("receiver");
    let amount = builder.arg_item("amount");
    let nonce = builder.arg_item("nonce");
    let addr = builder.addr(vk);

    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox");
    let hkey_balance = builder.hash2to1(hkey_prefix_balance, addr);
    let hkey_1=  builder.hash2to1(hkey_prefix_inbox, addr);
    let hkey_2=  builder.hash2to1(receiver, nonce);
    let hkey_inbox =  builder.hash2to1(hkey_1, hkey_2);
    let balance = builder.contslotitem(root, hkey_balance);

    let balance_updated = builder.u128_sub::<true>(balance, amount);
    builder.add_diff(hkey_balance, balance_updated);
    builder.add_diff(hkey_inbox, amount);
    (nonce, addr, vk, proof)
})));

static COLLECT: LazyLock<Contract> = LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();
    let sender = builder.arg_item("sender");
    let vk = builder.arg_vk("receiver");
    let proof = builder.arg_proof("proof");
    let nonce = builder.arg_item("nonce");
    let addr = builder.addr(vk);
    let inbox_nonce = builder.arg_item("inbox_nonce");

    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox");
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let hkey_1=  builder.hash2to1(hkey_prefix_inbox, sender);
    let hkey_2=  builder.hash2to1(addr, inbox_nonce);
    let hkey_inbox =  builder.hash2to1(hkey_1, hkey_2);
    let hkey_balance = builder.hash2to1(hkey_prefix_balance, addr);
    let balance = builder.contslotitem(root, hkey_balance);
    let amount = builder.contslotitem(root, hkey_inbox);

    let balance_updated = builder.u128_add::<true>(balance, amount);

    let zero = builder.constant_zeros();
    builder.add_diff(hkey_balance, balance_updated);
    builder.add_diff(hkey_inbox, zero);
    (nonce, addr, vk, proof)
})));

static NAME: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let hkey_prefix_name = builder.constant_hash_of_str("name");
    let hkey_name = builder.hash(hkey_prefix_name);
    let name = builder.contslotitem(root, hkey_name);
    builder.add_diff(hkey_name, name);
    builder.into()
});

static TOTAL_SUPPLY: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let hkey_prefix_total_supply = builder.constant_hash_of_str("total_supply");
    let total_supply = builder.contslotitem(root, hkey_prefix_total_supply);
    builder.add_diff(hkey_prefix_total_supply, total_supply);
    builder.into()
});

static DECIMAL: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let hkey_prefix_decimal = builder.constant_hash_of_str("decimal");
    let decimal = builder.contslotitem(root, hkey_prefix_decimal);
    builder.add_diff(hkey_prefix_decimal, decimal);
    builder.into()
});

static BALANCE_OF: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let addr = builder.arg_item("address");
    let hkey_balance = builder.hash2to1(hkey_prefix_balance, addr);
    let balance = builder.contslotitem(root, hkey_balance);
    builder.add_diff(hkey_balance, balance);
    builder.into()
});
