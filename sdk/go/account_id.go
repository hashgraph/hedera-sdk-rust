package hedera

/// AccountID is the unique identifier for a cryptocurrency account on Hedera.
type AccountID struct {
	Shard uint64
	Realm uint64
	Num   uint64
}

func (alias AccountID) _isAccountAlias() bool {
	return false
}

/// AccountAlias is the unique identifier for a cryptocurrency account on Hedera,
/// represented with an alias instead of an account number.
type AccountAlias struct {
	Shard uint64
	Realm uint64
	Alias PublicKey
}

func (alias AccountAlias) _isAccountAlias() bool {
	return true
}

/// AccountIDOrAlias is either `AccountId` or `AccountAlias`.
///
/// Some transactions and queries accept `AccountIdOrAlias` as an input.
/// All transactions and queries return only `AccountId` as an output however.
///
type AccountIDOrAlias interface {
	_isAccountAlias() bool
}
