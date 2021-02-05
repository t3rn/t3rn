<h1>Circuit RPC API</h1>
<h3>
RPC methods for interaction with Circuit.
</h3>


### composable_exec([origin, components, io, gasLimit, inputData])

* `origin` {`String`|`Array`} File patterns to be matched
* `components` {`Array<component>`} Array of composable smart contracts to execute via selected gateways
  * `component` single composable smart contract
    * `name` {`String`} Compose name
    * `codeTxt` {`String`} Code in plain text
    * `gatewayId` {`String`} Selected Gateway
    * `execType` {`String`} Type of execution at Gateway
    * `dest` {`String` | `null`} Optional destination at target chain (either contract or recipient)
    * `value` {`Number` | `null`} Optional value to be transferred to recipient
    * `bytes` {`HexString`} Code bytes
    * `inputData` {`HexString`} Input data
* `io` {`String`} Composable execution schedule deciding on execution order and phases
* `gasLimit` {`Number`} Aggregated gas limit for composable execution
* `inputData` {`HexString`} Input data for the first composable contract in the IO schedule

##### Example:
```javascript
{
    "origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
    "components": [{
        "name": "component1",
        "codeTxt": "let a = \"hello\"",
        "gatewayId": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
        "execType": "exec-volatile",
        "dest": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
        "value": 0,
        "bytes": "0x8c97db398c97db398c97db398c97db39",
        "inputData": "0x00"
    }],
    "io": "component1, component2 | component3;",
    "gasLimit": 1000000000000,
    "inputData": "0x8c97db39"
}
```

License: Apache-2.0