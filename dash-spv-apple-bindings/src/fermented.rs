#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod types {
    pub mod ffi {
        pub mod callbacks {
            #[doc = "FFI-representation of the GetLLMQSnapshotByBlockHashCallback"]
            #[allow(non_camel_case_types)]
            pub type GetLLMQSnapshotByBlockHashCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: models :: snapshot :: LLMQSnapshot ;
            #[doc = "FFI-representation of the SaveMasternodeListCallback"]
            #[allow(non_camel_case_types)]
            pub type SaveMasternodeListCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , masternode_list : * mut dash_spv_masternode_processor :: fermented :: types :: models :: masternode_list :: MasternodeList , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> bool ;
            #[doc = "FFI-representation of the ShouldProcessDiffWithRangeCallback"]
            #[allow(non_camel_case_types)]
            pub type ShouldProcessDiffWithRangeCallback = unsafe extern "C" fn (base_block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: processing :: processing_error :: ProcessingError ;
            #[doc = "FFI-representation of the GetBlockHeightByHashCallback"]
            #[allow(non_camel_case_types)]
            pub type GetBlockHeightByHashCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> u32 ;
            #[doc = "FFI-representation of the GetCLSignatureByBlockHashCallback"]
            #[allow(non_camel_case_types)]
            pub type GetCLSignatureByBlockHashCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt768 ;
            #[doc = "FFI-representation of the GetMerkleRootCallback"]
            #[allow(non_camel_case_types)]
            pub type GetMerkleRootCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 ;
            #[doc = "FFI-representation of the GetBlockHashByHeightCallback"]
            #[allow(non_camel_case_types)]
            pub type GetBlockHashByHeightCallback = unsafe extern "C" fn (block_height : u32 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 ;
            #[doc = "FFI-representation of the AddInsightCallback"]
            #[allow(non_camel_case_types)]
            pub type AddInsightCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) ;
            #[doc = "FFI-representation of the DestroyLLMQSnapshotCallback"]
            #[allow(non_camel_case_types)]
            pub type DestroyLLMQSnapshotCallback = unsafe extern "C" fn (snapshot : * mut dash_spv_masternode_processor :: fermented :: types :: models :: snapshot :: LLMQSnapshot) ;
            #[doc = "FFI-representation of the GetMasternodeListCallback"]
            #[allow(non_camel_case_types)]
            pub type GetMasternodeListCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> * mut dash_spv_masternode_processor :: fermented :: types :: models :: masternode_list :: MasternodeList ;
            #[doc = "FFI-representation of the DestroyMasternodeListCallback"]
            #[allow(non_camel_case_types)]
            pub type DestroyMasternodeListCallback = unsafe extern "C" fn (masternode_list : * mut dash_spv_masternode_processor :: fermented :: types :: models :: masternode_list :: MasternodeList) ;
            #[doc = "FFI-representation of the SaveCLSignatureCallback"]
            #[allow(non_camel_case_types)]
            pub type SaveCLSignatureCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , cl_signature : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt768 , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> bool ;
            #[doc = "FFI-representation of the DestroyHashCallback"]
            #[allow(non_camel_case_types)]
            pub type DestroyHashCallback = unsafe extern "C" fn (hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256) ;
            #[doc = "FFI-representation of the SaveLLMQSnapshotCallback"]
            #[allow(non_camel_case_types)]
            pub type SaveLLMQSnapshotCallback = unsafe extern "C" fn (block_hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt256 , snapshot : * mut dash_spv_masternode_processor :: fermented :: types :: models :: snapshot :: LLMQSnapshot , context : * mut ferment_interfaces :: fermented :: types :: OpaqueContext_FFI) -> bool ;
        }
    }
    pub mod address {
        pub mod addresses {
            #[doc = "FFI-representation of the address_from_hash160"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_from_hash160(
                hash : * mut dash_spv_masternode_processor :: fermented :: types :: crypto :: byte_util :: UInt160,
                chain_type : * mut dash_spv_masternode_processor :: fermented :: types :: chain :: common :: chain_type :: ChainType,
            ) -> *mut std::os::raw::c_char {
                let obj = crate::address::addresses::address_from_hash160(
                    ferment_interfaces::FFIConversion::ffi_from(hash),
                    ferment_interfaces::FFIConversion::ffi_from(chain_type),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the address_with_script_pubkey"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ffi_address_with_script_pubkey(
                script: *mut crate::fermented::generics::Vec_u8,
                chain_type : * mut dash_spv_masternode_processor :: fermented :: types :: chain :: common :: chain_type :: ChainType,
            ) -> *mut std::os::raw::c_char {
                let obj = crate::address::addresses::address_with_script_pubkey(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                    ferment_interfaces::FFIConversion::ffi_from(chain_type),
                );
                ferment_interfaces::FFIConversion::ffi_to_opt(obj)
            }
        }
    }
}
#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u8 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
}
