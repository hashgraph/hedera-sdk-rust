// swift-tools-version:5.6

/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

import PackageDescription

let exampleTargets = [
    "CreateAccount",
    "CreateFile",
    "CreateTopic",
    "DeleteAccount",
    "DeleteFile",
    "FileAppendChunked",
    "GenerateKey",
    "GenerateKeyWithMnemonic",
    "GetAccountBalance",
    "GetAccountInfo",
    "GetAddressBook",
    "GetExchangeRates",
    "GetFileContents",
    "TopicWithAdminKey",
    "TransferCrypto",
    "TransferTokens",
    "ValidateChecksum",
    "UpdateAccountPublicKey",
    "Prng",
].map { name in
    Target.executableTarget(
        name: "\(name)Example",
        dependencies: ["Hedera", .product(name: "SwiftDotenv", package: "swift-dotenv")],
        path: "Examples/\(name)",
        swiftSettings: [.unsafeFlags(["-parse-as-library"])]
    )
}

let package = Package(
    name: "Hedera",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v13),
    ],
    products: [
        .library(name: "Hedera", targets: ["Hedera"])
    ],
    dependencies: [
        .package(url: "https://github.com/objecthub/swift-numberkit.git", from: "2.4.1"),
        .package(url: "https://github.com/thebarndog/swift-dotenv.git", from: "1.0.0"),
        .package(url: "https://github.com/grpc/grpc-swift.git", from: "1.14.2"),
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.6.0"),
        .package(url: "https://github.com/vsanthanam/AnyAsyncSequence.git", from: "1.0.0"),
        .package(url: "https://github.com/apple/swift-atomics.git", from: "1.0.0"),
        // swift-asn1 wants swift 5.7+ past 0.4
        .package(url: "https://github.com/apple/swift-asn1.git", "0.3.0"..<"0.4.0"),
        .package(url: "https://github.com/GigaBitcoin/secp256k1.swift.git", .upToNextMajor(from: "0.10.0")),
        // we use this entirely for sha3-keccak256, yes, I'm serious.
        .package(url: "https://github.com/krzyzanowskim/CryptoSwift.git", from: "1.0.0"),
        .package(url: "https://github.com/apple/swift-docc-plugin", from: "1.0.0"),
    ],
    targets: [
        .target(
            name: "HederaProtobufs",
            dependencies: [
                .product(name: "SwiftProtobuf", package: "swift-protobuf"),
                .product(name: "GRPC", package: "grpc-swift"),
            ]),
        .target(
            name: "Hedera",
            dependencies: [
                "HederaProtobufs",
                "AnyAsyncSequence",
                .product(name: "SwiftASN1", package: "swift-asn1"),
                .product(name: "SwiftProtobuf", package: "swift-protobuf"),
                .product(name: "NumberKit", package: "swift-numberkit"),
                .product(name: "GRPC", package: "grpc-swift"),
                .product(name: "Atomics", package: "swift-atomics"),
                .product(name: "secp256k1", package: "secp256k1.swift"),
                "CryptoSwift",
            ]
            // todo: find some way to enable these locally.
            // swiftSettings: [
            //     .unsafeFlags(["-Xfrontend", "-warn-concurrency", "-Xfrontend", "-enable-actor-data-race-checks"])
            // ]
        ),
        .testTarget(name: "HederaTests", dependencies: ["Hedera"]),
    ] + exampleTargets
)
