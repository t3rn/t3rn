import config from "./config"
import types from "./types.json"
import{ ApiPromise, Keyring, WsProvider }from'@polkadot/api';
import { CircuitRelayer } from "./circuit";
import { register } from "./register/register";

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
                    this.circuitRelayer.signAndSend(tx)
                        .then(() => console.log("Registration Success!"))
                        .catch(err => {
                            console.log(err)
                            console.log("Registration Failed!")
                        })
                } else {
                    console.log(`Config for ${process.argv[3]} not found!`)
                }

            }

        }
    }
}

(async () => {
    let cli = new CircuitCLI();
    await cli.setup()
    await cli.run()
})()