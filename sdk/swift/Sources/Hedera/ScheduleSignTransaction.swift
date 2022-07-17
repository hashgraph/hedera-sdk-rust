/// Adds zero or more signing keys to a schedule.
public final class ScheduleSignTransaction: Transaction {
    /// Create a new `ScheduleSignTransaction`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
    }

    /// The schedule to add signing keys to.
    public var scheduleId: ScheduleId?

    /// Set the schedule to add signing keys to.
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
