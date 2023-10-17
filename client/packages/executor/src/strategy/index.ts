// import config from "../../config/config"
import { SideEffect } from "../executionManager/sideEffect";
import { Execution } from "../executionManager/execution";
import { config } from "../../config/config";

/**
 * Type used for describing XTX strategies When an XTX is created, the XTX strategy will be evaluated. If the XTX fails the evaluation, the
 * XTX will be rejected, preventing any bids to be submitted
 *
 * @group Strategy
 */
export type XtxStrategy = {
  minInsuranceAmountUsd?: number;
  minInsuranceShare?: number; // minInsuranceAmountUsd / maxProfit
};

/**
 * Type used for describing SFX strategies. These are used to determine the profitability of a given SFX, deciding if a bid should be submitted.
 *
 * @group Strategy
 */
export type SfxStrategy = {
  /** Minimum profit in USD that a SFX should have to be considered profitable. */
  minProfitUsd?: number;
  /** Minimum profit in target's asset that a SFX should have to be considered profitable. */
  minProfitTargetAssetAmount?: number;
  /** A percentage value for the minimum yield that a SFX should have Yield is defined by (minProfit / totalCost) */
  minYield?: number;
  /** The max tx costs in USD for an execution. This can be useful to prevent executions during network congestion. */
  maxTxFeesUsd?: number;
  /** The max share of txCost to profit. This is defined by txCost / maxProfit */
  maxTxFeeShare?: number; // txCost / maxProfit
  /**
   * The maximum cost in USD to spend on a single SFX. This only includes the cost of the assets that are being sent. This puts a cap on
   * the value of assets to be sent.
   */
  maxAssetCost?: number; // maximum value spend
};

/**
 * The strategy engine is used to decide if a SFX should be executed or not. The decision is seperated into two parts:
 *
 * XTX Strategy: Before the strategy engine evaluates individual SFXs, it evaluates the XTX as a whole. To complete a XTX, all SFXs must be
 * executed and confirmed. For this reason the XTX strategy can be used to evaluate a set of constraints for each SFX in the XTX. If any of
 * the SFXs in the XTX fail the XTX strategy, the XTX will be rejected and not further tracked.
 *
 * For example, this can be useful for ensuring every SFX as a certain amount of insurance. XTXs containing SFXs with low insurance risk
 * being reverted, as the fine for not completing the SFX is potentially too low. Executors can use the parameters provided in the
 * XtXStrategy to enforce XTX-wide constraints.
 *
 * SFX Strategy: If an XTX passes the XTX strategy, the strategy engine will evaluate each SFX individually. To goal here is to determine if
 * a SFX passes all constraints set by the SFX strategy. If so, the executor will then submit a bid for the SFX. The bid amount is _not_
 * determined by the strategy engine, but by the bidding engine.
 *
 * @group Strategy
 */
export class StrategyEngine {
  sfxStrategies: {
    [target: string]: SfxStrategy;
  } = {};

  xtxStrategies: {
    [target: string]: XtxStrategy;
  } = {};

  supportedAssets: {
    [target: string]: string[];
  } = {};

  constructor() {
    this.loadStrategies();
  }

  /** Loads the strategies from the config file. */
  loadStrategies() {
    const strategyTargets = Object.keys(config.strategies);
    for (let i = 0; i < strategyTargets.length; i++) {
      const strategy = config.strategies[strategyTargets[i]];
      this.sfxStrategies[strategyTargets[i]] = strategy.sfx;
      this.xtxStrategies[strategyTargets[i]] = strategy.xtx;
      this.supportedAssets[strategyTargets[i]] = strategy.supportedAssets;
    }
  }

  /**
   * Evaluates the XTX strategy for a given XTX. If the XTX fails the evaluation, the XTX will be rejected, preventing any bids to be submitted
   *
   * @param xtx Object of XTX to be evaluated
   */
  evaluateXtx(xtx: Execution): void | Error {
    for (const [, sfx] of xtx.sideEffects) {
      const strategy = this.xtxStrategies[sfx.target];
      try {
        this.minInsuranceAmountRejected(sfx, strategy);
        this.minInsuranceShareRejected(sfx, strategy);
      } catch (e) {
        throw new Error(e);
      }
    }
  }

