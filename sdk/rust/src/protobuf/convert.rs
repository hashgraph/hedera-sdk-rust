pub trait ToProtobuf: Send + Sync {
    type Protobuf;

    fn to_protobuf(&self) -> Self::Protobuf;
}

pub trait FromProtobuf<Protobuf> {
    fn from_protobuf(pb: Protobuf) -> crate::Result<Self>
    where
        Self: Sized;
}

impl<T, P> FromProtobuf<Vec<P>> for Vec<T>
where
    T: FromProtobuf<P>,
{
    fn from_protobuf(pb: Vec<P>) -> crate::Result<Self>
    where
        Self: Sized,
    {
        pb.into_iter().map(T::from_protobuf).collect::<crate::Result<Vec<_>>>()
    }
}
