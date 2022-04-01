import CircuitListener from "./circuit/listener";
import config from "./config.json"

class Executor {
    circuitListener: CircuitListener;

    constructor() {
        this.circuitListener = new CircuitListener();
    }

    async init() {
        await this.circuitListener.setup(config.circuit.rpc)
        await this.circuitListener.start()
        this.circuitListener.on('NewSideEffect', (data) => {
            console.log(data)
        })
    }

    

}


let exec = new Executor();
exec.init()