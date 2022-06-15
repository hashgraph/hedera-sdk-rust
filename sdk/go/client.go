package hedera

// #include "native/hedera.h"
import "C"

import "runtime"

// Client provides a connection to the Hedera network.
type Client struct {
	ptr *C.HederaClient
}

// ClientForTestnet constructs a Hedera client pre-configured for testnet access.
func ClientForTestnet() *Client {
	return _makeClient(C.hedera_client_for_testnet())
}

func _makeClient(ptr *C.HederaClient) *Client {
	client := new(Client)
	client.ptr = ptr

	runtime.SetFinalizer(client, _clientFinalizer)

	return client
}

func _clientFinalizer(client *Client) {
	C.hedera_client_free(client.ptr)
}
