require('dotenv').config()
import CircuitListener from "./circuit/listener";
import SubstrateRelayer from "./gateways/substrate/relayer";
import config from "./config.json"

class Executor {
    circuitListener: CircuitListener;
    gatewayInstances: any;

    constructor() {
        this.circuitListener = new CircuitListener();
    }

    async setup() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitListener.start()
        await this.initializeGateways()
    }

    async initializeGateways() {
        let gatewayInstances = {};
        for(let i = 0; i < config.gateways.length; i++) {
            const entry = config.gateways[i]
            if(entry.type === "substrate") {
                let instance = new SubstrateRelayer();
                await instance.setup(entry.rpc)
                gatewayInstances[entry.id] = instance;
            }
        }
        console.log(gatewayInstances)
        this.gatewayInstances = gatewayInstances;
    }

    async start() {
        this.circuitListener.on('NewSideEffect', (data) => {
            console.log(data)
        })
    }

    // async sideEffectRouter(eventData: any) {

    // }

    

}


let exec = new Executor();
exec.setup()