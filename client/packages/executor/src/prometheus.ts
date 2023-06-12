import http from "http";
import { Registry, Counter, collectDefaultMetrics } from "prom-client";

export class Prometheus {
  register: Registry;

  // State
  isActive: boolean;

  // Metrics
  bids: Counter;
  events: Counter;
  circuitDisconnects: Counter;
  circuitDisconnected: Counter;
  noBidAndNoCompetition: Counter;
  noBidButCompetition: Counter;
  beenOutBid: Counter;

  constructor(public readonly executorName: string) {
    this.register = new Registry();
    this.createMetrics();
  }

  createMetrics() {
    collectDefaultMetrics({ register: this.register });

    this.bids = new Counter({
      name: "bids",
      help: "Number of bids",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.events = new Counter({
      name: "events",
      help: "Number of events",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.circuitDisconnected = new Counter({
      name: "circuit_disconnect",
      help: "Information on circuit disconnections",
      registers: [this.register],
      labelNames: ["endpoint", "executor"],
    });

    this.circuitDisconnects = new Counter({
      name: "circuit_disconnects_total",
      help: "Number of times circuit rpc server has disconnected",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.noBidAndNoCompetition = new Counter({
      name: "no_bid_and_no_competition",
      help: "Number of times no bid and no competition",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.noBidButCompetition = new Counter({
      name: "no_bid_but_competition",
      help: "Number of times no bid but competition",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.beenOutBid = new Counter({
      name: "been_out_bid",
      help: "Number of times been out bid",
      registers: [this.register],
      labelNames: ["executor"],
    });

    this.startServer();
  }

  startServer() {
    const port = 8080;
    const server = http.createServer(async (req, res) => {
      try {
        if (req.url === "/metrics") {
          const metrics = await this.register.metrics();
          res.setHeader("Content-Type", this.register.contentType);
          res.end(metrics);
        } else if (req.url === "/status") {
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
