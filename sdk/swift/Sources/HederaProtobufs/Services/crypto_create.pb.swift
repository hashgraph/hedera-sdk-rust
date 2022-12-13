// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: crypto_create.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that you are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

///
/// Create a new account. After the account is created, the AccountID for it is in the receipt. It
/// can also be retrieved with a GetByKey query. Threshold values can be defined, and records are
/// generated and stored for 25 hours for any transfer that exceeds the thresholds. This account is
/// charged for each record generated, so the thresholds are useful for limiting record generation to
/// happen only for large transactions.
///
/// The Key field is the key used to sign transactions for this account. If the account has
/// receiverSigRequired set to true, then all cryptocurrency transfers must be signed by this
/// account's key, both for transfers in and out. If it is false, then only transfers out have to be
/// signed by it. When the account is created, the payer account is charged enough hbars so that the
/// new account will not expire for the next autoRenewPeriod seconds. When it reaches the expiration
/// time, the new account will then be automatically charged to renew for another autoRenewPeriod
/// seconds. If it does not have enough hbars to renew for that long, then the remaining hbars are
/// used to extend its expiration as long as possible. If it is has a zero balance when it expires,
/// then it is deleted. This transaction must be signed by the payer account. If receiverSigRequired
/// is false, then the transaction does not have to be signed by the keys in the keys field. If it is
/// true, then it must be signed by them, in addition to the keys of the payer account. If the
/// auto_renew_account field is set, the key of the referenced account must sign.
///
/// An entity (account, file, or smart contract instance) must be created in a particular realm. If
/// the realmID is left null, then a new realm will be created with the given admin key. If a new
/// realm has a null adminKey, then anyone can create/modify/delete entities in that realm. But if an
/// admin key is given, then any transaction to create/modify/delete an entity in that realm must be
/// signed by that key, though anyone can still call functions on smart contract instances that exist
/// in that realm. A realm ceases to exist when everything within it has expired and no longer
/// exists.
///
/// The current API ignores shardID, realmID, and newRealmAdminKey, and creates everything in shard 0
/// and realm 0, with a null key. Future versions of the API will support multiple realms and
/// multiple shards.
public struct Proto_CryptoCreateTransactionBody {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  ///*
  /// The key that must sign each transfer out of the account. If receiverSigRequired is true, then
  /// it must also sign any transfer into the account.
  public var key: Proto_Key {
    get {return _storage._key ?? Proto_Key()}
    set {_uniqueStorage()._key = newValue}
  }
  /// Returns true if `key` has been explicitly set.
  public var hasKey: Bool {return _storage._key != nil}
  /// Clears the value of `key`. Subsequent reads from it will return its default value.
  public mutating func clearKey() {_uniqueStorage()._key = nil}

  ///*
  /// The initial number of tinybars to put into the account
  public var initialBalance: UInt64 {
    get {return _storage._initialBalance}
    set {_uniqueStorage()._initialBalance = newValue}
  }

  ///*
  /// [Deprecated] ID of the account to which this account is proxy staked. If proxyAccountID is null, or is an
  /// invalid account, or is an account that isn't a node, then this account is automatically proxy
  /// staked to a node chosen by the network, but without earning payments. If the proxyAccountID
  /// account refuses to accept proxy staking , or if it is not currently running a node, then it
  /// will behave as if proxyAccountID was null.
  public var proxyAccountID: Proto_AccountID {
    get {return _storage._proxyAccountID ?? Proto_AccountID()}
    set {_uniqueStorage()._proxyAccountID = newValue}
  }
  /// Returns true if `proxyAccountID` has been explicitly set.
  public var hasProxyAccountID: Bool {return _storage._proxyAccountID != nil}
  /// Clears the value of `proxyAccountID`. Subsequent reads from it will return its default value.
  public mutating func clearProxyAccountID() {_uniqueStorage()._proxyAccountID = nil}

  ///*
  /// [Deprecated]. The threshold amount (in tinybars) for which an account record is created for
  /// any send/withdraw transaction
  public var sendRecordThreshold: UInt64 {
    get {return _storage._sendRecordThreshold}
    set {_uniqueStorage()._sendRecordThreshold = newValue}
  }

  ///*
  /// [Deprecated]. The threshold amount (in tinybars) for which an account record is created for
  /// any receive/deposit transaction
  public var receiveRecordThreshold: UInt64 {
    get {return _storage._receiveRecordThreshold}
    set {_uniqueStorage()._receiveRecordThreshold = newValue}
  }

