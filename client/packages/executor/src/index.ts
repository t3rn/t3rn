import "@polkadot/api-augment";
import * as dotenv from "dotenv";
import "@t3rn/types";
import {
  SubstrateRelayer,
  CostEstimator,
  Estimator,
  Estimate,
  InclusionProof,
} from "./gateways/substrate/relayer";
import { ExecutionManager, Queue } from "./executionManager";
import { Circuit, Strategy, Gateway, Config } from "../config/config";
import { BiddingEngine, BiddingStrategy } from "./bidding";
import { PriceEngine, CoingeckoPricing } from "./pricing";
import { StrategyEngine, SfxStrategy, XtxStrategy } from "./strategy";
import {
  SideEffect,
  Notification,
  NotificationType,
  TxOutput,
  TxStatus,
} from "./executionManager/sideEffect";
import { Execution } from "./executionManager/execution";
import {
  CircuitListener,
  ListenerEvents,
  ListenerEventData,
} from "./circuit/listener";
import { CircuitRelayer } from "./circuit/relayer";
import { Executor } from "./executor.class";

dotenv.config();

export {
  Executor,
  ExecutionManager,
  Queue,
  Execution,
  SideEffect,
  Notification,
  NotificationType,
  TxOutput,
  TxStatus,
  SubstrateRelayer,
  Estimator,
  CostEstimator,
  Estimate,
  InclusionProof,
  BiddingEngine,
  StrategyEngine,
  SfxStrategy,
  XtxStrategy,
  PriceEngine,
  CoingeckoPricing,
  CircuitListener,
  ListenerEvents,
  ListenerEventData,
  CircuitRelayer,
  Config,
  Circuit,
  Gateway,
  Strategy,
  BiddingStrategy,
};
