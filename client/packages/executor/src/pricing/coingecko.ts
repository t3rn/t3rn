import config from "../../config/config"
import { BehaviorSubject } from "rxjs"
const axios = require("axios")

export class CoingeckoPricing {
    endpoint: string
    assets: {
        [assetTicker: string]: string
    } = {}

    prices: {
        [assetTicker: string]: BehaviorSubject<number>
    } = {}

    constructor() {
        this.endpoint = config.priceSource.coingecko.endpoint
        this.getTrackingAssets()
        this.updateAssetPrices()
    }

    getTrackingAssets() {
        let keys = Object.keys(config.assets)
        for (let i = 0; i < keys.length; i++) {
            if (config.assets[keys[i]].priceSource === "coingecko") {
                this.assets[keys[i]] = config.assets[keys[i]].id
                this.prices[keys[i]] = new BehaviorSubject<number>(0)
            }
        }
    }

    async updateAssetPrices() {
        const ids = Object.keys(this.assets)
        for (let i = 0; i < ids.length; i++) {
            await axios
                .get(
                    config.priceSource.coingecko.endpoint +
                        this.assets[ids[i]] +
                        "?localization=false&tickers=false&community_data=false&developer_data=false&sparkline=false"
                )
                .then((res) => {
                    const price = parseFloat(res.data.market_data.current_price["usd"])
                    if (price !== this.prices[ids[i]].getValue()) {
                        this.prices[ids[i]].next(price)
                    }
                })
        }
        setTimeout(this.updateAssetPrices.bind(this), 30000)
    }
}
