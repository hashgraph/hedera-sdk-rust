import 'package:hedera/hedera.dart';

// TODO: from string
// TODO: to string
// TODO: to json
class AccountId extends EntityId {
  AccountId({
    super.shard,
    super.realm,
    // ignore: avoid_types_as_parameter_names
    required super.num,
  });
}
