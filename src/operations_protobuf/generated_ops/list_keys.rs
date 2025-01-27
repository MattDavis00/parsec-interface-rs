#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyInfo {
    #[prost(uint32, tag="1")]
    pub provider_id: u32,
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(message, optional, tag="3")]
    pub attributes: ::std::option::Option<super::psa_key_attributes::KeyAttributes>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    #[prost(message, repeated, tag="1")]
    pub keys: ::std::vec::Vec<KeyInfo>,
}
