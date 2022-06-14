package com.hedera.hashgraph.sdk

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeName
import java.time.Duration

/**
 * Create a new Hedera™ account.
 */
@JsonTypeName("accountCreate")
class AccountCreateTransaction(
    /**
     * The key that must sign each transfer out of the account.
     */
    @set:JvmSynthetic
    @JsonProperty
    var key: Key? = null,

    /**
     * The initial number of Hbar to put into the account.
     */
    @set:JvmSynthetic
    @JsonProperty
    var initialBalance: Long = 0,

    /**
     * If true, this account's key must sign any transaction depositing into this account.
     */
    @set:JvmSynthetic
    @JsonProperty
    var receiverSignatureRequired: Boolean = false,

    /**
     * The period until the account will be charged to extend its expiration date.
     */
    @set:JvmSynthetic
    @JsonProperty
    var autoRenewPeriod: Duration? = null,

    /**
     * The memo associated with the account.
     */
    @set:JvmSynthetic
    @JsonProperty
    var accountMemo: String? = null,

    /**
     * The maximum number of tokens that an Account can be implicitly associated with.
     */
    @set:JvmSynthetic
    @JsonProperty
    var maxAutomaticTokenAssociations: Long = 0,

    /**
     * ID of the account to which this account is staking.
     * This is mutually exclusive with stakedNodeId.
     */
    @set:JvmSynthetic
    @JsonProperty
    var stakedAccountId: AccountAddress? = null,

    /**
     * ID of the node this account is staked to.
     * This is mutually exclusive with stakedAccountId.
     */
    @set:JvmSynthetic
    @JsonProperty
    var stakedNodeId: Long = 0,

    /**
     * If true, the account declines receiving a staking reward. The default value is false.
     */
    @set:JvmSynthetic
    @JsonProperty
    var declineStakingReward: Boolean = false,
) : Transaction() {
    /**
     * Sets the key that must sign each transfer out of the account.
     */
    fun setKey(key: Key): AccountCreateTransaction {
        this.key = key

        return this
    }

    /**
     * Sets the initial number of Hbar to put into the account.
     */
    fun setInitialBalance(initialBalance: Long): AccountCreateTransaction {
        this.initialBalance = initialBalance

        return this
    }

    /**
     * Set to true to require this account to sign any transfer of hbars to this account.
     */
    fun setReceiverSignatureRequired(receiverSignatureRequired: Boolean): AccountCreateTransaction {
        this.receiverSignatureRequired = receiverSignatureRequired

        return this
    }

    /**
     * Sets the period until the account will be charged to extend its expiration date.
     */
    fun setAutoRenewPeriod(autoRenewPeriod: Duration): AccountCreateTransaction {
        this.autoRenewPeriod = autoRenewPeriod

        return this
    }

    /**
     * Sets the memo associated with the account.
     */
    fun setAccountMemo(accountMemo: String): AccountCreateTransaction {
        this.accountMemo = accountMemo

        return this
    }

    /**
     * Sets the maximum number of tokens that an Account can be implicitly associated with.
     */
    fun setMaxAutomaticTokenAssociations(maxAutomaticTokenAssociations: Long): AccountCreateTransaction {
        this.maxAutomaticTokenAssociations = maxAutomaticTokenAssociations

        return this
    }

    /**
     * Sets the ID of the account to which this account is staking.
     */
    fun setStakedAccountId(stakedAccountId: AccountAddress): AccountCreateTransaction {
        this.stakedAccountId = stakedAccountId

        return this
    }

    /**
     * Sets the ID of the node to which this account is staking.
     */
    fun setStakedNodeId(stakedNodeId: Long): AccountCreateTransaction {
        this.stakedNodeId = stakedNodeId

        return this
    }

    /**
     * Set to true, the account declines receiving a staking reward. The default value is false.
     */
    fun setDeclineStakingReward(declineStakingReward: Boolean): AccountCreateTransaction {
        this.declineStakingReward = declineStakingReward

        return this
    }
}
