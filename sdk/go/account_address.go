package hedera

/// AccountAddress is either `AccountId` or `AccountAlias`.
///
/// Some transactions and queries accept `AccountAddress` as an input.
/// All transactions and queries return only `AccountId` as an output however.
///
type AccountAddress interface {
	_isAccountAlias() bool
}
