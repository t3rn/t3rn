import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { createGatewayABIConfig, createGatewayGenesisConfig, createGatewaySysProps } from './utils/utils';

export const register = async (circuitApi: ApiPromise, target: any[]) => {
    const rococoUrl = 'wss://rococo-rpc.polkadot.io'; // ws endpoint of target chain
    const rococoProvider = new WsProvider(rococoUrl);
    const rococoApi = await ApiPromise.create({ provider: rococoProvider });
    
    const [rococoMetadata, rococoGenesisHash] = await Promise.all([
      await rococoApi.runtimeMetadata,
      await rococoApi.genesisHash,
    ]);
    const rococoAuthorityList = await rococoApi.query.session.validators();
    const authoritySetId = await rococoApi.query.grandpa.currentSetId()
    const rococoRegistrationHeader = await rococoApi.rpc.chain.getHeader()
    await rococoApi.disconnect();
    console.log(rococoAuthorityList.toHuman())
    
    
    const registerGateway = circuitApi.tx.circuitPortal.registerGateway(
      rococoUrl,
      String.fromCharCode(...target),
      createGatewayABIConfig(circuitApi, 32, 32, 32, 12, 'Sr25519', 'Blake2'),
      //GatewayVendor: 'Substrate' as rococo is substrate-based
      circuitApi.createType('GatewayVendor', 'Substrate'),
      //GatewayType: we connect as a ProgrammableExternal
      circuitApi.createType('GatewayType', { ProgrammableExternal: 1 }),
      createGatewayGenesisConfig(rococoMetadata, rococoGenesisHash, circuitApi),
      createGatewaySysProps(circuitApi, 60, '', 0), // GatewaySysProps
      //Initial rococo, acts as gateway activation point
      circuitApi.createType('Bytes', rococoRegistrationHeader.toHex()),
      //List of current rococo authorities
      circuitApi.createType('Option<Vec<AccountId>>', rococoAuthorityList),
      circuitApi.createType('Option<SetId>', authoritySetId),
      //SideEffects that are allowed on gateway instance
      circuitApi.createType('Vec<AllowedSideEffect>', ['tran']) // allowed side effects
    );
    
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 60 });
    const alice = keyring.addFromUri('//Alice');
    return circuitApi.tx.sudo.sudo(registerGateway).signAndSend(alice);
};

export const setOperational = async (circuit: ApiPromise, target: any[]) => {
  const setOperational =
    circuit.tx.multiFinalityVerifierPolkadotLike.setOperational(
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
