pub trait ToProtobuf: Send + Sync {
    type Protobuf;

    fn to_protobuf(&self) -> Self::Protobuf;
}

pub trait FromProtobuf {
    type Protobuf;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized;
}
