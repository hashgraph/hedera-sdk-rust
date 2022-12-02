# Hedera™ Swift SDK

> The SDK for interacting with Hedera Hashgraph: the official distributed
> consensus platform built using the hashgraph consensus algorithm for fast,
> fair and secure transactions. Hedera enables and empowers developers to
> build an entirely new class of decentralized applications.

<sub>Maintained with ❤️ by <a href="https://launchbadge.com" target="_blank">LaunchBadge</a>, <a href="https://www.swirlds.com/" target="_blank">Swirlds Labs</a>, and the Hedera community</sub>

## Requirements

- Swift v5.5+
- MacOS v10.15+ (2019, Catalina)
- iOS 13+ (2019)

## Install

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/hashgraph/hedera-sdk-swift.git", from: "0.1.0")
]
```

See ["Adding Package Dependencies to Your App"](https://developer.apple.com/documentation/swift_packages/adding_package_dependencies_to_your_app) for help on
adding a swift package to an Xcode project.

## Usage

```swift
import Hedera

// connect to the Hedera network
let client = Client.forTestnet()

// query the balance of an account
let ab = try await AccountBalanceQuery()
    .account_id(AccountId("0.0.1001")!)
    .execute(client)

print("balance = \(ab.balance)")
```

See [examples](./Examples) for more usage.

## Community and Support

If you have any questions on the Hedera SDK or Hedera more generally,
you can join our team and hundreds of other developers using Hedera in our
community Discord:

<a href="https://hedera.com/discord" target="_blank">
  <img alt="" src="https://user-images.githubusercontent.com/753919/167244200-b95cd3a6-6256-4eaf-b9b4-f1f192341485.png" height="60">
</a>

## License

Licensed under Apache License,
Version 2.0 – see [LICENSE](LICENSE)
or [apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

## Development (HederaProtobufs)

HederaProtobufs is entirely generated

### Required tooling
protoc
protoc-gen-swift (from https://github.com/apple/swift-protobuf)
protoc-gen-grpc-swift (from https://github.com/grpc/grpc-swift)

### Generate services
```bash
# cwd: `$REPO/sdk/swift`
protoc --swift_opt=Visibility=Public --swift_out=./Sources/HederaProtobufs/Services --proto_path=../../protobufs/services ../../protobufs/services/*
```

### Generate Mirror
```bash
# cwd: `$REPO/sdk/swift`
protoc --swift_opt=Visibility=Public --swift_opt=FileNaming=PathToUnderscores --swift_out=./Sources/HederaProtobufs/Mirror -I=../../protobufs/mirror -I=../../protobufs/services ../../protobufs/mirror/*

# generate GRPC (if needed)
protoc --grpc-swift_opt=Visibility=Public,FileNaming=PathToUnderscores,Server=false --grpc-swift_out=./Sources/HederaProtobufs/Mirror -I=../../protobufs/mirror -I=../../protobufs/services ../../protobufs/mirror/*
```

### Generate SDK
```bash
# cwd: `$REPO/sdk/swift`
protoc --swift_opt=Visibility=Public --swift_opt=FileNaming=PathToUnderscores --swift_out=./Sources/HederaProtobufs/Sdk -I=../../protobufs/sdk -I=../../protobufs/services ../../protobufs/sdk/*
```
