use std::fs::File;

use candid::{encode_one, Principal};
use ic_cdk::api::management_canister::main::CanisterId;
use pocket_ic::{PocketIc, WasmResult};

#[test]
fn test_todo_canister() {
    let pic = PocketIc::new();
    // Create an empty canister as the anonymous principal and add cycles.
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm_bytes = load_todos_wasm();
    pic.install_canister(canister_id, wasm_bytes, vec![], None);
    // test 'add' a new todo and check the id returned as 1.
    let res = add_new_todo(&pic, canister_id, "add");
    assert_eq!(res, WasmResult::Reply("1".to_owned().as_bytes().to_vec()));

    // test 'read'
    let res = get_todo(&pic, canister_id, "read", 1);
    assert_eq!(
        res,
        WasmResult::Reply("Content First Todo".to_owned().as_bytes().to_vec())
    );
}

fn add_new_todo(pic: &PocketIc, canister_id: CanisterId, method: &str) -> WasmResult {
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        method,
        encode_one("Content First Todo").unwrap(),
    )
    .expect("Failed to call counter canister")
}

fn get_todo(pic: &PocketIc, canister_id: CanisterId, method: &str, todoid: u16) -> WasmResult {
    pic.query_call(
        canister_id,
        Principal::anonymous(),
        method,
        encode_one(todoid).unwrap(),
    )
    .expect("Failed to call counter canister")
}

fn load_todos_wasm() -> Vec<u8> {
    // load the todo's was by opening as Vec<u8>
    // hardcoded wasm binaris DIR
    let wasm_path ""
    Vec::new()
}
