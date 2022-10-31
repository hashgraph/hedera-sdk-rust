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

use std::env;
use std::fs::{
    self,
    create_dir_all,
    read_dir,
};
use std::path::Path;

const DERIVE_EQ_HASH: &str = "#[derive(Eq, Hash)]";
const DERIVE_EQ_HASH_COPY: &str = "#[derive(Copy, Eq, Hash)]";

fn main() -> anyhow::Result<()> {
    // services is the "base" module for the hedera protobufs
    // in the beginning, there was only services and it was named "protos"

    let services: Vec<_> = read_dir("../../../protobufs/services")?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    let mut cfg = tonic_build::configure().build_server(cfg!(feature = "server"));

    // most of the protobufs in "basic types" should be Eq + Hash + Copy
    // any protobufs that would typically be used as parameter, that meet the requirements of those
    // traits
    cfg = cfg
        .type_attribute("proto.ShardID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.RealmID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.AccountID", DERIVE_EQ_HASH)
        .type_attribute("proto.AccountID.account", DERIVE_EQ_HASH)
        .type_attribute("proto.FileID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.ContractID", DERIVE_EQ_HASH)
        .type_attribute("proto.ContractID.contract", DERIVE_EQ_HASH)
        .type_attribute("proto.TransactionID", DERIVE_EQ_HASH)
        .type_attribute("proto.Timestamp", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.NftTransfer", DERIVE_EQ_HASH)
        .type_attribute("proto.Fraction", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.TopicID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.TokenID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.ScheduleID", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.FeeComponents", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.Key", DERIVE_EQ_HASH)
        .type_attribute("proto.KeyList", DERIVE_EQ_HASH)
        .type_attribute("proto.ThresholdKey", DERIVE_EQ_HASH)
        .type_attribute("proto.Key.key", DERIVE_EQ_HASH)
        .type_attribute("proto.SignaturePair", DERIVE_EQ_HASH)
        .type_attribute("proto.SignaturePair.signature", DERIVE_EQ_HASH)
        .type_attribute("proto.FeeData", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.TokenBalance", DERIVE_EQ_HASH_COPY)
        .type_attribute("proto.TokenAssociation", DERIVE_EQ_HASH)
        .type_attribute("proto.CryptoAllowance", DERIVE_EQ_HASH)
        .type_attribute("proto.TokenAllowance", DERIVE_EQ_HASH)
        .type_attribute("proto.GrantedCryptoAllowance", DERIVE_EQ_HASH)
        .type_attribute("proto.GrantedTokenAllowance", DERIVE_EQ_HASH)
        .type_attribute("proto.Duration", DERIVE_EQ_HASH_COPY);

    // the ResponseCodeEnum should be marked as #[non_exhaustive] so
    // adding variants does not trigger a breaking change
    cfg = cfg.type_attribute("proto.ResponseCodeEnum", "#[non_exhaustive]");

    // the ResponseCodeEnum is not documented in the proto source
    cfg = cfg.type_attribute(
        "proto.ResponseCodeEnum",
        r#"#[doc = "
Returned in `TransactionReceipt`, `Error::PreCheckStatus`, and `Error::ReceiptStatus`.

The success variant is `Success` which is what a `TransactionReceipt` will contain for a
successful transaction.
    "]"#,
    );

    if cfg!(feature = "serde") {
        cfg = cfg
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .type_attribute(
                "proto.ResponseCodeEnum",
                "#[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]",
            );
    }

    cfg.compile(&services, &["../../../protobufs/services/"])?;

    // NOTE: prost generates rust doc comments and fails to remove the leading * line
    remove_useless_comments(&Path::new(&env::var("OUT_DIR")?).join("proto.rs"))?;

    // mirror
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let mirror_out_dir = Path::new(&env::var("OUT_DIR")?).join("mirror");
    create_dir_all(&mirror_out_dir)?;

    tonic_build::configure()
        .build_server(false)
        .extern_path(".proto.Timestamp", "crate::services::Timestamp")
        .extern_path(".proto.TopicID", "crate::services::TopicId")
        .extern_path(".proto.FileID", "crate::services::FileId")
        .extern_path(".proto.NodeAddress", "crate::services::NodeAddress")
        .extern_path(
            ".proto.ConsensusMessageChunkInfo",
            "crate::services::ConsensusMessageChunkInfo",
        )
        .out_dir(&mirror_out_dir)
        .compile(
            &[
                "../../../protobufs/mirror/consensus_service.proto",
                "../../../protobufs/mirror/mirror_network_service.proto",
            ],
            &["../../../protobufs/mirror/", "../../../protobufs/services/"],
        )?;

    // streams
    // NOTE: must be compiled in a separate folder otherwise it will overwrite the previous build

    let streams_out_dir = Path::new(&env::var("OUT_DIR")?).join("streams");
    create_dir_all(&streams_out_dir)?;

    // NOTE: **ALL** protobufs defined in basic_types must be specified here
    let mut cfg = tonic_build::configure();

    cfg = cfg
        .extern_path(".proto.Fraction", "crate::services::Fraction")
        .extern_path(".proto.Timestamp", "crate::services::Timestamp")
        .extern_path(".proto.AccountID", "crate::services::AccountId")
        .extern_path(".proto.TokenID", "crate::services::TokenId")
        .extern_path(".proto.AccountAmount", "crate::services::AccountAmount")
        .extern_path(
            ".proto.CurrentAndNextFeeSchedule",
            "crate::services::CurrentAndNextFeeSchedule",
        )
        .extern_path(".proto.FeeComponents", "crate::services::FeeComponents")
        .extern_path(".proto.FeeData", "crate::services::FeeData")
        .extern_path(".proto.FeeSchedule", "crate::services::FeeSchedule")
        .extern_path(".proto.Key", "crate::services::Key")
        .extern_path(".proto.FileID", "crate::services::FileId")
        .extern_path(".proto.KeyList", "crate::services::KeyList")
        .extern_path(".proto.NftTransfer", "crate::services::NftTransfer")
        .extern_path(".proto.NodeAddress", "crate::services::NodeAddress")
        .extern_path(".proto.NodeAddressBook", "crate::services::NodeAddressBook")
        .extern_path(".proto.RealmID", "crate::services::RealmId")
        .extern_path(".proto.ScheduleID", "crate::services::ScheduleId")
        .extern_path(".proto.SemanticVersion", "crate::services::SemanticVersion")
        .extern_path(".proto.ServiceEndpoint", "crate::services::ServiceEndpoint")
        .extern_path(
            ".proto.ServicesConfigurationList",
            "crate::services::ServicesConfigurationList",
        )
        .extern_path(".proto.Setting", "crate::services::Setting")
        .extern_path(".proto.ShardID", "crate::services::ShardId")
        .extern_path(".proto.Signature", "crate::services::Signature")
        .extern_path(".proto.SignatureList", "crate::services::SignatureList")
        .extern_path(".proto.SignatureMap", "crate::services::SignatureMap")
        .extern_path(".proto.SignaturePair", "crate::services::SignaturePair")
        .extern_path(".proto.ThresholdKey", "crate::services::ThresholdKey")
        .extern_path(".proto.ThresholdSignature", "crate::services::ThresholdSignature")
        .extern_path(".proto.TimestampSeconds", "crate::services::TimestampSeconds")
        .extern_path(".proto.TokenBalance", "crate::services::TokenBalance")
        .extern_path(".proto.TokenBalances", "crate::services::TokenBalances")
        .extern_path(".proto.TokenRelationship", "crate::services::TokenRelationship")
        .extern_path(".proto.TokenTransferList", "crate::services::TokenTransferList")
        .extern_path(".proto.TopicID", "crate::services::TopicId")
        .extern_path(".proto.TransactionFeeSchedule", "crate::services::TransactionFeeSchedule")
        .extern_path(".proto.TransactionID", "crate::services::TransactionId")
        .extern_path(".proto.TransferList", "crate::services::TransferList")
        .extern_path(".proto.HederaFunctionality", "crate::services::HederaFunctionality")
        .extern_path(".proto.SubType", "crate::services::SubType")
        .extern_path(".proto.TokenFreezeStatus", "crate::services::TokenFreezeStatus")
        .extern_path(".proto.TokenKycStatus", "crate::services::TokenKycStatus")
        .extern_path(".proto.TokenSupplyType", "crate::services::TokenSupplyType")
        .extern_path(".proto.TokenType", "crate::services::TokenType")
        .extern_path(".proto.GrantedCryptoAllowance", "crate::services::GrantedCryptoAllowance")
        .extern_path(".proto.GrantedTokenAllowance", "crate::services::GrantedTokenAllowance")
        .extern_path(".proto.CryptoAllowance", "crate::services::CryptoAllowance")
        .extern_path(".proto.TokenAllowance", "crate::services::TokenAllowance")
        .extern_path(".proto.GrantedNftAllowance", "crate::services::GrantedNftAllowance")
        .extern_path(".proto.NftAllowance", "crate::services::NftAllowance")
        .extern_path(".proto.TokenPauseStatus", "crate::services::TokenPauseStatus")
        .extern_path(".proto.TokenAssociation", "crate::services::TokenAssociation")
        .extern_path(".proto.ContractID", "crate::services::ContractId");

    cfg.out_dir(&streams_out_dir).compile(
        &["../../../protobufs/streams/account_balance_file.proto"],
        &["../../../protobufs/streams/", "../../../protobufs/services/"],
    )?;

    Ok(())
}

fn remove_useless_comments(path: &Path) -> anyhow::Result<()> {
    let mut contents = fs::read_to_string(path)?;

    contents = contents.replace("///*", "");
    contents = contents.replace("/// UNDOCUMENTED", "");

    fs::write(path, contents)?;

    Ok(())
}
