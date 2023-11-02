import { collectDefaultMetrics, Counter, Gauge, Registry } from 'prom-client'
import { logger } from './utils/logger'
import http from 'http'
import { Config } from './config/config'

export class Prometheus {
  private readonly config: Config
  disconnects: Counter
  register: Registry
  height: Gauge
  submissions: Counter
  interval: Gauge
  txSize: Gauge

  constructor(config: Config) {
    this.config = config
    this.register = new Registry()
    this.createMetrics()
  }

  createMetrics() {
    collectDefaultMetrics({ register: this.register })
    this.submissions = new Counter({
      name: 'submissions_total',
      help: 'Number of successful submissions',
      registers: [this.register],
      labelNames: ['target', 'status', 'type'],
    })

    this.disconnects = new Counter({
      name: 'disconnects_total',
      help: 'Information on disconnections',
      registers: [this.register],
      labelNames: ['endpoint', 'target'],
    })

    this.interval = new Gauge({
      name: 'interval',
      help: 'The number of seconds between each submission',
      registers: [this.register],
    })

    this.txSize = new Gauge({
      name: 'tx_size',
      help: 'Size of the tx',
      registers: [this.register],
      labelNames: ['target'],
    })

    this.startServer()
  }

  startServer() {
    const server = http.createServer(async (req, res) => {
      try {
        if (req.url === '/metrics') {
          res.setHeader('Content-Type', this.register.contentType)
          const metrics = await this.register.metrics()
          res.end(metrics)
        } else if (req.url === '/status') {
          res.setHeader('Content-Type', 'text/plain')
          res.statusCode = 200
          res.end(
            JSON.stringify({
              status: 'ok',
            }),
          )
        } else {
          res.statusCode = 404
          res.end('Not found.')
        }
      } catch (error) {
        res.statusCode = 500
        res.end(error.toString())
      }
    })

    const port = this.config.prometheus.port
    server.listen(port, () => {
      logger.info(`Metrics server listening on port ${port}`)
    })
  }
}
