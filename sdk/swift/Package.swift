// swift-tools-version:5.5

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
    "GenerateKey",
    "GenerateKeyWithMnemonic",
    "GetAccountBalance",
    "GetAccountInfo",
    "TransferHbar",
    "GetAddressBook",
    "GetFileContents",
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
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.6.0"),
    ],
    targets: [
        .binaryTarget(name: "CHedera", path: "CHedera.xcframework"),
        .target(name: "HederaProtobufs", dependencies: [.product(name: "SwiftProtobuf", package: "swift-protobuf")]),
        .target(
            name: "Hedera",
            dependencies: [
                "HederaProtobufs", "CHedera", .product(name: "SwiftProtobuf", package: "swift-protobuf"),
                .product(name: "NumberKit", package: "swift-numberkit"),
            ]),
        .testTarget(name: "HederaTests", dependencies: ["Hedera"]),
    ] + exampleTargets
)
