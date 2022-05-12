// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "Hedera",
    platforms: [
        .macOS(.v10_13),
        .iOS(.v12),
    ],
    products: [
        .library(name: "Hedera", targets: ["Hedera"]),
    ],
    dependencies: [
    ],
    targets: [
        .binaryTarget(name: "CHedera", path: "CHedera.xcframework"),
        .target(name: "Hedera", dependencies: ["CHedera"]),

        // Examples
        .target(name: "GetAccountBalanceExample", dependencies: ["Hedera"], path: "Examples/GetAccountBalance"),
        .target(name: "GetAccountInfoExample", dependencies: ["Hedera"], path: "Examples/GetAccountInfo"),
    ]
)
