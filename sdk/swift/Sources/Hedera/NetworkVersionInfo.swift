public struct NetworkVersionInfo: Codable {
    /// Version of the protobuf schema in use by the network.
    public let protobufVersion: SemanticVersion

    /// Version of the Hedera services in use by the network.
    public let servicesVersion: SemanticVersion
}
