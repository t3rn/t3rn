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
  bids: Counter;
  events: Counter;
  circuitDisconnects: Counter;
  noBidAndNoCompetition: Counter;
  noBidButCompetition: Counter;
  beenOutBid: Counter;
  attestationsBatchesPending: Gauge;
  attestationsEvents: Counter
  attestationVerifierCurrentCommitteeSize: Gauge;
  attestationVerifierCurrentBatchIndex: Gauge;
  attestationVerifierCurrentCommitteeTransitionCount: Gauge;
  attestatonsBatchesProcessed: Counter;
  attestatonsBatchesFailed: Counter;

  constructor() {
    const Registry = client.Registry;
    this.register = new Registry();
    this.createMetrics();
  }

  createMetrics() {
    // Collects default metrics
    collectDefaultMetrics({ register: this.register });
    this.bids = new Counter({
      name: "bids",
      help: "Number of bids",
      registers: [this.register],
    });

    this.events = new Counter({
      name: "events",
      help: "Number of events",
      registers: [this.register],
    });

    this.circuitDisconnects = new Counter({
      name: "circuit_disconnects_total",
      help: "Number of times circuit rpc server has disconnected",
      labelNames: ["endpoint"],
      registers: [this.register],
    });

    this.noBidAndNoCompetition = new Counter({
      name: "no_bid_and_no_competition",
      help: "Number of times no bid and no competition",
      registers: [this.register],
    });

    this.noBidButCompetition = new Counter({
      name: "no_bid_but_competition",
      help: "Number of times no bid but competition",
      registers: [this.register],
    });

    this.beenOutBid = new Counter({
      name: "been_out_bid",
      help: "Number of times been out bid",
      registers: [this.register],
    });

    this.attestationsBatchesPending = new Gauge({
      name: "attestations_batches_pending_count",
      help: "Number of attestations batches pending",
      registers: [this.register],
    });

    this.attestationsEvents = new client.Counter({
        name: 'attestation_events_total',
        help: 'Number of attestations received',
        registers: [this.register],
        labelNames: ['method'],
    })

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

    this.attestatonsBatchesProcessed = new Counter({
      name: "attestations_batches_processed_total",
      help: "Number of attestations batches processed",
      registers: [this.register],
    });

    this.attestatonsBatchesFailed = new Counter({
      name: "attestations_batches_failed_total",
      help: "Number of attestations batches failed",
      registers: [this.register],
    });

    this.startServer();
    logger.info("Prometheus metrics server started");
  }

  startServer() {
    const port = process.env.PROMETHEUS_PORT || 8080;
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
