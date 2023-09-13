// use std::ffi::CStr;
// // use std::os::raw::c_char;
// use std::slice;
// use crate::chain::common::ChainType;
// use crate::crypto::byte_util::ConstDecodable;
use crate::crypto::UInt160;
// use crate::ffi::ByteArray;
use crate::util::address::address;
use crate::util::data_append::DataAppend;

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn address_from_hash160(
    hash: UInt160,
    chain_type: crate::chain::common::chain_type::ChainType,
) -> String {
    let script_map = chain_type.script_map();
    address::from_hash160_for_script_map(&hash, &script_map)
}

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn address_with_script_pubkey(
    script: Vec<u8>,
    chain_type: crate::chain::common::chain_type::ChainType,
) -> Option<String> {
    address::with_script_pub_key(&script, &chain_type.script_map())
}

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn address_with_script_sig(
    script: Vec<u8>,
    chain_type: crate::chain::common::chain_type::ChainType,
) -> Option<String> {
    address::with_script_sig(
        &script,
        &chain_type.script_map(),
    )
}

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn script_pubkey_for_address(
    address: Option<String>,
    chain_type: crate::chain::common::chain_type::ChainType,
) -> Option<Vec<u8>> {
    address.map(|address| {
        Vec::<u8>::script_pub_key_for_address(address.as_str(), &chain_type.script_map())
    })
}

#[rs_ffi_macro_derive::impl_ffi_fn_conv]
pub fn is_valid_dash_address_for_chain(
    address: Option<String>,
    chain_type: crate::chain::common::chain_type::ChainType,
) -> bool {
    address.map_or(false, |address| address::is_valid_dash_address_for_script_map(address.as_str(), &chain_type.script_map()))
}

// /// # Safety
// #[no_mangle]
// pub unsafe extern "C" fn address_from_hash160(hash: *const u8, chain_type: ChainType) -> *mut c_char {
//     let hash = UInt160::from_const(hash).unwrap_or(UInt160::MIN);
//     let script_map = chain_type.script_map();
//     rs_ffi_interfaces::FFIConversion::ffi_to(address::from_hash160_for_script_map(&hash, &script_map))
// }
//
// /// # Safety
// #[no_mangle]
// pub unsafe extern "C" fn address_with_script_pubkey(script: *const u8, script_len: usize, chain_type: ChainType) -> *mut c_char {
//     let script = slice::from_raw_parts(script, script_len);
//     let script_map = chain_type.script_map();
//     rs_ffi_interfaces::FFIConversion::ffi_to_opt(address::with_script_pub_key(&script.to_vec(), &script_map))
// }
//
// /// # Safety
// #[no_mangle]
// pub unsafe extern "C" fn address_with_script_sig(script: *const u8, script_len: usize, chain_type: ChainType) -> *mut c_char {
//     let script = slice::from_raw_parts(script, script_len);
//     let script_map = chain_type.script_map();
//     rs_ffi_interfaces::FFIConversion::ffi_to_opt(address::with_script_sig(&script.to_vec(), &script_map))
// }
//
// /// # Safety
// #[no_mangle]
// pub extern "C" fn script_pubkey_for_address(address: *const c_char, chain_type: ChainType) -> ByteArray {
//     if address.is_null() {
//         ByteArray::default()
//     } else {
//         let c_str = unsafe { CStr::from_ptr(address) };
//         let script_map = chain_type.script_map();
//         Vec::<u8>::script_pub_key_for_address(c_str.to_str().unwrap(), &script_map).into()
//     }
// }
//
// /// # Safety
// #[no_mangle]
// pub extern "C" fn is_valid_dash_address_for_chain(address: *const c_char, chain_type: ChainType) -> bool {
//     if address.is_null() {
//         false
//     } else {
//         let c_str = unsafe { CStr::from_ptr(address) };
//         let script_map = chain_type.script_map();
//         address::is_valid_dash_address_for_script_map(c_str.to_str().unwrap(), &script_map)
//     }
// }
