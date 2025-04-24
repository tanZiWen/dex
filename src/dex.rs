use placeholder_project_name_placeholder_sdk::Builder;
use placeholder_project_name_placeholder_sdk::Contract;
use placeholder_project_name_placeholder_sdk::stdlib::Compound;
use placeholder_project_name_placeholder_zk::field::goldilocks_field::GoldilocksField;

use placeholder_project_name_placeholder_zk::iop::target::Target;
use placeholder_project_name_placeholder_sdk::stdlib::Approval;

use std::sync::LazyLock;
pub struct Dex {
    pub contract: LazyLock<Contract>,
    pub init_address: [GoldilocksField; 4],
    pub deposit_address: [GoldilocksField; 4],
    pub withdraw_address: [GoldilocksField; 4],
    pub swap_address: [GoldilocksField; 4],
    pub addliquidity_address: [GoldilocksField; 4],
    pub removeliquity_address: [GoldilocksField; 4],
}

impl Dex {
    pub fn new() -> Self {
       Dex {
            contract:  LazyLock::new(||Compound::new_copy_args([&INIT, &DEPOSIT, &DEPOSIT_REDUCE, &DEPOSIT_COLLECT_AUTH, &WITHDRAW, &WITHDRAW_REDUCE, &WITHDRAW_SWAP_AUTH, &SWAP, &ADD_LIQUIDITY, &REMOVE_LIQUIDITY], [vec!["token_0", "token_1"], vec!["sender", "amount", "nonce", "proof", "inbox_nonce", "pool", "flag"], vec!["token"], vec![], vec!["sender", "amount", "nonce", "proof", "pool", "flag"], vec!["token", "nonce"], vec!["sender", "proof", "nonce", "flag_0", "flag_1", "token_0_amount", "token_1_amount"], vec!["sender", "proof", "pool", "nonce", "token_0_amount", "token_1_amount"], vec!["sender", "proof", "nonce", "token_0_amount", "token_1_amount"], vec![]]).into()),
            init_address: INIT.address(),
            deposit_address: DEPOSIT.address(),
            withdraw_address: WITHDRAW.address(),
            swap_address: SWAP.address(),
            addliquidity_address: ADD_LIQUIDITY.address(),
            removeliquity_address: REMOVE_LIQUIDITY.address(),
        }
    }
}

static INIT : LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    
    let token_0 = builder.arg_item("token_0");
    let token_1 = builder.arg_item("token_1");

    let is_equal = builder.equal(token_0, token_1);
    builder.enforce_zeros(is_equal);

    let hkey_prefix_pool = builder.constant_hash_of_str("pool");
    let hkey_pool_1 = builder.hash2to1(hkey_prefix_pool, token_0);
    let hkey_pool =  builder.hash2to1(hkey_pool_1, token_1);    

    let pool = builder.contslotitem(root, hkey_prefix_pool);
    builder.enforce_zeros(pool);
    let zero = builder.constant_zeros();
    let one = builder.constant_ones();

    let token_0_key = builder.hash2to1(hkey_pool, zero);
    let token_1_key = builder.hash2to1(hkey_pool, one);

    builder.add_diff(hkey_prefix_pool, hkey_pool);
    builder.add_diff(token_0_key, token_0);
    builder.add_diff(token_1_key, token_1);

    builder.into()
});

static DEPOSIT: LazyLock<Contract> =  LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();
    let vk = builder.arg_vk("sender");
    let amount = builder.arg_item("amount");
    let nonce = builder.arg_item("nonce");
    let proof = builder.arg_proof("proof");
    let inbox_nonce = builder.arg_item("inbox_nonce");
    let pool_flag = builder.arg_item("flag");
    let this_addr = builder.thisaddr();

    let sender = builder.addr(vk);

    let hkey_prefix_pool = builder.constant_hash_of_str("pool");
    let hkey_pool = builder.contslotitem(root, hkey_prefix_pool);

    let key_token = builder.hash2to1(hkey_pool, pool_flag);
    let token_addr: [Target; 4] = builder.contslotitem(root, key_token);

    let token_root = builder.viewcontroot(token_addr);
    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox");
    let inbox_key = builder.hash2to1(hkey_prefix_inbox, key_token);
    let hkey_1=  builder.hash2to1(hkey_prefix_inbox, sender);
    let hkey_2=  builder.hash2to1(this_addr, inbox_nonce);
    let hkey_inbox =  builder.hash2to1(hkey_1, hkey_2);    

    let inbox = builder.contslotitem(token_root, hkey_inbox);
    let zero = builder.constant_zeros();
    let is_zero = builder.equal(inbox, zero);
    builder.enforce_zeros(is_zero);

    //CHECK AMOUNT
    let is_equal = builder.equal(inbox, amount);
    builder.enforce_ones(is_equal);
    
    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let amount_key = builder.hash2to1(hkey_inbox, zero);
    let addr_key = builder.hash2to1(hkey_inbox, one);
    let token_key = builder.hash2to1(hkey_inbox, two);
    let inbox_nonce_key = builder.hash2to1(hkey_inbox, three);

    builder.add_diff(inbox_key, hkey_inbox);
    builder.add_diff(amount_key, amount);
    builder.add_diff(addr_key, sender);
    builder.add_diff(token_key, token_addr);
    builder.add_diff(inbox_nonce_key, hkey_inbox);
    (nonce, sender, vk, proof)
})));

