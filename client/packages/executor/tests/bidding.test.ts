import { BiddingEngine, Scenario } from '../src/bidding/index';
import { SideEffect } from '../src/executionManager/sideEffect';
import { config } from "../config/config"
import { expect } from 'chai';
import { mock, instance, when } from 'ts-mockito';
import { BehaviorSubject } from 'rxjs';
const pino = require("pino")
describe('Bidding: Configuration loading', () => {
    it('Correct config values are loaded', () => {
        const be = new BiddingEngine(pino());

        // Config values
        expect(be.bidAggressive).to.be.equal(config.bidding.bidAggressive);
        expect(be.bidPercentile).to.be.equal(config.bidding.bidPercentile);
        expect(be.bidMeek).to.be.equal(config.bidding.bidMeek);
        expect(be.overrideNoCompetition).to.be.equal(config.bidding.overrideNoCompetition);
        expect(be.equalMinProfitBid).to.be.equal(config.bidding.equalMinProfitBid);
        expect(be.closerPercentageBid).to.be.equal(config.bidding.closerPercentageBid);
    })
})

describe('Bidding: Storing executor bids for certain sfxs', () => {
    it("Correct storage on new key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine(pino());

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
        const be = new BiddingEngine(pino());

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
        const be = new BiddingEngine(pino());

        // Add two value to the storage
        be.storeWhoBidOnWhat("sfxId1", "xtxId1");
        be.storeWhoBidOnWhat("sfxId1", "xtxId2");
        be.storeWhoBidOnWhat("sfxId2", "xtxId3");

        // ASSERTS
        // Can correctly call first key values
        expect(be.whoBidsOnWhat.get("sfxId1")).to.deep.equal(["xtxId1", "xtxId2"]);
        // Can correctly call second key
        expect(be.whoBidsOnWhat.get("sfxId2")).to.deep.equal(["xtxId3"]);
    })
})

describe("Bidding: Scenario selection", () => {
    it("should compute correct 'outbid' scenario", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(false);
        when(mockedSideEffect.lastBids).thenReturn([]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).to.be.equal(Scenario.noBidAndNoCompetition);
    })

    it("should compute correct 'no bid and competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(true);
        when(mockedSideEffect.lastBids).thenReturn([0]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).to.be.equal(Scenario.noBidButCompetition);
    })

    it("should compute correct 'no bid and no competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(true);
        when(mockedSideEffect.lastBids).thenReturn([]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).to.be.equal(Scenario.noBidAndNoCompetition);
    })
})

describe("Bidding: computation at scenarios", () => {
    it("should compute correctly in 'no bid and no competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        // Create a mocked side effect
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.maxProfitUsd).thenReturn(new BehaviorSubject(100));
        when(mockedSideEffect.minProfitUsd).thenReturn(10);
        when(mockedSideEffect.txOutputCostUsd).thenReturn(5);

        // ACT + ASSERT
        be.overrideNoCompetition = true;
        let bid = be.computeNoBidAndNoCompetition(se);
        expect(bid).to.be.equal(15);

        be.overrideNoCompetition = false;
        bid = be.computeNoBidAndNoCompetition(se);
        expect(bid).to.be.equal(115);
    })

    it("should compute correctly in 'no bid but competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        // Create a mocked side effect
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.maxProfitUsd).thenCall(() => { return new BehaviorSubject(100); });
        when(mockedSideEffect.minProfitUsd).thenReturn(10);
        when(mockedSideEffect.txOutputCostUsd).thenReturn(5);

        // ACT + ASSERT
        be.bidAggressive = true;
        let bid = be.computeNoBidButCompetition(se);
        expect(bid).to.be.equal(5 + 10);

        be.bidAggressive = false;
        be.bidMeek = true;
        bid = be.computeNoBidButCompetition(se);
        expect(bid).to.be.equal(5 + 10 + 100);

        be.bidAggressive = false;
        be.bidMeek = false;
        bid = be.computeNoBidButCompetition(se);
        expect(bid).to.be.equal(5 + 10 + (100 - 10) * 0.75);
    })
})

describe("Bidding: check helper functions", () => {
    it("should compute been outbid", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        const mockedSideEffect: SideEffect = mock(SideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(true);
        when(mockedSideEffect.id).thenReturn("0");
        const se: SideEffect = instance(mockedSideEffect);

        // ACT + ASSERT
        expect(be.checkOutbid(se)).to.be.true;
    })

    it("should compute not been outbid", () => {
        // ARANGE
        const be = new BiddingEngine(pino());
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(false);

        // ACT + ASSERT
        expect(be.checkOutbid(se)).to.be.false;
    })

    it("should clean up stored values sfx and bidder ids", () => {
        // ARANGE
        const sfxId: string = "sfxId1";
        const bidderId: string = "bidderId1";

        const be = new BiddingEngine(pino());
        const mockedSideEffect: SideEffect = mock(SideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(true);
        when(mockedSideEffect.id).thenReturn('0');

        // ACT + ASSERT
        be.storeWhoBidOnWhat(sfxId, bidderId);
        expect(be.whoBidsOnWhat).to.have.keys(sfxId);
        expect(be.whoBidsOnWhat).to.deep.include([bidderId]);

        be.cleanUp(sfxId);
        expect(be.numberOfBidsOnSfx[sfxId]).to.equal(undefined);
        expect(be.whoBidsOnWhat[sfxId]).to.equal(undefined);
        expect(be.timesBeenOutbid[sfxId]).to.equal(undefined);
    })
})