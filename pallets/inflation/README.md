# pallet-inflation

This pallet aims to enable staking and rewards allocation to the participants of the t3rn ecosystem.
It is based upon [pallet-balances](https://docs.rs/pallet-balances/latest/pallet_balances/) and 
[pallet-staking](https://docs.rs/pallet-staking/3.0.0/pallet_staking/).

## Features

`pallet-inflation` features are in Level 1 (Alpha). This includes :

- Static implementations for `RewardCurve`, `RewardHandler`, `Era`, `SessionsPerEra`
- A first draft implementation for the Inflation trait.
- The same reward mechanism for all network participants.

## TODO
- Unlock rewards on every epoch, on the AccountIds belonging to authorities.
- Reward executors for completing `Xtx`s.
- Reward developers for successful contract executions.

## Storage

Includes a `InflationConfig` storage value with the inflation configuration.
