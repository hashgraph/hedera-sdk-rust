import HederaProtobufs
import NumberKit

extension Rational: ProtobufCodable where T == UInt64 {
    internal typealias Protobuf = Proto_Fraction

    internal init(protobuf proto: Protobuf) {
        self.init(UInt64(proto.numerator), UInt64(proto.denominator))
    }

    internal func toProtobuf() -> Protobuf {
        .with { proto in
            proto.numerator = Int64(self.numerator)
            proto.denominator = Int64(self.denominator)
        }
    }
}
