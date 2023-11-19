# [allow (dead_code , redundant_semicolons , unused_braces , unused_imports , unused_unsafe , unused_variables)] pub mod types { pub mod ffi { pub mod callbacks { use dash_spv_masternode_processor :: processing :: processing_error :: ProcessingError ; use dash_spv_masternode_processor :: models :: masternode_list :: MasternodeList ; use dash_spv_masternode_processor :: models :: snapshot :: LLMQSnapshot ; use dash_spv_masternode_processor :: crypto :: UInt256 ; use dash_spv_masternode_processor :: crypto :: UInt768 ; use dash_spv_masternode_processor :: fermented :: types :: models :: masternode_list :: MasternodeList_FFI ; use dash_spv_masternode_processor :: fermented :: types :: processing :: processing_error :: ProcessingError_FFI ; use dash_spv_masternode_processor :: fermented :: types :: models :: snapshot :: LLMQSnapshot_FFI ; use crate :: ffi :: callbacks :: GetMasternodeListCallback ; use crate :: ffi :: callbacks :: GetBlockHashByHeightCallback ; use crate :: ffi :: callbacks :: AddInsightCallback ; use crate :: ffi :: callbacks :: GetCLSignatureByBlockHashCallback ; use crate :: ffi :: callbacks :: ShouldProcessDiffWithRangeCallback ; use crate :: ffi :: callbacks :: SaveMasternodeListCallback ; use crate :: ffi :: callbacks :: SaveCLSignatureCallback ; use crate :: ffi :: callbacks :: SaveLLMQSnapshotCallback ; use crate :: ffi :: callbacks :: DestroyLLMQSnapshotCallback ; use crate :: ffi :: callbacks :: GetLLMQSnapshotByBlockHashCallback ; use crate :: ffi :: callbacks :: DestroyHashCallback ; use crate :: ffi :: callbacks :: GetMerkleRootCallback ; use crate :: ffi :: callbacks :: GetBlockHeightByHashCallback ; use crate :: ffi :: callbacks :: DestroyMasternodeListCallback ; use ferment_interfaces ; # [doc = "FFI-representation of the GetMerkleRootCallback_FFI"] # [allow (non_camel_case_types)] pub type GetMerkleRootCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> * mut [u8 ; 32] ; # [doc = "FFI-representation of the DestroyMasternodeListCallback_FFI"] # [allow (non_camel_case_types)] pub type DestroyMasternodeListCallback_FFI = unsafe extern "C" fn (masternode_list : * mut MasternodeList_FFI) ; # [doc = "FFI-representation of the ShouldProcessDiffWithRangeCallback_FFI"] # [allow (non_camel_case_types)] pub type ShouldProcessDiffWithRangeCallback_FFI = unsafe extern "C" fn (base_block_hash : * mut [u8 ; 32] , block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> * mut ProcessingError_FFI ; # [doc = "FFI-representation of the AddInsightCallback_FFI"] # [allow (non_camel_case_types)] pub type AddInsightCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) ; # [doc = "FFI-representation of the SaveMasternodeListCallback_FFI"] # [allow (non_camel_case_types)] pub type SaveMasternodeListCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , masternode_list : * mut MasternodeList_FFI , context : ferment_interfaces :: OpaqueContextFFI) -> bool ; # [doc = "FFI-representation of the SaveLLMQSnapshotCallback_FFI"] # [allow (non_camel_case_types)] pub type SaveLLMQSnapshotCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , snapshot : * mut LLMQSnapshot_FFI , context : ferment_interfaces :: OpaqueContextFFI) -> bool ; # [doc = "FFI-representation of the GetLLMQSnapshotByBlockHashCallback_FFI"] # [allow (non_camel_case_types)] pub type GetLLMQSnapshotByBlockHashCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> * mut LLMQSnapshot_FFI ; # [doc = "FFI-representation of the DestroyHashCallback_FFI"] # [allow (non_camel_case_types)] pub type DestroyHashCallback_FFI = unsafe extern "C" fn (hash : * mut [u8 ; 32]) ; # [doc = "FFI-representation of the DestroyLLMQSnapshotCallback_FFI"] # [allow (non_camel_case_types)] pub type DestroyLLMQSnapshotCallback_FFI = unsafe extern "C" fn (snapshot : * mut LLMQSnapshot_FFI) ; # [doc = "FFI-representation of the GetBlockHashByHeightCallback_FFI"] # [allow (non_camel_case_types)] pub type GetBlockHashByHeightCallback_FFI = unsafe extern "C" fn (block_height : u32 , context : ferment_interfaces :: OpaqueContextFFI) -> * mut [u8 ; 32] ; # [doc = "FFI-representation of the GetBlockHeightByHashCallback_FFI"] # [allow (non_camel_case_types)] pub type GetBlockHeightByHashCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> u32 ; # [doc = "FFI-representation of the SaveCLSignatureCallback_FFI"] # [allow (non_camel_case_types)] pub type SaveCLSignatureCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , cl_signature : * mut [u8 ; 96] , context : ferment_interfaces :: OpaqueContextFFI) -> bool ; # [doc = "FFI-representation of the GetMasternodeListCallback_FFI"] # [allow (non_camel_case_types)] pub type GetMasternodeListCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> * mut MasternodeList_FFI ; # [doc = "FFI-representation of the GetCLSignatureByBlockHashCallback_FFI"] # [allow (non_camel_case_types)] pub type GetCLSignatureByBlockHashCallback_FFI = unsafe extern "C" fn (block_hash : * mut [u8 ; 32] , context : ferment_interfaces :: OpaqueContextFFI) -> * mut [u8 ; 96] ; } } pub mod address { pub mod addresses { use dash_spv_masternode_processor :: fermented :: types :: chain :: common :: chain_type :: ChainType_FFI ; use crate :: address :: addresses :: address_with_script_pubkey ; use crate :: address :: addresses :: address_from_hash160 ; use dash_spv_masternode_processor :: chain :: common :: chain_type :: ChainType ; use dash_spv_masternode_processor :: crypto :: UInt160 ; use crate :: fermented :: generics :: Vec_u8_FFI ; # [doc = "FFI-representation of the address_with_script_pubkey"] # [doc = r" # Safety"] # [no_mangle] pub unsafe extern "C" fn ffi_address_with_script_pubkey (script : * mut Vec_u8_FFI , chain_type : * mut ChainType_FFI ,) -> * mut std :: os :: raw :: c_char { let obj = address_with_script_pubkey ({ let vec = & * script ; { let vec = vec ; ferment_interfaces :: from_simple_vec (vec . values , vec . count) } } , ferment_interfaces :: FFIConversion :: ffi_from (chain_type) ,) ; ferment_interfaces :: FFIConversion :: ffi_to_opt (obj) } # [doc = "FFI-representation of the address_from_hash160"] # [doc = r" # Safety"] # [no_mangle] pub unsafe extern "C" fn ffi_address_from_hash160 (hash : * mut [u8 ; 20] , chain_type : * mut ChainType_FFI ,) -> * mut std :: os :: raw :: c_char { let obj = address_from_hash160 (ferment_interfaces :: FFIConversion :: ffi_from (hash) , ferment_interfaces :: FFIConversion :: ffi_from (chain_type) ,) ; ferment_interfaces :: FFIConversion :: ffi_to (obj) } } } } # [allow (dead_code , redundant_semicolons , unused_braces , unused_imports , unused_unsafe , unused_variables)] pub mod generics { # [repr (C)] # [derive (Clone)] # [allow (non_camel_case_types)] pub struct Vec_u8_FFI { pub count : usize , pub values : * mut u8 , } impl ferment_interfaces :: FFIConversion < Vec < u8 >> for Vec_u8_FFI { unsafe fn ffi_from_const (ffi : * const Vec_u8_FFI) -> Vec < u8 > { let ffi_ref = & * ffi ; ferment_interfaces :: FFIVecConversion :: decode (ffi_ref) } unsafe fn ffi_to_const (obj : Vec < u8 >) -> * const Vec_u8_FFI { ferment_interfaces :: FFIVecConversion :: encode (obj) } unsafe fn destroy (ffi : * mut Vec_u8_FFI) { ferment_interfaces :: unbox_any (ffi) ; } } impl ferment_interfaces :: FFIVecConversion for Vec_u8_FFI { type Value = u8 ; unsafe fn decode (& self) -> Vec < Self :: Value > { ferment_interfaces :: from_simple_vec (self . values as * const Self :: Value , self . count) } unsafe fn encode (obj : Vec < Self :: Value >) -> * mut Self { ferment_interfaces :: boxed (Self { count : obj . len () , values : ferment_interfaces :: boxed_vec (obj) }) } } impl Drop for Vec_u8_FFI { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_vec_ptr (self . values , self . count) ; } } } }