package com.hedera.hashgraph.sdk;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

/**
 * A query that can be executed on the Hedera network.
 */
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "$type")
public sealed class Query<Response> permits AccountBalanceQuery {
    private final Class<Response> responseClass;

    protected Query(Class<Response> responseClass) {
        this.responseClass = responseClass;
    }

    public Response execute(Client client) {
        var objectMapper = new ObjectMapper();
        objectMapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);

        String request;

        try {
            request = objectMapper.writeValueAsString(this);
        } catch (JsonProcessingException e) {
            // BUG: should never happen if our serialization configuration is sane
            throw new RuntimeException(e);
        }

        var completableFuture = new CompletableFuture<String>();

        var executeErr = CHedera.instance.hedera_execute(client.ptr, request, null, (context, responseErr, response) -> {
            if (responseErr != CHedera.Error.OK) {
                // TODO: translate error to exception
                System.out.printf("ERROR hedera_execute callback invoked with error, %s\n", responseErr);

                // TODO: completableFuture.completeExceptionally();
                return;
            }

            completableFuture.complete(response);
        });

        if (executeErr != CHedera.Error.OK) {
            // TODO: translate error to exception
            System.out.printf("ERROR hedera_execute returned with error, %s\n", executeErr);

            throw new RuntimeException();
        }

        String response;

        try {
            response = completableFuture.get();
        } catch (InterruptedException | ExecutionException e) {
            throw new RuntimeException(e);
        }

        try {
            return objectMapper.readValue(response, responseClass);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
    }
}
