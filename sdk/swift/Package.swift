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
