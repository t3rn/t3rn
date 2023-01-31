import { BiddingEngine, Scenario } from '../src/bidding/index';
import { SideEffect } from '../src/executionManager/sideEffect';
import { config } from "../config/config"
import { mock, instance, when } from 'ts-mockito';
import { BehaviorSubject } from 'rxjs';

describe('Bidding: Configuration loading', () => {
    it('Correct config values are loaded', () => {
        const be = new BiddingEngine();

        // Config values
        expect(be.bidAggressive).toBe(config.bidding.bidAggressive);
        expect(be.bidPercentile).toBe(config.bidding.bidPercentile);
        expect(be.bidMeek).toBe(config.bidding.bidMeek);
        expect(be.overrideNoCompetition).toBe(config.bidding.overrideNoCompetition);
        expect(be.equalMinProfitBid).toBe(config.bidding.equalMinProfitBid);
        expect(be.closerPercentageBid).toBe(config.bidding.closerPercentageBid);
    })
})

describe('Bidding: Storing executor bids for certain sfxs', () => {
    it("Correct storage on new key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Add one value to the storage
        be.storeWhoBidOnWhat("sfxId1", "bidderId1");

        // CHECKS
        // is the correct type
        expect(be.whoBidsOnWhat).toBeInstanceOf(Map);
        // Has the correct data
        expect(be.whoBidsOnWhat).toMatchObject(new Map([["sfxId1", ["bidderId1"]]]));
    })

    it("Correct storage on existing key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Add two value to the storage
        be.storeWhoBidOnWhat("sfxId1", "bidderId1");
        be.storeWhoBidOnWhat("sfxId1", "bidderId2");

        // CHECKS
        // Is the correct type
        expect(be.whoBidsOnWhat).toBeInstanceOf(Map);
        expect(be.whoBidsOnWhat).toMatchObject(new Map([["sfxId1", ["bidderId1", "bidderId2"]]]));
    })

    it("Correct call on existing key", () => {
        // Create a bidding engine instance
        const be = new BiddingEngine();

        // Add two value to the storage
        be.storeWhoBidOnWhat("sfxId1", "bidderId1");
        be.storeWhoBidOnWhat("sfxId1", "bidderId2");
        be.storeWhoBidOnWhat("sfxId2", "xtxId3");

        // ASSERTS
        // Can correctly call first key values
        expect(be.whoBidsOnWhat.get("sfxId1")).toEqual(["bidderId1", "bidderId2"]);
        // Can correctly call second key
        expect(be.whoBidsOnWhat.get("sfxId2")).toEqual(["xtxId3"]);
    })
})

describe("Bidding: Scenario selection", () => {
    it("should compute correct 'outbid' scenario", () => {
        // ARANGE
        const be = new BiddingEngine();
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(false);
        when(mockedSideEffect.lastBids).thenReturn([]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).toBe(Scenario.beenOutbid);
    })

    it("should compute correct 'no bid and competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine();
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(true);
        when(mockedSideEffect.lastBids).thenReturn([0]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).toBe(Scenario.noBidButCompetition);
    })

    it("should compute correct 'no bid and no competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine();
        // Create a mock
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.isBidder).thenReturn(true);
        when(mockedSideEffect.lastBids).thenReturn([]);

        // ACT
        const scenario = be.checkScenario(se);
        // ASSERT
        expect(scenario).toBe(Scenario.noBidAndNoCompetition);
    })
})

describe("Bidding: computation at scenarios", () => {
    it("should compute correctly in 'no bid and no competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine();
        // Create a mocked side effect
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.maxProfitUsd).thenReturn(new BehaviorSubject(100));
        when(mockedSideEffect.minProfitUsd).thenReturn(10);
        when(mockedSideEffect.txOutputCostUsd).thenReturn(5);

        // ACT + ASSERT
        be.overrideNoCompetition = true;
        let bid = be.computeNoBidAndNoCompetition(se);
        expect(bid).toBe(15);

        be.overrideNoCompetition = false;
        bid = be.computeNoBidAndNoCompetition(se);
        expect(bid).toBe(115);
    })

    it("should compute correctly in 'no bid but competition' scenario", () => {
        // ARANGE
        const be = new BiddingEngine();
        // Create a mocked side effect
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.maxProfitUsd).thenCall(() => { return new BehaviorSubject(100); });
        when(mockedSideEffect.minProfitUsd).thenReturn(10);
        when(mockedSideEffect.txOutputCostUsd).thenReturn(5);

        // ACT + ASSERT
        be.bidAggressive = true;
        let bid = be.computeNoBidButCompetition(se);
        expect(bid).toBe(5 + 10);

        be.bidAggressive = false;
        be.bidMeek = true;
        bid = be.computeNoBidButCompetition(se);
        expect(bid).toBe(5 + 10 + 100);

        be.bidAggressive = false;
        be.bidMeek = false;
        bid = be.computeNoBidButCompetition(se);
        expect(bid).toBe(5 + 10 + (100 - 10) * 0.75);
    })
})

describe("Bidding: check helper functions", () => {
    it("should compute been outbid", () => {
        // ARANGE
        const be = new BiddingEngine();
        const mockedSideEffect: SideEffect = mock(SideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(true);
        when(mockedSideEffect.id).thenReturn("0");
        const se: SideEffect = instance(mockedSideEffect);

        // ACT + ASSERT
        expect(be.checkOutbid(se)).toBe(true);
    })

    it("should compute not been outbid", () => {
        // ARANGE
        const be = new BiddingEngine();
        const mockedSideEffect: SideEffect = mock(SideEffect);
        const se: SideEffect = instance(mockedSideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(false);

        // ACT + ASSERT
        expect(be.checkOutbid(se)).toBe(false);
    })

    it("should clean up stored values sfx and bidder ids", () => {
        // ARANGE
        const sfxId: string = "sfxId1";
        const bidderId: string = "bidderId1";

        const be = new BiddingEngine();
        const mockedSideEffect: SideEffect = mock(SideEffect);
        when(mockedSideEffect.changedBidLeader).thenReturn(true);
        when(mockedSideEffect.id).thenReturn('0');

        // ACT + ASSERT
        be.storeWhoBidOnWhat(sfxId, bidderId);
        expect(be.whoBidsOnWhat).toMatchObject(new Map([[sfxId, [bidderId]]]));
        be.cleanUp(sfxId);
        expect(be.numberOfBidsOnSfx[sfxId]).toBeUndefined();
        expect(be.whoBidsOnWhat[sfxId]).toBeUndefined();
        expect(be.timesBeenOutbid[sfxId]).toBeUndefined();
    })
})