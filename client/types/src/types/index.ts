import {types as contracts_registry} from './v1/contracts_registry'
import {types as execution_delivery} from './v1/execution_delivery'
import {types as primitives} from './v1/primitives'
import {types as volatile_vm} from './v1/volatile_vm'
import {types as xdns} from './v1/xdns'
import { definition as snowforkDefinition } from '@snowfork/snowbridge-types'

// hack to support snowfork's versioned types
let snowforkTypes = {}
if (snowforkDefinition.types && snowforkDefinition.types.length > 0) {
  snowforkTypes = snowforkDefinition.types[0].types
}

export const types = {
  ...contracts_registry,
  ...execution_delivery,
  ...primitives,
  ...volatile_vm,
  ...xdns,
  ...snowforkTypes
}