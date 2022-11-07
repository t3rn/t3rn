## t3rn CLI
A simple CLI tool for interacting with the t3rn circuit.

### Setup:
Before interacting with the t3rn circuit, we need to configure a couple of things. The is done in `config/setup.ts`. 

#### Circuit Section:
Here we simply specify the circuit WS endpoint we want to interact with
```
circuit: {
    rpc: "ws://127.0.0.1:9944",
},
```

#### Gateway Section:
Here we specify the different parameters that describe a gateway. This is important for registrations, but also for handling decimal points, address formats etc. in other transactions

The `transferData` section must also be noted. Here we can specify default values for the transfer command. Currently, fee and receiver can be specified.


### Supported Transactions:
A list of the supported transactions.

#### Register Gateway:
Currently, the registration for relaychains is only supported. The registration can be executed by running: `ts-node index.ts register roco`. Note that `roco` matches the gatewayId as specified in the configs.

#### setOperational:
to use a newly registered gateway, it must be set operations. This can be done with: `ts-node index.ts setOperational roco true`

#### Transfer:
A simple transfer, currently only supporting nativ substrate assets. 

`ts-node index.ts transfer roco 0.1 receiver (optinal) fee (optional)`

If the optional parameters are omitted, the default values specified in the config will be used. 
