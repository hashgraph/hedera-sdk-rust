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
        .target(name: "Hedera", dependencies: []),
    ]
)
