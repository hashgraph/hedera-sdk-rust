use once_cell::sync::Lazy;
use tokio::runtime;
use tokio::runtime::Runtime;

pub(super) static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread().enable_all().max_blocking_threads(8).build().unwrap()
});
