use std::str::FromStr;

use itertools::Itertools;

use crate::Error;

pub(crate) fn parse(s: &str) -> crate::Result<(u64, u64, u64)> {
    let parts: Vec<u64> =
        s.splitn(3, '.').map(u64::from_str).try_collect().map_err(Error::basic_parse)?;

    if parts.len() == 1 {
        Ok((0, 0, parts[0]))
    } else if parts.len() == 3 {
        Ok((parts[0], parts[1], parts[2]))
    } else {
        Err(Error::basic_parse("expecting <shard>.<realm>.<num> (ex. `0.0.1001`)"))
    }
}