  ///*
  /// If true, this account's key must sign any transaction depositing into this account (in
  /// addition to all withdrawals)
  public var receiverSigRequired: Bool {
    get {return _storage._receiverSigRequired}
    set {_uniqueStorage()._receiverSigRequired = newValue}
  }

  ///*
  /// The account is charged to extend its expiration date every this many seconds. If it doesn't
  /// have enough balance, it extends as long as possible. If it is empty when it expires, then it
  /// is deleted.
  public var autoRenewPeriod: Proto_Duration {
    get {return _storage._autoRenewPeriod ?? Proto_Duration()}
    set {_uniqueStorage()._autoRenewPeriod = newValue}
  }
  /// Returns true if `autoRenewPeriod` has been explicitly set.
  public var hasAutoRenewPeriod: Bool {return _storage._autoRenewPeriod != nil}
  /// Clears the value of `autoRenewPeriod`. Subsequent reads from it will return its default value.
  public mutating func clearAutoRenewPeriod() {_uniqueStorage()._autoRenewPeriod = nil}

  ///*
  /// The shard in which this account is created
  public var shardID: Proto_ShardID {
    get {return _storage._shardID ?? Proto_ShardID()}
    set {_uniqueStorage()._shardID = newValue}
  }
  /// Returns true if `shardID` has been explicitly set.
  public var hasShardID: Bool {return _storage._shardID != nil}
  /// Clears the value of `shardID`. Subsequent reads from it will return its default value.
  public mutating func clearShardID() {_uniqueStorage()._shardID = nil}

  ///*
  /// The realm in which this account is created (leave this null to create a new realm)
  public var realmID: Proto_RealmID {
    get {return _storage._realmID ?? Proto_RealmID()}
    set {_uniqueStorage()._realmID = newValue}
  }
  /// Returns true if `realmID` has been explicitly set.
  public var hasRealmID: Bool {return _storage._realmID != nil}
  /// Clears the value of `realmID`. Subsequent reads from it will return its default value.
  public mutating func clearRealmID() {_uniqueStorage()._realmID = nil}

  ///*
  /// If realmID is null, then this the admin key for the new realm that will be created
  public var newRealmAdminKey: Proto_Key {
    get {return _storage._newRealmAdminKey ?? Proto_Key()}
    set {_uniqueStorage()._newRealmAdminKey = newValue}
  }
  /// Returns true if `newRealmAdminKey` has been explicitly set.
  public var hasNewRealmAdminKey: Bool {return _storage._newRealmAdminKey != nil}
  /// Clears the value of `newRealmAdminKey`. Subsequent reads from it will return its default value.
  public mutating func clearNewRealmAdminKey() {_uniqueStorage()._newRealmAdminKey = nil}

  ///*
  /// The memo associated with the account (UTF-8 encoding max 100 bytes)
  public var memo: String {
    get {return _storage._memo}
    set {_uniqueStorage()._memo = newValue}
  }

  ///*
  /// The maximum number of tokens that an Account can be implicitly associated with. Defaults to 0
  /// and up to a maximum value of 1000.
  public var maxAutomaticTokenAssociations: Int32 {
    get {return _storage._maxAutomaticTokenAssociations}
    set {_uniqueStorage()._maxAutomaticTokenAssociations = newValue}
  }

  ///*
  /// ID of the account or node to which this account is staking.
  public var stakedID: OneOf_StakedID? {
    get {return _storage._stakedID}
    set {_uniqueStorage()._stakedID = newValue}
  }

  ///*
  /// ID of the account to which this account is staking.
  public var stakedAccountID: Proto_AccountID {
    get {
      if case .stakedAccountID(let v)? = _storage._stakedID {return v}
      return Proto_AccountID()
    }
    set {_uniqueStorage()._stakedID = .stakedAccountID(newValue)}
  }

  ///*
  /// ID of the node this account is staked to.
  public var stakedNodeID: Int64 {
    get {
      if case .stakedNodeID(let v)? = _storage._stakedID {return v}
      return 0
    }
    set {_uniqueStorage()._stakedID = .stakedNodeID(newValue)}
  }

  ///*
  /// If true, the account declines receiving a staking reward. The default value is false.
  public var declineReward: Bool {
    get {return _storage._declineReward}
    set {_uniqueStorage()._declineReward = newValue}
  }