static DEPOSIT_REDUCE: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let snap = builder.thissnapcontroot();
    let token = builder.arg_item("token");

    let hkey_prefix_deposit = builder.constant_hash_of_str("deposit");
    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox"); 
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");

    let inbox_key = builder.hash2to1(hkey_prefix_inbox, token);

    let inbox_tail = builder.leaf(snap, inbox_key);
    let (inbox_prev, inbox_item) = builder.slotnode(inbox_tail);  

    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let amount_key = builder.hash2to1(inbox_item, zero);
    let addr_key = builder.hash2to1(inbox_item, one);
    let token_key = builder.hash2to1(inbox_item, two);
    let inbox_key = builder.hash2to1(inbox_item, three);

    let amount = builder.contslotitem(root, amount_key);
    let addr = builder.contslotitem(root, addr_key);
    let token_addr = builder.contslotitem(root, token_key);
    let hkey_inbox = builder.contslotitem(root, inbox_key);

    let token_root = builder.viewcontroot(token_addr);
    let inbox = builder.contslotitem(token_root, hkey_inbox);
    builder.enforce_zeros(inbox);

    let hkey_1=  builder.hash2to1(hkey_prefix_balance, addr);
    let hkey_balance=  builder.hash2to1(hkey_1, token_addr);
    let hkey_deposit = builder.hash2to1(hkey_prefix_deposit, token_addr);
    let pointer = builder.contslotitem(root, hkey_deposit);
    let balance = builder.contslotitem(root, hkey_balance);

    let balance_updated = builder.u128_add::<true>(balance, amount);

    builder.enforce(pointer, inbox_prev);
    builder.add_diff(hkey_deposit, inbox_item);
    builder.add_diff(hkey_balance, balance_updated);

    builder.into()
});

static DEPOSIT_COLLECT_AUTH: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let snap = builder.thissnapcontroot();
    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox");
    let inbox_tail = builder.leaf(snap, hkey_prefix_inbox);
    let (inbox_prev, inbox_item) = builder.slotnode(inbox_tail);  

    let two = builder.constant_u128(2);
    let token_key = builder.hash2to1(inbox_item, two);
    let token_addr = builder.contslotitem(root, token_key);

    let token_root = builder.viewcontroot(token_addr);
    let prev_item = builder.contslotitem(token_root, inbox_prev);
    builder.enforce_zeros(prev_item);
    
    let current_item = builder.contslotitem(token_root, inbox_item);
    let zero = builder.constant_zeros();
    let is_zero =  builder.equal(current_item, zero);
    builder.enforce_zeros(is_zero);

    let hkey_prefix_deposit = builder.constant_hash_of_str("deposit");
    let hkey_deposit = builder.hash2to1(hkey_prefix_deposit, token_addr);
    let pointer = builder.contslotitem(root, hkey_deposit);
    builder.enforce(pointer, inbox_prev);

    builder.add_diff(hkey_deposit, current_item);
    builder.into()
});

