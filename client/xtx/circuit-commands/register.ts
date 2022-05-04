import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import{ createGatewayABIConfig, createGatewayGenesisConfig, createGatewaySysProps }from'./utils/utils';
import{ getHeaderProof } from "./utils/helpers";

const { exec } = require('child_process');

const triggerRegister = async (circuit: ApiPromise, params: any) => {
    
    const { 
      authorities,
      url,
      authoritySetId,
      rococoRegistrationHeader,
      metadata,
      genesisHash,
      target
    } = params;
    
    const registerGateway = await circuit.tx.circuitPortal.registerGateway(
        url,
        String.fromCharCode(...target),
        null,
        createGatewayABIConfig(circuit, 32, 32, 32, 12, 'Sr25519', 'Blake2'),
        //GatewayVendor: 'Substrate' as rococo is substrate-based
        circuit.createType('GatewayVendor', 'Substrate'),
        //GatewayType: we connect as a ProgrammableExternal
        circuit.createType('GatewayType', { ProgrammableExternal: 1 }),
        createGatewayGenesisConfig(metadata, genesisHash, circuit),
        createGatewaySysProps(circuit, 60, '', 0), // GatewaySysProps
        //Initial rococo, acts as gateway activation point
        circuit.createType('Bytes', rococoRegistrationHeader.toHex()),
        //List of current rococo authorities
        circuit.createType('Option<Vec<AccountId>>', authorities),
        circuit.createType('Option<SetId>', authoritySetId),
        //SideEffects that are allowed on gateway instance
        circuit.createType('Vec<AllowedSideEffect>', ['tran']) // allowed side effects
    );
    
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    return circuit.tx.sudo.sudo(registerGateway).signAndSend(alice);
};

export const setOperational = async (circuit: ApiPromise, target: any[]) => {
  const setOperational =
    circuit.tx.multiFinalityVerifierDefault.setOperational(
      true,
      target
    )
  const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
  const alice = keyring.addFromUri('//Alice');

  return new Promise(async (resolve, reject) => {
    await circuit.tx.sudo
      .sudo(setOperational)
      .signAndSend(alice, result => {
        if (result.isError) {
          reject('submitting setOperational failed')
        } else if (result.isInBlock) {
          console.log(`gateway ${target.toString()} operational`)
          resolve(undefined)
        }
      })
  })
}

export const register = async(circuit: ApiPromise, target: number[]): Promise<void> => {
     // ws endpoint of target chain
    const url = "wss://rococo-rpc.polkadot.io";
    const rococoProvider = new WsProvider(url);
    const api = await ApiPromise.create({ provider: rococoProvider });

    const [metadata, genesisHash] = await Promise.all([
      await api.runtimeMetadata,
      await api.genesisHash,
    ]);

    return new Promise(async (res, rej) => {
      let listener = await api.rpc.grandpa.subscribeJustifications(async (justification: any) => {
        let hex_justification = justification.toString().substring(2) // removes 0x
        const blockNumber: any = await new Promise((res, rej) => {
          return exec(`./rust_decode/target/release/decode_justification blocknumber ${hex_justification}`, (err, stdout, _) => {
            if (err) {
              throw err
            }
            if (stdout.includes("Error")) {
              throw new Error("GrandpaJustification decoding failed!")
            }
            return res(JSON.parse(stdout));
          });
        })

        console.log("BlockNumber:", blockNumber)
  
        const authorities: any[] = await new Promise((res, rej) => {
          return exec(`./rust_decode/target/release/decode_justification authority_set ${hex_justification}`, (err, stdout, _) => {
            if (err) {
              throw err
            }
            if (stdout.includes("Error")) {
              throw new Error("GrandpaJustification decoding failed!")
            }
            return res(JSON.parse(stdout));
          });
        })
  
        const rococoRegistrationHeader = await api.rpc.chain.getHeader(
          await api.rpc.chain.getBlockHash(blockNumber)
        )
        const authoritySetId = await api.query.grandpa.currentSetId()
  
        await triggerRegister(circuit, {
          authorities,
          url,
          authoritySetId,
          rococoRegistrationHeader,
          metadata,
          genesisHash,
          target,
          api
        })
  
        listener();
        return res()
      })
    })
}

export const registerParachain = async (circuitApi: ApiPromise, target: any[]) => {
  const moonBeamUrl = 'wss://node-6870830370282213376.rz.onfinality.io/ws?apikey=0f273197-e4d5-45e2-b23e-03b015cb7000'; // ws endpoint of target chain
  const rococoProvider = new WsProvider(moonBeamUrl);
  const moonApi = await ApiPromise.create({ provider: rococoProvider });

  const [moonCurrentHeader, moonMetadata, moonGenesisHash] = await Promise.all([
    await moonApi.rpc.chain.getHeader(),
    await moonApi.runtimeMetadata,
    await moonApi.genesisHash,
  ]);

  // const moonAtGenesis = await moonApi.at(moonGenesisHash);
  await moonApi.disconnect();


  const registerGateway = circuitApi.tx.circuitPortal.registerGateway(
    moonBeamUrl,
    String.fromCharCode(...target),
    {
      relayChainId: [97, 98, 99, 100],
      id: 2000
    },
    createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2'),
    //GatewayVendor: 'Substrate' as moon is substrate-based
    circuitApi.createType('GatewayVendor', 'Substrate'),
    //GatewayType: we connect as a ProgrammableExternal
    circuitApi.createType('GatewayType', { ProgrammableExternal: 1 }),
    createGatewayGenesisConfig(moonMetadata, moonGenesisHash, circuitApi),
    createGatewaySysProps(circuitApi, 60, '', 0), // GatewaySysProps
    //Initial moon, acts as gateway activation point
    circuitApi.createType('Bytes', moonCurrentHeader.toHex()),
    circuitApi.createType('Option<Vec<AccountId>>', null),
    circuitApi.createType('Option<SetId>', null),
    //SideEffects that are allowed on gateway instance
    circuitApi.createType('Vec<AllowedSideEffect>', ['tran']) // allowed side effects
  );

  const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
  const alice = keyring.addFromUri('//Alice');
  return circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice);
};

export const getStorageKey = async (rococoApi: ApiPromise) => {
  // const moonBeamUrl = 'wss://moonbeam-alpha.api.onfinality.io/public-ws'; // ws endpoint of target chain
  // const rococoProvider = new WsProvider(moonBeamUrl);
  // const moonApi = await ApiPromise.create({ provider: rococoProvider });

  // console.log(await (await rococoApi.query.paras.heads(2004)).toJSON());

  getHeaderProof(rococoApi, "0x9958e316f259194d2357c5e0bb3f14534b9c87034727d0af1c1a99e3de5bf8ec", 2004)

}


// DEDA5DD53E1D22706AD8D0DDA980DD7811A416E0E28368FA906B01EF3C6A96E656960E006EED942C8FD34F09AEBC3300CDE5B083BA8059ABB95BC157323FF2622D6DFB3A66DBD159B185565FEF489F713AF33E13AC42BC0C14FAFA1BEF7C44EDE05A5A470806617572612066AC32080000000005617572610101D8D79FF7D9F7298229BF60EFD18AB3DA0221A853CFEB8E1FEB246298EE30CB4CE108525F0189D86CC64972E2BC96961D3B86AF2B541066D077B74F7D9829FF80