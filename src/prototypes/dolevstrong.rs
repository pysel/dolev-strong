// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Value {
    Zero = 0,
    One = 1,
    Default = 2,
}
impl Value {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Value::Zero => "ZERO",
            Value::One => "ONE",
            Value::Default => "Default",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ZERO" => Some(Self::Zero),
            "ONE" => Some(Self::One),
            "Default" => Some(Self::Default),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusMsg {
    #[prost(enumeration="Value", tag="1")]
    pub value: i32,
    #[prost(message, optional, tag="2")]
    pub signature: ::core::option::Option<Signature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubkeyBroadcastMsg {
    #[prost(bytes="vec", tag="1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
}
include!("dolevstrong.serde.rs");
// @@protoc_insertion_point(module)