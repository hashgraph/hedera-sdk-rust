pub trait ToProtobuf: Send + Sync {
    type Protobuf;

    fn to_protobuf(&self) -> Self::Protobuf;
}

pub trait FromProtobuf<Protobuf> {
    fn from_protobuf(pb: Protobuf) -> crate::Result<Self>
    where
        Self: Sized;
}
