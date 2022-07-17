/// Marks a schedule in the network's action queue as deleted.
public final class ScheduleDeleteTransaction: Transaction {
    /// Create a new `ScheduleDeleteTransaction`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
    }

    /// The schedule to delete.
    public var scheduleId: ScheduleId?

    /// Sets the schedule to delete.
    @discardableResult
    public func scheduleId(_ scheduleId: ScheduleId) -> Self {
        self.scheduleId = scheduleId

        return self
    }

    private enum CodingKeys: String, CodingKey {
        case scheduleId
    }

    public override func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        try container.encodeIfPresent(scheduleId, forKey: .scheduleId)

        try super.encode(to: encoder)
    }
}
