use std::fmt;
use std::str::FromStr;

use tinystr::TinyAsciiStr;

use crate::ledger_id::RefLedgerId;
use crate::{
    EntityId,
    Error,
};

/// A checksum for an entity ID.
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Checksum(TinyAsciiStr<5>);

impl Checksum {
    /// Initialize a checksum for ascii-lowercase letters.
    ///
    /// # Panics
    /// May panic if `bytes` is not valid ascii-lowercase letters.
    const fn from_bytes(bytes: [u8; 5]) -> Self {
        let checksum = match TinyAsciiStr::from_bytes(&bytes) {
            Ok(it) => it,
            // I know this should format the error... Buuut.
            Err(_) => panic!("called `Result::unwrap()` on a `Err` value"),
        };

        // use debug assert here because we control the only function that should be calling this (in this very module).
        debug_assert!(checksum.is_ascii_alphabetic_lowercase());

        Self(checksum)
    }

    pub(crate) const fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(feature = "ffi")]
    pub(crate) const fn to_bytes(self) -> [u8; 5] {
        *self.0.all_bytes()
    }

    // I don't *like* passing a 32 byte struct where a 24 one would do, but, eh.
    pub(crate) fn generate(entity_id: &EntityId, ledger_id: &RefLedgerId) -> Self {
        // 3 digits in base 26
        const DIGITS_3: usize = 26_usize.pow(3);
        // 5 digits in base 26
        const DIGITS_5: usize = 26_usize.pow(6);

        // min prime greater than a million. Used for the final permutation.
        const M: usize = 1_000_003;

        // Sum s of digit values weights them by powers of W. Should be coprime to P5.
        const WEIGHT: usize = 31;
        const BYTES: &[u8] = &[0; 6];

        let h = [ledger_id.as_bytes(), BYTES].concat();

        let entity_id_string =
            crate::entity_id::format::format(entity_id.shard, entity_id.realm, entity_id.num);

        // Digits with 10 for ".", so if addr == "0.0.123" then d == [0, 10, 0, 10, 1, 2, 3]
        let d = entity_id_string.as_str().chars().map(|c| {
            if c == '.' {
                10_usize
            } else {
                c.to_digit(10).unwrap() as usize
            }
        });

        // Weighted sum of all positions (mod P3)
        let mut s = 0;

        // Sum of even positions (mod 11)
        let mut sum_even = 0;
        // Sum of odd positions (mod 11)
        let mut sum_odd = 0;
        for (i, digit) in d.enumerate() {
            s = (WEIGHT * s + digit) % DIGITS_3;

            if i % 2 == 0 {
                sum_even = (sum_even + digit) % 11;
            } else {
                sum_odd = (sum_odd + digit) % 11;
            }
        }

        let mut sh = 0; // Hash of the ledger ID
        for b in h {
            sh = (WEIGHT * sh + (b as usize)) % DIGITS_5;
        }

        // The checksum, as a single number
        let c = entity_id_string.as_str().len() % 5;
        let c = c * 11 + sum_even;
        let c = c * 11 + sum_odd;
        let c = c * DIGITS_3 + s + sh;
        let c = c % DIGITS_5;
        let mut c = (c * M) % DIGITS_5;

        let mut answer = [0_u8; 5];
        for i in (0..5).rev() {
            answer[i] = b'a' + ((c % 26) as u8);
            c /= 26;
        }

        Self::from_bytes(answer)
    }
}

impl FromStr for Checksum {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Checksum)
            .map_err(|_| Error::basic_parse("Expected checksum to be exactly 5 characters"))
    }
}

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}
