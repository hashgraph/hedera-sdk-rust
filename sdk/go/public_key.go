package hedera

// PublicKey is an Ed25519 or ECDSA(secp256k1) public key on the Hedera network.
type PublicKey struct{}

func (key PublicKey) _isKey() bool {
	return true
}
