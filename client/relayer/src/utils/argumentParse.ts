import { ApiPromise, Keyring } from '@polkadot/api';
import type { Vec, Bytes } from '@polkadot/types';
import { TransferArguments, GetStorageArguments } from './types';

export function parseTransferArguments(api: ApiPromise, params: Vec<Bytes>) : TransferArguments {

    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    let parsed = <TransferArguments>{};
    parsed.from = api.createType('AccountId',params[0].toHuman());
    parsed.to = api.createType('AccountId',params[1].toHuman());
    parsed.amount = api.createType('u128',params[2]);
    return parsed;
}

export function parseStorageArguments(api: ApiPromise, params: Vec<Bytes>) : GetStorageArguments {

    let parsed = <GetStorageArguments>{};
    parsed.key = api.createType('StorageKey',params[0].toHuman());
    return parsed;
}