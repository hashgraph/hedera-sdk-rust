/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use hedera_proto::services;

use crate::protobuf::FromProtobuf;
use crate::Error;

// sigh.
pub(super) enum TransactionData {
    Chunkable(Chunkable),
    Unchunkable(Box<Unchunkable>),
}

pub(super) enum Chunkable {
    TopicMessageSubmit(Vec<services::ConsensusSubmitMessageTransactionBody>),
    FileAppend(Vec<services::FileAppendTransactionBody>),
}

#[derive(Debug)]
pub(super) enum Unchunkable {
    AccountUpdate(services::CryptoUpdateTransactionBody),
    AccountCreate(services::CryptoCreateTransactionBody),
    AccountDelete(services::CryptoDeleteTransactionBody),
    AccountAllowanceApprove(services::CryptoApproveAllowanceTransactionBody),
    AccountAllowanceDelete(services::CryptoDeleteAllowanceTransactionBody),
    ContractCreate(services::ContractCreateTransactionBody),
    ContractUpdate(services::ContractUpdateTransactionBody),
    ContractDelete(services::ContractDeleteTransactionBody),
    ContractExecute(services::ContractCallTransactionBody),
    Transfer(services::CryptoTransferTransactionBody),
    TopicCreate(services::ConsensusCreateTopicTransactionBody),
    TopicUpdate(services::ConsensusUpdateTopicTransactionBody),
    TopicDelete(services::ConsensusDeleteTopicTransactionBody),
    FileCreate(services::FileCreateTransactionBody),
    FileUpdate(services::FileUpdateTransactionBody),
    FileDelete(services::FileDeleteTransactionBody),
    TokenAssociate(services::TokenAssociateTransactionBody),
    TokenBurn(services::TokenBurnTransactionBody),
    TokenCreate(services::TokenCreateTransactionBody),
    TokenDelete(services::TokenDeleteTransactionBody),
    TokenDissociate(services::TokenDissociateTransactionBody),
    TokenFeeScheduleUpdate(services::TokenFeeScheduleUpdateTransactionBody),
    TokenFreeze(services::TokenFreezeAccountTransactionBody),
    TokenGrantKyc(services::TokenGrantKycTransactionBody),
    TokenMint(services::TokenMintTransactionBody),
    TokenPause(services::TokenPauseTransactionBody),
    TokenRevokeKyc(services::TokenRevokeKycTransactionBody),
    TokenUnfreeze(services::TokenUnfreezeAccountTransactionBody),
    TokenUnpause(services::TokenUnpauseTransactionBody),
    TokenUpdate(services::TokenUpdateTransactionBody),
    TokenWipe(services::TokenWipeAccountTransactionBody),
    SystemDelete(services::SystemDeleteTransactionBody),
    SystemUndelete(services::SystemUndeleteTransactionBody),
    Freeze(services::FreezeTransactionBody),
    ScheduleCreate(services::ScheduleCreateTransactionBody),
    ScheduleSign(services::ScheduleSignTransactionBody),
    ScheduleDelete(services::ScheduleDeleteTransactionBody),
    Ethereum(services::EthereumTransactionBody),
    UtilPrng(services::UtilPrngTransactionBody),
}

impl From<Unchunkable> for TransactionData {
    fn from(value: Unchunkable) -> Self {
        Self::Unchunkable(Box::new(value))
    }
}

