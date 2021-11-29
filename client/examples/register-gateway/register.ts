import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { rpc, types } from '@t3rn/types';
import { createGatewayABIConfig, createGatewayGenesisConfig } from './utils/utils';

(async () => {
    const circuitProvider = new WsProvider('ws://127.0.0.1:9944') // t3rn circuit ws endpoint
    const circuitApi: ApiPromise =  await ApiPromise.create({
        provider: circuitProvider,
        types,
        rpc,
    })

    const rococoUrl = 'wss://rococo-rpc.polkadot.io'; // ws endpoint of target chain
    const rococoProvider = new WsProvider(rococoUrl);
    const rococoApi = await ApiPromise.create({ provider: rococoProvider });

    const [rococoCurrentHeader, rococoMetadata, rococoRuntimeVersion, rococoGenesisHash] = await Promise.all([
        await rococoApi.rpc.chain.getHeader(),
        await rococoApi.runtimeMetadata,
        await rococoApi.runtimeVersion,
        await rococoApi.genesisHash,
    ]);

    const rococoAtGenesis = await rococoApi.at(rococoGenesisHash);
    const rococoInitialAuthorityList = await rococoAtGenesis.query.session.validators();
    await rococoApi.disconnect();

    const gatewayId = String.fromCharCode(...[0, 0, 0, 0].map(() => Math.floor(97 + Math.random() * 26)));

    const abiConfig = await createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2')

    const registerGateway = circuitApi.tx.execDelivery.registerGateway(
        rococoUrl,
        gatewayId,
        abiConfig,
        circuitApi.createType('GatewayVendor', 'Substrate'), // GatewayVendor
        circuitApi.createType('GatewayType', { ProgrammableExternal: 1 }), // GatewayType
        createGatewayGenesisConfig(rococoMetadata, rococoRuntimeVersion, rococoGenesisHash, circuitApi), // GatewayGenesisConfig
        circuitApi.createType('Bytes', rococoCurrentHeader.toHex()), // first header
        circuitApi.createType('Option<Vec<AccountId>>', rococoInitialAuthorityList), // authorities
        circuitApi.createType('Vec<AllowedSideEffect>', ['transfer', 'get_storage']) // allowed side effects
    );

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    await circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice)

    // @ts-ignore neccecary because the compiler sometimes says xdns is undefined
    const xdns = await circuitApi.rpc.xdns.fetchRecords();

    console.log("xdns entries:", xdns)

    circuitApi.disconnect();
})();