import { definition } from '@snowfork/snowbridge-types'
let types = {}
if (definition.types && definition.types.length > 0) {
  types = definition.types[0].types
}
export default {types}