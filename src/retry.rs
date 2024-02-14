use futures_core::Future;
use tokio::time::sleep;

#[derive(Debug)]
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
pub(crate) async fn retry<B, Fn, O, Fut>(
    mut backoff: B,
    max_attempts: Option<usize>,
    mut f: Fn,
) -> crate::Result<O>
where
    B: backoff::backoff::Backoff + Send,
    Fn: FnMut() -> Fut + Send,
    Fut: Future<Output = Result<O>> + Send,
{
    let mut last_error: Option<crate::Error> = None;
    let mut attempt_number = 0;

    while max_attempts.map_or(true, |it| attempt_number < it) {
        attempt_number += 1;

        match f().await {
            Ok(it) => return Ok(it),
            Err(Error::Transient(e)) => last_error = Some(e),
            Err(Error::EmptyTransient) => {}
            Err(Error::Permanent(e)) => return Err(e),
        }

        if let Some(duration) = backoff.next_backoff() {
            let duration_ms = duration.as_millis();
            let err_suffix =
                last_error.as_ref().map(|l| format!(" due to {l:?}")).unwrap_or_default();

            log::warn!("Backing off for {duration_ms}ms after failure of attempt {attempt_number}{err_suffix}");
            sleep(duration).await;
            log::warn!("Backed off for {duration_ms}ms after failure of attempt {attempt_number}{err_suffix}");
        } else {
            let last_error = last_error.expect("timeout while network had no healthy nodes");
            return Err(crate::Error::TimedOut(last_error.into()));
        }
    }

    let last_error = last_error.expect("timeout while network had no healthy nodes");
    Err(crate::Error::TimedOut(last_error.into()))
}
