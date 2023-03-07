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

use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::evm_address::IdEvmAddress;
use crate::{
    Checksum,
    Client,
    Error,
    LedgerId,
};

/// Generic shard.realm.num formatting
///
/// This module serves two purposes
/// 1. Code size (one implementation of formatting when `opt-level=z`)
/// 2. Performance! This ducks the core::fmt machinary except when required (the display/debug functions).
pub(crate) mod format {
    use core::fmt;

    // 3 u64s (max length = 20), 2 separators (length = 1)
    const MAX_CAPACITY: usize = 20 * 3 + 1 * 2;
    pub(crate) struct Buffer(arrayvec::ArrayString<MAX_CAPACITY>);

    impl Buffer {
        #[inline(always)]
        pub(crate) fn as_str(&self) -> &str {
            self.0.as_str()
        }
    }

    impl std::ops::Deref for Buffer {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_str()
        }
    }

    /// Formats a `shard`, `realm`, `num` in the format `{shard}.{realm}.{num}`
    ///
    /// Returns a stack-based string, it's not super cheap to pass around, so, prefer [`to_string`] if faster.
    ///
    /// # Performance
    ///
    /// This ends up cheaper than multiple `fmt::Write` calls for `display`/`debug` (which is good because reusing this is less agonizing)
    #[inline]
    pub(crate) fn format(shard: u64, realm: u64, num: u64) -> Buffer {
        let mut buf = arrayvec::ArrayString::<MAX_CAPACITY>::new();

        buf.push_str(itoa::Buffer::new().format(shard));
        buf.push('.');
        buf.push_str(itoa::Buffer::new().format(realm));
        buf.push('.');
        buf.push_str(itoa::Buffer::new().format(num));

        Buffer(buf)
    }

    /// Formats an entity ID into a [`fmt::Formatter`] as [`fmt::Display`] would.
    ///
    /// Specifically the format is `{shard}.{realm}.{num}`
    pub(crate) fn display(
        f: &mut fmt::Formatter<'_>,
        shard: u64,
        realm: u64,
        num: u64,
    ) -> fmt::Result {
        let buf = format(shard, realm, num);

        f.write_str(buf.0.as_str())?;

        Ok(())
    }

    /// formats a `shard`, `realm`, and `num` into the format `{shard}.{realm}.{num}` avoiding the format machinery.
    ///
    /// # Performance
    ///
    /// This has one exact-sized allocation,
    /// and is probably at the limit for string formatting performance unless a better [`itoa`] algorithm is found.
    ///
    /// If you plan on using the string directly without passing it around, prefer [`format`].
    pub(crate) fn to_string(shard: u64, realm: u64, num: u64) -> String {
        // these need to be locals because we need to borrow them.
        let [mut shard_buf, mut realm_buf, mut num_buf] =
            [itoa::Buffer::new(), itoa::Buffer::new(), itoa::Buffer::new()];

        // we store these as locals so that we can count the length to avoid both reallocs and wasting space.
        let shard = shard_buf.format(shard);
        let realm = realm_buf.format(realm);
        let num = num_buf.format(num);

        // 2 separators of 1 byte each.
        let mut buf = String::with_capacity(shard.len() + realm.len() + num.len() + 1 * 2);

        buf.push_str(shard);
        buf.push('.');
        buf.push_str(realm);
        buf.push('.');
        buf.push_str(num);

        buf
    }
}

pub trait ValidateChecksums {
    /// Validates all entity-id checksums for `self` with the given ledger-id.
    ///
    /// # Errors
    /// - [`Error::BadEntityId`] if any of the expected checksums don't match the actual checksums.
    fn validate_checksums(&self, ledger_id: &LedgerId) -> crate::Result<()>;
}

impl<T: ValidateChecksums> ValidateChecksums for Option<T> {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> crate::Result<()> {
        if let Some(id) = &self {
            id.validate_checksums(ledger_id)?;
        }
        Ok(())
    }
}

