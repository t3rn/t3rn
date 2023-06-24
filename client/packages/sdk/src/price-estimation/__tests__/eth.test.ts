import { describe, expect, it } from "@jest/globals";
import fetch from "node-fetch";
import {
  EthSpeedModes,
  calculateGasFee,
  getGasPrice,
  getGasAmount,
  EthActions,
  ETH_TRANSFER_GAS_AMOUNT,
} from "../eth";

jest.mock("node-fetch", () => jest.fn());

const mockGetGasPriceSuccesfulResponse = () => {
  // @ts-ignore - mockImplementationOnce is not defined in type
  fetch.mockImplementationOnce(() =>
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

      const result = await getGasPrice();

      expect(result).not.toEqual(undefined);
      expect(result.rapid).toEqual(17796706627);
      expect(result.fast).toEqual(14000849292);
      expect(result.standard).toEqual(13969553812);
      expect(result.slow).toEqual(13969553812);
    });

    it("should throw error if unable to get current ETH gas price", async () => {
      mockGetGasPriceErrorResponse();

      try {
        await getGasPrice();
      } catch (e) {
        expect(e).toEqual(
          new Error("Failed to fetch gas price. ERROR_STATUS: 500")
        );
      }
    });
  });

  describe("calculateGasFee", () => {
    it("should get the current gas fee for a ETH transfer", async () => {
      mockGetGasPriceSuccesfulResponse();
      const gasFee = await calculateGasFee("tran", EthSpeedModes.Fast);
      expect(gasFee).toEqual(0.000294017835132);
    });

    it("should result to an error if unable to calculate ETH transfer gas fee", async () => {
      mockGetGasPriceErrorResponse();

      try {
        await calculateGasFee("tran", EthSpeedModes.Fast);
      } catch (e) {
        expect(e).toEqual(
          new Error("Failed to fetch gas price. ERROR_STATUS: 500")
        );
      }
    });
  });

  describe("getGasAmount", () => {
    it("should return the gas amount for a ETH transfer", () => {
      expect(getGasAmount(EthActions.Transfer)).toEqual(
        ETH_TRANSFER_GAS_AMOUNT
      );
    });
  });
});
