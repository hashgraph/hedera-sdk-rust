package com.hedera.hashgraph.sdk

/**
 * A query that can be executed on the Hedera network.
 */
sealed class Query<Response> constructor(responseClass: Class<Response>) : Request<Response>(responseClass) {
}