static WITHDRAW: LazyLock<Contract> = LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();

    let vk = builder.arg_vk("sender");
    let amount = builder.arg_item("amount");
    let nonce = builder.arg_item("nonce");
    let proof = builder.arg_proof("proof");
    let key_pool = builder.arg_item("pool");
    let pool_flag = builder.arg_item("flag");
    let this_addr = builder.thisaddr();
    let sender = builder.addr(vk);

    let key_token = builder.hash2to1(key_pool, pool_flag);
    let token_addr: [Target; 4] = builder.contslotitem(this_addr, key_token);

    let hkey_prefix_withdraw = builder.constant_hash_of_str("withdraw");
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let hkey_1=  builder.hash2to1(hkey_prefix_balance, sender);
    let hkey_balance=  builder.hash2to1(hkey_1, token_addr);
    let balance = builder.contslotitem(root, hkey_balance);
    
    let key_1= builder.hash2to1(hkey_prefix_withdraw, sender);
    let key_2= builder.hash2to1(token_addr, amount);
    let key_3 = builder.hash2to1(key_1, key_2);
    let key_4 = builder.hash2to1(key_3, nonce);

    let balance_updated = builder.u128_sub::<true>(balance, amount);

    let hkey_withdraw = builder.hash2to1(hkey_prefix_withdraw, key_token);

    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);


    let key_sender = builder.hash2to1(key_4, zero);
    let key_token = builder.hash2to1(key_4, one);
    let key_amount = builder.hash2to1(key_4, two);
    let key_nonce = builder.hash2to1(key_4, three);
    builder.add_diff(hkey_balance, balance_updated);
    builder.add_diff(hkey_withdraw, key_4);
    builder.add_diff(key_sender, sender);
    builder.add_diff(key_token, token_addr);
    builder.add_diff(key_amount, amount);
    builder.add_diff(key_nonce, nonce);

    (nonce, key_2, vk, proof)
})));

static WITHDRAW_REDUCE: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();

    let snap = builder.thissnapcontroot();

    let token = builder.arg_item("token");

    let hkey_prefix_withdraw = builder.constant_hash_of_str("withdraw");
    let hkey_withdraw = builder.hash2to1(hkey_prefix_withdraw, token);

    let hkey_prefix_withdraw_reduce = builder.constant_hash_of_str("withdraw_reduce");
    let hkey_withdraw_reduce = builder.hash2to1(hkey_prefix_withdraw_reduce, token);

    let inbox_tail = builder.leaf(snap, hkey_withdraw);
    let (inbox_prev, inbox_item) = builder.slotnode(inbox_tail);  

    let pointer = builder.contslotitem(root, hkey_withdraw_reduce);

    let one = builder.constant_ones();
    builder.enforce(pointer, inbox_prev);
    builder.add_diff(hkey_withdraw_reduce, inbox_item);
    builder.add_diff(inbox_item, one);

    builder.into()
});

static WITHDRAW_SWAP_AUTH: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();
    let root = builder.thisviewcontroot();
    let this_addr = builder.thisaddr();

    let snap = builder.thissnapcontroot();

    let token = builder.arg_item("token");

    let hkey_prefix_withdraw = builder.constant_hash_of_str("withdraw");
    let hkey_withdraw = builder.hash2to1(hkey_prefix_withdraw, token);

    let hkey_prefix_withdraw_reduce = builder.constant_hash_of_str("withdraw_reduce");
    let hkey_withdraw_reduce = builder.hash2to1(hkey_prefix_withdraw_reduce, token);

    let inbox_tail = builder.leaf(snap, hkey_withdraw);
    let (inbox_prev, inbox_item) = builder.slotnode(inbox_tail);  

    let pointer = builder.contslotitem(root, hkey_withdraw_reduce);
    builder.enforce(pointer, inbox_prev);
    builder.enforce_zeros(inbox_item);

    let hkey_prefix_inbox = builder.constant_hash_of_str("inbox");
    let zero = builder.constant_zeros();
    let key_receiver = builder.hash2to1(inbox_item, zero);
    let receiver = builder.contslotitem(root, key_receiver);

    let hkey_1=  builder.hash2to1(hkey_prefix_inbox, this_addr);
    let hkey_2=  builder.hash2to1(receiver, nonce);
    let item =  builder.hash2to1(hkey_1, hkey_2);

    builder.add_diff(hkey_withdraw, item);
    builder.into()
});


