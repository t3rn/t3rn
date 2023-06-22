import { getGasPrice } from "price-estimation/eth";

describe("eth", () => {
  describe("getGasPrice", () => {
    it("should return the current gas price of ETH", async () => {
      const result = await getGasPrice();
      expect(result).toEqual({});
    });
  });
});
