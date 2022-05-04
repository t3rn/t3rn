import '@t3rn/types';
import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api';
import { SideEffect } from '../utils/types';
import { TextDecoder } from 'util';

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


                if (notification.event.method === 'XTransactionReadyForExec') {
                    const { event } = notification;
                    const types = event.typeDef;
                    let sideEffect = new SideEffect()
                    let xtxId 
                    for (let index = 0; index < event.data.length; index++) {
                        switch (types[index].type) {
                            // case 'AccountId32':
                            //     sideEffect.setRequester(event.data[index]);
                            //     break;
                            case 'H256':
                                xtxId = event.data[index];
                                break;
                            // case 'Vec<T3rnPrimitivesSideEffect>':
                            //     sideEffect.setSideEffect(event.data[index][0]);
                                // break;
                        }
                    }

                    this.emit(
                        'XTransactionReadyForExec',
                        xtxId,
                    )
                }

                else if (notification.event.method === 'NewSideEffectsAvailable') {
                    const { event } = notification;
                    const types = event.typeDef;
                    let sideEffect = new SideEffect()
                    let all_side_effects: SideEffect[] = [];

                    for (let index = 0; index < event.data.length; index++) {
                        switch (types[index].type) {
                            case 'AccountId32':
                                sideEffect.setRequester(event.data[index]);
                                break;
                            case 'H256':
                                sideEffect.setXtxId(event.data[index]);
                                break;
                            case 'Vec<T3rnPrimitivesSideEffect>':
                                (event.data[index] as any).forEach(element => {
                                    sideEffect.setSideEffect(element);
                                    all_side_effects.push(
                                      sideEffect
                                    )
                                });
                                // sideEffect.setSideEffect(event.data[index][0]);
                                break;
                            case 'Vec<H256>':
                                (event.data[index] as any).forEach((element, cnt)=> {
                                    all_side_effects[cnt].setId(element);
                                });
                                break;
                        }
                    }

                    this.emit(
                        'NewSideEffect',
                        all_side_effects
                    )
                } else if (notification.event.method === 'NewHeaderRangeAvailable') {

                    const data = {
                        gatewayId: new TextDecoder().decode(notification.event.data[0].toU8a()),
                        height: notification.event.data[1],
                        range: notification.event.data[2]
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