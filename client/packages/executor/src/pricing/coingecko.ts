import BN from "bn.js"
import { config } from "../../config/config"
import { BehaviorSubject } from "rxjs"
const axios = require("axios")

/**
 * MVP implementation of sourcing prices from coingecko
 *
 * @group Pricing
 */
export class CoingeckoPricing {
    endpoint: string
    /** Maps ticker to coingecko ID */
    assets: {
        [assetTicker: string]: string
    } = {}

    /** Stores price in USD as a subject */
    prices: {
        [assetTicker: string]: BehaviorSubject<BN>
    } = {}

    constructor() {
        this.endpoint = config.pricing.coingecko.endpoint
        this.getTrackingAssets()
        // This cannot be called here if we want to test it (it doesn't finish)
        this.updateAssetPrices()
    }

    /** Read the config file to initialize the list of assets we want to track */
    getTrackingAssets() {
        let keys = Object.keys(config.assets)
        for (let i = 0; i < keys.length; i++) {
            config.assets[keys[i]].forEach((asset) => {
                if (asset.priceSource === "coingecko") {
                    this.assets[keys[i]] = asset.id
                    this.prices[keys[i]] = new BehaviorSubject<BN>(new BN.BN(0))
                }
            })
        }
    }

    /** Update the price of all assets we are tracking every 30 seconds */
    async updateAssetPrices() {
        const ids = Object.keys(this.assets)
        for (let i = 0; i < ids.length; i++) {
            await axios
                .get(
                    config.pricing.coingecko.endpoint +
                    this.assets[ids[i]] +
                    "?localization=false&tickers=false&community_data=false&developer_data=false&sparkline=false"
                )
                .then((res) => {
                    const price = new BN.BN(parseFloat(res.data.market_data.current_price["usd"]))
                    if (price !== this.prices[ids[i]].getValue()) {
                        this.prices[ids[i]].next(price)
                    }
                })
        }
        setTimeout(this.updateAssetPrices.bind(this), 30000)
    }
}
