package hedera

// Key describes a method that can be used to authorize an operation on Hedera.
type Key interface {
	_isKey() bool
}
