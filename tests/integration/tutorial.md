# How to write tests using the Parachains Integration Test tool

Since the arrival of XCMP-Lite, communication between different consensus systems became a reality in the Polkadot ecosystem.

[Parachains Integration Tests](https://github.com/paritytech/parachains-integration-tests) is a tool that eases testing interactions between Substrate based blockchains.
It can work alongside with Zombienet and Polkadot Launch, or against the testnet of your choice.

## What you need

Get the new feature from the branch [`feature/test-harness`](https://github.com/t3rn/t3rn/tree/feature/test-harness) or from [`development`](https://github.com/t3rn/t3rn) when merged.

## How to use it

Tests are stored in [`t3rn/tests/integration/`](https://github.com/t3rn/t3rn/tree/feature/test-harness/tests/integration).

Inside that folder, there can be several files: YAML files that contains the tests and one TOML configuration file.

### Important file: `config.toml`

The [`config.toml`](https://github.com/t3rn/t3rn/blob/feature/test-harness/tests/integration/tests/config.toml) file tells zombienet which networks/chains to span and with which settings.

It's similar to the `zombienet.toml` file from the folder `t3rn/tests/zombienet/`.

### Also important: `test.yml`

Each test goes inside a YAML file.
Behind the scenes, the YAML files are converted to Mocha tests with Chai assertions.

It that file, you need to include two sections: `settings` and `tests`.

```yml
settings:
    chains:
        ...
    
    variables:
        ...
    
    decodedCalls:
        ...

-------------------------------------------------

tests:
    - name: A simple test
      its:
        - name: Describe this action
          actions:
            ...
```

There are several good [examples](https://github.com/paritytech/parachains-integration-tests/tree/master/examples) on the main repository of the tool.

There's a dummy [test](https://github.com/t3rn/t3rn/blob/feature/test-harness/tests/integration/tests/basic_transfer.yml) in which a simple transfer is tested. This is the file and below there's an explanation:

```yml
settings:
  chains:
    relay_chain: &relay_chain
      wsPort: 9900
  variables:
    chains:
      relay_chain:
        alice_account: &alice_account //Alice
        bob_account: &bob_account "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
        amount: &amount 1

tests:
  - name: Basic transfer
    its:
      - name: Transfer from Alice to Bob
        actions:
          - extrinsics:
              - chain: *relay_chain
                sudo: true
                signer: *alice_account
                pallet: balances
                call: transfer
                args: [{ Id: *bob_account }, *amount]
                events:
                  - name: balances.Withdraw
                  - name: balances.Deposit
                  - name: transactionPayment.TransactionFeePaid
                  - name: system.ExtrinsicSuccess
    after:
      - name: Get Bob's balance after the transfer
        actions:
          - queries:
              balance_sender_after:
                chain: *relay_chain
                pallet: system
                call: account
                args: [ *bob_account ]
                events:
                  - name: balances.Balance
                    attributes:
                      - type: AccountId
                        value: *bob_account
                      - type: Balance
                        value: *amount
```

In the `settings:` section:

1. We define the chains (under `chains:`) this particular test will connect to. In our case, only to a relay chain which is accessible on `localhost` on port `9900`.

2. This section that allows you to define your own variables following the schema that better suits your test's logic. You can define variables under, for example, some chain (in our case, under the `relay_chain`).

In the `tests:` section:

1. Each test must have a name, and can have a `before` and `after` sections, which run things before or after the main part of the test (non-mandatory sections). These sections are similar to the `its` section below.

2. The tests itself is in the `its` section. Describe what's the action of this part of the test under the `- name:` section.

3. Under the `actions:` section, write what the test should do. In our case, we want to create an `extrinsic`, we select the `chain` we are going to act from, if it's `sudo` or not, and who the signer is going to be.

4. After that, we select which pallet to use (`pallet: balances`), with the arguments that are needed (`args: [{ Id: *bob_account }, *amount]`).

5. Finally, we write the events that we expect to receive. It's possible to add further options to the events to check for the values that are received.
