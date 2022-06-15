package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.annotation.JsonValue

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.WRAPPER_OBJECT)
sealed interface Key {
    companion object {
        @JvmStatic
        fun delegatedContractId(contractId: ContractId): Key {
            return DelegatedContractId(contractId)
        }

        @JsonTypeName("delegatedContractId")
        private class DelegatedContractId(val contractId: ContractId) : Key {
            @JsonValue
            override fun toString(): String = contractId.toString()
        }
    }
}
