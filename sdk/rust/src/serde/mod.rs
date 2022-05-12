pub(crate) mod duration;
pub(crate) mod duration_opt;

pub(crate) fn skip_if_string_empty(s: &str) -> bool {
    s.is_empty()
}
