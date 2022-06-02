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
            dependencies: ["Hedera"],
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
        .macOS(.v10_15),
        .iOS(.v12),
    ],
    products: [
        .library(name: "Hedera", targets: ["Hedera"])
    ],
    dependencies: [],
    targets: [
        .binaryTarget(name: "CHedera", path: "CHedera.xcframework"),
        .target(name: "Hedera", dependencies: ["CHedera"]),
    ] + exampleTargets
)
