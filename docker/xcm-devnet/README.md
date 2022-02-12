# XCM devnet

## Running

```nofmt
mkdir /tmp/alice /tmp/bob /tmp/charlie /tmp/acala /tmp/t3rn
docker-compose up
```

Spins up a rococo devnet consisting of 3 relay chain validators and 1 collator for each parachain with base path bind mounts as created above.

> Parachains must be registered and HRMP channels initialzed as described [in this Zenlink README](https://github.com/zenlinkpro/Zenlink-DEX-Module#register-parachain--establish-hrmp-channel)

<table>
  <tr>
    <td><b>Network</b></td>
    <td><b>Account</b></td>
    <td colspan="3"><b>Relaychain Ports</b></td>
    <td colspan="3"><b>Parachain Ports</b></td>
    <td><b>Parachain Id</b></td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Alice</td>
    <td>10001</td>
    <td>8844</td>
    <td>9944</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Bob</td>
    <td>10002</td>
    <td>8845</td>
    <td>9945</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Charlie</td>
    <td>10003</td>
    <td>8846</td>
    <td>9946</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Acala</td>
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
</table>

## Specs

To *regenerate* chain specs and artifacts simply run `./build-specs.sh`