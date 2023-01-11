import { BiddingEngine } from '../src/bidding/index';
import { config } from "../config/config"
import { SideEffect } from "../src/executionManager/sideEffect"
import { expect } from 'chai';

describe('Configuration loading', () => {
    it('Correct config values are loaded', () => {
        const be = new BiddingEngine();

        // const sf = new SideEffect();

        // Config values
        expect(be.bidAggressive).to.be.equal(config.bidding.bidAggressive);
        expect(be.bidPercentile).to.be.equal(config.bidding.bidPercentile);
        expect(be.bidMeek).to.be.equal(config.bidding.bidMeek);
        expect(be.overrideNoCompetition).to.be.equal(config.bidding.overrideNoCompetition);
        expect(be.equalMinBid).to.be.equal(config.bidding.equalMinBid);
        expect(be.closerPercentageBid).to.be.equal(config.bidding.closerPercentageBid);

        // Other values can be checked too
        expect(be.timesBeenOutbid).to.be.equal(0);

    })
})

describe('Storing xtx bids for certain sfxs', () => {
    it("Correct storage on new key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Create values
        const myData = new Map<string, string[]>([
            ["sfxId1", ["xtxId1"]],
        ]);
        // Add one value to the storage
        be.storeWhoBidOnWhat("sfxId1", "xtxId1");

        // CHECKS
        // is the correct type
        expect(be.whoBidsOnWhat).to.be.an('Map');
        // Has the correct data
        expect(be.whoBidsOnWhat).to.have.keys("sfxId1");
        expect(be.whoBidsOnWhat).to.deep.include(["xtxId1"]);
    })

    it("Correct storage on existing key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Create values
        const myData = new Map<string, string[]>([
            ["sfxId1", ["xtxId1"]],
        ]);
        // Add two value to the storage
        be.storeWhoBidOnWhat("sfxId1", "xtxId1");
        be.storeWhoBidOnWhat("sfxId1", "xtxId2");

        // CHECKS
        // Is the correct type
        expect(be.whoBidsOnWhat).to.be.an('Map');
        // Has the correct data
        expect(be.whoBidsOnWhat).to.have.keys("sfxId1");
        expect(be.whoBidsOnWhat).to.deep.include(["xtxId1", "xtxId2"]);
    })

    it("Correct call on existing key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Create values
        const myData = new Map<string, string[]>([
            ["sfxId1", ["xtxId1"]],
        ]);
        // Add two value to the storage
        be.storeWhoBidOnWhat("sfxId1", "xtxId1");
        be.storeWhoBidOnWhat("sfxId1", "xtxId2");
        be.storeWhoBidOnWhat("sfxId2", "xtxId3");

        // CHECKS
        // Can correctly call first key values
        expect(be.whoBidsOnWhat.get("sfxId1")).to.deep.equal(["xtxId1", "xtxId2"]);
        // Can correctly call second key
        expect(be.whoBidsOnWhat.get("sfxId2")).to.deep.equal(["xtxId3"]);
    })
})