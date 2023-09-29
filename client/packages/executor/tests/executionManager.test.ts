import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { SideEffect } from "../src/executionManager/sideEffect";
import { instance, mock, when } from "ts-mockito";
import { CircuitRelayer } from "../src/circuit/relayer";
import BN from "bn.js";
import { Circuit, Gateway, Sdk, Tx, WsProvider } from "@t3rn/sdk";

chai.use(chaiAsPromised);
chai.should();

describe("Execution", () => {
  let xtx;
  let sfx;

  //   beforeEach(async () => {
  //     xtx = new Execution(
  //       [
  //         "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  //         {
  //           toHex() {
  //             return "0xacabacab";
  //           },
  //         },
  //         [],
  //         [],
  //       ],
  //       sdk as any,
  //       null!,
  //       null!,
  //       mock(),
  //       mock(),
  //       mock(),
  //     );
  //   });
});

describe("SideEffect", () => {
  let sfx;

  //   beforeEach(async () => {
  //     sfx = new SideEffect(
  //       {
  //         action: {
  //           toHuman() {
  //             return "tran";
  //           },
  //         },
  //         encodedArgs: [],
  //         target: {
  //           toU8a() {
  //             return Uint8Array.from(Buffer.from("ROCO"));
  //           },
  //         },
  //         from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  //         to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
  //         value: 50,
  //         maxReward: 1,
  //         insurance: 100,
  //       },
  //       "0xacabacab",
  //       "0xacabacabacabacab",
  //       sdk as any,
  //       null!,
  //       null!,
  //       mock(),
  //       mock(),
  //       mock(),
  //     );
  //     this.vendor = record.gateway_record.verification_vendor.toHuman();
  //     this.executionVendor = record.gateway_record.execution_vendor.toHuman();
  //     let tokens: any[] = record.tokens.map((token) => token.toHuman());

  //     let nativeToken = tokens.filter((token) => token.gateway_id === this.id)[0];
  //     // @ts-ignore
  //     this.ticker = Object.values(nativeToken.token_props)[0].symbol;
  //     this.decimals = parseInt(
  //       // @ts-ignore
  //       Object.values(nativeToken.token_props)[0].decimals,
  //     );
  //     this.tokenId = parseInt(
  //       // @ts-ignore
  //       Object.values(nativeToken.token_props)[0].id,
  //     );
  //     this.allowedSideEffects = record.gateway_record.allowed_side_effects
  //       .toHuman()
  //       .map((entry) => entry[0]);
  //   });
});

describe("Circuit relayer functionality", () => {
  it("should increase nonce after first bidSfx", async () => {
    const sdkMock: Sdk = mock(Sdk);
    // Create mock instances for the properties you want to mock
    const circuitMock = mock(Circuit);
    const gatewaysMock: { [id: string]: Partial<Gateway> } = {
      TEST: {
        id: "test",
        rpc: "test_rpc",
        vendor: "test_vendor",
      },
    };
    const providerMock = mock(WsProvider);

    // this.sdk.circuit
    when(sdkMock.circuit).thenReturn(instance(circuitMock));
    // this.sdk.gateways
    when(sdkMock.gateways).thenReturn(gatewaysMock as any);
    // this.sdk.provider
    when(sdkMock.provider).thenReturn(providerMock);

    const clientMock = mock() as any;
    // this.sdk.client
    when(sdkMock.client).thenReturn(instance(clientMock));

    const txMock = mock() as any;
    // this.sdk.client.tx
    when(clientMock.tx).thenReturn(instance(txMock));
    // this.sdk.circuit.tx
    when(circuitMock.tx).thenReturn(instance(txMock));

    // this.sdk.circuit.tx.signAndSend
    when(txMock.signAndSend).thenReturn(async () => {});

    const txCircuitMock = mock() as any;
    // this.client.tx.circuit
    when(txMock.circuit).thenReturn(instance(txCircuitMock));

    // this.client.tx.circuit.bidSfx
    when(txCircuitMock.bidSfx).thenReturn(async () => {});

    // Create an instance of the Sdk class with the mocked properties
    const sdkInstance = instance(sdkMock);
    sdkInstance.nonce = 1;

    const circuitRelayer = new CircuitRelayer(sdkInstance);

    const sfx = new SideEffect(
      {
        action: {
          toHuman() {
            return "tran";
          },
        },
        encodedArgs: [],
        target: {
          toU8a() {
            return Uint8Array.from(Buffer.from("TEST"));
          },
        },
        from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        value: 50,
        maxReward: 1,
        insurance: 100,
      },
      "0xdebf2d86fba3676d63249807a872f2080eb0d9fda7b1dab8407fa8277f632511",
      "0xa4c83388c3d441a56210c655d389daec086bfec55b759b5519dbf784e258b5a0",
      sdkInstance,
      null!,
      null!,
      mock(),
      mock(),
      mock(),
    );
    expect(sfx.id).to.be.equal(
      "0xdebf2d86fba3676d63249807a872f2080eb0d9fda7b1dab8407fa8277f632511",
    );

    // We want to be sure our mock class is initialized correctly
    expect(sdkInstance.nonce).to.be.equal(1);

    await circuitRelayer.bidSfx(sfx.id, new BN(1));
    await circuitRelayer.bidSfx(sfx.id, new BN(1));

    expect(sdkInstance.nonce).to.be.equal(3);
  });
});
