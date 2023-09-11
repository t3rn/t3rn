import client from "prom-client"
import { logger } from "./logging"
import http from "http"

export class Prometheus {
  circuitActive: boolean
  targetActive: boolean
  targetDisconnectsTotal: any
  circuitDisconnectsTotal: any
  register: any
  circuitHeight: any
  targetHeight: any
  nextSubmission: any
  successesTotal: any
  errorsTotal: any
  circuitDisconnected: any
  targetDisconnected: any
  rangeInterval: any
  up: any
  target: string

  constructor(target: string) {
    this.target = target
    const Registry = client.Registry
    this.register = new Registry()
    this.createMetrics()
  }

  createMetrics() {
    const collectDefaultMetrics = client.collectDefaultMetrics
    collectDefaultMetrics({ register: this.register })
    this.circuitHeight = new client.Gauge({
      name: "circuit_height",
      help: "The header height stored on circuit",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.targetHeight = new client.Gauge({
      name: "target_height",
      help: "The current header height on the target",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.nextSubmission = new client.Gauge({
      name: "next_submission",
      help: "Unix timestamp of the next scheduled submission",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.successesTotal = new client.Counter({
      name: "successes_total",
      help: "Number of successful submissions",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.errorsTotal = new client.Counter({
      name: "errors_total",
      help: "Number of errored submissions",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.circuitDisconnected = new client.Counter({
      name: "circuit_disconnect",
      help: "Information on circuit disconnections",
      registers: [this.register],
      labelNames: ["endpoint", "target"],
    })

    this.circuitDisconnectsTotal = new client.Counter({
      name: "circuit_disconnects_total",
      help: "Number of times circuit rpc server has disconnected",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.targetDisconnected = new client.Counter({
      name: "target_disconnect",
      help: "Information on target disconnections",
      registers: [this.register],
      labelNames: ["endpoint", "target"],
    })

    this.targetDisconnectsTotal = new client.Counter({
      name: "target_disconnects_total",
      help: "Number of times target rpc server has disconnected",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.rangeInterval = new client.Counter({
      name: "range_interval",
      help: "The number of seconds between each range submission",
      registers: [this.register],
      labelNames: ["target"],
    })

    this.up = new client.Counter({
      name: "server_up",
      help: "If the server initialized successfully",
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
          res.statusCode = this.circuitActive && this.targetActive ? 200 : 500
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
      this.up.inc({ target: this.target }, 1)
    })
  }
}
