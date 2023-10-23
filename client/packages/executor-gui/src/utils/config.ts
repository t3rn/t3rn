import { Store } from "tauri-plugin-store-api";
import {
  CIRCUIT_SIGNER_KEY,
  CIRCUIT_WS_ENDPOINT,
  ETHEREUM_PRIVATE_KEY,
  EXECUTOR,
  LOG_LEVEL,
  LOG_PRETTY,
  PROCESS_BATCHES,
  RELAYCHAIN_SIGNER_KEY,
} from "@/consts";

export interface Config {
  executor?: string;
  logLevel?: string;
  logPretty?: string;
  circuitSignerKey?: string;
  circuitWsEndpoint?: string;
  ethereumPrivateKey?: string;
  relayChainSignerKey?: string;
  processBatches?: string;
}

export const getConfig = async (store: Store) => {
  const executor = await store.get(EXECUTOR);
  const logLevel = await store.get(LOG_LEVEL);
  const logPretty = await store.get(LOG_PRETTY);
  const circuitSignerKey = await store.get(CIRCUIT_SIGNER_KEY);
  const circuitWsEndpoint = await store.get(CIRCUIT_WS_ENDPOINT);
  const ethereumPrivateKey = await store.get(ETHEREUM_PRIVATE_KEY);
  const relayChainSignerKey = await store.get(RELAYCHAIN_SIGNER_KEY);
  const processBatches = await store.get(PROCESS_BATCHES);
  return {
    executor,
    logLevel,
    logPretty,
    circuitSignerKey,
    circuitWsEndpoint,
    ethereumPrivateKey,
    relayChainSignerKey,
    processBatches,
  } as Config;
};

export const getEnv = async (store: Store) => {
  const config = await getConfig(store);
  const env: Record<string, string> = {};

  if (config.executor) {
    env[EXECUTOR] = config.executor;
  }

  if (config.logLevel) {
    env[LOG_LEVEL] = config.logLevel;
  }

  if (config.logPretty) {
    env[LOG_PRETTY] = config.logPretty;
  }

  if (config.circuitSignerKey) {
    env[CIRCUIT_SIGNER_KEY] = config.circuitSignerKey;
  }

  if (config.circuitWsEndpoint) {
    env[CIRCUIT_WS_ENDPOINT] = config.circuitWsEndpoint;
  }

  if (config.ethereumPrivateKey) {
    env[ETHEREUM_PRIVATE_KEY] = config.ethereumPrivateKey;
  }

  if (config.relayChainSignerKey) {
    env[RELAYCHAIN_SIGNER_KEY] = config.relayChainSignerKey;
  }

  if (config.processBatches) {
    env[PROCESS_BATCHES] = config.processBatches;
  }

  return env;
};
