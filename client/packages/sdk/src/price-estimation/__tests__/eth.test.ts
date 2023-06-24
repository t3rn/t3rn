import { describe, expect, it } from "@jest/globals";
import { getGasPrice } from "../eth";

describe("eth", () => {
  describe("getGasPrice", () => {
    it("should return the current gas price of ETH", async () => {
      const result = await getGasPrice();
      expect(result).toEqual({});
    });
  });
});