impl FromProtobuf<Vec<services::transaction_body::Data>> for TransactionData {
    fn from_protobuf(pb: Vec<services::transaction_body::Data>) -> crate::Result<Self> {
        use services::transaction_body::Data;

        fn make_vec<T>(first: T, cap: usize) -> Vec<T> {
            let mut v = Vec::with_capacity(cap);
            v.push(first);
            v
        }

        let len = pb.len();

        let mut iter = pb.into_iter();

        let first = iter
            .next()
            .expect("empty transaction data list (should be handled earlier up the pipeline)");

        // note: this impl is what I (srr) believe to be the "best" impl
        let mut value = match (first, len) {
            (Data::FileAppend(it), len) => Chunkable::FileAppend(make_vec(it, len)),
            (Data::ConsensusSubmitMessage(it), len) => {
                Chunkable::TopicMessageSubmit(make_vec(it, len))
            }

            (Data::ContractCall(it), 1) => return Ok(Unchunkable::ContractExecute(it).into()),
            (Data::ContractCreateInstance(it), 1) => {
                return Ok(Unchunkable::ContractCreate(it).into())
            }
            (Data::ContractUpdateInstance(it), 1) => {
                return Ok(Unchunkable::ContractUpdate(it).into())
            }
            (Data::ContractDeleteInstance(it), 1) => {
                return Ok(Unchunkable::ContractDelete(it).into())
            }
            (Data::EthereumTransaction(it), 1) => return Ok(Unchunkable::Ethereum(it).into()),
            (Data::CryptoApproveAllowance(it), 1) => {
                return Ok(Unchunkable::AccountAllowanceApprove(it).into())
            }
            (Data::CryptoDeleteAllowance(it), 1) => {
                return Ok(Unchunkable::AccountAllowanceDelete(it).into())
            }
            (Data::CryptoCreateAccount(it), 1) => return Ok(Unchunkable::AccountCreate(it).into()),
            (Data::CryptoDelete(it), 1) => return Ok(Unchunkable::AccountDelete(it).into()),
            (Data::CryptoTransfer(it), 1) => return Ok(Unchunkable::Transfer(it).into()),
            (Data::CryptoUpdateAccount(it), 1) => return Ok(Unchunkable::AccountUpdate(it).into()),
            (Data::FileCreate(it), 1) => return Ok(Unchunkable::FileCreate(it).into()),
            (Data::FileDelete(it), 1) => return Ok(Unchunkable::FileDelete(it).into()),
            (Data::FileUpdate(it), 1) => return Ok(Unchunkable::FileUpdate(it).into()),
            (Data::SystemDelete(it), 1) => return Ok(Unchunkable::SystemDelete(it).into()),
            (Data::SystemUndelete(it), 1) => return Ok(Unchunkable::SystemUndelete(it).into()),
            (Data::Freeze(it), 1) => return Ok(Unchunkable::Freeze(it).into()),
            (Data::ConsensusCreateTopic(it), 1) => return Ok(Unchunkable::TopicCreate(it).into()),
            (Data::ConsensusUpdateTopic(it), 1) => return Ok(Unchunkable::TopicUpdate(it).into()),
            (Data::ConsensusDeleteTopic(it), 1) => return Ok(Unchunkable::TopicDelete(it).into()),
            (Data::TokenCreation(it), 1) => return Ok(Unchunkable::TokenCreate(it).into()),
            (Data::TokenFreeze(it), 1) => return Ok(Unchunkable::TokenFreeze(it).into()),
            (Data::TokenUnfreeze(it), 1) => return Ok(Unchunkable::TokenUnfreeze(it).into()),
            (Data::TokenGrantKyc(it), 1) => return Ok(Unchunkable::TokenGrantKyc(it).into()),
            (Data::TokenRevokeKyc(it), 1) => return Ok(Unchunkable::TokenRevokeKyc(it).into()),
            (Data::TokenDeletion(it), 1) => return Ok(Unchunkable::TokenDelete(it).into()),
            (Data::TokenUpdate(it), 1) => return Ok(Unchunkable::TokenUpdate(it).into()),
            (Data::TokenMint(it), 1) => return Ok(Unchunkable::TokenMint(it).into()),
            (Data::TokenBurn(it), 1) => return Ok(Unchunkable::TokenBurn(it).into()),
            (Data::TokenWipe(it), 1) => return Ok(Unchunkable::TokenWipe(it).into()),
            (Data::TokenAssociate(it), 1) => return Ok(Unchunkable::TokenAssociate(it).into()),
            (Data::TokenDissociate(it), 1) => return Ok(Unchunkable::TokenDissociate(it).into()),
            (Data::TokenFeeScheduleUpdate(it), 1) => {
                return Ok(Unchunkable::TokenFeeScheduleUpdate(it).into())
            }
            (Data::TokenPause(it), 1) => return Ok(Unchunkable::TokenPause(it).into()),
            (Data::TokenUnpause(it), 1) => return Ok(Unchunkable::TokenUnpause(it).into()),
            (Data::ScheduleCreate(it), 1) => return Ok(Unchunkable::ScheduleCreate(it).into()),
            (Data::ScheduleDelete(it), 1) => return Ok(Unchunkable::ScheduleDelete(it).into()),
            (Data::ScheduleSign(it), 1) => return Ok(Unchunkable::ScheduleSign(it).into()),
            (Data::UtilPrng(it), 1) => return Ok(Unchunkable::UtilPrng(it).into()),

            (
                Data::ContractCall(_)
                | Data::ContractCreateInstance(_)
                | Data::ContractUpdateInstance(_)
                | Data::ContractDeleteInstance(_)
                | Data::EthereumTransaction(_)
                | Data::CryptoApproveAllowance(_)
                | Data::CryptoDeleteAllowance(_)
                | Data::CryptoCreateAccount(_)
                | Data::CryptoDelete(_)
                | Data::CryptoTransfer(_)
                | Data::CryptoUpdateAccount(_)
                | Data::FileCreate(_)
                | Data::FileDelete(_)
                | Data::FileUpdate(_)
                | Data::SystemDelete(_)
                | Data::SystemUndelete(_)
                | Data::Freeze(_)
                | Data::ConsensusCreateTopic(_)
                | Data::ConsensusUpdateTopic(_)
                | Data::ConsensusDeleteTopic(_)
                | Data::TokenCreation(_)
                | Data::TokenFreeze(_)
                | Data::TokenUnfreeze(_)
                | Data::TokenGrantKyc(_)
                | Data::TokenRevokeKyc(_)
                | Data::TokenDeletion(_)
                | Data::TokenUpdate(_)
                | Data::TokenMint(_)
                | Data::TokenBurn(_)
                | Data::TokenWipe(_)
                | Data::TokenAssociate(_)
                | Data::TokenDissociate(_)
                | Data::TokenFeeScheduleUpdate(_)
                | Data::TokenPause(_)
                | Data::TokenUnpause(_)
                | Data::ScheduleCreate(_)
                | Data::ScheduleDelete(_)
                | Data::ScheduleSign(_)
                | Data::UtilPrng(_),
                _,
            ) => return Err(Error::from_protobuf("chunks in non chunkable transaction")),

            (Data::CryptoAddLiveHash(_), _) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `AddLiveHashTransaction`",
                ))
            }

            (Data::CryptoDeleteLiveHash(_), _) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `DeleteLiveHashTransaction`",
                ))
            }

            (Data::UncheckedSubmit(_), _) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `UncheckedSubmitTransaction`",
                ))
            }
            (Data::NodeStakeUpdate(_), _) => {
                return Err(Error::from_protobuf(
                    "unsupported transaction `NodeStakeUpdateTransaction`",
                ))
            }
        };

        for transaction in iter {
            match (&mut value, transaction) {
                (Chunkable::FileAppend(v), Data::FileAppend(element)) => v.push(element),
                (Chunkable::TopicMessageSubmit(v), Data::ConsensusSubmitMessage(element)) => {
                    v.push(element);
                }

                _ => return Err(Error::from_protobuf("mismatched transaction types")),
            }
        }

        Ok(Self::Chunkable(value))
    }
}
