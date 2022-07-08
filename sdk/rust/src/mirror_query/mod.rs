mod any;
mod subscribe;

pub(crate) use any::AnyMirrorQueryData;
pub use any::{
    AnyMirrorQuery,
    AnyMirrorQueryResponse,
};
pub(crate) use subscribe::MirrorQuerySubscribe;

/// A query that can be executed on the Hedera mirror network.
#[derive(Clone, Debug, Default)]
pub struct MirrorQuery<D>
where
    D: MirrorQuerySubscribe,
{
    pub(crate) data: D,
    // TODO: request_timeout
}

impl<D> MirrorQuery<D>
where
    D: MirrorQuerySubscribe + Default,
{
    /// Create a new query ready for configuration and execution.
    pub fn new() -> Self {
        Self::default()
    }
}
