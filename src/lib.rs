// SPDX-FileCopyrightText: Copyright The arm-firme Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

/// Feature discovery
pub mod features;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

/// Internal error type of the FIRME module
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Unrecognised error code
    #[error("unrecognised error code {0}")]
    UnrecognisedErrorCode(i32),
    /// Unrecognised FunctionId
    #[error("unrecognised FunctionId {0}")]
    UnrecognisedFunctionId(u32),
    /// Invalid shared buffer minimum size
    #[error("invalid shared buffer minimum size {0}")]
    InvalidSharedBufferMinSize(u8),
    /// Invalid protected physical address size
    #[error("invalid protected physical address size {0}")]
    InvalidProtectedPhysicalAddressSize(u8),
    /// Invalid level 0 GPT size
    #[error("invalid level 0 GPT size {0}")]
    InvalidLevel0GptSize(u8),
    /// Invalid physical granule size
    #[error("invalid physical granule size {0}")]
    InvalidPhysicalGranuleSize(u8),
    /// Invalid RAK format
    #[error("invalid RAK format {0}")]
    InvalidRakFormat(u8),
    /// Invalid feature index
    #[error("invalid feature index {0}")]
    InvalidFeatureIndex(u64),
    /// The major field is out of range
    #[error("the major field is out of range")]
    MajorVersionOutOfRange,
    /// Invalid ServiceId
    #[error("invalid ServiceId {0}")]
    InvalidServiceId(u8),
}

/// Error status codes
#[derive(Clone, Copy, Debug, Eq, Error, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedErrorCode))]
#[repr(i32)]
pub enum StatusCode {
    /// Successful operation
    #[error("successful operation")]
    Success = 0,
    /// Not supported
    #[error("not supported")]
    NotSupported = -1,
    /// Invalid parameters
    #[error("invalid parameters")]
    InvalidParameters = -2,
    /// Aborted
    #[error("aborted")]
    Aborted = -3,
    /// Incomplete
    #[error("incomplete")]
    Incomplete = -4,
    /// Denied
    #[error("denied")]
    Denied = -5,
    /// Retry
    #[error("retry")]
    Retry = -6,
    /// Another operation in progress
    #[error("another operation in progress")]
    InProgress = -7,
    /// Operation attempts to create an object that already exists.
    #[error("operation attempts to create an object that already exists")]
    Exists = -8,
    /// An object required to complete the requested operation does not exist.
    #[error("an object required to complete the requested operation does not exist")]
    NoEntry = -9,
    /// Unable to allocate memory required to complete the operation.
    #[error("unable to allocate memory required to complete the operation")]
    NoMemory = -10,
    /// The input data is malformed.
    #[error("the input data is malformed")]
    BadData = -11,
}
