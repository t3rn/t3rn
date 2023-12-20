---
sidebar_position: 2
---

# Brain Explanation

<p align="center">
    <img height="150" src="/img/t3rn-Carebral-Architecture.png?raw=true"/>
</p>

The Left Hemisphere focuses on the blockchain infrastructure of t3rn, with various networks serving testing, development, and mainnet functionalities. In contrast, the Right Hemisphere leverages smart contract technology to facilitate cross-chain operations and asset swaps within the Ethereum ecosystem and selected Layer 2 solutions. Together, these hemispheres represent a comprehensive approach to achieving interoperability and functionality in the t3rn ecosystem.

## Left Hemisphere: Standalone Networks and Parachains

The Left Hemisphere of t3rn comprises various blockchain networks, each serving distinct purposes within the ecosystem:

1. **t0rn - Testnet Parachain on Rococo**
   - **Nature:** A parachain deployed on Rococo, Polkadot's testnet relay chain, with 12-second block times.
   - **Features:**
     - Active GRANDPA (Polkadot, Rococo, Kusama) and Ethereum Light Clients (Sepolia + Ethereum).
     - Development network attesters set up.
     - XCM (Cross-Consensus Messaging) support for asset bridging between Rococo's AssetHub and t0rn.
     - Auto-asset minting into ERC-20 tokens on t0rn.
     - Compatibility with EVM and Ethereum RPC, enabling asset swaps on the Right Hemisphere.
     - Cross-parachain/relay chain execution against GRANDPA light clients.
     - Beta feature for bridging 1:1 t0rn token with Ethereum and L2 assets.
   - **Explorer Link:** [t0rn Explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.t0rn.io#/explorer)
2. **t1rn - Kusama Parachain**
   - **Nature:** A parachain deployed on Kusama's Relay Chain with 12-second block times, scheduled to go live on 30th December 2023.
   - **Explorer Link:** [t1rn Explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.t1rn.io#/explorer)
3. **t2rn - Standalone Network**
   - **Nature:** A combination of a relay chain and parachain, designed for quick 1-second block times, making it ideal for testing.
   - **Features:**
     - Absence of active Light Clients.
     - Development network (Dev-net) attesters are set up.
     - Attester route connectivity across all t2rn for Substrate and Ethereum, including Layer 2 (L2) assets.
     - Beta feature for bridging 1:1 t2rn token with Ethereum and L2 assets.
   - **Explorer Link:** [t2rn Explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.t2rn.io#/explorer)
4. **t3rn - Mainnet Release**

   - **Status:** The t3rn Circuit is not yet deployed here, but the parachain slot is active and secured. Full deployment will include the Circuit, airdrop initiatives, attesters proof of stake bootstrap, inflation, and XCM connectivity with Polkadot and its Asset Hub.
   - **Explorer Link:** [t3rn Mainnet Explorer](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fws.t3rn.io#/explorer)

   ## Right Hemisphere: Smart Contracts on Ethereum and Layer 2s

The Right Hemisphere involves the deployment of t3rn's protocol as a set of smart contracts on Ethereum and selected Layer 2 solutions, facilitating cross-chain orders:

1. **Functionality:**
   - Enables two-way cross-chain orders, including same-chain (local) swaps and cross-chain swaps of native and any ERC-20 assets on the destination network.
   - Beta features for bridging DOT and ETH (selected assets only) and supporting arbitrary calls (in beta, not yet fully supported).
2. **Cross-Chain Swaps with Attested Commitments:**
   - Involves a set of three co-dependent smart contracts: Remote Order, EscrowGMP, and Attestations Verifier.
   - Requires cooperation among three types of network actors: order makers (writers), order executors, and attesters.
   - Details the process of order creation, execution strategy, bidding, and execution phases, along with gas-fee breakdown for various operations.
3. **Smart Contract Deployment Locations:**
   - Provides detailed information on the deployment of these contracts across various testnets and Layer 2 networks, including contract addresses, tokens, and RPC endpoints.