/// The ID of an entity on the Hedera network.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "ffi", derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
pub struct EntityId {
    /// A non-negative number identifying the shard containing this entity.
    pub shard: u64,

    /// A non-negative number identifying the realm within the shard containing this entity.
    pub realm: u64,

    /// A non-negative number identifying the entity within the realm containing this entity.
    pub num: u64,

    /// A checksum if the entity ID was read from a user inputted string which included a checksum
    pub checksum: Option<Checksum>,
}

#[derive(Copy, Clone)]
pub(crate) enum PartialEntityId<'a> {
    ShortNum(u64),
    LongNum(EntityId),
    ShortOther(&'a str),
    LongOther { shard: u64, realm: u64, last: &'a str },
}

impl<'a> PartialEntityId<'a> {
    pub(crate) fn finish<T>(self) -> crate::Result<T>
    where
        EntityId: Into<T>,
    {
        match self {
            Self::ShortNum(num) => Ok(EntityId::from(num).into()),
            Self::LongNum(id) => Ok(id.into()),
            _ => Err(Error::basic_parse("expected `<shard>.<realm>.<num>`".to_owned())),
        }
    }

    // `FromStr` doesn't allow lifetime bounds.
    pub(crate) fn from_str(s: &'a str) -> crate::Result<Self> {
        let expecting =
            || Error::basic_parse(format!("expected `<shard>.<realm>.<num>`, got `{s}`"));

        // entity ID parsing is painful because there are 4 formats...
        // This way avoids allocations at the code of an extra layer of nesting.
        match s.split_once('.') {
            Some((shard, rest)) => {
                let (realm, last) = rest.split_once('.').ok_or_else(expecting)?;

                let shard = shard.parse().map_err(|_| expecting())?;
                let realm = realm.parse().map_err(|_| expecting())?;

                match last.rsplit_once('-') {
                    Some((num, checksum)) => {
                        let num = num.parse().map_err(|_| expecting())?;
                        let checksum = Some(checksum.parse()?);

                        Ok(Self::LongNum(EntityId { shard, realm, num, checksum }))
                    }

                    None => match last.parse() {
                        Ok(num) => {
                            Ok(Self::LongNum(EntityId { shard, realm, num, checksum: None }))
                        }

                        Err(_) => Ok(Self::LongOther { shard, realm, last }),
                    },
                }
            }
            None => match s.parse() {
                Ok(it) => return Ok(Self::ShortNum(it)),
                Err(_) => return Ok(Self::ShortOther(s)),
            },
        }
    }
}

impl EntityId {
    pub(crate) fn new(shard: u64, realm: u64, num: u64) -> Self {
        Self { shard, realm, num, checksum: None }
    }

    /// Parse an entity ID from a solidity address
    ///
    /// # Errors
    /// - [`Error::BasicParse`] if `address` cannot be parsed as a solidity address.
    pub(crate) fn from_solidity_address(address: &str) -> crate::Result<Self> {
        IdEvmAddress::from_str(address).map(Self::from)
    }

    /// Convert `self` into a solidity `address`.
    ///
    /// # Errors
    /// - [`Error::BasicParse`] if `self.shard` is larger than `u32::MAX`.
    pub(crate) fn to_solidity_address(self) -> crate::Result<String> {
        IdEvmAddress::try_from(self).map(|it| it.to_string())
    }

    pub(crate) fn generate_checksum(&self, ledger_id: &LedgerId) -> Checksum {
        Checksum::generate(self, ledger_id.as_ref())
    }

    /// Validates that the the checksum computed for the given `shard.realm.num` matches the given checksum.
    ///
    /// # Errors
    /// - [`Error::CannotPerformTaskWithoutLedgerId`] if the client has no `ledger_id`.
    /// - [`Error::BadEntityId`] if there is a checksum, and the checksum is not valid for the client's `ledger_id`.
    pub(crate) fn validate_checksum(
        shard: u64,
        realm: u64,
        num: u64,
        checksum: Option<Checksum>,
        client: &Client,
    ) -> crate::Result<()> {
        if let Some(present_checksum) = checksum {
            let ledger_id = client.ledger_id_internal();
            let ledger_id = ledger_id
                .as_deref()
                .ok_or(Error::CannotPerformTaskWithoutLedgerId { task: "validate checksum" })?;

            Self::validate_checksum_internal(shard, realm, num, present_checksum, ledger_id)
        } else {
            Ok(())
        }
    }

    pub(crate) fn validate_checksum_for_ledger_id(
        shard: u64,
        realm: u64,
        num: u64,
        checksum: Option<Checksum>,
        ledger_id: &LedgerId,
    ) -> Result<(), Error> {
        if let Some(present_checksum) = checksum {
            Self::validate_checksum_internal(shard, realm, num, present_checksum, ledger_id)
        } else {
            Ok(())
        }
    }

    fn validate_checksum_internal(
        shard: u64,
        realm: u64,
        num: u64,
        present_checksum: Checksum,
        ledger_id: &LedgerId,
    ) -> Result<(), Error> {
        let expected_checksum =
            Self { shard, realm, num, checksum: None }.generate_checksum(ledger_id);

        if present_checksum == expected_checksum {
            Ok(())
        } else {
            Err(Error::BadEntityId { shard, realm, num, present_checksum, expected_checksum })
        }
    }

    pub(crate) fn to_string_with_checksum(&self, client: &Client) -> crate::Result<String> {
        if let Some(ledger_id) = &*client.ledger_id_internal() {
            let checksum = self.generate_checksum(ledger_id);

            let mut entity_id_string = format::to_string(self.shard, self.realm, self.num);

            // no need to overallocate, we have exactly `-abcde`.
            // sadly this is guaranteed to realloc :/
            entity_id_string.reserve_exact(6);

            entity_id_string.push('-');
            entity_id_string.push_str(checksum.as_str());

            Ok(entity_id_string)
        } else {
            Err(Error::CannotPerformTaskWithoutLedgerId { task: "derive checksum for entity ID" })
        }
    }
}

impl Debug for EntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format::display(f, self.shard, self.realm, self.num)
    }
}

