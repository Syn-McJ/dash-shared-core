use crate::crypto::UInt256;

#[derive(Debug, Copy, Clone)]
#[rs_ffi_macro_derive::impl_ffi_conv]
pub struct LLMQTypedHash {
    pub r#type: crate::chain::common::llmq_type::LLMQType,
    pub hash: UInt256,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[rs_ffi_macro_derive::impl_ffi_conv]
pub struct LLMQIndexedHash {
    pub index: u32,
    pub hash: UInt256,
}

impl LLMQIndexedHash {
    pub fn new(hash: UInt256, index: u32) -> Self {
        LLMQIndexedHash { index, hash }
    }
}

impl From<(UInt256, usize)> for LLMQIndexedHash {
    fn from(value: (UInt256, usize)) -> Self {
        Self::new(value.0, value.1 as u32)
    }
}
impl From<(UInt256, u32)> for LLMQIndexedHash {
    fn from(value: (UInt256, u32)) -> Self {
        Self::new(value.0, value.1)
    }
}
