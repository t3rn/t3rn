import { BiddingStrategy } from "src/bidding";
import { SfxStrategy, XtxStrategy } from "src/index";

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
  /** The account prefix used by the target */
  accountPrefix: number;
};

/**
 * The circuit settings
 *
 * @group Configuration
 */
export type Circuit = {
  /** Name of the chain */
  name: string;
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
      },
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
      name: string;
      rpc: string;
      attestationVerifierAddress: string;
      privateKey: string | undefined;
    };
    processBatches: boolean;
  };
};

export const config: Config = {
  name: "example",
  circuit: {
    name: "t0rn",
    rpc: process.env.CIRCUIT_WS_ENDPOINT || "ws://127.0.0.1:9944",
    ticker: "TRN",
    decimals: 12,
    signerKey: process.env.CIRCUIT_SIGNER_KEY,
  },
  vendors: ["Polkadot"],
  gateways: [
    {
      name: "Rococo",
      id: "roco",
      rpc: "wss://rococo-rpc.polkadot.io",
      type: "Substrate",
      signerKey: process.env.RELAYCHAIN_SIGNER_KEY,
      accountPrefix: 42,
    },
    {
      name: "Polkadot",
      id: "pdot",
      rpc: "wss://rpc.polkadot.io",
      type: "Substrate",
      signerKey: process.env.RELAYCHAIN_SIGNER_KEY,
      accountPrefix: 42,
    },
    {
      name: "Basilisk",
      id: "bslk",
      rpc: "wss://rococo-basilisk-rpc.hydration.dev",
      type: "Substrate",
      nativeId: 2090,
      accountPrefix: 10041,
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
    DOT: [
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
    pdot: {
      supportedAssets: ["DOT"],
      sfx: {
        minProfitUsd: 1,
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
      name: "sepl",
      rpc: "https://endpoints.omniatech.io/v1/eth/sepolia/public",
      attestationVerifierAddress: "0x12b6B6F917b9B1af3751eBe41b0A1D7D1a0d4a29",
      privateKey: process.env.ETHEREUM_PRIVATE_KEY,
    },
    processBatches: process.env.PROCESS_BATCHES == "true" ? true : false,
  },
};
