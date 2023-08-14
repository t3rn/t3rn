import { describe, expect, it } from "@jest/globals";
import { createType } from "@t3rn/types"
import { ApiPromise, Keyring, Sdk, cryptoWaitReady } from "../../index";
import { Actions, Targets, estimateGasFee, estimateBidAmount, estimateMaxReward } from "../index";
import { EstimateSubmittableExtrinsicParams } from "../substrate";

const createCircuitContext = async (
  exportMode = false,
) => {
  await cryptoWaitReady()
  const keyring = new Keyring({ type: "sr25519" })
  const signer =
    process.env.CIRCUIT_KEY === undefined
      ? keyring.addFromUri("//Alice")
      : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
  const sdk = new Sdk(
    "ws://localhost:9944",
    signer,
    exportMode,
  )
  const circuit = await sdk.init()
  return {
    circuit,
    sdk,
    signer,
  }
}

const buildSfx = (circuit: ApiPromise
) => {
  return {
    sideEffects: createType("Vec<T3rnTypesSfxSideEffect>", [{
      target: "roco",
      maxReward: 1000000,
      insurance: 1000000,
      encodedArgs: ["0x0", "0x1"],
      action: "tran",
      signature: "",
      enforceExecutor: "",
      rewardAssetId: null
    }]).toJSON(),
    speed_mode: circuit.createType("T3rnPrimitivesSpeedMode", "Fast"),
  }
}

const buildArgs = async () => {
  const { circuit, signer } = await createCircuitContext()
  const mockTransferSfx = buildSfx(circuit)
  const tx = circuit.tx.circuit.onExtrinsicTrigger(
    mockTransferSfx.sideEffects as Parameters<
      typeof circuit.tx.circuit.onExtrinsicTrigger
    >[0],
    mockTransferSfx.speed_mode
  )
  return { tx, account: signer } as EstimateSubmittableExtrinsicParams
}

describe("substrate", () => {
  describe("estimateActionGasFee", () => {
    it("should estimate gas fee a transfer sfx on Rococo target", async () => {
      const result = await estimateGasFee<EstimateSubmittableExtrinsicParams>({
        target: Targets.Rococo,
        action: Actions.TransferAsset,
        args: await buildArgs()
      });
      expect(result).not.toEqual(undefined)
      console.log("Gas fee:", result, "ROC")
    })
  });

  describe("estimateBidAmount", () => {
    it("should estimate bid amount for a transfer sfx on Rococo target", async () => {
      const result = await estimateBidAmount<EstimateSubmittableExtrinsicParams>({
        target: Targets.Rococo,
        action: Actions.TransferAsset,
        args: await buildArgs()
      }, (fee) => fee * 0.1);
      expect(result).not.toEqual(undefined)
      console.log("Estimation:", result)
    })
  });

  describe("estimateMaxReward", () => {
    it("should estimate the max reward for a asset transfer, DOT -> ACA", async () => {
      const result = await estimateMaxReward<EstimateSubmittableExtrinsicParams>({
        target: Targets.Rococo,
        action: Actions.TransferAsset,
        baseAsset: "dot",
        targetAsset: "aca",
        targetAmount: 100,
        overSpendPercent: 0.1,
        args: await buildArgs()
      });
      expect(result).not.toEqual(undefined)
      console.log("Estimation:", result)
    })
  });
});
