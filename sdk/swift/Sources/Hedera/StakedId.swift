import HederaProtobufs

internal enum StakedId {
    case accountId(AccountId)
    case nodeId(UInt64)

    internal var accountId: AccountId? {
        guard case .accountId(let id) = self else {
            return nil
        }

        return id
    }

    internal var nodeId: UInt64? {
        guard case .nodeId(let id) = self else {
            return nil
        }

        return id
    }

    internal static func flatDecode(from decoder: Decoder) throws -> Self? {
        let container = try decoder.container(keyedBy: CodingKeys.self)

        return try container.decodeIfPresent(.stakedAccountId).map(Self.accountId)
            ?? container.decodeIfPresent(.stakedNodeId).map(Self.nodeId)
    }
}

extension StakedId: Codable {
    private enum CodingKeys: CodingKey {
        case stakedAccountId
        case stakedNodeId
    }

    init(from decoder: Decoder) throws {
        self = try .flatDecode(from: decoder)!
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        switch self {
        case .accountId(let id):
            try container.encode(id, forKey: .stakedAccountId)
        case .nodeId(let id):
            try container.encode(id, forKey: .stakedNodeId)
        }
    }
}