  ///*
  /// The bytes to be used as the account's alias. It will be the
  /// serialization of a protobuf Key message for an ED25519/ECDSA_SECP256K1 primitive key type. Currently only primitive key bytes are
  /// supported as the key for an account with an alias. ThresholdKey, KeyList, ContractID, and
  /// delegatable_contract_id are not supported.
  ///
  /// A given alias can map to at most one account on the network at a time. This uniqueness will be enforced
  /// relative to aliases currently on the network at alias assignment.
  ///
  /// If a transaction creates an account using an alias, any further crypto transfers to that alias will
  /// simply be deposited in that account, without creating anything, and with no creation fee being charged.
  public var alias: Data {
    get {return _storage._alias}
    set {_uniqueStorage()._alias = newValue}
  }

  ///*
  /// An account to charge for auto-renewal of this account . If not set, or set to an
  /// account with zero hbar balance, the account's own hbar balance will be used to
  /// cover auto-renewal fees.
  public var autoRenewAccount: Proto_AccountID {
    get {return _storage._autoRenewAccount ?? Proto_AccountID()}
    set {_uniqueStorage()._autoRenewAccount = newValue}
  }
  /// Returns true if `autoRenewAccount` has been explicitly set.
  public var hasAutoRenewAccount: Bool {return _storage._autoRenewAccount != nil}
  /// Clears the value of `autoRenewAccount`. Subsequent reads from it will return its default value.
  public mutating func clearAutoRenewAccount() {_uniqueStorage()._autoRenewAccount = nil}

  ///*
  /// EOA 20-byte address to create that is derived from the keccak-256 hash of a ECDSA_SECP256K1 primitive key.
  public var evmAddress: Data {
    get {return _storage._evmAddress}
    set {_uniqueStorage()._evmAddress = newValue}
  }

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  ///*
  /// ID of the account or node to which this account is staking.
  public enum OneOf_StakedID: Equatable {
    ///*
    /// ID of the account to which this account is staking.
    case stakedAccountID(Proto_AccountID)
    ///*
    /// ID of the node this account is staked to.
    case stakedNodeID(Int64)

  #if !swift(>=4.1)
    public static func ==(lhs: Proto_CryptoCreateTransactionBody.OneOf_StakedID, rhs: Proto_CryptoCreateTransactionBody.OneOf_StakedID) -> Bool {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch (lhs, rhs) {
      case (.stakedAccountID, .stakedAccountID): return {
        guard case .stakedAccountID(let l) = lhs, case .stakedAccountID(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.stakedNodeID, .stakedNodeID): return {
        guard case .stakedNodeID(let l) = lhs, case .stakedNodeID(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      default: return false
      }
    }
  #endif
  }

  public init() {}

  fileprivate var _storage = _StorageClass.defaultInstance
}

#if swift(>=5.5) && canImport(_Concurrency)
extension Proto_CryptoCreateTransactionBody: @unchecked Sendable {}
extension Proto_CryptoCreateTransactionBody.OneOf_StakedID: @unchecked Sendable {}
#endif  // swift(>=5.5) && canImport(_Concurrency)

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "proto"

extension Proto_CryptoCreateTransactionBody: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".CryptoCreateTransactionBody"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "key"),
    2: .same(proto: "initialBalance"),
    3: .same(proto: "proxyAccountID"),
    6: .same(proto: "sendRecordThreshold"),
    7: .same(proto: "receiveRecordThreshold"),
    8: .same(proto: "receiverSigRequired"),
    9: .same(proto: "autoRenewPeriod"),
    10: .same(proto: "shardID"),
    11: .same(proto: "realmID"),
    12: .same(proto: "newRealmAdminKey"),
    13: .same(proto: "memo"),
    14: .standard(proto: "max_automatic_token_associations"),
    15: .standard(proto: "staked_account_id"),
    16: .standard(proto: "staked_node_id"),
    17: .standard(proto: "decline_reward"),
    18: .same(proto: "alias"),
    19: .standard(proto: "auto_renew_account"),
    20: .standard(proto: "evm_address"),
  ]

