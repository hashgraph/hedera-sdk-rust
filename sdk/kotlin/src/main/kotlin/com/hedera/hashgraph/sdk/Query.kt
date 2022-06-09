package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonInclude
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.core.JsonProcessingException
import com.fasterxml.jackson.databind.ObjectMapper
import java.util.concurrent.CompletableFuture
import java.util.concurrent.ExecutionException

/**
 * A query that can be executed on the Hedera network.
 */
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "\$type")
open class Query<Response> protected constructor(private val responseClass: Class<Response>) {
    fun execute(client: Client): Response {
        val objectMapper = ObjectMapper()
        objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL)

        val request: String = try {
            objectMapper.writeValueAsString(this)
        } catch (e: JsonProcessingException) {
            // BUG: should never happen if our serialization configuration is sane
            throw RuntimeException(e)
        }

        val completableFuture = CompletableFuture<String?>()

        val executeErr = CHedera.instance.hedera_execute(client.ptr, request, null) { _, responseErr, response ->
            if (responseErr !== CHedera.Error.OK) {
                // TODO: translate error to exception
                System.out.printf("ERROR hedera_execute callback invoked with error, %s\n", responseErr)

                // TODO: completableFuture.completeExceptionally();
                return@hedera_execute
            }

            completableFuture.complete(response)
        }

        if (executeErr !== CHedera.Error.OK) {
            // TODO: translate error to exception
            System.out.printf("ERROR hedera_execute returned with error, %s\n", executeErr)
            throw RuntimeException()
        }

        val response: String? = try {
            completableFuture.get()
        } catch (e: InterruptedException) {
            throw RuntimeException(e)
        } catch (e: ExecutionException) {
            throw RuntimeException(e)
        }

        return try {
            objectMapper.readValue(response, responseClass)
        } catch (e: JsonProcessingException) {
            throw RuntimeException(e)
        }
    }
}
