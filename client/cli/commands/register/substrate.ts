import{ ApiPromise, Keyring, WsProvider } from'@polkadot/api';
import { extractAuthoritySetFromFinalityProof, decodeFinalityProof } from "../../utils/decoder";

const axios = require('axios').default;

export const registerSubstrate = async (circuit: ApiPromise, gatewayData: any) => {
    const target = await ApiPromise.create({
        provider: new WsProvider(gatewayData.rpc),
    })

    if(gatewayData.registrationData.relaychain === null) { // relaychain
        return registerRelaychain(circuit, target, gatewayData)
    } else {
        console.log("Not implemented!")
        return
    }
}

export const registerPortalSubstrate = async (circuit: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const target = await ApiPromise.create({
        provider: new WsProvider(gatewayData.rpc),
    })

    if(gatewayData.registrationData.parachain === null) { // relaychain
        return registerPortalRelaychain(circuit, target, gatewayData, epochsAgo)
    } else {
        console.log("Not implemented!")
        return
    }
}

const registerPortalRelaychain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const { registrationHeader, authorities, authoritySetId } = await fetchPortalConsensusData(circuit, target, gatewayData, epochsAgo)
    console.log("Registering Block #", registrationHeader.number.toNumber());
    return {
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
    }
}

const registerPortalParachain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const { registrationHeader, authorities, authoritySetId } = await fetchPortalConsensusData(circuit, target, gatewayData, epochsAgo)
    console.log("Registering Block #", registrationHeader.number.toNumber());
    return {
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
    }
}

const registerRelaychain = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any) => {
    const abiConfig = createAbiConfig(circuit, gatewayData.registrationData.gatewayConfig)
    const gatewayGenesis = createGatewayGenesis(circuit, target);
    const gatewaySysProps = createGatewaySysProps(circuit, gatewayData.registrationData.gatewaySysProps)
    const { registrationHeader, authorities, authoritySetId } = await fetchConsensusData(circuit, target, gatewayData, 0)
    const allowedSideEffects = circuit.createType('Vec<AllowedSideEffect>', gatewayData.registrationData.allowedSideEffects)
    return circuit.tx.circuitPortal.registerGateway(
        gatewayData.rpc,
        gatewayData.id,
        null,
        abiConfig,
        circuit.createType('GatewayVendor', 'Substrate'),
        circuit.createType('GatewayType', { ProgrammableExternal: 1 }),
        gatewayGenesis,
        gatewaySysProps,
        registrationHeader,
        authorities,
        authoritySetId,
        allowedSideEffects
    );
}

const createGatewayGenesis = async (circuit: ApiPromise, target: ApiPromise) => {
    const [metadata, genesisHash] = await Promise.all([
          await target.runtimeMetadata,
          await target.genesisHash,
    ]);
    return circuit.createType('GatewayGenesisConfig', [
        circuit.createType('Option<Bytes>', metadata.asV14.pallets.toHex()),
        metadata.asV14.extrinsic.version,
        genesisHash,
    ]);
}

const createAbiConfig = (circuiApi: ApiPromise, gatewayConfig: any) => {
    return circuiApi.createType('GatewayABIConfig', [
        circuiApi.createType('u16', gatewayConfig.blockNumberTypeSize),
        circuiApi.createType('u16', gatewayConfig.hashSize),
        circuiApi.createType('HasherAlgo', gatewayConfig.hasher),
        circuiApi.createType('CryptoAlgo', gatewayConfig.crypto),
        circuiApi.createType('u16', gatewayConfig.addressLength),
        circuiApi.createType('u16', gatewayConfig.valueTypeSize),
        circuiApi.createType('u16', gatewayConfig.decimals),
        circuiApi.createType('Vec<StructDecl>', gatewayConfig.structs),
    ]);
}

const createGatewaySysProps = (circuiApi: ApiPromise, gatewaySysProps: any) => {
   return circuiApi.createType('GatewaySysProps', [
        circuiApi.createType('u16', gatewaySysProps.ss58Format),
        circuiApi.createType('Vec<u8>', gatewaySysProps.tokenSymbol),
        circuiApi.createType('u8', gatewaySysProps.tokenDecimals),
   ]);
}

const fetchPortalConsensusData = async (circuit: ApiPromise, target: ApiPromise, gatewayData: any, epochsAgo: number) => {
    const registrationHeight = await fetchLatestAuthoritySetUpdateBlock(gatewayData, epochsAgo)

    const registrationHeader = await target.rpc.chain.getHeader(
        await target.rpc.chain.getBlockHash(registrationHeight)
    )

    const finalityProof = await target.rpc.grandpa.proveFinality(registrationHeight);
    const authorities= extractAuthoritySetFromFinalityProof(finalityProof)
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
    const authorities= extractAuthoritySetFromFinalityProof(finalityProof)
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