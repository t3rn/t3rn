import { CoingeckoPricing } from "../src/pricing/coingecko";
import { PriceEngine } from "../src/pricing/index";
import { expect } from "chai";
import { config } from "../config/config";

describe("Basic PriceEngine setup", () => {
    const pe = new PriceEngine(0, true);

    it("should be a class", () => {
        expect(PriceEngine).to.be.a("function");
    });

    it("should have a constructor", () => {
        expect(pe).to.have.property("constructor");
    });

    it("should have a method called 'getAssetPrice'", () => {
        expect(pe).to.have.property("getAssetPrice");
    });
});

describe("Basic CoinGecko setup", () => {
    it("should be a class", () => {
        expect(CoingeckoPricing).to.be.a("function");
    });

    it("should have a constructor and methods", async () => {
        const cg = await new CoingeckoPricing(0, true);
        expect(cg).to.have.property("constructor");
        expect(cg).to.have.property("getTrackingAssets");
        expect(cg).to.have.property("updateAssetPrices");
    });

    it("should load the assets for tracking", async () => {
        // Create a new instance of the class
        const cg = await new CoingeckoPricing(0, true);
        // Load the assets
        cg.getTrackingAssets();

        // CHECKS
        expect(cg.assets).to.include.all.keys(Object.keys(config.assets));
    })
})
