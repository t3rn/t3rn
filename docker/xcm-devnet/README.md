# ⚡*CM* devnet

## Run

```nofmt
mkdir -p ./data/{alice,bob,charlie,acala,t3rn,pchain}
docker-compose up
```

Spins up a rococo local devnet consisting of 3 relay chain validators and 1 collator for each parachain.

> Parachains must be registered (HRMP channels initialzed) as illustrated [in this Zenlink README](https://github.com/zenlinkpro/Zenlink-DEX-Module#register-parachain--establish-hrmp-channel).

<table>
  <tr>
    <td><b>Network</b></td>
    <td><b>Authority</b></td>
    <td colspan="3"><b>Relaychain Ports</b></td>
    <td colspan="3"><b>Parachain Ports</b></td>
    <td><b>Parachain Id</b></td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>alice</td>
    <td>10001</td>
    <td>8844</td>
    <td>9944</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>bob</td>
    <td>10002</td>
    <td>8845</td>
    <td>9945</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>charlie</td>
    <td>10003</td>
    <td>8846</td>
    <td>9946</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>acala</td>
    <td>-</td>
    <td>22221</td>
    <td>8821</td>
    <td>9921</td>
    <td>22222</td>
    <td>8822</td>
    <td>9922</td>
    <td>2000</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>-</td>
    <td>33332</td>
    <td>8832</td>
    <td>9932</td>
    <td>33333</td>
    <td>8833</td>
    <td>9933</td>
    <td>3000</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>-</td>
    <td>44444</td>
    <td>4488</td>
    <td>4499</td>
    <td>44443</td>
    <td>4487</td>
    <td>4498</td>
    <td>4000</td>
  </tr>
</table>

*The "pchain" is a plain [Substrate parachain instance](https://github.com/substrate-developer-hub/substrate-parachain-template)*. All code uses `polkadot-v0.9.13` Substrate.

## Specs

To *regenerate* chain specs and artifacts simply run `./build-specs.sh`.