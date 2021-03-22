use anyhow::Result;
use sp_keyring::AccountKeyring;
use subxt::sp_core::H256;
use subxt::{
    
    balances::{Balances, BalancesEventsDecoder, TransferCallExt, TransferEvent},
    contracts::*,
    contracts_gateway::*,
    runtime_gateway::*,
    runtime_gateway::{
        RuntimeGatewayVersatileCommitSuccessEvent, RuntimeGatewayVersatileExecutionSuccessEvent,
        RuntimeGatewayVersatileRevertSuccessEvent,
    },
    sp_core::Decode,
    system::System,
    ClientBuilder, ContractsTemplateRuntime, EventSubscription, EventsDecoder, KusamaRuntime,
    PairSigner, Signer,
};

pub async fn fetch_kusama_block() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::<KusamaRuntime>::new()
        .set_url("wss://kusama-rpc.polkadot.io")
        .build()
        .await?;
    let block_number = 1;
    let block_hash = client.block_hash(Some(block_number.into())).await?;

    if let Some(hash) = block_hash {
        println!("Block hash for block number {}: {}", block_number, hash);
    } else {
        println!("Block number {} not found.", block_number);
    }

    Ok(())
}

pub fn sync_fetch_kusama_block() -> Result<()> {
    async_std::task::block_on(async move {
        // let client = ClientBuilder::<KusamaRuntime>::new()
        let client = ClientBuilder::<ContractsTemplateRuntime>::new()
            // .set_url("wss://kusama-rpc.polkadot.io")
            .set_url("ws://localhost:9944")
            .build()
            .await?;
        let block_number = 1;
        let block_hash = client.block_hash(Some(block_number.into())).await?;

        if let Some(hash) = block_hash {
            println!("Block hash for block number {}: {}", block_number, hash);
        } else {
            println!("Block number {} not found.", block_number);
        }

        Ok(())
    })
}

// #[async_std::main]
pub fn subscribe_for_gateway_events() -> Result<(), Box<dyn std::error::Error>> {
    async_std::task::block_on(async move {
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let dest = AccountKeyring::Bob.to_account_id().into();
        println!(" signer {:?} vs dest {:?}", &signer.account_id(), dest);

        // let client = ClientBuilder::<DefaultNodeRuntime>::new().build().await?;
        let client = ClientBuilder::<ContractsTemplateRuntime>::new()
            .set_url("ws://localhost:9944")
            .build()
            .await?;

        println!(" CLIENT READY");

        let sub = client.subscribe_events().await?;
        println!(" CLIENT WAITING client.subscribe_events().await? ");

        let mut decoder = EventsDecoder::<ContractsTemplateRuntime>::new(client.metadata().clone());
        decoder.with_balances();
        println!(" CLIENT DECODER READY ");

        let mut sub = EventSubscription::<ContractsTemplateRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();
        println!(" sub.filter_event::<TransferEvent<_>>()");

        client.transfer(&signer, &dest, 10_000).await?;
        let raw = sub.next().await.unwrap().unwrap();
        println!("  sub.next().await.unwrap().unwrap(); {:?}", raw);

        let event = TransferEvent::<ContractsTemplateRuntime>::decode(&mut &raw.data[..]);
        if let Ok(e) = event {
            println!("Balance transfer success: value: {:?}", e.amount);
        } else {
            println!("Failed to subscribe to Balances::Transfer Event");
        }
        Ok(())
    })
}

// #[async_std::main]
async fn submit_static_gateway_contract_call() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// #[async_std::main]
async fn submit_side_effects_gateway_contract_call() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// #[async_std::main]
async fn submit_side_effects_gateway_exec() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// #[async_std::main]
async fn submit_volatile_gateway_exec() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// Put contract code to a smart contract enabled substrate chain.
// Returns the code hash of the deployed contract if successful.
//
// Optionally supply the contract wasm path, defaults to destination contract file inferred from
// Cargo.toml of the current contract project.
//
// Creates an extrinsic with the `Contracts::put_code` Call, submits via RPC, then waits for
// the `ContractsEvent::CodeStored` event.
// pub(crate) fn execute_deploy(
//     extrinsic_opts: &ExtrinsicOpts,
//     contract_wasm_path: Option<&PathBuf>,
// ) -> Result<H256> {
//     let code = load_contract_code(contract_wasm_path)?;
//
//     async_std::task::block_on(async move {
//         let cli = ClientBuilder::<ContractsTemplateRuntime>::new()
//             .set_url(&extrinsic_opts.url.to_string())
//             .build()
//             .await?;
//         let signer = extrinsic_opts.signer()?;
//
//         let events = cli.put_code_and_watch(&signer, &code).await?;
//         let code_stored = events
//             .code_stored()?
//             .ok_or(anyhow::anyhow!("Failed to find CodeStored event"))?;
//
//         Ok(code_stored.code_hash)
//     })
// }

// /**
//  let events = cli
//             .multistep_call_and_watch(
//                 &signer,
//                 requester,
//                 target_dest,
//                 phase, // phase = Execution
//                 &code,
//                 value,     // value
//                 gas_limit, // gas_limit
//                 &data.0,   // input data
//             )
//             .await?;
//         let execution_stamp = match phase {
//             0 => {
//                 events
//                     .runtime_gateway_versatile_execution_success()?
//                     .ok_or(anyhow::anyhow!(
//                         "Failed to find a RuntimeGatewayVersatileExecutionSuccessEvent event"
//                     ))?
//                     .execution_stamp
//             }
//             1 => {
//                 events
//                     .runtime_gateway_versatile_commit_success()?
//                     .ok_or(anyhow::anyhow!(
//                         "Failed to find a RuntimeGatewayVersatileExecutionCommitEvent event"
//                     ))?
//                     .execution_stamp
//             }
//             2 => {
//                 events
//                     .runtime_gateway_versatile_revert_success()?
//                     .ok_or(anyhow::anyhow!(
//                         "Failed to find a RuntimeGatewayVersatileExecutionRevertEvent event"
//                     ))?
//                     .execution_stamp
//             }
//             _ => Default::default(), // Phases should only be 0,1,2 at this point.
// **/
// #[cfg(test)]
// mod tests {
//     use std::{fs, io::Write};
//
//     use crate::{cmd::deploy::execute_deploy, util::tests::with_tmp_dir, ExtrinsicOpts};
//     use assert_matches::assert_matches;
//
//     const CONTRACT: &str = r#"
// (module
//     (func (export "call"))
//     (func (export "deploy"))
// )
// "#;
//
//     #[test]
//     fn deploy_contract() {
//         with_tmp_dir(|path| {
//             let wasm = wabt::wat2wasm(CONTRACT).expect("invalid wabt");
//
//             let wasm_path = path.join("test.wasm");
//             let mut file = fs::File::create(&wasm_path).unwrap();
//             let _ = file.write_all(&wasm);
//
//             let url = url::Url::parse("ws://localhost:9944").unwrap();
//             let extrinsic_opts = ExtrinsicOpts {
//                 url,
//                 suri: "//Alice".into(),
//                 password: None,
//             };
//             let result = execute_deploy(&extrinsic_opts, Some(&wasm_path));
//
//             assert_matches!(result, Ok(_));
//             Ok(())
//         })
//     }
// }