static SWAP: LazyLock<Contract> =  LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();

    let vk = builder.arg_vk("sender");
    let proof = builder.arg_proof("proof");
    let nonce = builder.arg_item("nonce");
    let flag_0 = builder.arg_item("flag_0");
    let flag_1 = builder.arg_item("flag_1");

    let token_0_amount = builder.arg_item("token_0_amount");
    let token_1_amount = builder.arg_item("token_1_amount");

    let addr = builder.addr(vk);

    let hkey_prefix_inbox    = builder.constant_hash_of_str("liquidity_inbox");
    let swap_liquidity    = builder.constant_hash_of_str("swap_liquidity");
    let hkey_prefix_pool = builder.constant_hash_of_str("pool");

    let key_pool_0 = builder.hash2to1(hkey_prefix_pool, flag_0);
    let token_0 = builder.contslotitem(root, key_pool_0);

    let key_pool_1 = builder.hash2to1(hkey_prefix_pool, flag_1);
    let token_1 = builder.contslotitem(root, key_pool_1);

    let key_inbox_0= builder.hash2to1(hkey_prefix_inbox, addr);
    let key_inbox = builder.hash2to1(key_inbox_0, nonce);

    builder.enforce(token_0_amount, token_1_amount);

    //check user balance 
    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let key_token = builder.hash2to1(hkey_prefix_balance, addr);
    let key_token_balance = builder.hash2to1(key_token, token_0);
    let balance = builder.contslotitem(root, key_token_balance);
    builder.u128_sub::<true>(balance, token_0_amount);

    //check pool reserve balance
    let hkey_prefix_reserve = builder.constant_hash_of_str("reserve");
    let key_token = builder.hash2to1(hkey_prefix_reserve, token_1);
    let reserve_token = builder.contslotitem(root, key_token);
    builder.u128_sub::<true>(reserve_token, token_1_amount);

    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let key_token_0_amount = builder.hash2to1(key_inbox, zero);
    let key_token_1_amount = builder.hash2to1(key_inbox, one);
    let key_method = builder.hash2to1(key_inbox, two);
    let key_user = builder.hash2to1(key_inbox, three);


    builder.add_diff(hkey_prefix_inbox, key_inbox);
    builder.add_diff(key_token_0_amount, token_0_amount);
    builder.add_diff(key_token_1_amount, token_1_amount);
    builder.add_diff(key_method, swap_liquidity);
    builder.add_diff(key_user, addr);

    (nonce, key_inbox, vk, proof)
})));

static MINIMUM_LIQUIDITY: GoldilocksField = GoldilocksField((10_u128.pow(3)) as u64);

static ADD_LIQUIDITY: LazyLock<Contract> =  LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let vk = builder.arg_vk("sender");
    let proof = builder.arg_proof("proof");
    let key_pool = builder.arg_item("pool");
    let nonce = builder.arg_item("nonce");
    let token_0_amount = builder.arg_item("token_0_amount");
    let token_1_amount = builder.arg_item("token_1_amount");

    let addr = builder.addr(vk);
    let root = builder.thisviewcontroot();

    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let token_0_key = builder.hash2to1(key_pool, zero);
    let token_1_key = builder.hash2to1(key_pool, one);

    let token_0 = builder.contslotitem(root, token_0_key);
    let token_1 = builder.contslotitem(root, token_1_key);

    let hkey_prefix_balance = builder.constant_hash_of_str("balance");
    let hkey_prefix_inbox    = builder.constant_hash_of_str("liquidity_inbox");
    let add_liquidity = builder.constant_hash_of_str("add_liquidiy");


    let key_token_0 = builder.hash2to1(hkey_prefix_balance, addr);
    let key_token_0_balance = builder.hash2to1(key_token_0, token_0);
    
    let key_token_1 = builder.hash2to1(hkey_prefix_balance, addr);
    let key_token_1_balance = builder.hash2to1(key_token_1, token_1);
    let token_0_balance = builder.contslotitem(root, key_token_0_balance);
    let token_1_balance = builder.contslotitem(root, key_token_1_balance);

    let key_inbox_0= builder.hash2to1(hkey_prefix_inbox, addr);
    let key_inbox = builder.hash2to1(key_inbox_0, nonce);

    let token_0_balance_updated: [Target; 4] = builder.u128_sub::<true>(token_0_balance, token_0_amount);
    let token_1_balance_updated: [Target; 4] = builder.u128_sub::<true>(token_1_balance, token_1_amount);

    let key_token_0_amount = builder.hash2to1(key_inbox, zero);
    let key_token_1_amount = builder.hash2to1(key_inbox, one);
    let key_method = builder.hash2to1(key_inbox, two);
    let key_user = builder.hash2to1(key_inbox, three);

    builder.add_diff(key_token_0_balance, token_0_balance_updated);
    builder.add_diff(key_token_1_balance, token_1_balance_updated);
    builder.add_diff(hkey_prefix_inbox, key_inbox);
    builder.add_diff(key_token_0_amount, token_0_amount);
    builder.add_diff(key_token_1_amount, token_1_amount);
    builder.add_diff(key_method, add_liquidity);
    builder.add_diff(key_user, addr);

    (nonce, key_inbox, vk, proof)
})));

