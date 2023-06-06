import { CoingeckoPricing } from "./coingecko";
import { BehaviorSubject } from "rxjs";

/**
 * Class used for keeping track of asset prices. This class has access to different price sources, allowing the generation of averages of
 * any kind. All values are subjects, allowing the user to subscribe to price updates.
 *
 * @group Pricing
 */
export class PriceEngine {
  coingecko: CoingeckoPricing;

  constructor(updateFrequency?: number, debugFlag = false) {
    this.coingecko = new CoingeckoPricing(updateFrequency || 0, debugFlag);
  }

  /**
   * Returns the price of an asset in USD
   *
   * @param assetId Ticker of the asset we want the price of. These are set in the config file.
   * @returns The price of the asset in USD as a subject
   */
  getAssetPrice(assetId: string): BehaviorSubject<number> {
    return this.coingecko.prices[assetId];
  }
}

export { CoingeckoPricing };
