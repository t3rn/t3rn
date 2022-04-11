import '@t3rn/types';
import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api';
import { SideEffect } from '../utils/types';

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
                    let sideEffect = new SideEffect()
                    for (let index = 0; index < event.data.length; index++) {
                        switch (types[index].type) {
                            case 'AccountId32':
                                sideEffect.setRequester(event.data[index]);
                                break;
                            case 'H256':
                                sideEffect.setXtxId(event.data[index]);
                                break;
                            case 'Vec<T3rnPrimitivesSideEffect>':
                                sideEffect.setSideEffect(event.data[index][0]);
                                break;
                        }
                    }

                    this.emit(
                        'NewSideEffect',
                        sideEffect
                    )
                } else if (notification.event.method === 'NewHeaderRangeAvailable') {
                    // unimplemented

                    const data = {
                        gatewayId: "abcd",
                        height: 123455,
                    }

                    this.emit(
                        'NewHeaderRangeAvailable',
                        data
                    )
                }
            })
        })
    }
}