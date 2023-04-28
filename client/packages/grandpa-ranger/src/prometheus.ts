const client = require('prom-client');
const http = require('http')
const url = require('url')
export class Prometheus {
	circuitActive: boolean
	targetActive: boolean

	register: any;
	circuitHeight: any
	targetHeight: any
	nextSubmission: any
	successCount: any
	errorCount: any
	circuitDisconnected: any
	targetDisconnected: any
	rangeBreak: any
	errors: any
	successes: any

	constructor() {
		const Registry = client.Registry;
		this.register = new Registry();
		this.createMetrics()
	}

	createMetrics() {
		const collectDefaultMetrics = client.collectDefaultMetrics;
		collectDefaultMetrics({ register: this.register });
		this.circuitHeight = new client.Gauge(
			{
				name: 'circuit_height',
				help: 'The header height stored on circuit',
				registers: [this.register]
			}
		);

		this.targetHeight = new client.Gauge(
			{
				name: 'target_height',
				help: 'The current header height on the target',
				registers: [this.register]
			}
		);

		this.nextSubmission = new client.Gauge(
			{
				name: 'next_submission',
				help: 'Unix timestamp of the next scheduled submission',
				registers: [this.register]
			}
		)

		this.successes = new client.Counter(
			{
				name: 'successes',
				help: 'Information on the latest successful submissions',
				registers: [this.register],
				labelNames: ['rangeSize', 'timestamp', 'circuitBlock']
			}
		);

		this.successCount = new client.Counter(
			{
				name: 'success_counter',
				help: 'Number of successful submissions',
				registers: [this.register],
			}
		)

		this.errors = new client.Counter(
			{
				name: 'errors',
				help: 'Information on the latest errored submissions',
				registers: [this.register],
				labelNames: ['rangeSize', 'timestamp']
			}
		);

		this.errorCount = new client.Counter(
			{
				name: 'error_counter',
				help: 'Number of errored submissions',
				registers: [this.register],
			}
		)

		this.circuitDisconnected = new client.Counter(
			{
				name: 'circuit_disconnect',
				help: 'Number of times circuit rpc server has disconnected',
				registers: [this.register],
				labelNames: ['endpoint', 'timestamp']
			}
		)

		this.targetDisconnected = new client.Counter(
			{
				name: 'target_disconnect',
				help: 'Number of times target rpc server has disconnected',
				registers: [this.register],
				labelNames: ['endpoint', 'timestamp']
			}
		)

		this.rangeBreak = new client.Counter({
			name: 'range_loop',
			help: 'The number of seconds between each range submission',
			registers: [this.register]
		})

		this.startServer()
	}

	startServer() {
		const server = http.createServer(async (req, res) => {
		  try {
			if (req.url === '/metrics') {
			  res.setHeader('Content-Type', this.register.contentType);
			  const metrics = await this.register.metrics();
			  res.end(metrics);
			} else if (req.url === '/status') {
			  res.setHeader('Content-Type', 'text/plain');
			  res.statusCode = this.circuitActive && this.targetActive ? 200 : 500;
			  res.end(JSON.stringify({circuitActive: this.circuitActive, targetActive: this.targetActive}));
			} else {
			  res.statusCode = 404;
			  res.end('Not found.');
			}
		  } catch (error) {
			res.statusCode = 500;
			res.end(error.toString());
		  }
		});

		const port = 8080;
		server.listen(port, () => {
		  console.log(`Metrics server listening on port ${port}`);
		});
	}

}