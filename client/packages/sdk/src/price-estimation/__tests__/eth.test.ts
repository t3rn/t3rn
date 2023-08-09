import { describe, expect, it } from "@jest/globals";
import fetch from "node-fetch";
import {
  SpeedModes,
  estimateGasFee,
  getGasPrice,
  getGasAmount,
  Actions,
  ETH_TRANSFER_GAS_AMOUNT,
  Targets,
  SpeedMode,
  estimateActionGasFee,
  EstimateEvmCallGasParams,
} from "../eth";

jest.mock("node-fetch", () => jest.fn());

const mockGetGasPriceSuccesfulResponse = () => {
  // @ts-ignore - mockImplementationOnce is not defined in type
  fetch.mockImplementation(() =>
    Promise.resolve({
      status: 200,
      async json() {
        return {
          code: 200,
          data: {
            rapid: 17796706627,
            fast: 14000849292,
            standard: 13969553812,
            slow: 13969553812,
            timestamp: 1687612735879,
            priceUSD: 1898.3793,
          },
        };
      },
    })
  );
};

const mockGetGasPriceErrorResponse = () => {
  // @ts-ignore - mockImplementationOnce is not defined in type
  fetch.mockImplementationOnce(() =>
    Promise.resolve({
      status: 500,
    })
  );
};

describe("eth", () => {
  describe("getGasPrice", () => {
    it("should return the current gas price of ETH", async () => {
      mockGetGasPriceSuccesfulResponse();

      const result = await getGasPrice("eth");

      expect(result).not.toEqual(undefined);
      expect(result.rapid).toEqual(17796706627);
      expect(result.fast).toEqual(14000849292);
      expect(result.standard).toEqual(13969553812);
      expect(result.slow).toEqual(13969553812);
    });

    it("should throw error if unable to get current ETH gas price", async () => {
      mockGetGasPriceErrorResponse();

      try {
        await getGasPrice('eth');
      } catch (e) {
        expect(e).toEqual(
          new Error("Failed to fetch gas price. ERROR_STATUS: 500")
        );
      }
    });
  });

  describe("estimateGasFee", () => {
    it("should get the estimate gas fee for a transfer", async () => {
      mockGetGasPriceSuccesfulResponse();
      const gasFeeFast = await estimateGasFee("eth", "transfer", SpeedModes.Fast);
      const gasFeeStandard = await estimateGasFee("eth", "transfer", SpeedModes.Standard);
      const gasFeeSlow = await estimateGasFee("eth", "transfer", SpeedModes.Slow);

      expect(gasFeeFast).toEqual(0.000294017835132);
      expect(gasFeeStandard).toEqual(0.000293360630052);
      expect(gasFeeSlow).toEqual(0.000293360630052);
    });

    it("should result to an error if unable to estimate transfer gas fee", async () => {
      mockGetGasPriceErrorResponse();

      try {
        await estimateGasFee("eth", "transfer", SpeedModes.Fast);
      } catch (e) {
        expect(e).toEqual(
          new Error("Failed to fetch gas price. ERROR_STATUS: 500")
        );
      }
    });
  });

  describe("getGasAmount", () => {
    it("should return the gas amount for a ETH transfer", () => {
      expect(getGasAmount(Actions.Transfer)).toEqual(
        ETH_TRANSFER_GAS_AMOUNT
      );
    });
  });


  describe("estimateActionGasFee", () => {
    it("should return a gas fee estimate for a ETH transfer action", async () => {
      mockGetGasPriceSuccesfulResponse();
      expect(await estimateActionGasFee<SpeedMode>(Targets.Eth, Actions.Transfer, SpeedModes.Standard)).toEqual(0.000293360630052)
    })

    it("should return an estimate for a call EVM action", async () => {
      expect(await estimateActionGasFee<EstimateEvmCallGasParams>(Targets.Sepolia, Actions.CallEvm, {
        fromAddress: "0x1234567890AbCdEfFeDcBa09876eFfEDCBA54321",
        toAddress: "0x9876543210FeDcBaABcDEfFeDCbA98765EDCBA12",
        data: "0x000000",
      })).toEqual(0.000293528264697744)
    })
  });
});
