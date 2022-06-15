package hedera

/// AccountAlias is the unique identifier for a cryptocurrency account on Hedera,
/// represented with an alias instead of an account number.
type AccountAlias struct {
	Shard uint64
	Realm uint64
	Alias PublicKey
}

func (accountAlias AccountAlias) _isAccountAlias() bool {
	return true
}
