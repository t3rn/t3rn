import { BiddingStrategy } from "src/bidding";
import { SfxStrategy, XtxStrategy } from "src/index";

const WS_CIRCUIT_ENDPOINT =
  process.env.WS_CIRCUIT_ENDPOINT || "ws://127.0.0.1:9944";

/**
 * The gateway configuration for the executor.
 *
 * @group Configuration
 */
export type Gateway = {
  /** Name of the gateway. */
  name: string;
  /** Id of the gateway as stored in circuit. */
  id: string;
  /** Rpc endpoint to connect to. */
  rpc: string;
  /** Gateway type, currently only Substrate */
  type: string;
  /** The native ID of the gateway within its consensus system. e.g. paraId in polkadot */
  nativeId?: any;
  /** The assets the executor is willing to execute on the target. Matches the key used in assets */
  signerKey?: string;
};

/**
 * The circuit settings
 *
 * @group Configuration
 */
export type Circuit = {
  /** Endpoint */
  rpc: string;
  /** Ticker of native asset */
  ticker: string;
  /** Decimals of native asset */
  decimals: number;
  /** The private key used */
  signerKey?: string;
};

/**
 * Strategy type, used to define XTX and SFX strategies
 *
 * @group Configuration
 */
export type Strategy = {
  sfx: SfxStrategy;
  xtx: XtxStrategy;
  /** Assets that are supported for the target */
  supportedAssets: string[];
};

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
 *             rpc: "wss://rococo-basilisk-rpc.hydration.dev",
 *             type: "Substrate",
 *             nativeId: 2090,
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
  name: string;
  circuit: Circuit;
  /** The gateways that are being tracked */
  gateways: Gateway[];

  vendors: string[];
  /** The price sources that are being used */
  pricing: {
    [source: string]: {
      endpoint: string;
      endpointDefaults: string;
      frequency: number;
    };
  };
  /** Assets prices that should be tracked */
  assets: {
    /** The ticker of the asset */
    [assetTicker: string]: [
      {
        /** The price source, which must equal a key in the pricing object */
        priceSource: string;
        /** The internal id used by the price source. */
        id: string;
      }
    ];
  };
  /** The strategies that are being used for each gateway */
  strategies: {
    [targetId: string]: Strategy;
  };
  /** Parameters for tunning the behavior in the bidding stage */
  bidding: BiddingStrategy;

  /** Configuration for ethereum */
  attestations: {
    ethereum: {
      rpc: string;
      attestationVerifierAddress: string;
      privateKey: string | undefined;
    },
    batchesCatchUp: boolean
  }
};

export const config: Config = {
  name: "example",
  circuit: {
    rpc: WS_CIRCUIT_ENDPOINT,
    ticker: "TRN",
    decimals: 12,
    signerKey:
      "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561",
  },
  vendors: ["Rococo"],
  gateways: [
    {
      name: "Rococo",
      id: "roco",
      rpc: "wss://rococo-rpc.polkadot.io",
      type: "Substrate",
      signerKey:
        "0x0177d124e501887c2470e260c8f0da60db9ed3dba808a682f09afb39eff0c561",
    },
    {
      name: "Basilisk",
      id: "bslk",
      rpc: "wss://rococo-basilisk-rpc.hydration.dev",
      type: "Substrate",
      nativeId: 2090,
    },
  ],
  pricing: {
    coingecko: {
      endpoint: "https://api.coingecko.com/api/v3/coins/",
      endpointDefaults:
        "?localization=false&tickers=false&community_data=false&developer_data=false&sparkline=false",
      frequency: 30000,
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
    bslk: {
      supportedAssets: ["BSX"],
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
  bidding: {
    bidPercentile: 0.75,
    closerPercentageBid: 0.1,
    bidAggressive: true,
    bidMeek: false,
    overrideNoCompetition: true,
    equalMinProfitBid: false,
  },
  attestations: {
    ethereum: {
      rpc: "https://endpoints.omniatech.io/v1/eth/sepolia/public",
      attestationVerifierAddress: '0xAF1e49B67B4c8274f20d278f1f888b33Da5Ec284',
      privateKey: process.env.ETHEREUM_PRIVATE_KEY,
    },
    batchesCatchUp: process.env.BATCHES_CATCH_UP == "true" ? true : false

  }
};
