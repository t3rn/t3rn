import { StrategyEngine } from '../src/strategy/index';
import { BiddingEngine } from '../src/bidding/index';
import { SideEffect } from '../src/executionManager/sideEffect';

describe('Basic StrategyEngine setup', () => {
    it('should be a class', () => {
        expect(StrategyEngine).toBeInstanceOf(Function);
    });

    it('should have a constructor', () => {
        expect(StrategyEngine).toHaveProperty('constructor');
    });

    const se = new StrategyEngine();

    it('should have a method called "getMinProfitUsd"', () => {
        expect(se).toHaveProperty('getMinProfitUsd');
    });
})
