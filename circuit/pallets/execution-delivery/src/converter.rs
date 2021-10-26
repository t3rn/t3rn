use super::*;
use ethabi_decode::Address;
use snowbridge_basic_channel::outbound::Module as snowbridge;
use sp_runtime::DispatchResult;

pub fn submit_messages_to_relayers<T: Config>(
    dispatch_account: &T::AccountId,
    messages: Vec<CircuitOutboundMessage>,
) -> Result<(), &'static str> {
    // loop through each message and filter ethereum messages
    let res = messages
        .into_iter()
        .filter(|msg| msg.gateway_chain == GatewayChain::Ethereum)
        .map(|msg| {
            let target = msg
                .module_name
                .get(0..20)
                .ok_or("invalid token address")
                .map(|v| Address::from_slice(v))?;

            let data = msg.extra_payload.ok_or("no payload given")?;
            snowbridge::<T>::submit(dispatch_account, target, data.call_bytes.as_slice())?;
            Ok(())
        })
        .all(|res: Result<(), &'static str>| res.is_ok());

    match res {
        true => Ok(()),
        false => Err("failed to dispatch messages"),
    }
}