  /**
   * Evaluates the SFX strategy for a given SFX.
   *
   * @param sfx Object of SFX to be evaluated
   */
  evaluateSfx(sfx: SideEffect) {
    const strategy = this.sfxStrategies[sfx.target];

    try {
      this.isSupportedGateway(sfx);
      this.assetIsSupported(sfx);
      this.minProfitRejected(sfx, strategy);
      this.minYieldRejected(sfx, strategy);
      this.maxTxFeesRejected(sfx, strategy);
      this.maxTxFeeShareRejected(sfx, strategy);
      this.maxAssetCostRejected(sfx, strategy);
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Checks if the gateway is supported by the executor. If a config for the gateway exists, this is the case.
   *
   * @param sfx Object of SFX to be evaluated
   */
  isSupportedGateway(sfx: SideEffect) {
    if (!(sfx.target in this.sfxStrategies)) {
      throw new Error("Gateway not supported");
    }
  }

  /**
   * Returns minProfitUsd constraint from the SFX strategy for a given target.
   *
   * @param sfx Object of SFX to be evaluated
   * @returns MinProfitUsd constraint from the SFX strategy for a given target
   */
  getMinProfitUsd(sfx: SideEffect): number {
    const strategy = this.sfxStrategies[sfx.target];
    if (strategy.minProfitUsd) {
      return strategy.minProfitUsd;
    }
    return 0;
  }

  /**
   * Returns minProfitUsd constraint from the SFX strategy for a given target.
   *
   * @param sfx Object of SFX to be evaluated
   * @returns MinProfitUsd constraint from the SFX strategy for a given target
   */
  getMinProfitTargetAsset(sfx: SideEffect): number {
    const strategy = this.sfxStrategies[sfx.target];
    if (strategy.minProfitTargetAssetAmount) {
      return strategy.minProfitTargetAssetAmount;
    }
    return 0;
  }
  /**
   * Evaluates the minProfitUsd constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the minProfitUsd constraint is not met
   */
  minProfitRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
    if (strategy.minProfitUsd) {
      if (sfx.maxProfitUsd.getValue() <= strategy.minProfitUsd) {
        throw new Error("Min Profit condition not met!");
      }
    }
  }

  /**
   * Evaluates the minYield constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the minYield constraint is not met
   */
  minYieldRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
    if (strategy.minYield) {
      if (this.computeShare(sfx, "yield") <= strategy.minYield) {
        throw new Error("Min Yield condition not met!");
      }
    }
  }

  /**
   * Evaluates the maxTxFeesUsd constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the maxTxFeesUsd constraint is not met
   */
  maxTxFeesRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
    if (strategy.maxTxFeesUsd) {
      if (sfx.txCostUsd >= strategy.maxTxFeesUsd) {
        throw new Error("Max Tx Fees condition not met!");
      }
    }
  }

  /**
   * Evaluates the output asset is supported by the executor
   *
   * @param sfx Side effect to be evaluated
   * @returns Error if asset is not supported
   */
  assetIsSupported(sfx: SideEffect): void | Error {
    const txOutputs = sfx.getTxOutputs();
    const assetTicker = txOutputs?.asset;
    if (!this.supportedAssets[sfx.target].includes(assetTicker)) {
      throw new Error("Asset is not supported by the target gateway!");
    }
  }

  /**
   * Evaluates the maxTxFeeShare constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the maxTxFeeShare constraint is not met
   */
  maxTxFeeShareRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
    if (strategy.maxTxFeeShare) {
      if (this.computeShare(sfx, "fee") >= strategy.maxTxFeeShare) {
        throw new Error("Max Tx Fee Share condition not met!");
      }
    }
  }

  /**
   * Evaluates the maxAssetCost constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the maxAssetCost constraint is not met
   */
  maxAssetCostRejected(sfx: SideEffect, strategy: SfxStrategy): void | Error {
    if (strategy.maxAssetCost) {
      if (sfx.txOutputCostUsd >= strategy.maxAssetCost) {
        throw new Error("Max Asset Cost condition not met!");
      }
    }
  }

  /**
   * Evaluates the minInsuranceAmountUsd constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the minInsuranceAmountUsd constraint is not met
   */
  minInsuranceAmountRejected(
    sfx: SideEffect,
    strategy: XtxStrategy,
  ): void | Error {
    if (strategy.minInsuranceAmountUsd) {
      if (sfx.insurance < strategy.minInsuranceAmountUsd) {
        throw new Error("Min Insurance Amount  condition not met!");
      }
    }
  }

  /**
   * Evaluates the minInsuranceAmountUsd constraint from the SFX strategy for a given SFX.
   *
   * @param sfx Side effect to be evaluated
   * @param strategy Strategy for the specific target
   * @returns Error if the minInsuranceAmountUsd constraint is not met
   */
  minInsuranceShareRejected(
    sfx: SideEffect,
    strategy: XtxStrategy,
  ): void | Error {
    if (strategy.minInsuranceShare) {
      // reward and insurance are in the same asset, so no USD conversion is needed
      if (this.computeShare(sfx, "insurance") > strategy.minInsuranceShare) {
        throw new Error("Min Insurance Share condition not met!");
      }
    }
  }

  /**
   * Computes different types of shares for a given SFX.
   *
   * @param sfx Object of SFX compute
   * @param type - Fee, insurance or yield
   * @returns Share of the given type
   */
  computeShare(sfx: SideEffect, type: string): number {
    if (type === "fee") {
      return sfx.txCostUsd / sfx.maxProfitUsd.getValue();
    } else if (type === "insurance") {
      return sfx.insurance / sfx.reward.getValue();
    } else if (type === "yield") {
      return sfx.maxProfitUsd.getValue() / sfx.txOutputCostUsd;
    }

    return 0;
  }
}
