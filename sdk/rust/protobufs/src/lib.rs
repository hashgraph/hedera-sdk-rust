#![allow(non_camel_case_types)]
#![allow(clippy::default_trait_access, clippy::doc_markdown)]

#[cfg(feature = "chrono_0_4")]
mod chrono_0_4;

#[cfg(feature = "time_0_2")]
mod time_0_2;

#[cfg(feature = "time_0_3")]
mod time_0_3;

#[cfg(feature = "fraction")]
mod fraction;

pub mod services {
    tonic::include_proto!("proto");
}

pub mod mirror {
    tonic::include_proto!("mirror/com.hedera.mirror.api.proto");
}

pub mod streams {
    tonic::include_proto!("streams/proto");
}
