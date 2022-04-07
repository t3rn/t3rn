import '@t3rn/types';
import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api';
import { SideEffectStateManager } from '../utils/types';

export default class CircuitListener extends EventEmitter {

    api: ApiPromise;

    async setup(rpc: string) {
        this.api = await ApiPromise.create({
            provider: new WsProvider(rpc)
        })
    }

    async start() {
        this.api.query.system.events((notifications) => {
            notifications.forEach(notification => {
                if (notification.event.method === 'NewSideEffectsAvailable') {
                    const { event } = notification;
                    const types = event.typeDef;
                    let sideEffectStateManager = new SideEffectStateManager()
                    for (let index = 0; index < event.data.length; index++) {
                        switch (types[index].type) {
                            case 'AccountId32':
                                sideEffectStateManager.setRequester(event.data[index]);
                                break;
                            case 'H256':
                                sideEffectStateManager.setXtxId(event.data[index]);
                                break;
                            case 'Vec<T3rnPrimitivesSideEffect>':
                                sideEffectStateManager.setSideEffect(event.data[index][0]);
                                break;
                        }
                    }

                    this.emit(
                        'NewSideEffect',
                        sideEffectStateManager
                    )
                }
            })
        })
    }
}