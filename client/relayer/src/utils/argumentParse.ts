import { Keyring } from '@polkadot/api';
import type { Vec, Bytes } from '@polkadot/types';
import { TransferArguments } from './types';

export function parseTransferArguments(params: Vec<Bytes>) : TransferArguments {
    // do some magic here and assing the correct values.
    // TODO : Do this later.

    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    let parsed = <TransferArguments>{};
    parsed.amount = 100000;
    parsed.to = bob.address;
    parsed.from = alice.address;
    return parsed;
}