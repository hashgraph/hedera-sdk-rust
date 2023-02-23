use futures_core::Future;
use tokio::time::sleep;

pub(crate) enum Error {
    /// An error that may be resolved after backoff is applied (connection issues for example)
    Transient(crate::Error),

    /// An error that *cannot* be resolved.
    Permanent(crate::Error),

    /// A transient error with no associated error (happens when there are no healthy nodes)
    EmptyTransient,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Durably retry some function according to the `backoff` until the backoff expires.
pub(crate) async fn retry<B, Fn, O, Fut>(mut backoff: B, mut f: Fn) -> crate::Result<O>
where
    B: backoff::backoff::Backoff,
    Fn: FnMut() -> Fut,
    Fut: Future<Output = Result<O>>,
{
    let mut last_error: Option<crate::Error> = None;
    loop {
        match f().await {
            Ok(it) => return Ok(it),
            Err(Error::Transient(e)) => last_error = Some(e),
            Err(Error::EmptyTransient) => {}
            Err(Error::Permanent(e)) => return Err(e),
        }

        if let Some(duration) = backoff.next_backoff() {
            sleep(duration).await;
        } else {
            let last_error = last_error.expect("timeout while network had no healthy nodes");
            return Err(crate::Error::TimedOut(last_error.into()));
        }
    }
}
