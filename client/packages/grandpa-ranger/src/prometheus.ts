import client from "prom-client"
import { logger } from "./logging"
import http from "http"

export class Prometheus {
  circuitActive: boolean
  targetActive: boolean
  disconnects: any
  register: any
  height: any
  submissions: any
  rangeInterval: any
  txSize: any
  target: string
  heightDiff: number = 0

  constructor(target: string) {
    this.target = target
    const Registry = client.Registry
    this.register = new Registry()
    this.createMetrics()
  }

  createMetrics() {
    const collectDefaultMetrics = client.collectDefaultMetrics
    collectDefaultMetrics({ register: this.register })
    this.height = new client.Gauge({
      name: "height",
      help: "The current header height",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.submissions = new client.Counter({
      name: "submissions_total",
      help: "Number of successful submissions",
      registers: [this.register],
      labelNames: ["target", "status"],
    })

    this.disconnects = new client.Counter({
      name: "disconnects_total",
      help: "Information on disconnections",
      registers: [this.register],
      labelNames: ["endpoint", "target"],
    })

    this.rangeInterval = new client.Counter({
      name: "range_interval",
      help: "The number of seconds between each range submission",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.txSize = new client.Gauge({
      name: "tx_size",
      help: "Size of the tx",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.startServer()
  }

  startServer() {
    const server = http.createServer(async (req, res) => {
      try {
        if (req.url === "/metrics") {
          res.setHeader("Content-Type", this.register.contentType)
          const metrics = await this.register.metrics()
          res.end(metrics)
        } else if (req.url === "/status") {
          res.setHeader("Content-Type", "text/plain")
          res.statusCode = this.circuitActive && this.targetActive ? 200 : (this.heightDiff > 250 ? 500 : res.statusCode);
          res.end(
            JSON.stringify({
              circuitActive: this.circuitActive,
              targetActive: this.targetActive,
            })
          )
        } else {
          res.statusCode = 404
          res.end("Not found.")
        }
      } catch (error) {
        res.statusCode = 500
        res.end(error.toString())
      }
    })

    const port = 8080
    server.listen(port, () => {
      logger.info(`Metrics server listening on port ${port}`)
    })
  }
}
