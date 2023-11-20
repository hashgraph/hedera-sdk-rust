# Hedera™ Rust SDK

> The SDK for interacting with Hedera Hashgraph: the official distributed
> consensus platform built using the hashgraph consensus algorithm for fast,
> fair and secure transactions. Hedera enables and empowers developers to
> build an entirely new class of decentralized applications.

<sub>Maintained with ❤️ by <a href="https://launchbadge.com" target="_blank">LaunchBadge</a>, <a href="https://www.swirlds.com/" target="_blank">Swirlds Labs</a>, and the Hedera community</sub>

## SDK IS NOT READY FOR PRODUCTION USE. IT IS CURRENTLY STILL UNDER DEVELOPEMENT.

## Requirements

- [Rust](https://rustup.rs)
- [protoc](https://grpc.io/docs/protoc-installation)

Execute command in root directory:
```bash
cargo check
```

## API Docs

Check out the Hedera Rust SDK API reference docs [here](http://docs.rs/hedera/latest/hedera/index.html).

## Community and Support

If you have any questions on the Hedera SDK or Hedera more generally,
you can join our team and hundreds of other developers using Hedera in our 
community Discord:

<a href="https://hedera.com/discord" target="_blank">
  <img alt="" src="https://user-images.githubusercontent.com/753919/167244200-b95cd3a6-6256-4eaf-b9b4-f1f192341485.png" height="60">
</a>

###  Integration Tests

Before running the integration tests, an operator key, operator account id, and a network name must be set in an `.env` file. 
```bash
# Account that will pay query and transaction fees
TEST_OPERATOR_ACCOUNT_ID=
# Default private key to use to sign for all transactions and queries
TEST_OPERATOR_KEY=
# Network names: `"localhost"`, `"testnet"`, `"previewnet"`, `"mainnet"`
TEST_NETWORK_NAME=
```
```bash
# Run tests
$  cargo test 
```

#### Local Environment Testing
Hedera offers a way to run tests through your localhost using the `hedera-local-node` service. 

For instructions on how to set up and run local node, follow the steps in the git repository:
https://github.com/hashgraph/hedera-local-node

Once the local node is running in Docker, the appropriate `.env` values must be set:
```bash
TEST_OPERATOR_ACCOUNT_ID=0.0.2
TEST_OPERATOR_KEY=3030020100300706052b8104000a042204205bc004059ffa2943965d306f2c44d266255318b3775bacfec42a77ca83e998f2
TEST_NETWORK_NAME=localhost
```

Lastly, run the tests using `cargo test`

## Contributing

Contributions are welcome. Please see the
[contributing guide](https://github.com/hashgraph/.github/blob/main/CONTRIBUTING.md)
to see how you can get involved.

## Code of Conduct

This project is governed by the
[Contributor Covenant Code of Conduct](https://github.com/hashgraph/.github/blob/main/CODE_OF_CONDUCT.md). By
participating, you are expected to uphold this code of conduct. Please report unacceptable behavior
to [oss@hedera.com](mailto:oss@hedera.com).

## License

[Apache License 2.0](LICENSE)
