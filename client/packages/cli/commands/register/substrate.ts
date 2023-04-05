import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';
import { Encodings } from "@t3rn/sdk"
import "@t3rn/types"
// @ts-ignore
import { GatewayGenesisConfig, GatewayABIConfig, T3rnPrimitivesTokenInfo, T3rnPrimitivesSubstrateToken } from '@polkadot/types/lookup'
import { fetchBestFinalizedHash, fetchLatestPossibleParachainHeader } from "../../utils/substrate";
import {Codec} from "@polkadot/types-codec/types";

const axios = require('axios').default;

export const registerSubstrate = async (circuit: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const target = await ApiPromise.create({
        provider: new WsProvider(gatewayData.rpc),
    })

    if(!gatewayData.registrationData.parachain) { // relaychain
        return registerRelaychain(circuit, target, gatewayData, epochsAgo)
    } else {
        return registerParachain(circuit, target, gatewayData)
    }
}

const registerRelaychain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const { registrationHeader, authorities, authoritySetId } = await fetchPortalConsensusData(circuit, target, gatewayData, epochsAgo)
    console.log("Registering Block #", registrationHeader.number.toNumber());
    return {
        gateway_id: circuit.createType("ChainId", gatewayData.id),
        token_id: circuit.createType("ChainId", gatewayData.tokenId),
        verification_vendor: circuit.createType('GatewayVendor', 'Rococo'),
        execution_vendor: circuit.createType('ExecutionVendor', 'Substrate'),
        codec: circuit.createType('RuntimeCodec', 'Scale'),
        registrant: null,
        escrow_accounts: null,
        allowed_side_effects: circuit.createType('Vec<([u8; 4], Option<u8>)>', gatewayData.registrationData.allowedSideEffects),
        token_sys_props: createTokenSysProps(circuit, gatewayData.registrationData.gatewaySysProps),
        registration_data: circuit.createType('RelaychainRegistrationData', [
            registrationHeader.toHex(),
            Array.from(authorities),
            authoritySetId,
            gatewayData.registrationData.owner
        ])
    }
}

const registerParachain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any) => {
    return {
        gateway_id: circuit.createType("ChainId", gatewayData.id),
        token_id: circuit.createType("ChainId", gatewayData.tokenId),
        verification_vendor: circuit.createType('GatewayVendor', 'Rococo'),
        execution_vendor: circuit.createType('ExecutionVendor', 'Substrate'),
        codec: circuit.createType('RuntimeCodec', 'Scale'),
        registrant: null,
        escrow_accounts: null,
        allowed_side_effects: circuit.createType('Vec<([u8; 4], Option<u8>)>', gatewayData.registrationData.allowedSideEffects),
        token_sys_props: createTokenSysProps(circuit, gatewayData.registrationData.gatewaySysProps),
        registration_data: circuit.createType("ParachainRegistrationData", [gatewayData.registrationData.parachain.relayChainId, gatewayData.registrationData.parachain.id])
    }
}

const createTokenSysProps = (circuiApi: ApiPromise, gatewaySysProps: any) => {
    const param = {
        ss58_format: gatewaySysProps.ss58Format,
        token_symbol: gatewaySysProps.tokenSymbol,
        token_decimals: gatewaySysProps.tokenDecimals
    }
    return circuiApi.createType('TokenInfo', {Substrate: param});
}

const fetchPortalConsensusData = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const registrationHeight = await fetchLatestAuthoritySetUpdateBlock(gatewayData, epochsAgo)

    const registrationHeader = await target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(registrationHeight)
    )

    const finalityProof = await target.rpc.grandpa.proveFinality(registrationHeight);
    const authorities= Encodings.Substrate.Decoders.extractAuthoritySetFromFinalityProof(finalityProof)
    const registratationHeaderHash = await target.rpc.chain.getBlockHash(registrationHeight);
    const targetAt = await target.at(registratationHeaderHash);
    const authoritySetId = await targetAt.query.grandpa.currentSetId()
    return {
        registrationHeader,
        authorities:  circuit.createType('Vec<AccountId>', authorities),
        authoritySetId: circuit.createType('SetId', authoritySetId),
    }
}

//for registrations we want to get the justification cotaining the latest authoritySetUpdate, as we can be sure that all authorties are included.
const fetchLatestAuthoritySetUpdateBlock = async (gatewayData: any, epochsAgo: number) => {
    return axios.post(gatewayData.subscan + '/api/scan/events', {
            row: 20,
            page: 0,
            module: "grandpa",
            call: "newauthorities"
        },
        {
            headers: {
                'content-type': 'text/json'
            }
        }
    )
    .then(function (response) {
        return response.data.data.events.map(entry => entry.block_num)[epochsAgo]
    })
}