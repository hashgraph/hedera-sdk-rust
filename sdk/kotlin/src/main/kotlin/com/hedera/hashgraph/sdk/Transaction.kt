package com.hedera.hashgraph.sdk

sealed class Transaction : Request<TransactionResponse>(TransactionResponse::class.java) {
}
