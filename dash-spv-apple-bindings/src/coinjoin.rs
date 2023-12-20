// use core::slice;
// use std::io::Cursor;

// use crate::dash_spv_coinjoin::messages;
// use dash_spv_masternode_processor::consensus::Decodable;
// use ferment_interfaces::boxed;

// pub mod coinjoin {
//     use std::io::Cursor;

//     use dash_spv_masternode_processor::consensus::Decodable;
//     use dash_spv_coinjoin::messages::CoinJoinAcceptMessage;

//     #[ferment_macro::export]
//     pub fn process_coinjoin_accept_message(message: Vec<u8>) -> CoinJoinAcceptMessage {
//         let mut cursor = Cursor::new(message);
//         return CoinJoinAcceptMessage::consensus_decode(&mut cursor).unwrap();
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_accept_message(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinAcceptMessage {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinAcceptMessage::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_broadcast_tx(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinBroadcastTx {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinBroadcastTx::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_complete_message(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinCompleteMessage {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinCompleteMessage::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_entry(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinEntry {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinEntry::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_final_transaction(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinFinalTransaction {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinFinalTransaction::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_queue_message(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinQueueMessage {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinQueueMessage::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_signed_inputs(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinSignedInputs {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinSignedInputs::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_coinjoin_status_update(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::CoinJoinStatusUpdate {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::CoinJoinStatusUpdate::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }

// #[no_mangle]
// pub unsafe extern "C" fn process_send_coinjoin_queue(
//     message: *const u8,
//     message_length: usize
// ) -> *mut messages::SendCoinJoinQueue {
//     let message: &[u8] = slice::from_raw_parts(message, message_length);
//     let mut cursor = Cursor::new(message);
//     let result = messages::SendCoinJoinQueue::consensus_decode(&mut cursor).unwrap();

//     boxed(result)
// }