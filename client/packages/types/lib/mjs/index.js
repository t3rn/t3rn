import './augment/types-lookup';
import './augment/registry';
import './augment/augment-api';
import { TypeRegistry, } from '@polkadot/types';
import defs from './augment/lookup';
export const registry = new TypeRegistry();
registry.register(defs);
export function createType(typeName, value) {
    return registry.createType(typeName, value);
}
export function keysOf(typeName) {
    return registry.createType(typeName).defKeys;
}
export async function entriesByIds(apiMethod) {
    const entries = (await apiMethod.entries()).map(([storageKey, value]) => [
        storageKey.args[0],
        value,
    ]);
    return entries.sort((a, b) => a[0].toNumber() - b[0].toNumber());
}
