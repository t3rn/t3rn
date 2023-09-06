import http from "http";
import client from "prom-client";
import { Registry, Counter, collectDefaultMetrics, Gauge } from "prom-client";
import { logger } from "./logging";

export class Prometheus {
  server: ReturnType<typeof http.createServer>;
  register: Registry;

  // State
  isActive: boolean;

  // Metrics
  events: Counter;
  circuitDisconnects: Counter;
  executorXtxStrategyRejects: Counter;
  executorBid: Counter;
  executorBidRejected: Counter;
  executorBalance: Gauge;
  executorClientBalance: Gauge;
  executorTargetBalance: Gauge;
  executorConfirmationErrors: Counter;
  executorExecutionCompleted: Counter;
  attestationsBatchesPending: Gauge;
  attestationEvents: Counter;
  attestationVerifierCurrentCommitteeSize: Gauge;
  attestationVerifierCurrentBatchIndex: Gauge;
  attestationVerifierCurrentCommitteeTransitionCount: Gauge;
  attestationBatchesProcessed: Counter;
  attestatonBatchesFailed: Counter;
  gatewayHeight: Gauge;

  constructor() {
    const Registry = client.Registry;
    this.register = new Registry();
    this.createMetrics();
  }

  createMetrics() {
    // Collects default metrics
    collectDefaultMetrics({ register: this.register });
    this.events = new Counter({
      name: "events",
      help: "Number of events",
      registers: [this.register],
      labelNames: ["event"],
    });

    this.circuitDisconnects = new Counter({
      name: "circuit_disconnects_total",
      help: "Number of times circuit rpc server has disconnected",
      labelNames: ["endpoint"],
      registers: [this.register],
    });

    this.executorXtxStrategyRejects = new Counter({
      name: "executor_xtx_strategy_rejects_total",
      help: "Number of times executor xtx strategy rejects",
      registers: [this.register],
    });

    this.executorBid = new Counter({
      name: "executor_bid_total",
      help: "Number of times bid happened",
      labelNames: ["scenario"],
      registers: [this.register],
    });

    this.executorBidRejected = new Counter({
      name: "executor_bid_rejected",
      help: "Number of times bid rejected",
      labelNames: ["error"],
      registers: [this.register],
    });

    this.executorBalance = new Gauge({
      name: "executor_balance",
      help: "Executor balance",
      registers: [this.register],
      labelNames: ["signer", "target"],
    });

    this.executorClientBalance = new Gauge({
      name: "executor_client_balance",
      help: "Executor client balance",
      registers: [this.register],
      labelNames: ["signer", "target"],
    });

    this.executorTargetBalance = new Gauge({
      name: "executor_target_balance",
      help: "Executor target balance",
      registers: [this.register],
      labelNames: ["signer", "target"],
    });

    this.executorConfirmationErrors = new Counter({
      name: "executor_confirmation_errors_total",
      help: "Number of times confirmation errors happened",
      registers: [this.register],
    });

    this.executorExecutionCompleted = new Counter({
      name: "executor_execution_completed_total",
      help: "Number of times execution completed",
      registers: [this.register],
      labelNames: ["target"],
    });

    this.attestationsBatchesPending = new Gauge({
      name: "attestations_batches_pending_count",
      help: "Number of attestations batches pending",
      registers: [this.register],
    });

    this.attestationEvents = new client.Counter({
      name: "attestation_events_total",
      help: "Number of attestations received",
      registers: [this.register],
      labelNames: ["method"],
    });

    this.attestationVerifierCurrentCommitteeSize = new Gauge({
      name: "attestation_verifier_current_committee_size",
      help: "Current committee size",
      registers: [this.register],
    });

    this.attestationVerifierCurrentBatchIndex = new Gauge({
      name: "attestation_verifier_current_batch_index",
      help: "Current batch index",
      registers: [this.register],
    });

    this.attestationVerifierCurrentCommitteeTransitionCount = new Gauge({
      name: "attestation_verifier_current_committee_transition_count",
      help: "Current committee transition count",
      registers: [this.register],
    });

    this.attestationBatchesProcessed = new Counter({
      name: "attestations_batches_processed_total",
      help: "Number of attestations batches processed",
      registers: [this.register],
    });

    this.attestatonBatchesFailed = new Counter({
      name: "attestations_batches_failed_total",
      help: "Number of attestations batches failed",
      registers: [this.register],
      labelNames: ["error"],
    });

    this.gatewayHeight = new Gauge({
      name: "gateway_height",
      help: "Gateway height",
      registers: [this.register],
      labelNames: ["vendor"],
    });

    this.startServer();
    logger.info("Prometheus metrics server started");
  }

  startServer() {
    const port = process.env.PROMETHEUS_PORT || 9333;
    this.server = http.createServer(async (req, res) => {
      try {
        if (req.url === "/metrics") {
          const metrics = await this.register.metrics();
          res.setHeader("Content-Type", this.register.contentType);
          res.end(metrics);
        } else if (req.url === "/healthz") {
          res.setHeader("Content-Type", "text/plain");
          res.statusCode = this.isActive ? 200 : 500;
          res.end(JSON.stringify({ isExecutorActive: this.isActive }));
        } else {
          res.statusCode = 404;
          res.end("Not found.");
        }
      } catch (error) {
        res.statusCode = 500;
        res.end(error.toString());
      }
    });

    this.server.listen(port, () => {
      logger.info(`Metrics server listening on port ${port}`);
    });
  }

  stopServer() {
    this.server.close();
  }
}
