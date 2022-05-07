/// Get an optional field from a protobuf object, returning an error if the field does not exist.
macro_rules! pb_getf {
    ($expr:expr, $field:ident, $source:literal) => {
        $expr.$field.ok_or_else(|| {
            $crate::Error::from_protobuf(concat!(
                "unexpected missing `",
                stringify!($field),
                "` in `",
                $source,
                "`"
            ))
        })
    };

    ($expr:expr, $field:ident, $field_name:literal, $source:literal) => {
        $expr.$field.ok_or_else(|| {
            $crate::Error::from_protobuf(concat!(
                "unexpected missing `",
                $field_name,
                "` in `",
                $source,
                "`"
            ))
        })
    };
}

/// Get a specific variant from a `oneof` field of a protobuf object, returning an error
/// if the variant is not as expected.
macro_rules! pb_getv {
    ($expr:expr, $variant:ident, $ty:ty) => {{
        use $ty::*;

        match $expr {
            $variant(it) => it,

            _ => {
                return Err($crate::Error::from_protobuf(format!(
                    concat!("unexpected {:?} received, expecting `", stringify!($variant), "`"),
                    $expr
                )));
            }
        }
    }};
}
