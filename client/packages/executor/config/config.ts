import { SfxStrategy, XtxStrategy } from "../src"

/**
 * The gateway configuration for the executor.
 *
 * @group Configuration
 */
export type Gateway = {
    /** Name of the gateway. */
    name: string
    /** Id of the gateway as stored in circuit. */
    id: string
    /** Rpc endpoint to connect to. */
    rpc: string
    /** Gateway type, currently only Substrate */
    type: string
    /** The assets the executor is willing to execute on the target. Matches the key used in assets */
    signerKey?: string
}

/**
 * The circuit settings
 *
 * @group Configuration
 */
export type Circuit = {
    /** Endpoint */
    rpc: string
    /** Ticker of native asset */
    ticker: string
    /** Decimals of native asset */
    decimals: number
    /** The private key used */
    signerKey?: string
}

/**
 * Strategy type, used to define XTX and SFX strategies
 *
 * @group Configuration
 */
export type Strategy = {
    sfx: SfxStrategy
    xtx: XtxStrategy,
    /** Assets that are supported for the target */
    supportedAssets: string[]
}

/**
 * The configuration for the executor.
 *
 * Example Configuration:
 *
 * ```ts
 * export const config: Config = {
 *     circuit: {
 *         rpc: "ws://127.0.0.1::9944",
 *         ticker: "TRN",
 *         decimals: 12,
 *     },
 *     gateways: [
 *         {
 *             name: "Rococo",
 *             id: "roco",
 *             rpc: "wss://rococo-rpc.polkadot.io",
 *             type: "Substrate",
 *         },
 *         {
 *             name: "Basilisk",
 *             id: "bslk",
 *             rpc: "wss://rpc-01.basilisk-rococo.hydradx.io",
 *             type: "Substrate",
 *         },
 *     ],
 *     pricing: {
 *         coingecko: {
 *             endpoint: "https://api.coingecko.com/api/v3/coins/",
 *         },
 *     },
 *     assets: {
 *         BSX: [
 *             {
 *                 priceSource: "coingecko",
 *                 id: "basilisk",
 *             },
 *         ],
 *         ROC: [
 *             {
 *                 priceSource: "coingecko",
 *                 id: "polkadot",
 *             },
 *         ],
 *         TRN: [
 *             {
 *                 priceSource: "coingecko",
 *                 id: "tether",
 *             },
 *         ],
 *     },
 *     strategies: {
 *         roco: {
 *             supportedAssets: ["ROC"],
 *             sfx: {
 *                 minProfitUsd: 3,
 *                 minYield: 0.05,
 *             },
 *             xtx: {
 *                 minInsuranceAmountUsd: 1,
 *                 minInsuranceShare: 0.1,
 *             },
 *         },
 *     },
 * }
 * ```
 *
 * - @group Configuration
 */
export type Config = {
    circuit: Circuit
    /** The gateways that are being tracked */
    gateways: Gateway[]
    /** The price sources that are being used */
    pricing: {
        [source: string]: {
            [key: string]: string
        }
    }
    /** Assets prices that should be tracked */
    assets: {
        /** The ticker of the asset */
        [assetTicker: string]: [
            {
                /** The price source, which must equal a key in the pricing object */
                priceSource: string
                /** The internal id used by the price source. */
                id: string
            }
        ]
    }
    /** The strategies that are being used for each gateway */
    strategies: {
        [targetId: string]: Strategy
    }
}

export const config: Config = {
    circuit: {
        rpc: "ws://127.0.0.1::9944",
        ticker: "TRN",
        decimals: 12,
        signerKey: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561"
    },
    gateways: [
        {
            name: "Rococo",
            id: "roco",
            rpc: "wss://rococo-rpc.polkadot.io",
            type: "Substrate",
            signerKey: "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561"
        },
        {
            name: "Basilisk",
            id: "bslk",
            rpc: "wss://rpc-01.basilisk-rococo.hydradx.io",
            type: "Substrate",
        },
    ],
    pricing: {
        coingecko: {
            endpoint: "https://api.coingecko.com/api/v3/coins/",
        },
    },
    assets: {
        BSX: [
            {
                priceSource: "coingecko",
                id: "basilisk",
            },
        ],
        ROC: [
            {
                priceSource: "coingecko",
                id: "polkadot",
            },
        ],
        TRN: [
            {
                priceSource: "coingecko",
                id: "tether",
            },
        ],
    },
    strategies: {
        roco: {
            supportedAssets: ["ROC"],
            sfx: {
                minProfitUsd: 3,
                minYield: 0.05,
            },
            xtx: {
                minInsuranceAmountUsd: 1,
                minInsuranceShare: 0.1,
            },
        },
    },
}
