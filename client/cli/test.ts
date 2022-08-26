import "@t3rn/types"
import{ ApiPromise, WsProvider }from '@polkadot/api';
import { H256 } from '@polkadot/types/interfaces';
import { Execution } from "./execution"

(async () => {
    const circuit = await ApiPromise.create({
        provider: new WsProvider("ws://127.0.0.1:9944")
    })


    circuit.query.system.events((notifications: any) => {
        notifications.forEach((notification: any) => {
            if (notification.event.method === "XTransactionReadyForExec") {
                const {event} = notification
                const types = event.typeDef
                let xtxId: H256;
                for (let index = 0; index < event.data.length; index++) {
                    switch (types[index].type) {
                        case "H256":
                            xtxId = event.data[index]
                            break
                    }
                }
            } else if (notification.event.method === "NewSideEffectsAvailable") {
                const {event} = notification
                const types = event.typeDef
                console.log("event:", event.data.toHuman())
                console.log("types:", types)
                const execution = new Execution(event.data)
                // let all_side_effects: SideEffect[] = []
                // let sideEffect = new SideEffect()
            }
        })
    })
})()