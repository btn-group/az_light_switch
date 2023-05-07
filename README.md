<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/btn-group">
    <img src="images/logo.png" alt="Logo" height="80">
  </a>

  <h3 align="center">Aleph Zero Light Switch Smart Contract by btn.group</h3>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#setting-up-locally">Setting up locally</a></li>
      </ul>
    </li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This is an Aleph Zero smart contract that we created to help us understand and experiment with ink and paintbrush. It's a contract that lets users turn a light switch on and off.

1. Initialise with a on_fee, off_payment, minimum amount of seconds before a user can turn off the light.
2. Allow user to turn on light for a fee.
3. Allow user to turn off light and receive a payment. A certain amount of seconds must have passed before user can turn off light.
4. Enable admin to change the admin, on_fee, off_payment and minimum_on_time_in_seconds.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

* [Cargo](https://doc.rust-lang.org/cargo/)
* [Rust](https://www.rust-lang.org/)
* [ink!](https://use.ink/)
* [OpenBrush](https://openbrush.io/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple example steps.

* Open brush required that I use rust nightly but building the contract required stable.

### Prerequisites

* A pre-requisite for compiling smart contracts is to have a stable Rust version and Cargo installed. Here's an [installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).
* The first tool we will be installing is [cargo-contract](https://github.com/paritytech/cargo-contract), a CLI tool for helping setting up and managing WebAssembly smart contracts written with ink!.
* Setup developer blockchain and Docker: https://docs.scrt.network/dev/developing-secret-contracts.html#personal-secret-network-for-secret-contract-development

### Building contract

By default, cargo-contract builds the contract in debug mode. This means that the contract will e.g. print statements like

```sh
ink::env::debug_println!("magic number: {}", value);
```
to the node's console if debugging was enabled on the node ([instructions here](https://use.ink/faq#how-do-i-print-something-to-the-console-from-the-runtime)). To support functionality like this the debug build of a contract includes some heavy-weight logic.

For contracts that are supposed to run in production you should always build the contract with --release:
```sh
cargo +stable contract build --release
```
This will ensure that nothing unnecessary is compiled into the Wasm blob, making your contract faster and cheaper to deploy and execute.

### Setting up locally

The [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) is a simple Substrate blockchain which is configured to include the Substrate module for smart contract functionality – the contracts pallet (see [How it Works](https://use.ink/how-it-works) for more). It's a comfortable option if you want to get a quickstart. Download the binary [here](https://github.com/paritytech/substrate-contracts-node/releases).

[After successfully installing substrate-contracts-node](https://use.ink/getting-started/setup#installing-the-substrate-smart-contracts-node), you can start a local development chain by running:

```sh
substrate-contracts-node
```

You can interact with your node using the [Contracts UI](https://contracts-ui.substrate.io/). Once you have the webpage open, click on the dropdown selector at the top left corner and choose "Local Node".

Note that blocks are only created when you execute a function in substrate-contracts-node, so trigger a another function first if a function depends on a time delay.

## References

1. https://realtakahashi-work.medium.com/substrate-ink-how-to-define-own-struct-%E7%8B%AC%E8%87%AA%E6%A7%8B%E9%80%A0%E4%BD%93%E3%81%AE%E5%AE%A3%E8%A8%80%E6%96%B9%E6%B3%95-8f4893089ba4
2. https://github.com/paritytech/ink-examples/blob/main/contract-transfer/lib.rs
3. https://github.com/paritytech/ink
4. https://learn.brushfam.io/docs/OpenBrush/smart-contracts/ownable
5. https://github.com/Brushfam/openbrush-contracts/blob/main/lang/src/test_utils.rs#L39
6. https://github.com/Brushfam/openbrush-contracts/blob/main/contracts/src/access/ownable/mod.rs
7. https://github.com/Supercolony-net/openbrush-contracts/blob/96b556536a9806cb2a0db7a6159a412754ceba9b/contracts/src/traits/errors/psp22.rs#L100
8. https://paritytech.github.io/ink/ink_env/test/index.html
9. https://use.ink/basics/contract-testing/
10. https://use.ink/examples/smart-contracts
11. https://use.ink/getting-started/building-your-contract
12. https://substrate.stackexchange.com/questions/1174/why-is-it-a-bad-idea-to-use-string-in-an-ink-smart-contract
