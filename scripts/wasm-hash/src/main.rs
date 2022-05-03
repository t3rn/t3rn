fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let payload =
        std::fs::read_to_string(&args[1]).expect(&format!("usage: {} /tmp/wasm_file", args[0]));
    let hex = if payload.trim().starts_with("0x") {
        &payload.trim()[2..]
    } else {
        &payload.trim()
    };
    let wasm_buf = hex::decode(hex).expect("Hex decoding error");
    let hash = substrate_runtime_proposal_hash::get_parachainsystem_authorize_upgrade(&wasm_buf);
    let hex = format!("0x{}", hex::encode(hash));
    println!("{}", hex);
}
