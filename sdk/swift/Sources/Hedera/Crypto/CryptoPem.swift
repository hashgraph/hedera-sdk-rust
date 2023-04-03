import Foundation

extension Crypto {
    internal enum Pem {}
}

extension Crypto.Pem {
    private static func isValidLabelCharacter(_ char: Character) -> Bool {
        let visibleAscii: ClosedRange<UInt8> = 0x21...0x7e
        let hyphenMinus: Character = "-"

        return char != hyphenMinus && (char.asciiValue.map(visibleAscii.contains)) ?? false
    }

    private static let endOfLabel: String = "-----"
    private static let beginLabel: String = "-----BEGIN "
    private static let endLabel: String = "-----END "

    internal struct Document {
        internal let typeLabel: String
        internal let der: Data
    }

    // todo: use data instead of string
    internal static func decode(_ message: String) throws -> Document {
        let message = message.components(separatedBy: .newlines)

        guard let (typeLabel, message) = message.splitFirst(),
            let typeLabel = typeLabel.stripPrefix(beginLabel),
            let typeLabel = typeLabel.stripSuffix(endOfLabel)
        else {
            throw HError.keyParse("Invalid Pem")
        }

        guard typeLabel.allSatisfy({ isValidLabelCharacter($0) || $0 == " " }), typeLabel.last != " " else {
            throw HError.keyParse("Invalid Pem")
        }

        guard let (end, message) = message.splitLast(),
            let end = end.stripPrefix(endLabel),
            let end = end.stripSuffix(endOfLabel),
            typeLabel == end
        else {
            throw HError.keyParse("Invalid Pem")
        }

        let (base64Final, base64Lines) = message.splitLast() ?? ("", [])

        var base64Message: String = ""

        for line in base64Lines {
            guard line.count == 64 else {
                throw HError.keyParse("Invalid Pem")
            }

            base64Message += line
        }

        guard base64Final.count <= 64 else {
            throw HError.keyParse("Invalid Pem")
        }

        base64Message += base64Final

        // fixme: ensure that `+/` are the characterset used.
        guard let message = Data(base64Encoded: base64Message) else {
            throw HError.keyParse("Invalid Pem")
        }

        return Document(typeLabel: String(typeLabel), der: message)
    }
}
