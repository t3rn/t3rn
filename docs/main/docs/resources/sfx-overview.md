---
sidebar_position: 5
---

# What is a Side Effect

A Side Effect is the description of a transaction that needs to be executed on an external consensus system. When initialised, the creator of the Side Effect (can be a user or smart contract) sets all the neccessary attribtues and commits it to the Circuit, where Executors are able to lacate them.

## Side Effect structure

```
pub struct SideEffect<AccountId, BalanceOf> {
    pub target: TargetId,
    pub max_reward: BalanceOf,
    pub insurance: BalanceOf,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executor: Option<AccountId>,
}
```

#### target:

`target` describes the destination consensus system the Side Effect should be executed on.

#### max_reward

`max_reward` sets the reward for the Executor in `TRN`.

#### insurance

`insurance` sets the minimum amount of insurance and Executor has to Bond.

#### encoded_action

`encoded_action` sets the action as an `id` that should be called on smart contract.

#### encoded_args

`encoded_args` sets the arguments that should be passed in `function_y`, when calling a function on a `contract_x` e.g.: contract_x.function_y(`args`).

#### signature

`signature` attribute holds the signature of the creator of the Side Effect.

#### enforce_executor

`enforce_executor` set the executor that should execute the Side Effect.
