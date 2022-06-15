// TODO: from string
// TODO: to string
// TODO: to json
class EntityId {
  EntityId({
    this.shard = 0,
    this.realm = 0,
    required this.num,
  });

  /// The shard number (non-negative).
  final int realm;

  /// The realm number (non-negative).
  final int shard;

  /// The entity (account, file, contract, token, topic, or schedule) number (non-negative).
  final int num;
}
