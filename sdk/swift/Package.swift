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

// swift-tools-version:5.5
import PackageDescription

// collect example targets
var exampleTargets: [PackageDescription.Target] = []
for name in [
    "GetAccountBalance",
    "GenerateKey",
    "GetAccountInfo",
    "TransferHbar",
    "CreateAccount",
    "DeleteAccount",
    "GetFileContents",
] {
    exampleTargets.append(
        .executableTarget(
            name: "\(name)Example",
            dependencies: ["Hedera", .product(name: "SwiftDotenv", package: "swift-dotenv")],
            path: "Examples/\(name)",
            swiftSettings: [
                .unsafeFlags([
                    "-parse-as-library"
                ])
            ]))
}

let package = Package(
    name: "Hedera",
    platforms: [
        .macOS(.v10_15)
        // .iOS(.v12),
    ],
    products: [
        .library(name: "Hedera", targets: ["Hedera"])
    ],
    dependencies: [
        .package(url: "https://github.com/objecthub/swift-numberkit.git", .upToNextMajor(from: "2.4.1")),
        .package(url: "https://github.com/thebarndog/swift-dotenv.git", .upToNextMajor(from: "1.0.0")),
    ],
    targets: [
        .binaryTarget(name: "CHedera", path: "CHedera.xcframework"),
        .target(name: "Hedera", dependencies: ["CHedera", .product(name: "NumberKit", package: "swift-numberkit")]),
    ] + exampleTargets
)
