import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';
import { Encodings } from "@t3rn/sdk"
import "@t3rn/types"
// @ts-ignore
import { GatewayGenesisConfig, GatewayABIConfig, T3rnPrimitivesTokenInfo, T3rnPrimitivesSubstrateToken } from '@polkadot/types/lookup'
import { fetchBestFinalizedHash, fetchLatestPossibleParachainHeader } from "../../utils/substrate";
import {Codec} from "@polkadot/types-codec/types";

const axios = require('axios').default;

export const registerSubstrate = async (circuit: ApiPromise, gatewayData: any) => {
    if(!gatewayData.registrationData.parachain) { // relaychain
        const target = await ApiPromise.create({
            provider: new WsProvider(gatewayData.rpc),
        })
        return registerRelaychain(circuit, target, gatewayData)
    } else {
        return registerParachain(circuit, gatewayData)
    }
}

const registerRelaychain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any) => {
    const { registrationHeader, authorities, authoritySetId } = await fetchPortalConsensusData(circuit, target, gatewayData)
    console.log("Registering Block #", registrationHeader.number.toNumber());
    return circuit.createType('RelaychainRegistrationData', [
        registrationHeader.toHex(),
        Array.from(authorities),
        authoritySetId,
        gatewayData.registrationData.owner
    ]).toHex()

}

const registerParachain = async (circuit: ApiPromise, gatewayData: any) => {
    return circuit.createType("ParachainRegistrationData", [gatewayData.registrationData.parachain.relayChainId, gatewayData.registrationData.parachain.id]).toHex()
}

const fetchPortalConsensusData = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any) => {
    const registrationHeight = await fetchLatestAuthoritySetUpdateBlock(gatewayData)

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
const fetchLatestAuthoritySetUpdateBlock = async (gatewayData: any) => {
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
        return response.data.data.events.map(entry => entry.block_num)[0]
    })
}