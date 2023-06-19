import http from "http";
import { Registry, Counter, collectDefaultMetrics } from "prom-client";

export class Prometheus {
  register: Registry;

  // State
  isActive: boolean;

  // Metrics
  // Number of bids
  bids: Counter;

  // Number of events
  events: Counter;

  // Number of times circuit rpc server has disconnected
  circuitDisconnects: Counter;

  // Number of times no bid and no competition
  noBidAndNoCompetition: Counter;

  // Number of times no bid but competition
  noBidButCompetition: Counter;

  // Number of times been out bid
  beenOutBid: Counter;

  constructor() {
    this.register = new Registry();
    this.createMetrics();
  }

  // Creates all the metrics for Prometheus
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

    this.startServer();
  }

  startServer() {
    const port = process.env.PROMETHEUS_PORT || 4001;
    const server = http.createServer(async (req, res) => {
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

    server.listen(port, () => {
      console.log(`Metrics server listening on port ${port}`);
    });
  }
}