static REMOVE_LIQUIDITY:  LazyLock<Contract> =  LazyLock::new(|| Contract::from(Approval::new(|builder| {
    let root = builder.thisviewcontroot();

    let vk = builder.arg_vk("sender");
    let proof = builder.arg_proof("proof");
    let nonce = builder.arg_item("nonce");
    let token_0_amount = builder.arg_item("token_0_amount");
    let token_1_amount = builder.arg_item("token_1_amount");

    let addr = builder.addr(vk);
    let zero = builder.constant_zeros();
    let one = builder.constant_ones();

    let hkey_prefix_inbox    = builder.constant_hash_of_str("liquidity_inbox");
    let remove_liquidity    = builder.constant_hash_of_str("remove_liquidity");

    let hkey_prefix_pool = builder.constant_hash_of_str("pool");

    let key_pool_0 = builder.hash2to1(hkey_prefix_pool, zero);
    let token_0 = builder.contslotitem(root, key_pool_0);

    let key_pool_1 = builder.hash2to1(hkey_prefix_pool, one);
    let token_1 = builder.contslotitem(root, key_pool_1);

    builder.enforce(token_0_amount, token_1_amount);

    //check user liquidity token_0 balance
    let hkey_prefix_balance = builder.constant_hash_of_str("reserve_liquidity");
    let key_token = builder.hash2to1(hkey_prefix_balance, addr);
    let key_token_0_balance = builder.hash2to1(key_token, token_0);
    let reserve_0 = builder.contslotitem(root, key_token_0_balance);
    builder.u128_sub::<true>(reserve_0, token_0_amount);

    //check user liquidity token_1 balance
    let key_token_1_balance = builder.hash2to1(key_token, token_1);
    let reserve_1 = builder.contslotitem(root, key_token_1_balance);
    builder.u128_sub::<true>(reserve_1, token_1_amount);

    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let key_inbox_0= builder.hash2to1(hkey_prefix_inbox, addr);
    let key_inbox = builder.hash2to1(key_inbox_0, nonce);

    let key_token_0_amount = builder.hash2to1(key_inbox, zero);
    let key_token_1_amount = builder.hash2to1(key_inbox, one);
    let key_method = builder.hash2to1(key_inbox, two);
    let key_user = builder.hash2to1(key_inbox, three);

    builder.add_diff(hkey_prefix_inbox, key_inbox);
    builder.add_diff(key_token_0_amount, token_0_amount);
    builder.add_diff(key_token_1_amount, token_1_amount);
    builder.add_diff(key_method, remove_liquidity);
    builder.add_diff(key_user, addr);

    (nonce, key_inbox, vk, proof)

})));

