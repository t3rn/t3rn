import config from "./config"
import types from "./types.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { CircuitRelayer } from "./circuit";
import { register } from "./register/register";
import { setOperational } from "./operational";

class CircuitCLI {
    circuit: ApiPromise;
    circuitRelayer: CircuitRelayer;
    signer: any;

    async setup() {
        this.circuit = await ApiPromise.create({
            provider: new WsProvider("ws://127.0.0.1:9944"),
            types: types as any
        })
        this.circuitRelayer = new CircuitRelayer((this.circuit))
        const keyring = new Keyring({ type: "sr25519" })
        this.signer =
            process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)
    }

    async close() {
        return this.circuit.disconnect()
    }

    async run() {
        await this.setup();
        const args = process.argv[2]
        switch(args) {
            case "register": {
                const data = config.gateways.find(elem => elem.id === process.argv[3])
                if(data) {
                    const tx = await register(this.circuit, data)
                    this.circuitRelayer.sudoSignAndSend(tx)
                        .then(() => console.log("Registered and Activated!"))
                        .catch(err => {
                            console.log(err)
                            console.log("Registration Failed!")
                        })
                } else {
                    console.log(`Config for ${process.argv[3]} not found!`)
                }
                break;
            }
            case "setOperational": {
                const data = config.gateways.find(elem => elem.id === process.argv[3])
                const hasArgument = !!process.argv[4];
                if (data && hasArgument) {
                    const argument = JSON.parse(process.argv[4]);
                    const tx = await setOperational(this.circuit, data, argument)
                    this.circuitRelayer.sudoSignAndSend(tx)
                        .then(() => console.log("setOperational Completed!"))
                        .catch(err => {
                            console.log(err)
                            console.log("setOperational Failed!")
                        })
                } else {
                    console.log(`Config or argument for ${process.argv[3]} not found!`)
                }
                break
            }
        }
    }
}

(async () => {
    let cli = new CircuitCLI();
    await cli.setup()
    await cli.run()
})()