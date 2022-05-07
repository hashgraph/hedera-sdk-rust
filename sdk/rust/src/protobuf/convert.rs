use prost::Message;

pub trait ToProtobuf: Send + Sync {
    type Protobuf: Message;

    fn to_protobuf(&self) -> Self::Protobuf;
}

pub trait FromProtobuf {
    type Protobuf: Message;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized;
}
