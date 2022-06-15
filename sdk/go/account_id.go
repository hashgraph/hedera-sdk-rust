package hedera

/// AccountID is the unique identifier for a cryptocurrency account on Hedera.
type AccountID struct {
	Shard uint64
	Realm uint64
	Num   uint64
}

func (accountID AccountID) _isAccountAlias() bool {
	return false
}
