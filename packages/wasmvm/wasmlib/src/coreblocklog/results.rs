// Code generated by schema tool; DO NOT EDIT.

// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::*;
use crate::coreblocklog::*;

#[derive(Clone)]
pub struct ImmutableGetBlockInfoResults {
    pub proxy: Proxy,
}

impl ImmutableGetBlockInfoResults {
    // index of returned block
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // serialized block info
    pub fn block_info(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_BLOCK_INFO))
    }
}

#[derive(Clone)]
pub struct MutableGetBlockInfoResults {
    pub proxy: Proxy,
}

impl MutableGetBlockInfoResults {
    pub fn new() -> MutableGetBlockInfoResults {
        MutableGetBlockInfoResults {
            proxy: results_proxy(),
        }
    }

    // index of returned block
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // serialized block info
    pub fn block_info(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_BLOCK_INFO))
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableBytes {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableBytes {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_bytes(&self, index: u32) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct ImmutableGetEventsForBlockResults {
    pub proxy: Proxy,
}

impl ImmutableGetEventsForBlockResults {
    // Array of serialized events
    pub fn event(&self) -> ArrayOfImmutableBytes {
        ArrayOfImmutableBytes { proxy: self.proxy.root(RESULT_EVENT) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableBytes {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableBytes {
    pub fn append_bytes(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.append())
    }

    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_bytes(&self, index: u32) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct MutableGetEventsForBlockResults {
    pub proxy: Proxy,
}

impl MutableGetEventsForBlockResults {
    pub fn new() -> MutableGetEventsForBlockResults {
        MutableGetEventsForBlockResults {
            proxy: results_proxy(),
        }
    }

    // Array of serialized events
    pub fn event(&self) -> ArrayOfMutableBytes {
        ArrayOfMutableBytes { proxy: self.proxy.root(RESULT_EVENT) }
    }
}

#[derive(Clone)]
pub struct ImmutableGetEventsForRequestResults {
    pub proxy: Proxy,
}

impl ImmutableGetEventsForRequestResults {
    // Array of serialized events
    pub fn event(&self) -> ArrayOfImmutableBytes {
        ArrayOfImmutableBytes { proxy: self.proxy.root(RESULT_EVENT) }
    }
}

#[derive(Clone)]
pub struct MutableGetEventsForRequestResults {
    pub proxy: Proxy,
}

impl MutableGetEventsForRequestResults {
    pub fn new() -> MutableGetEventsForRequestResults {
        MutableGetEventsForRequestResults {
            proxy: results_proxy(),
        }
    }

    // Array of serialized events
    pub fn event(&self) -> ArrayOfMutableBytes {
        ArrayOfMutableBytes { proxy: self.proxy.root(RESULT_EVENT) }
    }
}

#[derive(Clone)]
pub struct ArrayOfImmutableRequestID {
    pub(crate) proxy: Proxy,
}

impl ArrayOfImmutableRequestID {
    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_request_id(&self, index: u32) -> ScImmutableRequestID {
        ScImmutableRequestID::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestIDsForBlockResults {
    pub proxy: Proxy,
}

impl ImmutableGetRequestIDsForBlockResults {
    // index of block containing request
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // Array of request IDs
    pub fn request_id(&self) -> ArrayOfImmutableRequestID {
        ArrayOfImmutableRequestID { proxy: self.proxy.root(RESULT_REQUEST_ID) }
    }
}

#[derive(Clone)]
pub struct ArrayOfMutableRequestID {
    pub(crate) proxy: Proxy,
}

impl ArrayOfMutableRequestID {
    pub fn append_request_id(&self) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.append())
    }

    pub fn clear(&self) {
        self.proxy.clear_array();
    }

    pub fn length(&self) -> u32 {
        self.proxy.length()
    }

    pub fn get_request_id(&self, index: u32) -> ScMutableRequestID {
        ScMutableRequestID::new(self.proxy.index(index))
    }
}

#[derive(Clone)]
pub struct MutableGetRequestIDsForBlockResults {
    pub proxy: Proxy,
}

impl MutableGetRequestIDsForBlockResults {
    pub fn new() -> MutableGetRequestIDsForBlockResults {
        MutableGetRequestIDsForBlockResults {
            proxy: results_proxy(),
        }
    }

    // index of block containing request
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // Array of request IDs
    pub fn request_id(&self) -> ArrayOfMutableRequestID {
        ArrayOfMutableRequestID { proxy: self.proxy.root(RESULT_REQUEST_ID) }
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestReceiptResults {
    pub proxy: Proxy,
}

impl ImmutableGetRequestReceiptResults {
    // index of block containing request
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // index within block containing request
    pub fn request_index(&self) -> ScImmutableUint16 {
        ScImmutableUint16::new(self.proxy.root(RESULT_REQUEST_INDEX))
    }

    // serialized request receipt
    pub fn request_receipt(&self) -> ScImmutableBytes {
        ScImmutableBytes::new(self.proxy.root(RESULT_REQUEST_RECEIPT))
    }
}

#[derive(Clone)]
pub struct MutableGetRequestReceiptResults {
    pub proxy: Proxy,
}

impl MutableGetRequestReceiptResults {
    pub fn new() -> MutableGetRequestReceiptResults {
        MutableGetRequestReceiptResults {
            proxy: results_proxy(),
        }
    }

    // index of block containing request
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // index within block containing request
    pub fn request_index(&self) -> ScMutableUint16 {
        ScMutableUint16::new(self.proxy.root(RESULT_REQUEST_INDEX))
    }

    // serialized request receipt
    pub fn request_receipt(&self) -> ScMutableBytes {
        ScMutableBytes::new(self.proxy.root(RESULT_REQUEST_RECEIPT))
    }
}

#[derive(Clone)]
pub struct ImmutableGetRequestReceiptsForBlockResults {
    pub proxy: Proxy,
}

impl ImmutableGetRequestReceiptsForBlockResults {
    // index of block containing request
    pub fn block_index(&self) -> ScImmutableUint32 {
        ScImmutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // Array of request receipts
    pub fn request_receipts(&self) -> ArrayOfImmutableBytes {
        ArrayOfImmutableBytes { proxy: self.proxy.root(RESULT_REQUEST_RECEIPTS) }
    }
}

#[derive(Clone)]
pub struct MutableGetRequestReceiptsForBlockResults {
    pub proxy: Proxy,
}

impl MutableGetRequestReceiptsForBlockResults {
    pub fn new() -> MutableGetRequestReceiptsForBlockResults {
        MutableGetRequestReceiptsForBlockResults {
            proxy: results_proxy(),
        }
    }

    // index of block containing request
    pub fn block_index(&self) -> ScMutableUint32 {
        ScMutableUint32::new(self.proxy.root(RESULT_BLOCK_INDEX))
    }

    // Array of request receipts
    pub fn request_receipts(&self) -> ArrayOfMutableBytes {
        ArrayOfMutableBytes { proxy: self.proxy.root(RESULT_REQUEST_RECEIPTS) }
    }
}

#[derive(Clone)]
pub struct ImmutableIsRequestProcessedResults {
    pub proxy: Proxy,
}

impl ImmutableIsRequestProcessedResults {
    // whether request has been processed
    pub fn request_processed(&self) -> ScImmutableBool {
        ScImmutableBool::new(self.proxy.root(RESULT_REQUEST_PROCESSED))
    }
}

#[derive(Clone)]
pub struct MutableIsRequestProcessedResults {
    pub proxy: Proxy,
}

impl MutableIsRequestProcessedResults {
    pub fn new() -> MutableIsRequestProcessedResults {
        MutableIsRequestProcessedResults {
            proxy: results_proxy(),
        }
    }

    // whether request has been processed
    pub fn request_processed(&self) -> ScMutableBool {
        ScMutableBool::new(self.proxy.root(RESULT_REQUEST_PROCESSED))
    }
}
