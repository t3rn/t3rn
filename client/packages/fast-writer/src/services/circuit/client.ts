import '@t3rn/types'
import { Prometheus } from '../../prometheus'
import { CircuitConnection } from './connection.class'
import { Config } from 'src/config/config'

export class CircuitClient {
  private readonly prometheus: Prometheus
  private readonly circuitConnection: CircuitConnection
  private readonly config
  readonly sdk

  constructor(
    circuitConnection: CircuitConnection,
    prometheus: Prometheus,
    config: Config,
  ) {
    this.circuitConnection = circuitConnection
    this.prometheus = prometheus
    this.sdk = this.circuitConnection.sdk
    this.config = config
  }

}
