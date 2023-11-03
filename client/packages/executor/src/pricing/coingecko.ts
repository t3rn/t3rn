import { config } from "../../config/config";
import { BehaviorSubject } from "rxjs";
import axios from "axios";
import { logger } from "../logging";
import * as console from "console";

/**
 * MVP implementation of sourcing prices from coingecko
 *
 * @group Pricing
 */
export class CoingeckoPricing {
  /** Where to get the prices from */
  endpoint: string;
  /** Maps ticker to coingecko ID */
  assets: {
    [assetTicker: string]: string;
  } = {};
  /** How often to update the values. 0 disables updating */
  updateFrequency: number;
  /** Stores price in USD as a subject */
  prices: {
    [assetTicker: string]: BehaviorSubject<number>;
  } = {};
  /** Flag for finishing tests when debuging */
  debugFlag: boolean;

  constructor(updateFrequency?: number, debugFlag = false) {
    // Get the frequency when instantiating the class
    if (updateFrequency) {
      this.updateFrequency = updateFrequency;
    } else {
      this.updateFrequency = config.pricing.coingecko.frequency || 0;
    }
    this.debugFlag = debugFlag;
    this.endpoint = config.pricing.coingecko.endpoint;
    this.getTrackingAssets();
    // If testing, don't update the prices since they will never finish
    if (!this.debugFlag) {
      this.updateAssetPrices();
    }
  }

  /** Read the config file to initialize the list of assets we want to track */
  getTrackingAssets() {
    logger.info("Tracking assets");
    const keys = Object.keys(config.assets);
    for (let i = 0; i < keys.length; i++) {
      config.assets[keys[i]].forEach((asset) => {
        if (asset.priceSource === "coingecko") {
          logger.info(`Asset id: ${asset.id}`);
          this.assets[keys[i]] = asset.id;
          this.prices[keys[i]] = new BehaviorSubject<number>(0);
        }
      });
    }
  }

  /** Update the price of all assets we are tracking every 30 seconds */
  async updateAssetPrices() {
    const ids = Object.keys(this.assets);
    const alreadyUpdated = [];
    for (let i = 0; i < ids.length; i++) {
      if (alreadyUpdated[ids[i]]) {
        this.prices[ids[i]].next(alreadyUpdated[ids[i]]);
        continue;
      }
      await axios
        .get(
          config.pricing.coingecko.endpoint +
            this.assets[ids[i]] +
            config.pricing.coingecko.endpointDefaults,
        )
        .then((res) => {
          const price = parseFloat(res.data.market_data.current_price["usd"]);
          if (price !== this.prices[ids[i]].getValue()) {
            this.prices[ids[i]].next(price);
          }
          alreadyUpdated[ids[i]] = price;
          return new Promise((resolve) => setTimeout(resolve, 2000));
        })
        .catch((err) => {
          logger.error({ err }, "Failed fetching prices for asset %s", ids[i]);
        });
    }
    setTimeout(this.updateAssetPrices.bind(this), this.updateFrequency);
  }
}
