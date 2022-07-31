/// Hedera follows semantic versioning for both the HAPI protobufs and
/// the Services software.
public struct SemanticVersion: Codable {
    /// Increases with incompatible API changes
    public let major: UInt32

    /// Increases with backwards-compatible new functionality
    public let minor: UInt32

    /// Increases with backwards-compatible bug fixes
    public let patch: UInt32
}
