import { describe, expect, it } from "@jest/globals";
import { estimateMaxReward } from "../index";

describe("price-estimation", () => {
  describe("estimateMaxReward", () => {
    it("should estimate the max reward estimate for a 3 USDT transfer on Ethereum mainnet", async () => {
      const estimation = await estimateMaxReward({
        action: "tass",
        asset: "dot",
        target: "eth",
        targetAsset: "usdc",
        targetAmount: 3,
        overSpendPercent: 0.5
      });

      expect(estimation).not.toBeUndefined();
      expect(Array.isArray(estimation.gasFee)).toEqual(true);
      expect(estimation.gasFee[0].symbol).toEqual("eth");
      expect(estimation.gasFee[1].symbol).toEqual("dot");
    });
  });

});
