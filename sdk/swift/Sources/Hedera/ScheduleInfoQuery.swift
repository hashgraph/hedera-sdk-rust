/// Get all the information about a schedule.
public class ScheduleInfoQuery: Query<ScheduleInfo> {
    /// Create a new `ScheduleInfoQuery`.
    public init(
        scheduleId: ScheduleId? = nil
    ) {
        self.scheduleId = scheduleId
    }

    /// The schedule ID for which information is requested.
    public var scheduleId: ScheduleId?

    /// Sets the schedule ID for which information is requested.
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