  fileprivate class _StorageClass {
    var _key: Proto_Key? = nil
    var _initialBalance: UInt64 = 0
    var _proxyAccountID: Proto_AccountID? = nil
    var _sendRecordThreshold: UInt64 = 0
    var _receiveRecordThreshold: UInt64 = 0
    var _receiverSigRequired: Bool = false
    var _autoRenewPeriod: Proto_Duration? = nil
    var _shardID: Proto_ShardID? = nil
    var _realmID: Proto_RealmID? = nil
    var _newRealmAdminKey: Proto_Key? = nil
    var _memo: String = String()
    var _maxAutomaticTokenAssociations: Int32 = 0
    var _stakedID: Proto_CryptoCreateTransactionBody.OneOf_StakedID?
    var _declineReward: Bool = false
    var _alias: Data = Data()
    var _autoRenewAccount: Proto_AccountID? = nil
    var _evmAddress: Data = Data()

    static let defaultInstance = _StorageClass()

    private init() {}

    init(copying source: _StorageClass) {
      _key = source._key
      _initialBalance = source._initialBalance
      _proxyAccountID = source._proxyAccountID
      _sendRecordThreshold = source._sendRecordThreshold
      _receiveRecordThreshold = source._receiveRecordThreshold
      _receiverSigRequired = source._receiverSigRequired
      _autoRenewPeriod = source._autoRenewPeriod
      _shardID = source._shardID
      _realmID = source._realmID
      _newRealmAdminKey = source._newRealmAdminKey
      _memo = source._memo
      _maxAutomaticTokenAssociations = source._maxAutomaticTokenAssociations
      _stakedID = source._stakedID
      _declineReward = source._declineReward
      _alias = source._alias
      _autoRenewAccount = source._autoRenewAccount
      _evmAddress = source._evmAddress
    }
  }

