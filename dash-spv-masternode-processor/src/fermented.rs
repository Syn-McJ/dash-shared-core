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
    pub mod processing {
        pub mod processing_error {
            #[doc = "FFI-representation of the ProcessingError"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum ProcessingError {
                None = 0,
                PersistInRetrieval = 1,
                LocallyStored = 2,
                ParseError = 3,
                HasNoBaseBlockHash = 4,
                UnknownBlockHash = 5,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::processing::processing_error::ProcessingError,
                > for ProcessingError
            {
                unsafe fn ffi_from_const(
                    ffi: *const ProcessingError,
                ) -> crate::processing::processing_error::ProcessingError {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ProcessingError::None => {
                            crate::processing::processing_error::ProcessingError::None
                        },
                        ProcessingError::PersistInRetrieval => {
                            crate::processing::processing_error::ProcessingError::PersistInRetrieval
                        },
                        ProcessingError::LocallyStored => {
                            crate::processing::processing_error::ProcessingError::LocallyStored
                        },
                        ProcessingError::ParseError => {
                            crate::processing::processing_error::ProcessingError::ParseError
                        },
                        ProcessingError::HasNoBaseBlockHash => {
                            crate::processing::processing_error::ProcessingError::HasNoBaseBlockHash
                        },
                        ProcessingError::UnknownBlockHash => {
                            crate::processing::processing_error::ProcessingError::UnknownBlockHash
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::processing::processing_error::ProcessingError,
                ) -> *const ProcessingError {
                    ferment_interfaces :: boxed (match obj { crate :: processing :: processing_error :: ProcessingError :: None => ProcessingError :: None , crate :: processing :: processing_error :: ProcessingError :: PersistInRetrieval => ProcessingError :: PersistInRetrieval , crate :: processing :: processing_error :: ProcessingError :: LocallyStored => ProcessingError :: LocallyStored , crate :: processing :: processing_error :: ProcessingError :: ParseError => ProcessingError :: ParseError , crate :: processing :: processing_error :: ProcessingError :: HasNoBaseBlockHash => ProcessingError :: HasNoBaseBlockHash , crate :: processing :: processing_error :: ProcessingError :: UnknownBlockHash => ProcessingError :: UnknownBlockHash , })
                }
                unsafe fn destroy(ffi: *mut ProcessingError) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ProcessingError {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ProcessingError::None => {},
                            ProcessingError::PersistInRetrieval => {},
                            ProcessingError::LocallyStored => {},
                            ProcessingError::ParseError => {},
                            ProcessingError::HasNoBaseBlockHash => {},
                            ProcessingError::UnknownBlockHash => {},
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_None_ctor() -> *mut ProcessingError {
                ferment_interfaces::boxed(ProcessingError::None)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_PersistInRetrieval_ctor(
            ) -> *mut ProcessingError {
                ferment_interfaces::boxed(ProcessingError::PersistInRetrieval)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_LocallyStored_ctor() -> *mut ProcessingError {
                ferment_interfaces::boxed(ProcessingError::LocallyStored)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_ParseError_ctor() -> *mut ProcessingError {
                ferment_interfaces::boxed(ProcessingError::ParseError)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_HasNoBaseBlockHash_ctor(
            ) -> *mut ProcessingError {
                ferment_interfaces::boxed(ProcessingError::HasNoBaseBlockHash)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_UnknownBlockHash_ctor() -> *mut ProcessingError
            {
                ferment_interfaces::boxed(ProcessingError::UnknownBlockHash)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn ProcessingError_destroy(ffi: *mut ProcessingError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod qr_info_result {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: processing :: qr_info_result :: QRInfoResult\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct QRInfoResult { pub result_at_tip : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub result_at_h : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub result_at_h_c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub result_at_h_2c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub result_at_h_3c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub result_at_h_4c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult , pub snapshot_at_h_c : * mut crate :: fermented :: types :: models :: snapshot :: LLMQSnapshot , pub snapshot_at_h_2c : * mut crate :: fermented :: types :: models :: snapshot :: LLMQSnapshot , pub snapshot_at_h_3c : * mut crate :: fermented :: types :: models :: snapshot :: LLMQSnapshot , pub snapshot_at_h_4c : * mut crate :: fermented :: types :: models :: snapshot :: LLMQSnapshot , pub extra_share : bool , pub last_quorum_per_index : * mut crate :: fermented :: generics :: Vec_crate_models_llmq_entry_LLMQEntry , pub quorum_snapshot_list : * mut crate :: fermented :: generics :: Vec_crate_models_snapshot_LLMQSnapshot , pub mn_list_diff_list : * mut crate :: fermented :: generics :: Vec_crate_processing_mn_listdiff_result_MNListDiffResult , }
            impl ferment_interfaces::FFIConversion<crate::processing::qr_info_result::QRInfoResult>
                for QRInfoResult
            {
                unsafe fn ffi_from_const(
                    ffi: *const QRInfoResult,
                ) -> crate::processing::qr_info_result::QRInfoResult {
                    let ffi_ref = &*ffi;
                    crate::processing::qr_info_result::QRInfoResult {
                        result_at_tip: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.result_at_tip,
                        ),
                        result_at_h: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.result_at_h,
                        ),
                        result_at_h_c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.result_at_h_c,
                        ),
                        result_at_h_2c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.result_at_h_2c,
                        ),
                        result_at_h_3c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.result_at_h_3c,
                        ),
                        result_at_h_4c: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.result_at_h_4c,
                        ),
                        snapshot_at_h_c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.snapshot_at_h_c,
                        ),
                        snapshot_at_h_2c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.snapshot_at_h_2c,
                        ),
                        snapshot_at_h_3c: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.snapshot_at_h_3c,
                        ),
                        snapshot_at_h_4c: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.snapshot_at_h_4c,
                        ),
                        extra_share: ffi_ref.extra_share,
                        last_quorum_per_index: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.last_quorum_per_index,
                        ),
                        quorum_snapshot_list: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.quorum_snapshot_list,
                        ),
                        mn_list_diff_list: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.mn_list_diff_list,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::processing::qr_info_result::QRInfoResult,
                ) -> *const QRInfoResult {
                    ferment_interfaces::boxed(QRInfoResult {
                        result_at_tip: ferment_interfaces::FFIConversion::ffi_to(obj.result_at_tip),
                        result_at_h: ferment_interfaces::FFIConversion::ffi_to(obj.result_at_h),
                        result_at_h_c: ferment_interfaces::FFIConversion::ffi_to(obj.result_at_h_c),
                        result_at_h_2c: ferment_interfaces::FFIConversion::ffi_to(
                            obj.result_at_h_2c,
                        ),
                        result_at_h_3c: ferment_interfaces::FFIConversion::ffi_to(
                            obj.result_at_h_3c,
                        ),
                        result_at_h_4c: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.result_at_h_4c,
                        ),
                        snapshot_at_h_c: ferment_interfaces::FFIConversion::ffi_to(
                            obj.snapshot_at_h_c,
                        ),
                        snapshot_at_h_2c: ferment_interfaces::FFIConversion::ffi_to(
                            obj.snapshot_at_h_2c,
                        ),
                        snapshot_at_h_3c: ferment_interfaces::FFIConversion::ffi_to(
                            obj.snapshot_at_h_3c,
                        ),
                        snapshot_at_h_4c: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.snapshot_at_h_4c,
                        ),
                        extra_share: obj.extra_share,
                        last_quorum_per_index: ferment_interfaces::FFIConversion::ffi_to(
                            obj.last_quorum_per_index,
                        ),
                        quorum_snapshot_list: ferment_interfaces::FFIConversion::ffi_to(
                            obj.quorum_snapshot_list,
                        ),
                        mn_list_diff_list: ferment_interfaces::FFIConversion::ffi_to(
                            obj.mn_list_diff_list,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut QRInfoResult) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for QRInfoResult {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.result_at_tip);
                        ferment_interfaces::unbox_any(ffi_ref.result_at_h);
                        ferment_interfaces::unbox_any(ffi_ref.result_at_h_c);
                        ferment_interfaces::unbox_any(ffi_ref.result_at_h_2c);
                        ferment_interfaces::unbox_any(ffi_ref.result_at_h_3c);
                        if !ffi_ref.result_at_h_4c.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.result_at_h_4c);
                        };
                        ferment_interfaces::unbox_any(ffi_ref.snapshot_at_h_c);
                        ferment_interfaces::unbox_any(ffi_ref.snapshot_at_h_2c);
                        ferment_interfaces::unbox_any(ffi_ref.snapshot_at_h_3c);
                        if !ffi_ref.snapshot_at_h_4c.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.snapshot_at_h_4c);
                        };
                        ferment_interfaces::unbox_any(ffi_ref.last_quorum_per_index);
                        ferment_interfaces::unbox_any(ffi_ref.quorum_snapshot_list);
                        ferment_interfaces::unbox_any(ffi_ref.mn_list_diff_list);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn QRInfoResult_ctor(
                result_at_tip : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                result_at_h : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                result_at_h_c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                result_at_h_2c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                result_at_h_3c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                result_at_h_4c : * mut crate :: fermented :: types :: processing :: mn_listdiff_result :: MNListDiffResult,
                snapshot_at_h_c: *mut crate::fermented::types::models::snapshot::LLMQSnapshot,
                snapshot_at_h_2c: *mut crate::fermented::types::models::snapshot::LLMQSnapshot,
                snapshot_at_h_3c: *mut crate::fermented::types::models::snapshot::LLMQSnapshot,
                snapshot_at_h_4c: *mut crate::fermented::types::models::snapshot::LLMQSnapshot,
                extra_share: bool,
                last_quorum_per_index : * mut crate :: fermented :: generics :: Vec_crate_models_llmq_entry_LLMQEntry,
                quorum_snapshot_list : * mut crate :: fermented :: generics :: Vec_crate_models_snapshot_LLMQSnapshot,
                mn_list_diff_list : * mut crate :: fermented :: generics :: Vec_crate_processing_mn_listdiff_result_MNListDiffResult,
            ) -> *mut QRInfoResult {
                ferment_interfaces::boxed(QRInfoResult {
                    result_at_tip,
                    result_at_h,
                    result_at_h_c,
                    result_at_h_2c,
                    result_at_h_3c,
                    result_at_h_4c,
                    snapshot_at_h_c,
                    snapshot_at_h_2c,
                    snapshot_at_h_3c,
                    snapshot_at_h_4c,
                    extra_share,
                    last_quorum_per_index,
                    quorum_snapshot_list,
                    mn_list_diff_list,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn QRInfoResult_destroy(ffi: *mut QRInfoResult) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod mn_listdiff_result {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: processing :: mn_listdiff_result :: MNListDiffResult\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MNListDiffResult { pub base_block_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub block_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub has_found_coinbase : bool , pub has_valid_coinbase : bool , pub has_valid_mn_list_root : bool , pub has_valid_llmq_list_root : bool , pub has_valid_quorums : bool , pub masternode_list : * mut crate :: fermented :: types :: models :: masternode_list :: MasternodeList , pub added_masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry , pub modified_masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry , pub added_quorums : * mut crate :: fermented :: generics :: Vec_crate_models_llmq_entry_LLMQEntry , pub needed_masternode_lists : * mut crate :: fermented :: generics :: Vec_crate_crypto_byte_util_UInt256 , }
            impl
                ferment_interfaces::FFIConversion<
                    crate::processing::mn_listdiff_result::MNListDiffResult,
                > for MNListDiffResult
            {
                unsafe fn ffi_from_const(
                    ffi: *const MNListDiffResult,
                ) -> crate::processing::mn_listdiff_result::MNListDiffResult {
                    let ffi_ref = &*ffi;
                    crate::processing::mn_listdiff_result::MNListDiffResult {
                        base_block_hash: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.base_block_hash,
                        ),
                        block_hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.block_hash),
                        has_found_coinbase: ffi_ref.has_found_coinbase,
                        has_valid_coinbase: ffi_ref.has_valid_coinbase,
                        has_valid_mn_list_root: ffi_ref.has_valid_mn_list_root,
                        has_valid_llmq_list_root: ffi_ref.has_valid_llmq_list_root,
                        has_valid_quorums: ffi_ref.has_valid_quorums,
                        masternode_list: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.masternode_list,
                        ),
                        added_masternodes: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.added_masternodes,
                        ),
                        modified_masternodes: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.modified_masternodes,
                        ),
                        added_quorums: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.added_quorums,
                        ),
                        needed_masternode_lists: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.needed_masternode_lists,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::processing::mn_listdiff_result::MNListDiffResult,
                ) -> *const MNListDiffResult {
                    ferment_interfaces::boxed(MNListDiffResult {
                        base_block_hash: ferment_interfaces::FFIConversion::ffi_to(
                            obj.base_block_hash,
                        ),
                        block_hash: ferment_interfaces::FFIConversion::ffi_to(obj.block_hash),
                        has_found_coinbase: obj.has_found_coinbase,
                        has_valid_coinbase: obj.has_valid_coinbase,
                        has_valid_mn_list_root: obj.has_valid_mn_list_root,
                        has_valid_llmq_list_root: obj.has_valid_llmq_list_root,
                        has_valid_quorums: obj.has_valid_quorums,
                        masternode_list: ferment_interfaces::FFIConversion::ffi_to(
                            obj.masternode_list,
                        ),
                        added_masternodes: ferment_interfaces::FFIConversion::ffi_to(
                            obj.added_masternodes,
                        ),
                        modified_masternodes: ferment_interfaces::FFIConversion::ffi_to(
                            obj.modified_masternodes,
                        ),
                        added_quorums: ferment_interfaces::FFIConversion::ffi_to(obj.added_quorums),
                        needed_masternode_lists: ferment_interfaces::FFIConversion::ffi_to(
                            obj.needed_masternode_lists,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut MNListDiffResult) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for MNListDiffResult {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.base_block_hash);
                        ferment_interfaces::unbox_any(ffi_ref.block_hash);
                        ferment_interfaces::unbox_any(ffi_ref.masternode_list);
                        ferment_interfaces::unbox_any(ffi_ref.added_masternodes);
                        ferment_interfaces::unbox_any(ffi_ref.modified_masternodes);
                        ferment_interfaces::unbox_any(ffi_ref.added_quorums);
                        ferment_interfaces::unbox_any(ffi_ref.needed_masternode_lists);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MNListDiffResult_ctor(
                base_block_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                block_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                has_found_coinbase: bool,
                has_valid_coinbase: bool,
                has_valid_mn_list_root: bool,
                has_valid_llmq_list_root: bool,
                has_valid_quorums: bool,
                masternode_list : * mut crate :: fermented :: types :: models :: masternode_list :: MasternodeList,
                added_masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry,
                modified_masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry,
                added_quorums : * mut crate :: fermented :: generics :: Vec_crate_models_llmq_entry_LLMQEntry,
                needed_masternode_lists : * mut crate :: fermented :: generics :: Vec_crate_crypto_byte_util_UInt256,
            ) -> *mut MNListDiffResult {
                ferment_interfaces::boxed(MNListDiffResult {
                    base_block_hash,
                    block_hash,
                    has_found_coinbase,
                    has_valid_coinbase,
                    has_valid_mn_list_root,
                    has_valid_llmq_list_root,
                    has_valid_quorums,
                    masternode_list,
                    added_masternodes,
                    modified_masternodes,
                    added_quorums,
                    needed_masternode_lists,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MNListDiffResult_destroy(ffi: *mut MNListDiffResult) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod crypto {
        pub mod byte_util {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt768\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt768(*mut [u8; 96]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt768> for UInt768 {
                unsafe fn ffi_from_const(ffi: *const UInt768) -> crate::crypto::byte_util::UInt768 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt768(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt768) -> *const UInt768 {
                    ferment_interfaces::boxed(UInt768(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt768) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt768 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt768_ctor(o_0: *mut [u8; 96]) -> *mut UInt768 {
                ferment_interfaces::boxed(UInt768(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt768_destroy(ffi: *mut UInt768) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt384\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt384(*mut [u8; 48]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt384> for UInt384 {
                unsafe fn ffi_from_const(ffi: *const UInt384) -> crate::crypto::byte_util::UInt384 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt384(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt384) -> *const UInt384 {
                    ferment_interfaces::boxed(UInt384(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt384) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt384 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt384_ctor(o_0: *mut [u8; 48]) -> *mut UInt384 {
                ferment_interfaces::boxed(UInt384(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt384_destroy(ffi: *mut UInt384) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt256\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt256(*mut [u8; 32]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt256> for UInt256 {
                unsafe fn ffi_from_const(ffi: *const UInt256) -> crate::crypto::byte_util::UInt256 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt256(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt256) -> *const UInt256 {
                    ferment_interfaces::boxed(UInt256(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt256) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt256 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt256_ctor(o_0: *mut [u8; 32]) -> *mut UInt256 {
                ferment_interfaces::boxed(UInt256(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt256_destroy(ffi: *mut UInt256) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt512\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt512(*mut [u8; 64]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt512> for UInt512 {
                unsafe fn ffi_from_const(ffi: *const UInt512) -> crate::crypto::byte_util::UInt512 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt512(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt512) -> *const UInt512 {
                    ferment_interfaces::boxed(UInt512(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt512) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt512 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt512_ctor(o_0: *mut [u8; 64]) -> *mut UInt512 {
                ferment_interfaces::boxed(UInt512(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt512_destroy(ffi: *mut UInt512) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt128\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt128(*mut [u8; 16]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt128> for UInt128 {
                unsafe fn ffi_from_const(ffi: *const UInt128) -> crate::crypto::byte_util::UInt128 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt128(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt128) -> *const UInt128 {
                    ferment_interfaces::boxed(UInt128(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt128) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt128 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt128_ctor(o_0: *mut [u8; 16]) -> *mut UInt128 {
                ferment_interfaces::boxed(UInt128(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt128_destroy(ffi: *mut UInt128) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: crypto :: byte_util :: UInt160\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct UInt160(*mut [u8; 20]);
            impl ferment_interfaces::FFIConversion<crate::crypto::byte_util::UInt160> for UInt160 {
                unsafe fn ffi_from_const(ffi: *const UInt160) -> crate::crypto::byte_util::UInt160 {
                    let ffi_ref = &*ffi;
                    crate::crypto::byte_util::UInt160(*ffi_ref.0)
                }
                unsafe fn ffi_to_const(obj: crate::crypto::byte_util::UInt160) -> *const UInt160 {
                    ferment_interfaces::boxed(UInt160(ferment_interfaces::boxed(obj.0)))
                }
                unsafe fn destroy(ffi: *mut UInt160) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for UInt160 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt160_ctor(o_0: *mut [u8; 20]) -> *mut UInt160 {
                ferment_interfaces::boxed(UInt160(o_0))
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn UInt160_destroy(ffi: *mut UInt160) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod tx {
        pub mod transaction {
            #[doc = "FFI-representation of the TransactionType"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum TransactionType {
                Classic = 0,
                ProviderRegistration = 1,
                ProviderUpdateService = 2,
                ProviderUpdateRegistrar = 3,
                ProviderUpdateRevocation = 4,
                Coinbase = 5,
                QuorumCommitment = 6,
                AssetLock = 8,
                AssetUnlock = 9,
                TypeMax = 10,
                SubscriptionCloseAccount = 11,
                Transition = 12,
                CreditFunding = 255,
            }
            impl ferment_interfaces::FFIConversion<crate::tx::transaction::TransactionType>
                for TransactionType
            {
                unsafe fn ffi_from_const(
                    ffi: *const TransactionType,
                ) -> crate::tx::transaction::TransactionType {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        TransactionType::Classic => {
                            crate::tx::transaction::TransactionType::Classic
                        },
                        TransactionType::ProviderRegistration => {
                            crate::tx::transaction::TransactionType::ProviderRegistration
                        },
                        TransactionType::ProviderUpdateService => {
                            crate::tx::transaction::TransactionType::ProviderUpdateService
                        },
                        TransactionType::ProviderUpdateRegistrar => {
                            crate::tx::transaction::TransactionType::ProviderUpdateRegistrar
                        },
                        TransactionType::ProviderUpdateRevocation => {
                            crate::tx::transaction::TransactionType::ProviderUpdateRevocation
                        },
                        TransactionType::Coinbase => {
                            crate::tx::transaction::TransactionType::Coinbase
                        },
                        TransactionType::QuorumCommitment => {
                            crate::tx::transaction::TransactionType::QuorumCommitment
                        },
                        TransactionType::AssetLock => {
                            crate::tx::transaction::TransactionType::AssetLock
                        },
                        TransactionType::AssetUnlock => {
                            crate::tx::transaction::TransactionType::AssetUnlock
                        },
                        TransactionType::TypeMax => {
                            crate::tx::transaction::TransactionType::TypeMax
                        },
                        TransactionType::SubscriptionCloseAccount => {
                            crate::tx::transaction::TransactionType::SubscriptionCloseAccount
                        },
                        TransactionType::Transition => {
                            crate::tx::transaction::TransactionType::Transition
                        },
                        TransactionType::CreditFunding => {
                            crate::tx::transaction::TransactionType::CreditFunding
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::tx::transaction::TransactionType,
                ) -> *const TransactionType {
                    ferment_interfaces::boxed(match obj {
                        crate::tx::transaction::TransactionType::Classic => {
                            TransactionType::Classic
                        },
                        crate::tx::transaction::TransactionType::ProviderRegistration => {
                            TransactionType::ProviderRegistration
                        },
                        crate::tx::transaction::TransactionType::ProviderUpdateService => {
                            TransactionType::ProviderUpdateService
                        },
                        crate::tx::transaction::TransactionType::ProviderUpdateRegistrar => {
                            TransactionType::ProviderUpdateRegistrar
                        },
                        crate::tx::transaction::TransactionType::ProviderUpdateRevocation => {
                            TransactionType::ProviderUpdateRevocation
                        },
                        crate::tx::transaction::TransactionType::Coinbase => {
                            TransactionType::Coinbase
                        },
                        crate::tx::transaction::TransactionType::QuorumCommitment => {
                            TransactionType::QuorumCommitment
                        },
                        crate::tx::transaction::TransactionType::AssetLock => {
                            TransactionType::AssetLock
                        },
                        crate::tx::transaction::TransactionType::AssetUnlock => {
                            TransactionType::AssetUnlock
                        },
                        crate::tx::transaction::TransactionType::TypeMax => {
                            TransactionType::TypeMax
                        },
                        crate::tx::transaction::TransactionType::SubscriptionCloseAccount => {
                            TransactionType::SubscriptionCloseAccount
                        },
                        crate::tx::transaction::TransactionType::Transition => {
                            TransactionType::Transition
                        },
                        crate::tx::transaction::TransactionType::CreditFunding => {
                            TransactionType::CreditFunding
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut TransactionType) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for TransactionType {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            TransactionType::Classic => {},
                            TransactionType::ProviderRegistration => {},
                            TransactionType::ProviderUpdateService => {},
                            TransactionType::ProviderUpdateRegistrar => {},
                            TransactionType::ProviderUpdateRevocation => {},
                            TransactionType::Coinbase => {},
                            TransactionType::QuorumCommitment => {},
                            TransactionType::AssetLock => {},
                            TransactionType::AssetUnlock => {},
                            TransactionType::TypeMax => {},
                            TransactionType::SubscriptionCloseAccount => {},
                            TransactionType::Transition => {},
                            TransactionType::CreditFunding => {},
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_Classic_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::Classic)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_ProviderRegistration_ctor(
            ) -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::ProviderRegistration)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_ProviderUpdateService_ctor(
            ) -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::ProviderUpdateService)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_ProviderUpdateRegistrar_ctor(
            ) -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::ProviderUpdateRegistrar)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_ProviderUpdateRevocation_ctor(
            ) -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::ProviderUpdateRevocation)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_Coinbase_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::Coinbase)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_QuorumCommitment_ctor() -> *mut TransactionType
            {
                ferment_interfaces::boxed(TransactionType::QuorumCommitment)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_AssetLock_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::AssetLock)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_AssetUnlock_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::AssetUnlock)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_TypeMax_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::TypeMax)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_SubscriptionCloseAccount_ctor(
            ) -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::SubscriptionCloseAccount)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_Transition_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::Transition)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_CreditFunding_ctor() -> *mut TransactionType {
                ferment_interfaces::boxed(TransactionType::CreditFunding)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionType_destroy(ffi: *mut TransactionType) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: tx :: transaction :: Transaction\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Transaction {
                pub inputs:
                    *mut crate::fermented::generics::Vec_crate_tx_transaction_TransactionInput,
                pub outputs:
                    *mut crate::fermented::generics::Vec_crate_tx_transaction_TransactionOutput,
                pub lock_time: u32,
                pub version: u16,
                pub tx_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub tx_type: *mut crate::fermented::types::tx::transaction::TransactionType,
                pub payload_offset: usize,
                pub block_height: u32,
            }
            impl ferment_interfaces::FFIConversion<crate::tx::transaction::Transaction> for Transaction {
                unsafe fn ffi_from_const(
                    ffi: *const Transaction,
                ) -> crate::tx::transaction::Transaction {
                    let ffi_ref = &*ffi;
                    crate::tx::transaction::Transaction {
                        inputs: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.inputs),
                        outputs: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.outputs),
                        lock_time: ffi_ref.lock_time,
                        version: ffi_ref.version,
                        tx_hash: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.tx_hash),
                        tx_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.tx_type),
                        payload_offset: ffi_ref.payload_offset,
                        block_height: ffi_ref.block_height,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::tx::transaction::Transaction,
                ) -> *const Transaction {
                    ferment_interfaces::boxed(Transaction {
                        inputs: ferment_interfaces::FFIConversion::ffi_to(obj.inputs),
                        outputs: ferment_interfaces::FFIConversion::ffi_to(obj.outputs),
                        lock_time: obj.lock_time,
                        version: obj.version,
                        tx_hash: ferment_interfaces::FFIConversion::ffi_to_opt(obj.tx_hash),
                        tx_type: ferment_interfaces::FFIConversion::ffi_to(obj.tx_type),
                        payload_offset: obj.payload_offset,
                        block_height: obj.block_height,
                    })
                }
                unsafe fn destroy(ffi: *mut Transaction) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Transaction {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.inputs);
                        ferment_interfaces::unbox_any(ffi_ref.outputs);
                        if !ffi_ref.tx_hash.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.tx_hash);
                        };
                        ferment_interfaces::unbox_any(ffi_ref.tx_type);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Transaction_ctor(
                inputs: *mut crate::fermented::generics::Vec_crate_tx_transaction_TransactionInput,
                outputs : * mut crate :: fermented :: generics :: Vec_crate_tx_transaction_TransactionOutput,
                lock_time: u32,
                version: u16,
                tx_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                tx_type: *mut crate::fermented::types::tx::transaction::TransactionType,
                payload_offset: usize,
                block_height: u32,
            ) -> *mut Transaction {
                ferment_interfaces::boxed(Transaction {
                    inputs,
                    outputs,
                    lock_time,
                    version,
                    tx_hash,
                    tx_type,
                    payload_offset,
                    block_height,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Transaction_destroy(ffi: *mut Transaction) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: tx :: transaction :: TransactionInput\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransactionInput {
                pub input_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub index: u32,
                pub script: *mut crate::fermented::generics::Vec_u8,
                pub signature: *mut crate::fermented::generics::Vec_u8,
                pub sequence: u32,
            }
            impl ferment_interfaces::FFIConversion<crate::tx::transaction::TransactionInput>
                for TransactionInput
            {
                unsafe fn ffi_from_const(
                    ffi: *const TransactionInput,
                ) -> crate::tx::transaction::TransactionInput {
                    let ffi_ref = &*ffi;
                    crate::tx::transaction::TransactionInput {
                        input_hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.input_hash),
                        index: ffi_ref.index,
                        script: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.script),
                        signature: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.signature,
                        ),
                        sequence: ffi_ref.sequence,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::tx::transaction::TransactionInput,
                ) -> *const TransactionInput {
                    ferment_interfaces::boxed(TransactionInput {
                        input_hash: ferment_interfaces::FFIConversion::ffi_to(obj.input_hash),
                        index: obj.index,
                        script: match obj.script {
                            Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                            None => std::ptr::null_mut(),
                        },
                        signature: match obj.signature {
                            Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                            None => std::ptr::null_mut(),
                        },
                        sequence: obj.sequence,
                    })
                }
                unsafe fn destroy(ffi: *mut TransactionInput) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for TransactionInput {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.input_hash);
                        if !ffi_ref.script.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.script);
                        };
                        if !ffi_ref.signature.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.signature);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionInput_ctor(
                input_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                index: u32,
                script: *mut crate::fermented::generics::Vec_u8,
                signature: *mut crate::fermented::generics::Vec_u8,
                sequence: u32,
            ) -> *mut TransactionInput {
                ferment_interfaces::boxed(TransactionInput {
                    input_hash,
                    index,
                    script,
                    signature,
                    sequence,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionInput_destroy(ffi: *mut TransactionInput) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: tx :: transaction :: TransactionOutput\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct TransactionOutput {
                pub amount: u64,
                pub script: *mut crate::fermented::generics::Vec_u8,
                pub address: *mut crate::fermented::generics::Vec_u8,
            }
            impl ferment_interfaces::FFIConversion<crate::tx::transaction::TransactionOutput>
                for TransactionOutput
            {
                unsafe fn ffi_from_const(
                    ffi: *const TransactionOutput,
                ) -> crate::tx::transaction::TransactionOutput {
                    let ffi_ref = &*ffi;
                    crate::tx::transaction::TransactionOutput {
                        amount: ffi_ref.amount,
                        script: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.script),
                        address: ferment_interfaces::FFIConversion::ffi_from_opt(ffi_ref.address),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::tx::transaction::TransactionOutput,
                ) -> *const TransactionOutput {
                    ferment_interfaces::boxed(TransactionOutput {
                        amount: obj.amount,
                        script: match obj.script {
                            Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                            None => std::ptr::null_mut(),
                        },
                        address: match obj.address {
                            Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                            None => std::ptr::null_mut(),
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut TransactionOutput) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for TransactionOutput {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        if !ffi_ref.script.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.script);
                        };
                        if !ffi_ref.address.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.address);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionOutput_ctor(
                amount: u64,
                script: *mut crate::fermented::generics::Vec_u8,
                address: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut TransactionOutput {
                ferment_interfaces::boxed(TransactionOutput {
                    amount,
                    script,
                    address,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn TransactionOutput_destroy(ffi: *mut TransactionOutput) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod coinbase_transaction {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: tx :: coinbase_transaction :: CoinbaseTransaction\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct CoinbaseTransaction {
                pub base: *mut crate::fermented::types::tx::transaction::Transaction,
                pub coinbase_transaction_version: u16,
                pub height: u32,
                pub merkle_root_mn_list: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub merkle_root_llmq_list: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub best_cl_height_diff: u64,
                pub best_cl_signature: *mut crate::fermented::types::crypto::byte_util::UInt768,
                pub credit_pool_balance: i64,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::tx::coinbase_transaction::CoinbaseTransaction,
                > for CoinbaseTransaction
            {
                unsafe fn ffi_from_const(
                    ffi: *const CoinbaseTransaction,
                ) -> crate::tx::coinbase_transaction::CoinbaseTransaction {
                    let ffi_ref = &*ffi;
                    crate::tx::coinbase_transaction::CoinbaseTransaction {
                        base: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.base),
                        coinbase_transaction_version: ffi_ref.coinbase_transaction_version,
                        height: ffi_ref.height,
                        merkle_root_mn_list: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.merkle_root_mn_list,
                        ),
                        merkle_root_llmq_list: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.merkle_root_llmq_list,
                        ),
                        best_cl_height_diff: ffi_ref.best_cl_height_diff,
                        best_cl_signature: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.best_cl_signature,
                        ),
                        credit_pool_balance: (ffi_ref.credit_pool_balance > 0)
                            .then_some(ffi_ref.credit_pool_balance),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::tx::coinbase_transaction::CoinbaseTransaction,
                ) -> *const CoinbaseTransaction {
                    ferment_interfaces::boxed(CoinbaseTransaction {
                        base: ferment_interfaces::FFIConversion::ffi_to(obj.base),
                        coinbase_transaction_version: obj.coinbase_transaction_version,
                        height: obj.height,
                        merkle_root_mn_list: ferment_interfaces::FFIConversion::ffi_to(
                            obj.merkle_root_mn_list,
                        ),
                        merkle_root_llmq_list: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.merkle_root_llmq_list,
                        ),
                        best_cl_height_diff: obj.best_cl_height_diff,
                        best_cl_signature: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.best_cl_signature,
                        ),
                        credit_pool_balance: obj.credit_pool_balance.unwrap_or(0),
                    })
                }
                unsafe fn destroy(ffi: *mut CoinbaseTransaction) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for CoinbaseTransaction {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.base);
                        ferment_interfaces::unbox_any(ffi_ref.merkle_root_mn_list);
                        if !ffi_ref.merkle_root_llmq_list.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.merkle_root_llmq_list);
                        };
                        if !ffi_ref.best_cl_signature.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.best_cl_signature);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoinbaseTransaction_ctor(
                base: *mut crate::fermented::types::tx::transaction::Transaction,
                coinbase_transaction_version: u16,
                height: u32,
                merkle_root_mn_list: *mut crate::fermented::types::crypto::byte_util::UInt256,
                merkle_root_llmq_list: *mut crate::fermented::types::crypto::byte_util::UInt256,
                best_cl_height_diff: u64,
                best_cl_signature: *mut crate::fermented::types::crypto::byte_util::UInt768,
                credit_pool_balance: i64,
            ) -> *mut CoinbaseTransaction {
                ferment_interfaces::boxed(CoinbaseTransaction {
                    base,
                    coinbase_transaction_version,
                    height,
                    merkle_root_mn_list,
                    merkle_root_llmq_list,
                    best_cl_height_diff,
                    best_cl_signature,
                    credit_pool_balance,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn CoinbaseTransaction_destroy(ffi: *mut CoinbaseTransaction) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod common {
        pub mod llmq_version {
            #[doc = "FFI-representation of the LLMQVersion"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum LLMQVersion {
                Default = 1,
                Indexed = 2,
                BLSBasicDefault = 3,
                BLSBasicIndexed = 4,
            }
            impl ferment_interfaces::FFIConversion<crate::common::llmq_version::LLMQVersion> for LLMQVersion {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQVersion,
                ) -> crate::common::llmq_version::LLMQVersion {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        LLMQVersion::Default => crate::common::llmq_version::LLMQVersion::Default,
                        LLMQVersion::Indexed => crate::common::llmq_version::LLMQVersion::Indexed,
                        LLMQVersion::BLSBasicDefault => {
                            crate::common::llmq_version::LLMQVersion::BLSBasicDefault
                        },
                        LLMQVersion::BLSBasicIndexed => {
                            crate::common::llmq_version::LLMQVersion::BLSBasicIndexed
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::common::llmq_version::LLMQVersion,
                ) -> *const LLMQVersion {
                    ferment_interfaces::boxed(match obj {
                        crate::common::llmq_version::LLMQVersion::Default => LLMQVersion::Default,
                        crate::common::llmq_version::LLMQVersion::Indexed => LLMQVersion::Indexed,
                        crate::common::llmq_version::LLMQVersion::BLSBasicDefault => {
                            LLMQVersion::BLSBasicDefault
                        },
                        crate::common::llmq_version::LLMQVersion::BLSBasicIndexed => {
                            LLMQVersion::BLSBasicIndexed
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut LLMQVersion) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQVersion {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            LLMQVersion::Default => {},
                            LLMQVersion::Indexed => {},
                            LLMQVersion::BLSBasicDefault => {},
                            LLMQVersion::BLSBasicIndexed => {},
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQVersion_Default_ctor() -> *mut LLMQVersion {
                ferment_interfaces::boxed(LLMQVersion::Default)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQVersion_Indexed_ctor() -> *mut LLMQVersion {
                ferment_interfaces::boxed(LLMQVersion::Indexed)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQVersion_BLSBasicDefault_ctor() -> *mut LLMQVersion {
                ferment_interfaces::boxed(LLMQVersion::BLSBasicDefault)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQVersion_BLSBasicIndexed_ctor() -> *mut LLMQVersion {
                ferment_interfaces::boxed(LLMQVersion::BLSBasicIndexed)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQVersion_destroy(ffi: *mut LLMQVersion) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod socket_address {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: common :: socket_address :: SocketAddress\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct SocketAddress {
                pub ip_address: *mut crate::fermented::types::crypto::byte_util::UInt128,
                pub port: u16,
            }
            impl ferment_interfaces::FFIConversion<crate::common::socket_address::SocketAddress>
                for SocketAddress
            {
                unsafe fn ffi_from_const(
                    ffi: *const SocketAddress,
                ) -> crate::common::socket_address::SocketAddress {
                    let ffi_ref = &*ffi;
                    crate::common::socket_address::SocketAddress {
                        ip_address: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.ip_address),
                        port: ffi_ref.port,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::common::socket_address::SocketAddress,
                ) -> *const SocketAddress {
                    ferment_interfaces::boxed(SocketAddress {
                        ip_address: ferment_interfaces::FFIConversion::ffi_to(obj.ip_address),
                        port: obj.port,
                    })
                }
                unsafe fn destroy(ffi: *mut SocketAddress) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for SocketAddress {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.ip_address);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn SocketAddress_ctor(
                ip_address: *mut crate::fermented::types::crypto::byte_util::UInt128,
                port: u16,
            ) -> *mut SocketAddress {
                ferment_interfaces::boxed(SocketAddress { ip_address, port })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn SocketAddress_destroy(ffi: *mut SocketAddress) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod bitset {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: common :: bitset :: Bitset\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Bitset {
                pub count: usize,
                pub bitset: *mut crate::fermented::generics::Vec_u8,
            }
            impl ferment_interfaces::FFIConversion<crate::common::bitset::Bitset> for Bitset {
                unsafe fn ffi_from_const(ffi: *const Bitset) -> crate::common::bitset::Bitset {
                    let ffi_ref = &*ffi;
                    crate::common::bitset::Bitset {
                        count: ffi_ref.count,
                        bitset: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.bitset),
                    }
                }
                unsafe fn ffi_to_const(obj: crate::common::bitset::Bitset) -> *const Bitset {
                    ferment_interfaces::boxed(Bitset {
                        count: obj.count,
                        bitset: ferment_interfaces::FFIConversion::ffi_to(obj.bitset),
                    })
                }
                unsafe fn destroy(ffi: *mut Bitset) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Bitset {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.bitset);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Bitset_ctor(
                count: usize,
                bitset: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut Bitset {
                ferment_interfaces::boxed(Bitset { count, bitset })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Bitset_destroy(ffi: *mut Bitset) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod block {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: common :: block :: Block\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct Block {
                pub height: u32,
                pub hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            }
            impl ferment_interfaces::FFIConversion<crate::common::block::Block> for Block {
                unsafe fn ffi_from_const(ffi: *const Block) -> crate::common::block::Block {
                    let ffi_ref = &*ffi;
                    crate::common::block::Block {
                        height: ffi_ref.height,
                        hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.hash),
                    }
                }
                unsafe fn ffi_to_const(obj: crate::common::block::Block) -> *const Block {
                    ferment_interfaces::boxed(Block {
                        height: obj.height,
                        hash: ferment_interfaces::FFIConversion::ffi_to(obj.hash),
                    })
                }
                unsafe fn destroy(ffi: *mut Block) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Block {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.hash);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Block_ctor(
                height: u32,
                hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            ) -> *mut Block {
                ferment_interfaces::boxed(Block { height, hash })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn Block_destroy(ffi: *mut Block) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod masternode_type {
            #[doc = "FFI-representation of the MasternodeType"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum MasternodeType {
                Regular = 0,
                HighPerformance = 1,
            }
            impl ferment_interfaces::FFIConversion<crate::common::masternode_type::MasternodeType>
                for MasternodeType
            {
                unsafe fn ffi_from_const(
                    ffi: *const MasternodeType,
                ) -> crate::common::masternode_type::MasternodeType {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        MasternodeType::Regular => {
                            crate::common::masternode_type::MasternodeType::Regular
                        },
                        MasternodeType::HighPerformance => {
                            crate::common::masternode_type::MasternodeType::HighPerformance
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::common::masternode_type::MasternodeType,
                ) -> *const MasternodeType {
                    ferment_interfaces::boxed(match obj {
                        crate::common::masternode_type::MasternodeType::Regular => {
                            MasternodeType::Regular
                        },
                        crate::common::masternode_type::MasternodeType::HighPerformance => {
                            MasternodeType::HighPerformance
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut MasternodeType) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for MasternodeType {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            MasternodeType::Regular => {},
                            MasternodeType::HighPerformance => {},
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeType_Regular_ctor() -> *mut MasternodeType {
                ferment_interfaces::boxed(MasternodeType::Regular)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeType_HighPerformance_ctor() -> *mut MasternodeType {
                ferment_interfaces::boxed(MasternodeType::HighPerformance)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeType_destroy(ffi: *mut MasternodeType) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod llmq_snapshot_skip_mode {
            #[doc = "FFI-representation of the LLMQSnapshotSkipMode"]
            #[repr(C)]
            #[allow(non_camel_case_types)]
            #[derive(Clone)]
            pub enum LLMQSnapshotSkipMode {
                NoSkipping = 0,
                SkipFirst = 1,
                SkipExcept = 2,
                SkipAll = 3,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode,
                > for LLMQSnapshotSkipMode
            {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQSnapshotSkipMode,
                ) -> crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        LLMQSnapshotSkipMode::NoSkipping => {
                            crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode::NoSkipping
                        },
                        LLMQSnapshotSkipMode::SkipFirst => {
                            crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode::SkipFirst
                        },
                        LLMQSnapshotSkipMode::SkipExcept => {
                            crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode::SkipExcept
                        },
                        LLMQSnapshotSkipMode::SkipAll => {
                            crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode::SkipAll
                        },
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode,
                ) -> *const LLMQSnapshotSkipMode {
                    ferment_interfaces :: boxed (match obj { crate :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode :: NoSkipping => LLMQSnapshotSkipMode :: NoSkipping , crate :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode :: SkipFirst => LLMQSnapshotSkipMode :: SkipFirst , crate :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode :: SkipExcept => LLMQSnapshotSkipMode :: SkipExcept , crate :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode :: SkipAll => LLMQSnapshotSkipMode :: SkipAll , })
                }
                unsafe fn destroy(ffi: *mut LLMQSnapshotSkipMode) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQSnapshotSkipMode {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            LLMQSnapshotSkipMode::NoSkipping => {},
                            LLMQSnapshotSkipMode::SkipFirst => {},
                            LLMQSnapshotSkipMode::SkipExcept => {},
                            LLMQSnapshotSkipMode::SkipAll => {},
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshotSkipMode_NoSkipping_ctor(
            ) -> *mut LLMQSnapshotSkipMode {
                ferment_interfaces::boxed(LLMQSnapshotSkipMode::NoSkipping)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshotSkipMode_SkipFirst_ctor(
            ) -> *mut LLMQSnapshotSkipMode {
                ferment_interfaces::boxed(LLMQSnapshotSkipMode::SkipFirst)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshotSkipMode_SkipExcept_ctor(
            ) -> *mut LLMQSnapshotSkipMode {
                ferment_interfaces::boxed(LLMQSnapshotSkipMode::SkipExcept)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshotSkipMode_SkipAll_ctor() -> *mut LLMQSnapshotSkipMode
            {
                ferment_interfaces::boxed(LLMQSnapshotSkipMode::SkipAll)
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshotSkipMode_destroy(ffi: *mut LLMQSnapshotSkipMode) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
    pub mod chain {
        pub mod common {
            pub mod chain_type {
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_VTable { pub genesis_hash : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub genesis_height : unsafe extern "C" fn (obj : * const () ,) -> u32 , pub is_llmq_type : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType , pub isd_llmq_type : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType , pub chain_locks_type : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType , pub platform_type : unsafe extern "C" fn (obj : * const () ,) -> * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType , pub should_process_llmq_of_type : unsafe extern "C" fn (obj : * const () , llmq_type : * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType ,) -> bool , pub is_evolution_enabled : unsafe extern "C" fn (obj : * const () ,) -> bool , pub name : unsafe extern "C" fn (obj : * const () ,) -> * mut std :: os :: raw :: c_char , }
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct IHaveChainSettings_TraitObject {
                    pub object: *const (),
                    pub vtable: *const IHaveChainSettings_VTable,
                }
                #[doc = "FFI-representation of the ChainType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum ChainType {
                    MainNet,
                    TestNet,
                    DevNet(*mut crate::fermented::types::chain::common::chain_type::DevnetType),
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::chain_type::ChainType> for ChainType {
                    unsafe fn ffi_from_const(
                        ffi: *const ChainType,
                    ) -> crate::chain::common::chain_type::ChainType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ChainType::MainNet => {
                                crate::chain::common::chain_type::ChainType::MainNet
                            },
                            ChainType::TestNet => {
                                crate::chain::common::chain_type::ChainType::TestNet
                            },
                            ChainType::DevNet(o_0) => {
                                crate::chain::common::chain_type::ChainType::DevNet(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::chain_type::ChainType,
                    ) -> *const ChainType {
                        ferment_interfaces::boxed(match obj {
                            crate::chain::common::chain_type::ChainType::MainNet => {
                                ChainType::MainNet
                            },
                            crate::chain::common::chain_type::ChainType::TestNet => {
                                ChainType::TestNet
                            },
                            crate::chain::common::chain_type::ChainType::DevNet(o_0) => {
                                ChainType::DevNet(ferment_interfaces::FFIConversion::ffi_to(o_0))
                            },
                        })
                    }
                    unsafe fn destroy(ffi: *mut ChainType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ChainType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ChainType::MainNet => {},
                                ChainType::TestNet => {},
                                ChainType::DevNet(o_0) => {
                                    ferment_interfaces::unbox_any(o_0.to_owned());
                                },
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_MainNet_ctor() -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::MainNet)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_TestNet_ctor() -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::TestNet)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_DevNet_ctor(
                    o_o_0: *mut crate::fermented::types::chain::common::chain_type::DevnetType,
                ) -> *mut ChainType {
                    ferment_interfaces::boxed(ChainType::DevNet(o_o_0))
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_destroy(ffi: *mut ChainType) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[allow(non_snake_case, non_upper_case_globals)]
                static ChainType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn ChainType_genesis_hash(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::crypto::byte_util::UInt256
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_hash (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_height (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn ChainType_is_llmq_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: is_llmq_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_isd_llmq_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: isd_llmq_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_chain_locks_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: chain_locks_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_platform_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: platform_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn ChainType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: should_process_llmq_of_type (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (llmq_type) ,) ;
                        obj
                    }
                    unsafe extern "C" fn ChainType_is_evolution_enabled(obj: *const ()) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: is_evolution_enabled (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn ChainType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::ChainType));
                        let obj = < crate :: chain :: common :: chain_type :: ChainType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: name (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    IHaveChainSettings_VTable {
                        genesis_hash: ChainType_genesis_hash,
                        genesis_height: ChainType_genesis_height,
                        is_llmq_type: ChainType_is_llmq_type,
                        isd_llmq_type: ChainType_isd_llmq_type,
                        chain_locks_type: ChainType_chain_locks_type,
                        platform_type: ChainType_platform_type,
                        should_process_llmq_of_type: ChainType_should_process_llmq_of_type,
                        is_evolution_enabled: ChainType_is_evolution_enabled,
                        name: ChainType_name,
                    }
                };
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub extern "C" fn ChainType_as_IHaveChainSettings_TraitObject(
                    obj: *const crate::chain::common::chain_type::ChainType,
                ) -> IHaveChainSettings_TraitObject {
                    IHaveChainSettings_TraitObject {
                        object: obj as *const (),
                        vtable: &ChainType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn ChainType_as_IHaveChainSettings_TraitObject_destroy(
                    obj: IHaveChainSettings_TraitObject,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::ChainType,
                    );
                }
                #[doc = "FFI-representation of the DevnetType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum DevnetType {
                    JackDaniels = 0,
                    Devnet333 = 1,
                    Chacha = 2,
                    Mojito = 3,
                    WhiteRussian = 4,
                    MiningTest = 5,
                    Mobile2 = 6,
                    Zero = 7,
                    Screwdriver = 8,
                    Absinthe = 9,
                    Bintang = 10,
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::chain_type::DevnetType>
                    for DevnetType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const DevnetType,
                    ) -> crate::chain::common::chain_type::DevnetType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            DevnetType::JackDaniels => {
                                crate::chain::common::chain_type::DevnetType::JackDaniels
                            },
                            DevnetType::Devnet333 => {
                                crate::chain::common::chain_type::DevnetType::Devnet333
                            },
                            DevnetType::Chacha => {
                                crate::chain::common::chain_type::DevnetType::Chacha
                            },
                            DevnetType::Mojito => {
                                crate::chain::common::chain_type::DevnetType::Mojito
                            },
                            DevnetType::WhiteRussian => {
                                crate::chain::common::chain_type::DevnetType::WhiteRussian
                            },
                            DevnetType::MiningTest => {
                                crate::chain::common::chain_type::DevnetType::MiningTest
                            },
                            DevnetType::Mobile2 => {
                                crate::chain::common::chain_type::DevnetType::Mobile2
                            },
                            DevnetType::Zero => crate::chain::common::chain_type::DevnetType::Zero,
                            DevnetType::Screwdriver => {
                                crate::chain::common::chain_type::DevnetType::Screwdriver
                            },
                            DevnetType::Absinthe => {
                                crate::chain::common::chain_type::DevnetType::Absinthe
                            },
                            DevnetType::Bintang => {
                                crate::chain::common::chain_type::DevnetType::Bintang
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::chain_type::DevnetType,
                    ) -> *const DevnetType {
                        ferment_interfaces::boxed(match obj {
                            crate::chain::common::chain_type::DevnetType::JackDaniels => {
                                DevnetType::JackDaniels
                            },
                            crate::chain::common::chain_type::DevnetType::Devnet333 => {
                                DevnetType::Devnet333
                            },
                            crate::chain::common::chain_type::DevnetType::Chacha => {
                                DevnetType::Chacha
                            },
                            crate::chain::common::chain_type::DevnetType::Mojito => {
                                DevnetType::Mojito
                            },
                            crate::chain::common::chain_type::DevnetType::WhiteRussian => {
                                DevnetType::WhiteRussian
                            },
                            crate::chain::common::chain_type::DevnetType::MiningTest => {
                                DevnetType::MiningTest
                            },
                            crate::chain::common::chain_type::DevnetType::Mobile2 => {
                                DevnetType::Mobile2
                            },
                            crate::chain::common::chain_type::DevnetType::Zero => DevnetType::Zero,
                            crate::chain::common::chain_type::DevnetType::Screwdriver => {
                                DevnetType::Screwdriver
                            },
                            crate::chain::common::chain_type::DevnetType::Absinthe => {
                                DevnetType::Absinthe
                            },
                            crate::chain::common::chain_type::DevnetType::Bintang => {
                                DevnetType::Bintang
                            },
                        })
                    }
                    unsafe fn destroy(ffi: *mut DevnetType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for DevnetType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                DevnetType::JackDaniels => {},
                                DevnetType::Devnet333 => {},
                                DevnetType::Chacha => {},
                                DevnetType::Mojito => {},
                                DevnetType::WhiteRussian => {},
                                DevnetType::MiningTest => {},
                                DevnetType::Mobile2 => {},
                                DevnetType::Zero => {},
                                DevnetType::Screwdriver => {},
                                DevnetType::Absinthe => {},
                                DevnetType::Bintang => {},
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_JackDaniels_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::JackDaniels)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Devnet333_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Devnet333)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Chacha_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Chacha)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Mojito_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Mojito)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_WhiteRussian_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::WhiteRussian)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_MiningTest_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::MiningTest)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Mobile2_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Mobile2)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Zero_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Zero)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Screwdriver_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Screwdriver)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Absinthe_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Absinthe)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_Bintang_ctor() -> *mut DevnetType {
                    ferment_interfaces::boxed(DevnetType::Bintang)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_destroy(ffi: *mut DevnetType) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[allow(non_snake_case, non_upper_case_globals)]
                static DevnetType_IHaveChainSettings_VTable: IHaveChainSettings_VTable = {
                    unsafe extern "C" fn DevnetType_genesis_hash(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::crypto::byte_util::UInt256
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_hash (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_genesis_height(obj: *const ()) -> u32 {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: genesis_height (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn DevnetType_is_llmq_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: is_llmq_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_isd_llmq_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: isd_llmq_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_chain_locks_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: chain_locks_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_platform_type(
                        obj: *const (),
                    ) -> *mut crate::fermented::types::chain::common::llmq_type::LLMQType
                    {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: platform_type (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    unsafe extern "C" fn DevnetType_should_process_llmq_of_type(
                        obj: *const (),
                        llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                    ) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: should_process_llmq_of_type (cast_obj , ferment_interfaces :: FFIConversion :: ffi_from (llmq_type) ,) ;
                        obj
                    }
                    unsafe extern "C" fn DevnetType_is_evolution_enabled(obj: *const ()) -> bool {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: is_evolution_enabled (cast_obj ,) ;
                        obj
                    }
                    unsafe extern "C" fn DevnetType_name(
                        obj: *const (),
                    ) -> *mut std::os::raw::c_char {
                        let cast_obj =
                            &(*(obj as *const crate::chain::common::chain_type::DevnetType));
                        let obj = < crate :: chain :: common :: chain_type :: DevnetType as crate :: chain :: common :: chain_type :: IHaveChainSettings > :: name (cast_obj ,) ;
                        ferment_interfaces::FFIConversion::ffi_to(obj)
                    }
                    IHaveChainSettings_VTable {
                        genesis_hash: DevnetType_genesis_hash,
                        genesis_height: DevnetType_genesis_height,
                        is_llmq_type: DevnetType_is_llmq_type,
                        isd_llmq_type: DevnetType_isd_llmq_type,
                        chain_locks_type: DevnetType_chain_locks_type,
                        platform_type: DevnetType_platform_type,
                        should_process_llmq_of_type: DevnetType_should_process_llmq_of_type,
                        is_evolution_enabled: DevnetType_is_evolution_enabled,
                        name: DevnetType_name,
                    }
                };
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub extern "C" fn DevnetType_as_IHaveChainSettings_TraitObject(
                    obj: *const crate::chain::common::chain_type::DevnetType,
                ) -> IHaveChainSettings_TraitObject {
                    IHaveChainSettings_TraitObject {
                        object: obj as *const (),
                        vtable: &DevnetType_IHaveChainSettings_VTable,
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DevnetType_as_IHaveChainSettings_TraitObject_destroy(
                    obj: IHaveChainSettings_TraitObject,
                ) {
                    ferment_interfaces::unbox_any(
                        obj.object as *mut crate::chain::common::chain_type::DevnetType,
                    );
                }
            }
            pub mod llmq_type {
                #[doc = "FFI-representation of the LLMQType"]
                #[repr(C)]
                #[allow(non_camel_case_types)]
                #[derive(Clone)]
                pub enum LLMQType {
                    LlmqtypeUnknown = 0,
                    Llmqtype50_60 = 1,
                    Llmqtype400_60 = 2,
                    Llmqtype400_85 = 3,
                    Llmqtype100_67 = 4,
                    Llmqtype60_75 = 5,
                    Llmqtype25_67 = 6,
                    LlmqtypeTest = 100,
                    LlmqtypeDevnet = 101,
                    LlmqtypeTestV17 = 102,
                    LlmqtypeTestDIP0024 = 103,
                    LlmqtypeTestInstantSend = 104,
                    LlmqtypeDevnetDIP0024 = 105,
                    LlmqtypeTestnetPlatform = 106,
                    LlmqtypeDevnetPlatform = 107,
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::llmq_type::LLMQType> for LLMQType {
                    unsafe fn ffi_from_const(
                        ffi: *const LLMQType,
                    ) -> crate::chain::common::llmq_type::LLMQType {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            LLMQType::LlmqtypeUnknown => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeUnknown
                            },
                            LLMQType::Llmqtype50_60 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype50_60
                            },
                            LLMQType::Llmqtype400_60 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype400_60
                            },
                            LLMQType::Llmqtype400_85 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype400_85
                            },
                            LLMQType::Llmqtype100_67 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype100_67
                            },
                            LLMQType::Llmqtype60_75 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype60_75
                            },
                            LLMQType::Llmqtype25_67 => {
                                crate::chain::common::llmq_type::LLMQType::Llmqtype25_67
                            },
                            LLMQType::LlmqtypeTest => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeTest
                            },
                            LLMQType::LlmqtypeDevnet => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnet
                            },
                            LLMQType::LlmqtypeTestV17 => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeTestV17
                            },
                            LLMQType::LlmqtypeTestDIP0024 => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeTestDIP0024
                            },
                            LLMQType::LlmqtypeTestInstantSend => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeTestInstantSend
                            },
                            LLMQType::LlmqtypeDevnetDIP0024 => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnetDIP0024
                            },
                            LLMQType::LlmqtypeTestnetPlatform => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeTestnetPlatform
                            },
                            LLMQType::LlmqtypeDevnetPlatform => {
                                crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnetPlatform
                            },
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::llmq_type::LLMQType,
                    ) -> *const LLMQType {
                        ferment_interfaces::boxed(match obj {
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeUnknown => {
                                LLMQType::LlmqtypeUnknown
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype50_60 => {
                                LLMQType::Llmqtype50_60
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype400_60 => {
                                LLMQType::Llmqtype400_60
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype400_85 => {
                                LLMQType::Llmqtype400_85
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype100_67 => {
                                LLMQType::Llmqtype100_67
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype60_75 => {
                                LLMQType::Llmqtype60_75
                            },
                            crate::chain::common::llmq_type::LLMQType::Llmqtype25_67 => {
                                LLMQType::Llmqtype25_67
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeTest => {
                                LLMQType::LlmqtypeTest
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnet => {
                                LLMQType::LlmqtypeDevnet
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeTestV17 => {
                                LLMQType::LlmqtypeTestV17
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeTestDIP0024 => {
                                LLMQType::LlmqtypeTestDIP0024
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeTestInstantSend => {
                                LLMQType::LlmqtypeTestInstantSend
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnetDIP0024 => {
                                LLMQType::LlmqtypeDevnetDIP0024
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeTestnetPlatform => {
                                LLMQType::LlmqtypeTestnetPlatform
                            },
                            crate::chain::common::llmq_type::LLMQType::LlmqtypeDevnetPlatform => {
                                LLMQType::LlmqtypeDevnetPlatform
                            },
                        })
                    }
                    unsafe fn destroy(ffi: *mut LLMQType) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for LLMQType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                LLMQType::LlmqtypeUnknown => {},
                                LLMQType::Llmqtype50_60 => {},
                                LLMQType::Llmqtype400_60 => {},
                                LLMQType::Llmqtype400_85 => {},
                                LLMQType::Llmqtype100_67 => {},
                                LLMQType::Llmqtype60_75 => {},
                                LLMQType::Llmqtype25_67 => {},
                                LLMQType::LlmqtypeTest => {},
                                LLMQType::LlmqtypeDevnet => {},
                                LLMQType::LlmqtypeTestV17 => {},
                                LLMQType::LlmqtypeTestDIP0024 => {},
                                LLMQType::LlmqtypeTestInstantSend => {},
                                LLMQType::LlmqtypeDevnetDIP0024 => {},
                                LLMQType::LlmqtypeTestnetPlatform => {},
                                LLMQType::LlmqtypeDevnetPlatform => {},
                            }
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeUnknown_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeUnknown)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype50_60_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype50_60)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype400_60_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype400_60)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype400_85_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype400_85)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype100_67_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype100_67)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype60_75_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype60_75)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_Llmqtype25_67_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::Llmqtype25_67)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeTest_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeTest)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeDevnet_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeDevnet)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeTestV17_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeTestV17)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeTestDIP0024_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeTestDIP0024)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeTestInstantSend_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeTestInstantSend)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeDevnetDIP0024_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeDevnetDIP0024)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeTestnetPlatform_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeTestnetPlatform)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_LlmqtypeDevnetPlatform_ctor() -> *mut LLMQType {
                    ferment_interfaces::boxed(LLMQType::LlmqtypeDevnetPlatform)
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQType_destroy(ffi: *mut LLMQType) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: chain :: common :: llmq_type :: LLMQParams\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct LLMQParams {
                    pub r#type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                    pub name: *mut std::os::raw::c_char,
                    pub size: u32,
                    pub min_size: u32,
                    pub threshold: u32,
                    pub dkg_params:
                        *mut crate::fermented::types::chain::common::llmq_type::DKGParams,
                    pub signing_active_quorum_count: u32,
                    pub keep_old_connections: u32,
                    pub recovery_members: u32,
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::llmq_type::LLMQParams> for LLMQParams {
                    unsafe fn ffi_from_const(
                        ffi: *const LLMQParams,
                    ) -> crate::chain::common::llmq_type::LLMQParams {
                        let ffi_ref = &*ffi;
                        crate::chain::common::llmq_type::LLMQParams {
                            r#type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.r#type),
                            name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                            size: ffi_ref.size,
                            min_size: ffi_ref.min_size,
                            threshold: ffi_ref.threshold,
                            dkg_params: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.dkg_params,
                            ),
                            signing_active_quorum_count: ffi_ref.signing_active_quorum_count,
                            keep_old_connections: ffi_ref.keep_old_connections,
                            recovery_members: ffi_ref.recovery_members,
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::llmq_type::LLMQParams,
                    ) -> *const LLMQParams {
                        ferment_interfaces::boxed(LLMQParams {
                            r#type: ferment_interfaces::FFIConversion::ffi_to(obj.r#type),
                            name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                            size: obj.size,
                            min_size: obj.min_size,
                            threshold: obj.threshold,
                            dkg_params: ferment_interfaces::FFIConversion::ffi_to(obj.dkg_params),
                            signing_active_quorum_count: obj.signing_active_quorum_count,
                            keep_old_connections: obj.keep_old_connections,
                            recovery_members: obj.recovery_members,
                        })
                    }
                    unsafe fn destroy(ffi: *mut LLMQParams) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for LLMQParams {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.r#type);
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (ffi_ref . name) ;
                            ferment_interfaces::unbox_any(ffi_ref.dkg_params);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQParams_ctor(
                    r#type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                    name: *mut std::os::raw::c_char,
                    size: u32,
                    min_size: u32,
                    threshold: u32,
                    dkg_params: *mut crate::fermented::types::chain::common::llmq_type::DKGParams,
                    signing_active_quorum_count: u32,
                    keep_old_connections: u32,
                    recovery_members: u32,
                ) -> *mut LLMQParams {
                    ferment_interfaces::boxed(LLMQParams {
                        r#type,
                        name,
                        size,
                        min_size,
                        threshold,
                        dkg_params,
                        signing_active_quorum_count,
                        keep_old_connections,
                        recovery_members,
                    })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn LLMQParams_destroy(ffi: *mut LLMQParams) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: chain :: common :: llmq_type :: DKGParams\"]"]
                #[repr(C)]
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub struct DKGParams {
                    pub interval: u32,
                    pub phase_blocks: u32,
                    pub mining_window_start: u32,
                    pub mining_window_end: u32,
                    pub bad_votes_threshold: u32,
                }
                impl ferment_interfaces::FFIConversion<crate::chain::common::llmq_type::DKGParams> for DKGParams {
                    unsafe fn ffi_from_const(
                        ffi: *const DKGParams,
                    ) -> crate::chain::common::llmq_type::DKGParams {
                        let ffi_ref = &*ffi;
                        crate::chain::common::llmq_type::DKGParams {
                            interval: ffi_ref.interval,
                            phase_blocks: ffi_ref.phase_blocks,
                            mining_window_start: ffi_ref.mining_window_start,
                            mining_window_end: ffi_ref.mining_window_end,
                            bad_votes_threshold: ffi_ref.bad_votes_threshold,
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: crate::chain::common::llmq_type::DKGParams,
                    ) -> *const DKGParams {
                        ferment_interfaces::boxed(DKGParams {
                            interval: obj.interval,
                            phase_blocks: obj.phase_blocks,
                            mining_window_start: obj.mining_window_start,
                            mining_window_end: obj.mining_window_end,
                            bad_votes_threshold: obj.bad_votes_threshold,
                        })
                    }
                    unsafe fn destroy(ffi: *mut DKGParams) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for DKGParams {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DKGParams_ctor(
                    interval: u32,
                    phase_blocks: u32,
                    mining_window_start: u32,
                    mining_window_end: u32,
                    bad_votes_threshold: u32,
                ) -> *mut DKGParams {
                    ferment_interfaces::boxed(DKGParams {
                        interval,
                        phase_blocks,
                        mining_window_start,
                        mining_window_end,
                        bad_votes_threshold,
                    })
                }
                #[doc = r" # Safety"]
                #[allow(non_snake_case)]
                #[no_mangle]
                pub unsafe extern "C" fn DKGParams_destroy(ffi: *mut DKGParams) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
        }
    }
    pub mod models {
        pub mod masternode_entry {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: masternode_entry :: MasternodeEntry\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MasternodeEntry { pub provider_registration_transaction_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub confirmed_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub confirmed_hash_hashed_with_provider_registration_transaction_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub socket_address : * mut crate :: fermented :: types :: common :: socket_address :: SocketAddress , pub operator_public_key : * mut crate :: fermented :: types :: models :: operator_public_key :: OperatorPublicKey , pub previous_operator_public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey , pub previous_entry_hashes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256 , pub previous_validity : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_bool , pub known_confirmed_at_height : u32 , pub update_height : u32 , pub key_id_voting : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt160 , pub is_valid : bool , pub mn_type : * mut crate :: fermented :: types :: common :: masternode_type :: MasternodeType , pub platform_http_port : u16 , pub platform_node_id : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt160 , pub entry_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , }
            impl ferment_interfaces::FFIConversion<crate::models::masternode_entry::MasternodeEntry>
                for MasternodeEntry
            {
                unsafe fn ffi_from_const(
                    ffi: *const MasternodeEntry,
                ) -> crate::models::masternode_entry::MasternodeEntry {
                    let ffi_ref = &*ffi;
                    crate :: models :: masternode_entry :: MasternodeEntry { provider_registration_transaction_hash : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . provider_registration_transaction_hash) , confirmed_hash : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . confirmed_hash) , confirmed_hash_hashed_with_provider_registration_transaction_hash : ferment_interfaces :: FFIConversion :: ffi_from_opt (ffi_ref . confirmed_hash_hashed_with_provider_registration_transaction_hash) , socket_address : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . socket_address) , operator_public_key : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . operator_public_key) , previous_operator_public_keys : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . previous_operator_public_keys) , previous_entry_hashes : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . previous_entry_hashes) , previous_validity : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . previous_validity) , known_confirmed_at_height : (ffi_ref . known_confirmed_at_height > 0) . then_some (ffi_ref . known_confirmed_at_height) , update_height : ffi_ref . update_height , key_id_voting : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . key_id_voting) , is_valid : ffi_ref . is_valid , mn_type : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . mn_type) , platform_http_port : ffi_ref . platform_http_port , platform_node_id : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . platform_node_id) , entry_hash : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . entry_hash) , }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::masternode_entry::MasternodeEntry,
                ) -> *const MasternodeEntry {
                    ferment_interfaces :: boxed (MasternodeEntry { provider_registration_transaction_hash : ferment_interfaces :: FFIConversion :: ffi_to (obj . provider_registration_transaction_hash) , confirmed_hash : ferment_interfaces :: FFIConversion :: ffi_to (obj . confirmed_hash) , confirmed_hash_hashed_with_provider_registration_transaction_hash : ferment_interfaces :: FFIConversion :: ffi_to_opt (obj . confirmed_hash_hashed_with_provider_registration_transaction_hash) , socket_address : ferment_interfaces :: FFIConversion :: ffi_to (obj . socket_address) , operator_public_key : ferment_interfaces :: FFIConversion :: ffi_to (obj . operator_public_key) , previous_operator_public_keys : ferment_interfaces :: FFIConversion :: ffi_to (obj . previous_operator_public_keys) , previous_entry_hashes : ferment_interfaces :: FFIConversion :: ffi_to (obj . previous_entry_hashes) , previous_validity : ferment_interfaces :: FFIConversion :: ffi_to (obj . previous_validity) , known_confirmed_at_height : obj . known_confirmed_at_height . unwrap_or (0) , update_height : obj . update_height , key_id_voting : ferment_interfaces :: FFIConversion :: ffi_to (obj . key_id_voting) , is_valid : obj . is_valid , mn_type : ferment_interfaces :: FFIConversion :: ffi_to (obj . mn_type) , platform_http_port : obj . platform_http_port , platform_node_id : ferment_interfaces :: FFIConversion :: ffi_to (obj . platform_node_id) , entry_hash : ferment_interfaces :: FFIConversion :: ffi_to (obj . entry_hash) , })
                }
                unsafe fn destroy(ffi: *mut MasternodeEntry) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for MasternodeEntry {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(
                            ffi_ref.provider_registration_transaction_hash,
                        );
                        ferment_interfaces::unbox_any(ffi_ref.confirmed_hash);
                        if !ffi_ref
                            .confirmed_hash_hashed_with_provider_registration_transaction_hash
                            .is_null()
                        {
                            ferment_interfaces :: unbox_any (ffi_ref . confirmed_hash_hashed_with_provider_registration_transaction_hash) ;
                        };
                        ferment_interfaces::unbox_any(ffi_ref.socket_address);
                        ferment_interfaces::unbox_any(ffi_ref.operator_public_key);
                        ferment_interfaces::unbox_any(ffi_ref.previous_operator_public_keys);
                        ferment_interfaces::unbox_any(ffi_ref.previous_entry_hashes);
                        ferment_interfaces::unbox_any(ffi_ref.previous_validity);
                        ferment_interfaces::unbox_any(ffi_ref.key_id_voting);
                        ferment_interfaces::unbox_any(ffi_ref.mn_type);
                        ferment_interfaces::unbox_any(ffi_ref.platform_node_id);
                        ferment_interfaces::unbox_any(ffi_ref.entry_hash);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeEntry_ctor(
                provider_registration_transaction_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256,
                confirmed_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                confirmed_hash_hashed_with_provider_registration_transaction_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256,
                socket_address: *mut crate::fermented::types::common::socket_address::SocketAddress,
                operator_public_key : * mut crate :: fermented :: types :: models :: operator_public_key :: OperatorPublicKey,
                previous_operator_public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey,
                previous_entry_hashes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256,
                previous_validity : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_common_block_Block_values_bool,
                known_confirmed_at_height: u32,
                update_height: u32,
                key_id_voting: *mut crate::fermented::types::crypto::byte_util::UInt160,
                is_valid: bool,
                mn_type: *mut crate::fermented::types::common::masternode_type::MasternodeType,
                platform_http_port: u16,
                platform_node_id: *mut crate::fermented::types::crypto::byte_util::UInt160,
                entry_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            ) -> *mut MasternodeEntry {
                ferment_interfaces::boxed(MasternodeEntry {
                    provider_registration_transaction_hash,
                    confirmed_hash,
                    confirmed_hash_hashed_with_provider_registration_transaction_hash,
                    socket_address,
                    operator_public_key,
                    previous_operator_public_keys,
                    previous_entry_hashes,
                    previous_validity,
                    known_confirmed_at_height,
                    update_height,
                    key_id_voting,
                    is_valid,
                    mn_type,
                    platform_http_port,
                    platform_node_id,
                    entry_hash,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeEntry_destroy(ffi: *mut MasternodeEntry) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod llmq_indexed_hash {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: llmq_indexed_hash :: LLMQIndexedHash\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct LLMQIndexedHash {
                pub index: u32,
                pub hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            }
            impl
                ferment_interfaces::FFIConversion<crate::models::llmq_indexed_hash::LLMQIndexedHash>
                for LLMQIndexedHash
            {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQIndexedHash,
                ) -> crate::models::llmq_indexed_hash::LLMQIndexedHash {
                    let ffi_ref = &*ffi;
                    crate::models::llmq_indexed_hash::LLMQIndexedHash {
                        index: ffi_ref.index,
                        hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.hash),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::llmq_indexed_hash::LLMQIndexedHash,
                ) -> *const LLMQIndexedHash {
                    ferment_interfaces::boxed(LLMQIndexedHash {
                        index: obj.index,
                        hash: ferment_interfaces::FFIConversion::ffi_to(obj.hash),
                    })
                }
                unsafe fn destroy(ffi: *mut LLMQIndexedHash) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQIndexedHash {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.hash);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQIndexedHash_ctor(
                index: u32,
                hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            ) -> *mut LLMQIndexedHash {
                ferment_interfaces::boxed(LLMQIndexedHash { index, hash })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQIndexedHash_destroy(ffi: *mut LLMQIndexedHash) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod masternode_list {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: masternode_list :: MasternodeList\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct MasternodeList { pub block_hash : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub known_height : u32 , pub masternode_merkle_root : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub llmq_merkle_root : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt256 , pub masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry , pub quorums : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry , }
            impl ferment_interfaces::FFIConversion<crate::models::masternode_list::MasternodeList>
                for MasternodeList
            {
                unsafe fn ffi_from_const(
                    ffi: *const MasternodeList,
                ) -> crate::models::masternode_list::MasternodeList {
                    let ffi_ref = &*ffi;
                    crate::models::masternode_list::MasternodeList {
                        block_hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.block_hash),
                        known_height: ffi_ref.known_height,
                        masternode_merkle_root: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.masternode_merkle_root,
                        ),
                        llmq_merkle_root: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.llmq_merkle_root,
                        ),
                        masternodes: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.masternodes,
                        ),
                        quorums: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.quorums),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::masternode_list::MasternodeList,
                ) -> *const MasternodeList {
                    ferment_interfaces::boxed(MasternodeList {
                        block_hash: ferment_interfaces::FFIConversion::ffi_to(obj.block_hash),
                        known_height: obj.known_height,
                        masternode_merkle_root: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.masternode_merkle_root,
                        ),
                        llmq_merkle_root: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.llmq_merkle_root,
                        ),
                        masternodes: ferment_interfaces::FFIConversion::ffi_to(obj.masternodes),
                        quorums: ferment_interfaces::FFIConversion::ffi_to(obj.quorums),
                    })
                }
                unsafe fn destroy(ffi: *mut MasternodeList) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for MasternodeList {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.block_hash);
                        if !ffi_ref.masternode_merkle_root.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.masternode_merkle_root);
                        };
                        if !ffi_ref.llmq_merkle_root.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.llmq_merkle_root);
                        };
                        ferment_interfaces::unbox_any(ffi_ref.masternodes);
                        ferment_interfaces::unbox_any(ffi_ref.quorums);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeList_ctor(
                block_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                known_height: u32,
                masternode_merkle_root: *mut crate::fermented::types::crypto::byte_util::UInt256,
                llmq_merkle_root: *mut crate::fermented::types::crypto::byte_util::UInt256,
                masternodes : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry,
                quorums : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry,
            ) -> *mut MasternodeList {
                ferment_interfaces::boxed(MasternodeList {
                    block_hash,
                    known_height,
                    masternode_merkle_root,
                    llmq_merkle_root,
                    masternodes,
                    quorums,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn MasternodeList_destroy(ffi: *mut MasternodeList) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod snapshot {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: snapshot :: LLMQSnapshot\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct LLMQSnapshot { pub member_list : * mut crate :: fermented :: generics :: Vec_u8 , pub skip_list : * mut crate :: fermented :: generics :: Vec_i32 , pub skip_list_mode : * mut crate :: fermented :: types :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode , }
            impl ferment_interfaces::FFIConversion<crate::models::snapshot::LLMQSnapshot> for LLMQSnapshot {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQSnapshot,
                ) -> crate::models::snapshot::LLMQSnapshot {
                    let ffi_ref = &*ffi;
                    crate::models::snapshot::LLMQSnapshot {
                        member_list: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.member_list,
                        ),
                        skip_list: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.skip_list),
                        skip_list_mode: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.skip_list_mode,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::snapshot::LLMQSnapshot,
                ) -> *const LLMQSnapshot {
                    ferment_interfaces::boxed(LLMQSnapshot {
                        member_list: ferment_interfaces::FFIConversion::ffi_to(obj.member_list),
                        skip_list: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list),
                        skip_list_mode: ferment_interfaces::FFIConversion::ffi_to(
                            obj.skip_list_mode,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut LLMQSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.member_list);
                        ferment_interfaces::unbox_any(ffi_ref.skip_list);
                        ferment_interfaces::unbox_any(ffi_ref.skip_list_mode);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshot_ctor(
                member_list: *mut crate::fermented::generics::Vec_u8,
                skip_list: *mut crate::fermented::generics::Vec_i32,
                skip_list_mode : * mut crate :: fermented :: types :: common :: llmq_snapshot_skip_mode :: LLMQSnapshotSkipMode,
            ) -> *mut LLMQSnapshot {
                ferment_interfaces::boxed(LLMQSnapshot {
                    member_list,
                    skip_list,
                    skip_list_mode,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQSnapshot_destroy(ffi: *mut LLMQSnapshot) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod llmq_entry {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: llmq_entry :: LLMQEntry\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct LLMQEntry {
                pub version: *mut crate::fermented::types::common::llmq_version::LLMQVersion,
                pub llmq_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub index: u16,
                pub public_key: *mut crate::fermented::types::crypto::byte_util::UInt384,
                pub threshold_signature: *mut crate::fermented::types::crypto::byte_util::UInt768,
                pub verification_vector_hash:
                    *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub all_commitment_aggregated_signature:
                    *mut crate::fermented::types::crypto::byte_util::UInt768,
                pub llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                pub signers: *mut crate::fermented::types::common::bitset::Bitset,
                pub valid_members: *mut crate::fermented::types::common::bitset::Bitset,
                pub entry_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                pub verified: bool,
                pub saved: bool,
                pub commitment_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            }
            impl ferment_interfaces::FFIConversion<crate::models::llmq_entry::LLMQEntry> for LLMQEntry {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQEntry,
                ) -> crate::models::llmq_entry::LLMQEntry {
                    let ffi_ref = &*ffi;
                    crate::models::llmq_entry::LLMQEntry {
                        version: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.version),
                        llmq_hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.llmq_hash),
                        index: (ffi_ref.index > 0).then_some(ffi_ref.index),
                        public_key: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.public_key),
                        threshold_signature: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.threshold_signature,
                        ),
                        verification_vector_hash: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.verification_vector_hash,
                        ),
                        all_commitment_aggregated_signature:
                            ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.all_commitment_aggregated_signature,
                            ),
                        llmq_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.llmq_type),
                        signers: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.signers),
                        valid_members: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.valid_members,
                        ),
                        entry_hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.entry_hash),
                        verified: ffi_ref.verified,
                        saved: ffi_ref.saved,
                        commitment_hash: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.commitment_hash,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::llmq_entry::LLMQEntry,
                ) -> *const LLMQEntry {
                    ferment_interfaces::boxed(LLMQEntry {
                        version: ferment_interfaces::FFIConversion::ffi_to(obj.version),
                        llmq_hash: ferment_interfaces::FFIConversion::ffi_to(obj.llmq_hash),
                        index: obj.index.unwrap_or(0),
                        public_key: ferment_interfaces::FFIConversion::ffi_to(obj.public_key),
                        threshold_signature: ferment_interfaces::FFIConversion::ffi_to(
                            obj.threshold_signature,
                        ),
                        verification_vector_hash: ferment_interfaces::FFIConversion::ffi_to(
                            obj.verification_vector_hash,
                        ),
                        all_commitment_aggregated_signature:
                            ferment_interfaces::FFIConversion::ffi_to(
                                obj.all_commitment_aggregated_signature,
                            ),
                        llmq_type: ferment_interfaces::FFIConversion::ffi_to(obj.llmq_type),
                        signers: ferment_interfaces::FFIConversion::ffi_to(obj.signers),
                        valid_members: ferment_interfaces::FFIConversion::ffi_to(obj.valid_members),
                        entry_hash: ferment_interfaces::FFIConversion::ffi_to(obj.entry_hash),
                        verified: obj.verified,
                        saved: obj.saved,
                        commitment_hash: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.commitment_hash,
                        ),
                    })
                }
                unsafe fn destroy(ffi: *mut LLMQEntry) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQEntry {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.version);
                        ferment_interfaces::unbox_any(ffi_ref.llmq_hash);
                        ferment_interfaces::unbox_any(ffi_ref.public_key);
                        ferment_interfaces::unbox_any(ffi_ref.threshold_signature);
                        ferment_interfaces::unbox_any(ffi_ref.verification_vector_hash);
                        ferment_interfaces::unbox_any(ffi_ref.all_commitment_aggregated_signature);
                        ferment_interfaces::unbox_any(ffi_ref.llmq_type);
                        ferment_interfaces::unbox_any(ffi_ref.signers);
                        ferment_interfaces::unbox_any(ffi_ref.valid_members);
                        ferment_interfaces::unbox_any(ffi_ref.entry_hash);
                        if !ffi_ref.commitment_hash.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.commitment_hash);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQEntry_ctor(
                version: *mut crate::fermented::types::common::llmq_version::LLMQVersion,
                llmq_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                index: u16,
                public_key: *mut crate::fermented::types::crypto::byte_util::UInt384,
                threshold_signature: *mut crate::fermented::types::crypto::byte_util::UInt768,
                verification_vector_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                all_commitment_aggregated_signature : * mut crate :: fermented :: types :: crypto :: byte_util :: UInt768,
                llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                signers: *mut crate::fermented::types::common::bitset::Bitset,
                valid_members: *mut crate::fermented::types::common::bitset::Bitset,
                entry_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
                verified: bool,
                saved: bool,
                commitment_hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            ) -> *mut LLMQEntry {
                ferment_interfaces::boxed(LLMQEntry {
                    version,
                    llmq_hash,
                    index,
                    public_key,
                    threshold_signature,
                    verification_vector_hash,
                    all_commitment_aggregated_signature,
                    llmq_type,
                    signers,
                    valid_members,
                    entry_hash,
                    verified,
                    saved,
                    commitment_hash,
                })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQEntry_destroy(ffi: *mut LLMQEntry) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod operator_public_key {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: operator_public_key :: OperatorPublicKey\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct OperatorPublicKey {
                pub data: *mut crate::fermented::types::crypto::byte_util::UInt384,
                pub version: u16,
            }
            impl
                ferment_interfaces::FFIConversion<
                    crate::models::operator_public_key::OperatorPublicKey,
                > for OperatorPublicKey
            {
                unsafe fn ffi_from_const(
                    ffi: *const OperatorPublicKey,
                ) -> crate::models::operator_public_key::OperatorPublicKey {
                    let ffi_ref = &*ffi;
                    crate::models::operator_public_key::OperatorPublicKey {
                        data: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.data),
                        version: ffi_ref.version,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::operator_public_key::OperatorPublicKey,
                ) -> *const OperatorPublicKey {
                    ferment_interfaces::boxed(OperatorPublicKey {
                        data: ferment_interfaces::FFIConversion::ffi_to(obj.data),
                        version: obj.version,
                    })
                }
                unsafe fn destroy(ffi: *mut OperatorPublicKey) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for OperatorPublicKey {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.data);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn OperatorPublicKey_ctor(
                data: *mut crate::fermented::types::crypto::byte_util::UInt384,
                version: u16,
            ) -> *mut OperatorPublicKey {
                ferment_interfaces::boxed(OperatorPublicKey { data, version })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn OperatorPublicKey_destroy(ffi: *mut OperatorPublicKey) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod llmq_typed_hash {
            #[doc = "FFI-representation of the # [doc = \"FFI-representation of the crate :: models :: llmq_typed_hash :: LLMQTypedHash\"]"]
            #[repr(C)]
            #[derive(Clone)]
            #[allow(non_camel_case_types)]
            pub struct LLMQTypedHash {
                pub llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                pub hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            }
            impl ferment_interfaces::FFIConversion<crate::models::llmq_typed_hash::LLMQTypedHash>
                for LLMQTypedHash
            {
                unsafe fn ffi_from_const(
                    ffi: *const LLMQTypedHash,
                ) -> crate::models::llmq_typed_hash::LLMQTypedHash {
                    let ffi_ref = &*ffi;
                    crate::models::llmq_typed_hash::LLMQTypedHash {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.llmq_type),
                        hash: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.hash),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::models::llmq_typed_hash::LLMQTypedHash,
                ) -> *const LLMQTypedHash {
                    ferment_interfaces::boxed(LLMQTypedHash {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_to(obj.llmq_type),
                        hash: ferment_interfaces::FFIConversion::ffi_to(obj.hash),
                    })
                }
                unsafe fn destroy(ffi: *mut LLMQTypedHash) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for LLMQTypedHash {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.llmq_type);
                        ferment_interfaces::unbox_any(ffi_ref.hash);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQTypedHash_ctor(
                llmq_type: *mut crate::fermented::types::chain::common::llmq_type::LLMQType,
                hash: *mut crate::fermented::types::crypto::byte_util::UInt256,
            ) -> *mut LLMQTypedHash {
                ferment_interfaces::boxed(LLMQTypedHash { llmq_type, hash })
            }
            #[doc = r" # Safety"]
            #[allow(non_snake_case)]
            #[no_mangle]
            pub unsafe extern "C" fn LLMQTypedHash_destroy(ffi: *mut LLMQTypedHash) {
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
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_models_snapshot_LLMQSnapshot {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::models::snapshot::LLMQSnapshot,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::models::snapshot::LLMQSnapshot>>
        for Vec_crate_models_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_models_snapshot_LLMQSnapshot,
        ) -> Vec<crate::models::snapshot::LLMQSnapshot> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::models::snapshot::LLMQSnapshot>,
        ) -> *const Vec_crate_models_snapshot_LLMQSnapshot {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_models_snapshot_LLMQSnapshot) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_models_snapshot_LLMQSnapshot {
        type Value = Vec<crate::models::snapshot::LLMQSnapshot>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_models_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_common_block_Block_values_bool {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::common::block::Block,
        pub values: *mut bool,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<crate::common::block::Block, bool>,
        > for std_collections_Map_keys_crate_common_block_Block_values_bool
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_crate_common_block_Block_values_bool,
        ) -> std::collections::BTreeMap<crate::common::block::Block, bool> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| o,
            )
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<crate::common::block::Block, bool>,
        ) -> *const std_collections_Map_keys_crate_common_block_Block_values_bool {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_primitive_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_crate_common_block_Block_values_bool) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_crate_common_block_Block_values_bool {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_processing_mn_listdiff_result_MNListDiffResult {
        pub count: usize,
        pub values:
            *mut *mut crate::fermented::types::processing::mn_listdiff_result::MNListDiffResult,
    }
    impl
        ferment_interfaces::FFIConversion<
            Vec<crate::processing::mn_listdiff_result::MNListDiffResult>,
        > for Vec_crate_processing_mn_listdiff_result_MNListDiffResult
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_processing_mn_listdiff_result_MNListDiffResult,
        ) -> Vec<crate::processing::mn_listdiff_result::MNListDiffResult> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::processing::mn_listdiff_result::MNListDiffResult>,
        ) -> *const Vec_crate_processing_mn_listdiff_result_MNListDiffResult {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_processing_mn_listdiff_result_MNListDiffResult) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion
        for Vec_crate_processing_mn_listdiff_result_MNListDiffResult
    {
        type Value = Vec<crate::processing::mn_listdiff_result::MNListDiffResult>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_processing_mn_listdiff_result_MNListDiffResult {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::crypto::byte_util::UInt256,
        pub values: *mut *mut crate::fermented::types::models::masternode_entry::MasternodeEntry,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: masternode_entry :: MasternodeEntry > > for std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry) -> std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: masternode_entry :: MasternodeEntry > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: masternode_entry :: MasternodeEntry >) -> * const std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_masternode_entry_MasternodeEntry { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
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
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_models_llmq_entry_LLMQEntry {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::models::llmq_entry::LLMQEntry,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::models::llmq_entry::LLMQEntry>>
        for Vec_crate_models_llmq_entry_LLMQEntry
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_models_llmq_entry_LLMQEntry,
        ) -> Vec<crate::models::llmq_entry::LLMQEntry> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::models::llmq_entry::LLMQEntry>,
        ) -> *const Vec_crate_models_llmq_entry_LLMQEntry {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_models_llmq_entry_LLMQEntry) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_models_llmq_entry_LLMQEntry {
        type Value = Vec<crate::models::llmq_entry::LLMQEntry>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_models_llmq_entry_LLMQEntry {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_crypto_byte_util_UInt256 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::crypto::byte_util::UInt256,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::crypto::byte_util::UInt256>>
        for Vec_crate_crypto_byte_util_UInt256
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_crypto_byte_util_UInt256,
        ) -> Vec<crate::crypto::byte_util::UInt256> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::crypto::byte_util::UInt256>,
        ) -> *const Vec_crate_crypto_byte_util_UInt256 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_crypto_byte_util_UInt256) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_crypto_byte_util_UInt256 {
        type Value = Vec<crate::crypto::byte_util::UInt256>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_crypto_byte_util_UInt256 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_tx_transaction_TransactionOutput {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::tx::transaction::TransactionOutput,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::tx::transaction::TransactionOutput>>
        for Vec_crate_tx_transaction_TransactionOutput
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_tx_transaction_TransactionOutput,
        ) -> Vec<crate::tx::transaction::TransactionOutput> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::tx::transaction::TransactionOutput>,
        ) -> *const Vec_crate_tx_transaction_TransactionOutput {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_tx_transaction_TransactionOutput) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_tx_transaction_TransactionOutput {
        type Value = Vec<crate::tx::transaction::TransactionOutput>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_tx_transaction_TransactionOutput {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { pub count : usize , pub keys : * mut * mut crate :: fermented :: types :: chain :: common :: llmq_type :: LLMQType , pub values : * mut * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry , }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: chain :: common :: llmq_type :: LLMQType , std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry > > > for std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry) -> std :: collections :: BTreeMap < crate :: chain :: common :: llmq_type :: LLMQType , std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry > > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: chain :: common :: llmq_type :: LLMQType , std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry > >) -> * const std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_chain_common_llmq_type_LLMQType_values_std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::common::block::Block,
        pub values:
            *mut *mut crate::fermented::types::models::operator_public_key::OperatorPublicKey,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: common :: block :: Block , crate :: models :: operator_public_key :: OperatorPublicKey > > for std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey) -> std :: collections :: BTreeMap < crate :: common :: block :: Block , crate :: models :: operator_public_key :: OperatorPublicKey > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: common :: block :: Block , crate :: models :: operator_public_key :: OperatorPublicKey >) -> * const std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_common_block_Block_values_crate_models_operator_public_key_OperatorPublicKey { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::crypto::byte_util::UInt256,
        pub values: *mut *mut crate::fermented::types::models::llmq_entry::LLMQEntry,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry > > for std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry) -> std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: crypto :: byte_util :: UInt256 , crate :: models :: llmq_entry :: LLMQEntry >) -> * const std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_crypto_byte_util_UInt256_values_crate_models_llmq_entry_LLMQEntry { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::common::block::Block,
        pub values: *mut *mut crate::fermented::types::crypto::byte_util::UInt256,
    }
    impl
        ferment_interfaces::FFIConversion<
            std::collections::BTreeMap<
                crate::common::block::Block,
                crate::crypto::byte_util::UInt256,
            >,
        >
        for std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256,
        ) -> std::collections::BTreeMap<
            crate::common::block::Block,
            crate::crypto::byte_util::UInt256,
        > {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }        unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: common :: block :: Block , crate :: crypto :: byte_util :: UInt256 >) -> * const std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256{
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                keys: ferment_interfaces::to_complex_vec(obj.keys().cloned()),
                values: ferment_interfaces::to_complex_vec(obj.values().cloned()),
            })
        }
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop
        for std_collections_Map_keys_crate_common_block_Block_values_crate_crypto_byte_util_UInt256
    {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.keys, self.count);
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_crate_tx_transaction_TransactionInput {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::tx::transaction::TransactionInput,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::tx::transaction::TransactionInput>>
        for Vec_crate_tx_transaction_TransactionInput
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_tx_transaction_TransactionInput,
        ) -> Vec<crate::tx::transaction::TransactionInput> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::tx::transaction::TransactionInput>,
        ) -> *const Vec_crate_tx_transaction_TransactionInput {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_tx_transaction_TransactionInput) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_tx_transaction_TransactionInput {
        type Value = Vec<crate::tx::transaction::TransactionInput>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_tx_transaction_TransactionInput {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[repr(C)]
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub struct Vec_i32 {
        pub count: usize,
        pub values: *mut i32,
    }
    impl ferment_interfaces::FFIConversion<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_i32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_i32 {
        type Value = Vec<i32>;
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
    impl Drop for Vec_i32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
}
