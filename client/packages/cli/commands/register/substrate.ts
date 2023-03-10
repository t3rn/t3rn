import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';
import { Encodings } from "@t3rn/sdk"
import "@t3rn/types"
// @ts-ignore
import { GatewayGenesisConfig, GatewayABIConfig, GatewaySysProps } from '@polkadot/types/lookup'
import { fetchBestFinalizedHash, fetchLatestPossibleParachainHeader } from "../../utils/substrate";
import {Codec} from "@polkadot/types-codec/types";

const axios = require('axios').default;

export const registerSubstrate = async (circuit: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const target = await ApiPromise.create({
        provider: new WsProvider(gatewayData.rpc),
    })

    if(gatewayData.registrationData.parachain === null) { // relaychain
        return registerRelaychain(circuit, target, gatewayData, epochsAgo)
    } else {
        return registerParachain(circuit, target, gatewayData)
    }
}

const registerRelaychain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const { registrationHeader, authorities, authoritySetId } = await fetchPortalConsensusData(circuit, target, gatewayData, epochsAgo)
    console.log("Registering Block #", registrationHeader.number.toNumber());
    return [{
        url: circuit.createType("Vec<u8>", gatewayData.rpc),
        gateway_id: circuit.createType("ChainId", gatewayData.id),
        gateway_abi: createAbiConfig(circuit, gatewayData.registrationData.gatewayConfig),
        gateway_vendor: circuit.createType('GatewayVendor', 'Rococo'),
        gateway_type: circuit.createType('GatewayType', { ProgrammableExternal: 1 }),
        gateway_genesis: await createGatewayGenesis(circuit, target),
        gateway_sys_props: createGatewaySysProps(circuit, gatewayData.registrationData.gatewaySysProps),
        allowed_side_effects: circuit.createType('Vec<AllowedSideEffect>', gatewayData.registrationData.allowedSideEffects),
        registration_data: circuit.createType('GrandpaRegistrationData', [
            registrationHeader.toHex(),
            Array.from(authorities),
            authoritySetId,
            gatewayData.registrationData.owner,
            null
        ])
    }]
}

const registerParachain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any) => {
    const latestRelayChainHeader = await fetchBestFinalizedHash(circuit, gatewayData.registrationData.parachain.relayChainId)
    const parachainHeader: any = await fetchLatestPossibleParachainHeader(
        gatewayData.relaychainRpc,
        latestRelayChainHeader.toJSON(),
        gatewayData.registrationData.parachain.id
    )

    return [{
        url: circuit.createType("Vec<u8>", gatewayData.rpc),
        gateway_id: circuit.createType("ChainId", gatewayData.id),
        gateway_abi: createAbiConfig(circuit, gatewayData.registrationData.gatewayConfig),
        gateway_vendor: circuit.createType('GatewayVendor', 'Rococo'),
        gateway_type: circuit.createType('GatewayType', { ProgrammableExternal: 1 }),
        gateway_genesis: await createGatewayGenesis(circuit, target),
        gateway_sys_props: createGatewaySysProps(circuit, gatewayData.registrationData.gatewaySysProps),
        allowed_side_effects: circuit.createType('Vec<AllowedSideEffect>', gatewayData.registrationData.allowedSideEffects),
        registration_data: circuit.createType('GrandpaRegistrationData', [
            parachainHeader.toJSON(),
            null,
            null,
            gatewayData.registrationData.owner,
            circuit.createType("Parachain", [gatewayData.registrationData.parachain.relayChainId, gatewayData.registrationData.parachain.id])
        ])
    }]
}

const createGatewayGenesis = async (circuit: ApiPromise, target: ApiPromise) => {
    const [metadata, genesisHash] = await Promise.all([
          await target.runtimeMetadata,
          await target.genesisHash,
    ]);

    const config: GatewayGenesisConfig = {
        module_encoded: metadata.asV14.pallets,
        extrinsics_version: metadata.asV14.extrinsic.version,
        genesis_hash: genesisHash.toHex(),
    }
    return circuit.createType('GatewayGenesisConfig', config);
}

const createAbiConfig = (circuiApi: ApiPromise, gatewayConfig: any) => {
    const config: GatewayABIConfig = {
        block_number_type_size: gatewayConfig.blockNumberTypeSize,
        hash_size: gatewayConfig.hashSize,
        hasher: gatewayConfig.hasher,
        crypto: gatewayConfig.crypto,
        address_length: gatewayConfig.addressLength,
        value_type_size: gatewayConfig.valueTypeSize,
        decimals: gatewayConfig.decimals,
        structs: gatewayConfig.structs
    }
    return circuiApi.createType('GatewayABIConfig', config);
}

const createGatewaySysProps = (circuiApi: ApiPromise, gatewaySysProps: any) => {
    const props: GatewaySysProps = {
        ss58_format: gatewaySysProps.ss58Format,
        token_symbol: gatewaySysProps.tokenSymbol,
        token_decimals: gatewaySysProps.tokenDecimals
    }
    return circuiApi.createType('GatewaySysProps', props);
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

const fetchConsensusData = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const registrationHeight = await fetchLatestAuthoritySetUpdateBlock(gatewayData, epochsAgo)
    console.log("Latest AuthoritySetUpdate:", registrationHeight)

    const registrationHeader = await target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(registrationHeight)
    )

    const finalityProof = await target.rpc.grandpa.proveFinality(registrationHeight);
    const authorities = Encodings.Substrate.Decoders.extractAuthoritySetFromFinalityProof(finalityProof)
    const authoritySetId = await target.query.grandpa.currentSetId()
    return {
        registrationHeader: circuit.createType('Bytes', registrationHeader.toHex()),
        authorities:  circuit.createType('Option<Vec<AccountId>>', authorities),
        authoritySetId: circuit.createType('Option<SetId>', authoritySetId),
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