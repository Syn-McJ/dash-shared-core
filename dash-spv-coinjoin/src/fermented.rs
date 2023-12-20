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
    pub mod messages {
        pub mod coinjoin_accept_message {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: messages :: coinjoin_accept_message :: CoinJoinAcceptMessage\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CoinJoinAcceptMessage {
                pub denomination: u32,
                pub tx_collateral: *mut dash_spv_masternode_processor::tx::transaction::Transaction,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::messages::coinjoin_accept_message::CoinJoinAcceptMessage,
                > for CoinJoinAcceptMessage
            {
                unsafe fn ffi_from_const(
                    ffi: *const CoinJoinAcceptMessage,
                ) -> crate::messages::coinjoin_accept_message::CoinJoinAcceptMessage
                {
                    let ffi_ref = &*ffi;
                    crate::messages::coinjoin_accept_message::CoinJoinAcceptMessage {
                        denomination: ffi_ref.denomination,
                        tx_collateral: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.tx_collateral,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::messages::coinjoin_accept_message::CoinJoinAcceptMessage,
                ) -> *const CoinJoinAcceptMessage {
                    ferment_interfaces::boxed(CoinJoinAcceptMessage {
                        denomination: obj.denomination,
                        tx_collateral: ferment_interfaces::FFIConversion::ffi_to(obj.tx_collateral),
                    })
                }
                unsafe fn destroy(ffi: *mut CoinJoinAcceptMessage) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for CoinJoinAcceptMessage {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.tx_collateral);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoinJoinAcceptMessage_ctor(
                denomination: u32,
                tx_collateral: *mut dash_spv_masternode_processor::tx::transaction::Transaction,
            ) -> *mut CoinJoinAcceptMessage {
                ferment_interfaces::boxed(CoinJoinAcceptMessage {
                    denomination,
                    tx_collateral,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoinJoinAcceptMessage_destroy(
                ffi: *mut CoinJoinAcceptMessage,
            ) {
                ferment_interfaces::unbox_any(ffi);
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
pub mod generics {}