impl From<u64> for EntityId {
    fn from(num: u64) -> Self {
        Self { num, shard: 0, realm: 0, checksum: None }
    }
}

impl FromStr for EntityId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PartialEntityId::from_str(s)?.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        EntityId,
        LedgerId,
    };

    #[test]
    fn from_solidity_address() {
        assert_eq!(
            EntityId::from_solidity_address("000000000000000000000000000000000000138D").unwrap(),
            EntityId { shard: 0, realm: 0, num: 5005, checksum: None }
        );
    }

    #[test]
    fn from_solidity_address_with_0x() {
        assert_eq!(
            EntityId::from_solidity_address("0x000000000000000000000000000000000000138D").unwrap(),
            EntityId { shard: 0, realm: 0, num: 5005, checksum: None }
        );
    }

    #[test]
    fn to_solidity_address() {
        assert!(EntityId { shard: 0, realm: 0, num: 5005, checksum: None }
            .to_solidity_address()
            .unwrap()
            .eq_ignore_ascii_case("000000000000000000000000000000000000138D"));
    }

    #[test]
    fn generate_checksum_mainnet() {
        const EXPECTED: [&str; 256] = [
            "uvnqa", "dfkxr", "lpifi", "tzfmz", "cjcuq", "ktach", "tcxjy", "bmurp", "jwrzg",
            "sgpgx", "hiafh", "rdxmy", "uuuup", "eqscg", "ompjx", "yimro", "iejzf", "sahgw",
            "bweon", "lsbwe", "diuio", "nerqf", "qvoxw", "armfn", "knjne", "ujguv", "efecm",
            "obbkd", "xwyru", "hsvzl", "zjolv", "jfltm", "mwjbd", "wsgiu", "godql", "qkayc",
            "afyft", "kbvnk", "txsvb", "dtqcs", "vkipc", "fgfwt", "ixdek", "stamb", "coxts",
            "mkvbj", "wgsja", "gcpqr", "pymyi", "zukfz", "rlcsj", "tqaaa", "xgxhr", "hcupi",
            "qyrwz", "aupeq", "kqmmh", "umjty", "eihbp", "oeejg", "fuwvq", "pqudh", "thrky",
            "ddosp", "mzmag", "wvjhx", "grgpo", "qndxf", "ajbew", "keymn", "bvqyx", "lrogo",
            "pilof", "zeivw", "jagdn", "swdle", "csasv", "mnyam", "wjvid", "gfspu", "xwlce",
            "hsijv", "ljfrm", "vfczd", "fbagu", "owxol", "ysuwc", "iosdt", "skplk", "cgmtb",
            "txffl", "dtcnc", "hjzut", "rfxck", "bbukb", "kxrrs", "utozj", "epmha", "oljor",
            "yhgwi", "hhghj", "prdpa", "ybawr", "gkyei", "ouvlz", "xestq", "foqbh", "nyniy",
            "wikqp", "eshyg", "euakq", "ndxsh", "vnuzy", "dxshp", "mhppg", "urmwx", "dbkeo",
            "llhmf", "tvetw", "cfcbn", "wbunx", "elrvo", "mvpdf", "vfmkw", "dpjsn", "lzhae",
            "ujehv", "ctbpm", "lcyxd", "tmweu", "toore", "bylyv", "kijgm", "ssgod", "bcdvu",
            "jmbdl", "rvylc", "afvst", "iptak", "qzqib", "rbiul", "zlgcc", "hvdjt", "qfark",
            "yoxzb", "gyvgs", "pisoj", "xspwa", "gcndr", "omkli", "oocxs", "wyafj", "fhxna",
            "nruur", "wbsci", "elpjz", "mvmrq", "vfjzh", "dphgy", "lzeop", "maxaz", "ukuiq",
            "curqh", "leoxy", "tomfp", "byjng", "kigux", "sseco", "bcbkf", "jlyrw", "jnreg",
            "rxolx", "ahlto", "irjbf", "rbgiw", "zldqn", "hvaye", "qeyfv", "yovnm", "gysvd",
            "halhn", "pkipe", "xufwv", "gedem", "ooamd", "wxxtu", "fhvbl", "nrsjc", "wbpqt",
            "elmyk", "enfku", "mxcsl", "vhaac", "dqxht", "maupk", "ukrxb", "cupes", "lemmj",
            "tojua", "byhbr", "klges", "svdmj", "bfaua", "joybr", "ryvji", "aisqz", "ispyq",
            "rcngh", "zmkny", "rthvp", "hyahz", "qhxpq", "yruxh", "hbsey", "plpmp", "xvmug",
            "gfkbx", "ophjo", "wzerf", "pgbyw", "zfulg", "hprsx", "pzpao", "yjmif", "gtjpw",
            "pdgxn", "xnefe", "fxbmv", "ogyum", "gnwcd", "wsoon", "fclwe", "nmjdv", "vwglm",
            "egdtd", "mqbau", "uzyil", "djvqc", "ltsxt", "eaqfk", "ufiru", "cpfzl", "kzdhc",
            "tjaot", "bsxwk", "kcveb", "smsls", "awptj", "jgnba", "bnkir", "rscvb", "acacs",
            "ilxkj", "qvusa", "zfrzr", "hpphi",
        ];

        for (index, expected) in EXPECTED.iter().enumerate() {
            let actual =
                EntityId::from(index as u64).generate_checksum(&LedgerId::mainnet()).to_string();

            assert_eq!(expected, &actual);
        }
    }

    #[test]
    fn generate_checksum_testnet() {
        const EXPECTED: [&str; 256] = [
            "eiyxj", "mswfa", "vctmr", "dmqui", "lwobz", "ugljq", "cqirh", "lafyy", "tkdgp",
            "buaog", "qvlmq", "ariuh", "eigby", "oedjp", "yaarg", "hvxyx", "rrvgo", "bnsof",
            "ljpvw", "vfndn", "mwfpx", "wscxo", "ajaff", "kexmw", "uauun", "dwsce", "nspjv",
            "xomrm", "hkjzd", "rghgu", "iwzte", "ssxav", "wjuim", "gfrqd", "qboxu", "zxmfl",
            "jtjnc", "tpgut", "dleck", "nhbkb", "extwl", "otrec", "skolt", "cgltk", "mcjbb",
            "vygis", "fudqj", "pqaya", "zlyfr", "jhvni", "aynzs", "ddlhj", "guipa", "qqfwr",
            "amdei", "kialz", "udxtq", "dzvbh", "nvsiy", "xrpqp", "piicz", "zefkq", "cvcsh",
            "mqzzy", "wmxhp", "giupg", "qerwx", "aapeo", "jwmmf", "tsjtw", "ljcgg", "veznx",
            "yvwvo", "irudf", "snrkw", "cjosn", "mfmae", "wbjhv", "fxgpm", "ptdxd", "hjwjn",
            "rftre", "uwqyv", "esogm", "oolod", "ykivu", "iggdl", "scdlc", "byast", "ltyak",
            "dkqmu", "ngnul", "qxlcc", "atijt", "kpfrk", "ulczb", "ehags", "ocxoj", "xyuwa",
            "husdr", "quros", "zeowj", "homea", "pyjlr", "yigti", "gseaz", "pcbiq", "xlyqh",
            "fvvxy", "oftfp", "ohlrz", "wrizq", "fbghh", "nldoy", "vvawp", "eeyeg", "movlx",
            "uysto", "diqbf", "lsniw", "fpfvg", "nzdcx", "wjako", "esxsf", "ncuzw", "vmshn",
            "dwppe", "mgmwv", "uqkem", "dahmd", "dbzyn", "llxge", "tvunv", "cfrvm", "kppdd",
            "szmku", "bjjsl", "jthac", "sdeht", "anbpk", "aoubu", "iyrjl", "riorc", "zslyt",
            "icjgk", "qmgob", "ywdvs", "hgbdj", "ppyla", "xzvsr", "ybofb", "gllms", "oviuj",
            "xfgca", "fpdjr", "nzari", "wixyz", "esvgq", "ncsoh", "vmpvy", "voiii", "dyfpz",
            "micxq", "usafh", "dbxmy", "lluup", "tvscg", "cfpjx", "kpmro", "szjzf", "tbclp",
            "bkztg", "juxax", "seuio", "aorqf", "iyoxw", "rimfn", "zsjne", "icguv", "qmecm",
            "qnwow", "yxtwn", "hhree", "prolv", "ybltm", "gljbd", "ovgiu", "xfdql", "fpayc",
            "nyyft", "oaqsd", "wknzu", "eulhl", "neipc", "vofwt", "dydek", "miamb", "urxts",
            "dbvbj", "llsja", "tyrmb", "ciots", "ksmbj", "tcjja", "bmgqr", "jwdyi", "sgbfz",
            "apynq", "izvvh", "bgtcy", "rllpi", "zviwz", "ifgeq", "qpdmh", "yzaty", "hiybp",
            "psvjg", "ycsqx", "gmpyo", "ytngf", "itfsp", "rddag", "znahx", "hwxpo", "qguxf",
            "yqsew", "hapmn", "pkmue", "xukbv", "qbhjm", "gfzvw", "opxdn", "wzule", "fjrsv",
            "ntpam", "wdmid", "enjpu", "mxgxl", "vhefc", "nobmt", "dstzd", "mcrgu", "umool",
            "cwlwc", "lgjdt", "tqglk", "cadtb", "kkbas", "styij", "lavqa", "bfock", "jplkb",
            "rzirs", "ajfzj", "itdha", "rdaor",
        ];

        for (index, expected) in EXPECTED.iter().enumerate() {
            let actual =
                EntityId::from(index as u64).generate_checksum(&LedgerId::testnet()).to_string();

            assert_eq!(expected, &actual);
        }
    }
}
