import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api';
import types from "../types.json";

export default class CircuitListener extends EventEmitter {

    api: ApiPromise;

    async setup(rpc: string) {
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc),
            types: types as any,
        })
    }

    async start() {
        this.api.query.system.events((notifications) => {
            notifications.forEach(notification => {
                if (notification.event.method === 'NewSideEffectsAvailable') {
                    this.emit(
                        'NewSideEffect',
                        notification.event.data
                    )
                }
            })
        })
    }
}



