import { default as chai, expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { Execution } from "../src/executionManager/execution";
import {
  NotificationType,
  SideEffect,
} from "../src/executionManager/sideEffect";
import { instance, mock, when } from "ts-mockito";
import { CircuitRelayer } from "../src/circuit/relayer";
import BN from "bn.js";
import { ApiPromise, Circuit, Gateway, Sdk, WsProvider } from "@t3rn/sdk";
import { ExecutionManager } from "../src/executionManager";

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

// describe("Circuit relayer functionality", async () => {
//   it("should increase nonce after first bidSfx", () => {
//     const sdkMock: Sdk = mock(Sdk);
//     // Create mock instances for the properties you want to mock
//     const circuitMock = mock(Circuit)
//     const gatewaysMock: { [id: string]: Partial<Gateway> } = {
//       "TEST": {
//         id: "test",
//         rpc: "test_rpc",
//         vendor: "test_vendor",
//       },
//     };
//     const providerMock = mock(WsProvider)
//     const clientMock = mock(ApiPromise)

//     // Set up mock behavior for the properties
//     when(sdkMock.circuit).thenReturn(circuitMock);
//     when(sdkMock.gateways).thenReturn(gatewaysMock as any);
//     when(sdkMock.provider).thenReturn(providerMock);
//     when(sdkMock.client).thenReturn(clientMock);

//     // Create an instance of the Sdk class with the mocked properties
//     const sdkInstance = instance(sdkMock);

//     const executionManager = new ExecutionManager(mock(), sdkInstance, mock(), mock(), mock())
//     sdkInstance.nonce = 0;

//     const sfx = new SideEffect(
//       {
//         action: {
//           toHuman() {
//             return "tran";
//           },
//         },
//         encodedArgs: [],
//         target: {
//           toU8a() {
//             return Uint8Array.from(Buffer.from("TEST"));
//           },
//         },
//         from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
//         to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
//         value: 50,
//         maxReward: 1,
//         insurance: 100,
//       },
//       "0xdebf2d86fba3676d63249807a872f2080eb0d9fda7b1dab8407fa8277f632511",
//       "0xa4c83388c3d441a56210c655d389daec086bfec55b759b5519dbf784e258b5a0",
//       sdkInstance,
//       null!,
//       null!,
//       mock(),
//       mock(),
//       mock(),
//     );
//     expect(sfx.id).to.be.equal("0xdebf2d86fba3676d63249807a872f2080eb0d9fda7b1dab8407fa8277f632511");

//     executionManager.initSfxListeners(sfx);
//     sfx.emit("Notification", {
//       type: NotificationType.SubmitBid,
//       payload: {
//         sfxId: sfx.id,
//         bidAmount: "100"
//       },
//     });

//     expect(sdkInstance.nonce).to.be.equal(2);
//     executionManager.stopSfxListener(sfx)
//   });
// });