static SWAP_REMOVE_ADDD_LIQUIDITY_REDUCE: LazyLock<Contract> = LazyLock::new(|| {
    let mut builder = Builder::new();

    let root = builder.thisviewcontroot();

    let snap = builder.thissnapcontroot();
    
    let hkey_prefix_inbox    = builder.constant_hash_of_str("liquidity_inbox");
    let hkey_prefix_liquidity_reduce = builder.constant_hash_of_str("liquidity_reduce");

    let inbox_tail = builder.leaf(snap, hkey_prefix_inbox);
    let (inbox_prev, inbox_item) = builder.slotnode(inbox_tail);  

    let pointer = builder.contslotitem(root, hkey_prefix_liquidity_reduce);

    let hkey_prefix_pool = builder.constant_hash_of_str("pool");
    let pool = builder.contslotitem(root, hkey_prefix_pool);
    let zero = builder.constant_zeros();
    let one = builder.constant_ones();
    let two = builder.constant_u128(2);
    let three = builder.constant_u128(3);

    let token_0_key = builder.hash2to1(pool, zero);
    let token_1_key: [Target; 4] = builder.hash2to1(pool, one);

    let key_user = builder.hash2to1(inbox_item, three);
    let user = builder.contslotitem(root, key_user);

    let hkey_prefix_reserve    = builder.constant_hash_of_str("reserve_liquidity");
    let hkey_token_0_reserve = builder.hash2to1(hkey_prefix_reserve, token_0_key);
    let hkey_token_1_reserve = builder.hash2to1(hkey_prefix_reserve, token_1_key);

    let reserve_token_0 = builder.contslotitem(root, hkey_token_0_reserve);
    let reserve_token_1 = builder.contslotitem(root, hkey_token_1_reserve);

    let hkey_token = builder.hash2to1(hkey_prefix_reserve, user);
    let hkey_user_token_0 = builder.hash2to1(hkey_token, token_0_key);
    let user_token_0_reserve = builder.contslotitem(root, hkey_user_token_0);

    let hkey_user_token_1 = builder.hash2to1(hkey_token, token_1_key);
    let user_token_1_reserve = builder.contslotitem(root, hkey_user_token_1);

    let hkey_token_0 = builder.hash2to1(inbox_item, zero);
    let hkey_token_1 = builder.hash2to1(inbox_item, one);
    let hkey_method = builder.hash2to1(inbox_item, two);

    let token_0_amount = builder.contslotitem(root, hkey_token_0);
    let token_1_amount = builder.contslotitem(root, hkey_token_1);
    let method = builder.contslotitem(root, hkey_method);

    let swap_token_0_amount = builder.u128_add::<true>(reserve_token_0, token_0_amount);
    let swap_token_1_amount = builder.u128_sub::<true>(reserve_token_1, token_1_amount);

    let add_token_0_amount = builder.u128_add::<true>(reserve_token_0, token_0_amount);
    let add_token_1_amount = builder.u128_add::<true>(reserve_token_1, token_1_amount);
    let add_user_token_0_amount = builder.u128_add::<true>(user_token_0_reserve, token_0_amount);
    let add_user_token_1_amount = builder.u128_add::<true>(user_token_0_reserve, token_1_amount);

    let remove_token_0_amount = builder.u128_sub::<true>(reserve_token_0, token_0_amount);
    let remove_token_1_amount = builder.u128_sub::<true>(reserve_token_1, token_1_amount);
    let remove_user_token_0_amount = builder.u128_add::<true>(user_token_0_reserve, token_0_amount);
    let remove_user_token_1_amount = builder.u128_add::<true>(user_token_1_reserve, token_1_amount);

    let add_liquidity = builder.constant_hash_of_str("add_liquidiy");
    let remove_liquidity = builder.constant_hash_of_str("remove_liquidiy");
    let swap_liquidity    = builder.constant_hash_of_str("swap_liquidity");

    let is_add_liquidity = builder.equal(method, add_liquidity);
    let is_remove_liquidity = builder.equal(method, remove_liquidity);
    let is_swap_liquidity = builder.equal(method, swap_liquidity);

    let swap_or_reserve_token_0_amount = builder.cond(is_swap_liquidity, swap_token_0_amount, reserve_token_0);
    let remove_or_swap_token_0_amount = builder.cond(is_remove_liquidity, remove_token_0_amount, swap_or_reserve_token_0_amount);
    let reserve_token_0 = builder.cond(is_add_liquidity, add_token_0_amount, remove_or_swap_token_0_amount);
    let swap_or_reserve_token_1_amount = builder.cond(is_swap_liquidity, swap_token_1_amount, reserve_token_1);
    let remove_or_swap_token_1_amount = builder.cond(is_remove_liquidity, remove_token_1_amount, swap_or_reserve_token_1_amount);
    let reserve_token_1 = builder.cond(is_add_liquidity, add_token_1_amount, remove_or_swap_token_1_amount);

    let remove_or_add_user_token_0_amount = builder.cond(is_add_liquidity, add_user_token_0_amount, user_token_0_reserve);
    let user_token_0_amount = builder.cond(is_remove_liquidity, remove_user_token_0_amount, remove_or_add_user_token_0_amount);
    
    let remove_or_add_user_token_1_amount = builder.cond(is_add_liquidity, add_user_token_1_amount, user_token_1_reserve);
    let user_token_1_amount = builder.cond(is_remove_liquidity, remove_user_token_1_amount, remove_or_add_user_token_1_amount);

    builder.enforce(pointer, inbox_prev);
    builder.add_diff(hkey_token_0_reserve, reserve_token_0);
    builder.add_diff(hkey_token_1_reserve, reserve_token_1);
    builder.add_diff(hkey_token_0_reserve, user_token_0_amount);
    builder.add_diff(hkey_user_token_1, user_token_1_amount);
    builder.add_diff(hkey_prefix_liquidity_reduce, inbox_item);

    builder.into()
});