  fileprivate mutating func _uniqueStorage() -> _StorageClass {
    if !isKnownUniquelyReferenced(&_storage) {
      _storage = _StorageClass(copying: _storage)
    }
    return _storage
  }

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    _ = _uniqueStorage()
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      while let fieldNumber = try decoder.nextFieldNumber() {
        // The use of inline closures is to circumvent an issue where the compiler
        // allocates stack space for every case branch when no optimizations are
        // enabled. https://github.com/apple/swift-protobuf/issues/1034
        switch fieldNumber {
        case 1: try { try decoder.decodeSingularMessageField(value: &_storage._key) }()
        case 2: try { try decoder.decodeSingularUInt64Field(value: &_storage._initialBalance) }()
        case 3: try { try decoder.decodeSingularMessageField(value: &_storage._proxyAccountID) }()
        case 6: try { try decoder.decodeSingularUInt64Field(value: &_storage._sendRecordThreshold) }()
        case 7: try { try decoder.decodeSingularUInt64Field(value: &_storage._receiveRecordThreshold) }()
        case 8: try { try decoder.decodeSingularBoolField(value: &_storage._receiverSigRequired) }()
        case 9: try { try decoder.decodeSingularMessageField(value: &_storage._autoRenewPeriod) }()
        case 10: try { try decoder.decodeSingularMessageField(value: &_storage._shardID) }()
        case 11: try { try decoder.decodeSingularMessageField(value: &_storage._realmID) }()
        case 12: try { try decoder.decodeSingularMessageField(value: &_storage._newRealmAdminKey) }()
        case 13: try { try decoder.decodeSingularStringField(value: &_storage._memo) }()
        case 14: try { try decoder.decodeSingularInt32Field(value: &_storage._maxAutomaticTokenAssociations) }()
        case 15: try {
          var v: Proto_AccountID?
          var hadOneofValue = false
          if let current = _storage._stakedID {
            hadOneofValue = true
            if case .stakedAccountID(let m) = current {v = m}
          }
          try decoder.decodeSingularMessageField(value: &v)
          if let v = v {
            if hadOneofValue {try decoder.handleConflictingOneOf()}
            _storage._stakedID = .stakedAccountID(v)
          }
        }()
        case 16: try {
          var v: Int64?
          try decoder.decodeSingularInt64Field(value: &v)
          if let v = v {
            if _storage._stakedID != nil {try decoder.handleConflictingOneOf()}
            _storage._stakedID = .stakedNodeID(v)
          }
        }()
        case 17: try { try decoder.decodeSingularBoolField(value: &_storage._declineReward) }()
        case 18: try { try decoder.decodeSingularBytesField(value: &_storage._alias) }()
        case 19: try { try decoder.decodeSingularMessageField(value: &_storage._autoRenewAccount) }()
        case 20: try { try decoder.decodeSingularBytesField(value: &_storage._evmAddress) }()
        default: break
        }
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    try withExtendedLifetime(_storage) { (_storage: _StorageClass) in
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every if/case branch local when no optimizations
      // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
      // https://github.com/apple/swift-protobuf/issues/1182
      try { if let v = _storage._key {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 1)
      } }()
      if _storage._initialBalance != 0 {
        try visitor.visitSingularUInt64Field(value: _storage._initialBalance, fieldNumber: 2)
      }
      try { if let v = _storage._proxyAccountID {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 3)
      } }()
      if _storage._sendRecordThreshold != 0 {
        try visitor.visitSingularUInt64Field(value: _storage._sendRecordThreshold, fieldNumber: 6)
      }
      if _storage._receiveRecordThreshold != 0 {
        try visitor.visitSingularUInt64Field(value: _storage._receiveRecordThreshold, fieldNumber: 7)
      }
      if _storage._receiverSigRequired != false {
        try visitor.visitSingularBoolField(value: _storage._receiverSigRequired, fieldNumber: 8)
      }
      try { if let v = _storage._autoRenewPeriod {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 9)
      } }()
      try { if let v = _storage._shardID {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 10)
      } }()
      try { if let v = _storage._realmID {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 11)
      } }()
      try { if let v = _storage._newRealmAdminKey {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 12)
      } }()
      if !_storage._memo.isEmpty {
        try visitor.visitSingularStringField(value: _storage._memo, fieldNumber: 13)
      }
      if _storage._maxAutomaticTokenAssociations != 0 {
        try visitor.visitSingularInt32Field(value: _storage._maxAutomaticTokenAssociations, fieldNumber: 14)
      }
      switch _storage._stakedID {
      case .stakedAccountID?: try {
        guard case .stakedAccountID(let v)? = _storage._stakedID else { preconditionFailure() }
        try visitor.visitSingularMessageField(value: v, fieldNumber: 15)
      }()
      case .stakedNodeID?: try {
        guard case .stakedNodeID(let v)? = _storage._stakedID else { preconditionFailure() }
        try visitor.visitSingularInt64Field(value: v, fieldNumber: 16)
      }()
      case nil: break
      }
      if _storage._declineReward != false {
        try visitor.visitSingularBoolField(value: _storage._declineReward, fieldNumber: 17)
      }
      if !_storage._alias.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._alias, fieldNumber: 18)
      }
      try { if let v = _storage._autoRenewAccount {
        try visitor.visitSingularMessageField(value: v, fieldNumber: 19)
      } }()
      if !_storage._evmAddress.isEmpty {
        try visitor.visitSingularBytesField(value: _storage._evmAddress, fieldNumber: 20)
      }
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Proto_CryptoCreateTransactionBody, rhs: Proto_CryptoCreateTransactionBody) -> Bool {
    if lhs._storage !== rhs._storage {
      let storagesAreEqual: Bool = withExtendedLifetime((lhs._storage, rhs._storage)) { (_args: (_StorageClass, _StorageClass)) in
        let _storage = _args.0
        let rhs_storage = _args.1
        if _storage._key != rhs_storage._key {return false}
        if _storage._initialBalance != rhs_storage._initialBalance {return false}
        if _storage._proxyAccountID != rhs_storage._proxyAccountID {return false}
        if _storage._sendRecordThreshold != rhs_storage._sendRecordThreshold {return false}
        if _storage._receiveRecordThreshold != rhs_storage._receiveRecordThreshold {return false}
        if _storage._receiverSigRequired != rhs_storage._receiverSigRequired {return false}
        if _storage._autoRenewPeriod != rhs_storage._autoRenewPeriod {return false}
        if _storage._shardID != rhs_storage._shardID {return false}
        if _storage._realmID != rhs_storage._realmID {return false}
        if _storage._newRealmAdminKey != rhs_storage._newRealmAdminKey {return false}
        if _storage._memo != rhs_storage._memo {return false}
        if _storage._maxAutomaticTokenAssociations != rhs_storage._maxAutomaticTokenAssociations {return false}
        if _storage._stakedID != rhs_storage._stakedID {return false}
        if _storage._declineReward != rhs_storage._declineReward {return false}
        if _storage._alias != rhs_storage._alias {return false}
        if _storage._autoRenewAccount != rhs_storage._autoRenewAccount {return false}
        if _storage._evmAddress != rhs_storage._evmAddress {return false}
        return true
      }
      if !storagesAreEqual {return false}
    }
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}