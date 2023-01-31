import { CoingeckoPricing } from "../src/pricing/coingecko";
import { PriceEngine } from "../src/pricing/index";
import { config } from "../config/config";

describe("Basic PriceEngine setup", () => {

    it("should be a callable", () => {
        expect(PriceEngine).toBeInstanceOf(Function);
    });

    it("should have a constructor and method called 'getAssetPrice'", async () => {
        const pe = await new PriceEngine();
        expect(pe).toHaveProperty("constructor");
        expect(pe).toHaveProperty("getAssetPrice");
    });
});

describe("Basic CoinGecko setup", () => {
    it("should be a callable", () => {
        expect(CoingeckoPricing).toBeInstanceOf(Function);
    });

    it("should have a constructor and methods", async () => {
        const cg = await new CoingeckoPricing();
        expect(cg).toHaveProperty("constructor");
        expect(cg).toHaveProperty("getTrackingAssets");
        expect(cg).toHaveProperty("updateAssetPrices");
    });

    it("should load the assets for tracking", async () => {
        // Create a new instance of the class
        const cg = await new CoingeckoPricing();
        // Load the assets
        await cg.getTrackingAssets();
        // Assert
        expect(Object.keys(cg.assets)).toEqual(expect.arrayContaining(Object.keys(config.assets)));
    })
})