// SPDX-FileCopyrightText: Copyright The arm-firme Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

/// Feature discovery
pub mod features;

use crate::features::FeatureRegister;
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
    /// Unsuccessful FIRME call
    #[error("FIRME call return status: {0}")]
    UnsuccessfulCall(StatusCode),
    /// Invalid GPI encoding
    #[error("invalid GPIAccessType encoding {0}")]
    InvalidGPIEncoding(u8),
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
    /// Busy
    #[error("busy")]
    Busy = -6,
    /// Another operation in progress
    #[error("another operation in progress")]
    OpConflict = -7,
    /// Operation attemps to create an object that already exists.
    #[error("operation attemps to create an object that already exists")]
    AlreadyExists = -8,
    /// An object required to complete the requested operation does not exist.
    #[error("an object required to complete the requested operation does not exist")]
    NotFound = -9,
    /// Unable to allocate memory required to complete the operation.
    #[error("unable to allocate memory required to complete the operation")]
    NoMemory = -10,
    /// The input data is malformed.
    #[error("the input data is malformed")]
    BadData = -11,
}

/// FIRME services
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidServiceId))]
#[repr(u8)]
pub enum ServiceId {
    /// Base service
    Base = 0,
    /// Granule management service
    GranuleManagement = 1,
    /// IDE key management service
    IDEKeyManagement = 2,
    /// MECID management service
    MECIDManagement = 3,
    /// Attestation service
    Attestation = 4,
    /// Integrated device management service
    IntegratedDeviceManagement = 5,
}

/// Each implemented FIRME service has its own `ServiceVersion`. The implemented FIRME version is
/// derived from the versions of implemented services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceVersion {
    /// Major version number
    pub major: u16,
    /// Minor version number
    pub minor: u16,
}

impl ServiceVersion {
    /// Create a ServiceVersion
    pub const fn new(major: u16, minor: u16) -> Self {
        assert!(major <= 0x7fff);
        Self { major, minor }
    }
}

impl TryFrom<u32> for ServiceVersion {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        let major = (value >> 16) as u16;

        if major & 0x7fff != major {
            return Err(Error::MajorVersionOutOfRange);
        }

        Ok(Self {
            major,
            minor: (value & 0xFFFF) as u16,
        })
    }
}

impl From<ServiceVersion> for u32 {
    fn from(version: ServiceVersion) -> Self {
        assert!(version.major <= 0x7fff);
        (version.major as u32) << 16 | version.minor as u32
    }
}

impl From<ServiceVersion> for u64 {
    fn from(version: ServiceVersion) -> Self {
        u32::from(version).into()
    }
}

impl TryFrom<u64> for ServiceVersion {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Error> {
        u32::try_from(value)
            .map_err(|_| Error::MajorVersionOutOfRange)?
            .try_into()
    }
}

/// Function IDs based on Chapter 8.
#[derive(Clone, Copy, Debug, Eq, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::UnrecognisedFunctionId))]
#[repr(u32)]
pub enum FunctionId {
    /// FIRME_SERVICE_VERSION function id
    ServiceVersion = 0xC4000400,
    /// FIRME_SERVICE_FEATURES function id
    ServiceFeatures = 0xC4000401,
    /// FIRME_GM_GPI_SET function id
    GmGPISet = 0xC4000402,
}

/// Enum for representing FIRME requests and their arguments.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Function {
    /// FIRME_SERVICE_VERSION function
    ServiceVersion {
        /// Service id
        service_id: ServiceId,
    },
    /// FIRME_SERVICE_FEATURES function
    ServiceFeatures {
        /// Service id
        service_id: ServiceId,
        /// Feature register index
        feature_reg_index: u8,
    },
    /// FIRME_GM_GPI_SET function
    GmGPISet {
        /// Base address
        base_address: u64,
        /// Granule count
        granule_count: u64,
        /// Target GPI
        target_gpi: GPIAccessType,
    },
}

impl Function {
    /// Returns the FunctionId of the current Function.
    pub fn id(&self) -> FunctionId {
        match self {
            Function::ServiceVersion { .. } => FunctionId::ServiceVersion,
            Function::ServiceFeatures { .. } => FunctionId::ServiceFeatures,
            Function::GmGPISet { .. } => FunctionId::GmGPISet,
        }
    }
}

impl TryFrom<&[u64; 4]> for Function {
    type Error = Error;

    fn try_from(regs: &[u64; 4]) -> Result<Self, Error> {
        let fid = FunctionId::try_from(regs[0] as u32)?;

        let func = match fid {
            FunctionId::ServiceVersion => Self::ServiceVersion {
                service_id: ServiceId::try_from(regs[1] as u8)?,
            },
            FunctionId::ServiceFeatures => Self::ServiceFeatures {
                service_id: ServiceId::try_from(regs[1] as u8)?,
                feature_reg_index: regs[2] as u8,
            },
            FunctionId::GmGPISet => Self::GmGPISet {
                base_address: regs[1],
                granule_count: regs[2],
                target_gpi: GPIAccessType::try_from(regs[3])?,
            },
        };
        Ok(func)
    }
}

/// Enum for representing the return parameters of FIRME calls
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Response {
    /// FIRME_SERVICE_VERSION response
    ServiceVersion {
        /// ServiceVersion
        service_version: ServiceVersion,
    },
    /// FIRME_SERVICE_FEATURES response
    ServiceFeatures {
        /// If the Return status is SUCCESS, this is the value of the feature
        /// register corresponding to the [`ServiceId`] and feature register
        /// index input parameters.
        register: FeatureRegister,
    },
    /// FIRME_GM_GPI_SET response
    GmGPISet {
        /// Generic response for all defined return values.
        response: GpiSetStatus,
    },
}

/// Represents the different statuses `FIRME_GM_GPI_SET` can return
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GpiSetStatus {
    /// `FIRME_GM_GPI_SET` ABI completed
    Complete {
        /// Count of granules starting from the first granule at the Base Address
        /// whose GPI encoding was changed
        granule_count: u64,
    },
    /// `FIRME_GM_GPI_SET` ABI was partially completed
    Incomplete {
        /// Count of granules starting from the first granule at the Base Address
        /// whose GPI encoding was changed
        granule_count: u64,
        /// A cookie representing the incomplete transaction.
        /// The caller passes the cookie to the callee in an invocation of the
        /// `FIRME_GM_GPI_OP_CONTINUE` ABI.
        cookie: u64,
    },
    /// `FIRME_GM_GPI_SET` ABI returned `Denied`, `OpConflict` or `NotFound` status
    Error {
        /// Exact status code of the response
        status: StatusCode,
        /// Count of granules starting from the first granule at the Base Address
        /// whose GPI encoding was changed
        granule_count: u64,
    },
}

impl TryFrom<(FunctionId, &[u64; 4])> for Response {
    type Error = Error;

    fn try_from((function_id, regs): (FunctionId, &[u64; 4])) -> Result<Self, Error> {
        match function_id {
            FunctionId::ServiceVersion => {
                // The ServiceVersion and the StatusCode share reg[0].
                let reg = regs[0] as u32;
                if reg & (1 << 31) == 0 {
                    Ok(Self::ServiceVersion {
                        service_version: ServiceVersion::try_from(reg)?,
                    })
                } else {
                    let status: StatusCode = (regs[0] as i32).try_into()?;
                    Err(Error::UnsuccessfulCall(status))
                }
            }
            FunctionId::ServiceFeatures => {
                let status = StatusCode::try_from(regs[0] as i32)?;
                if status == StatusCode::Success {
                    Ok(Self::ServiceFeatures {
                        register: FeatureRegister(regs[1]),
                    })
                } else {
                    Err(Error::UnsuccessfulCall(status))
                }
            }
            FunctionId::GmGPISet => {
                let status = StatusCode::try_from(regs[0] as i32)?;
                let granule_count = regs[1];

                match status {
                    StatusCode::Incomplete => {
                        let cookie = regs[2];
                        Ok(Self::GmGPISet {
                            response: GpiSetStatus::Incomplete {
                                granule_count,
                                cookie,
                            },
                        })
                    }
                    StatusCode::Success => Ok(Self::GmGPISet {
                        response: GpiSetStatus::Complete { granule_count },
                    }),
                    StatusCode::Denied | StatusCode::OpConflict | StatusCode::NotFound => {
                        Ok(Self::GmGPISet {
                            response: GpiSetStatus::Error {
                                status,
                                granule_count,
                            },
                        })
                    }
                    _ => Err(Error::UnsuccessfulCall(status)),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidGPIEncoding))]
#[repr(u8)]
/// Access control restriction that can be applied to a memory region in the GPT.
pub enum GPIAccessType {
    /// No accesses permitted.
    NoAccess = 0b0000,
    /// Accesses permitted to System Agent PA space only.
    /// This encoding is reserved if GPCCR_EL3.SA is 0, or if FEAT_RME_GDI is not implemented.
    SystemAgent = 0b0100,
    /// Accesses permitted to Non-secure Protected PA space only.
    /// This encoding is reserved if GPCCR_EL3.NSP is 0, or if FEAT_RME_GDI is not implemented.
    NonSecureProtected = 0b0101,
    /// No accesses permitted.
    /// This encoding is reserved if GPCCR_EL3.NA6 is 0, or if FEAT_RME_GDI is not implemented.
    NoAccess6 = 0b0110,
    /// No accesses permitted.
    /// This encoding is reserved if GPCCR_EL3.NA7 is 0, or if FEAT_RME_GDI is not implemented.
    NoAccess7 = 0b0111,
    /// Accesses permitted to Secure PA space only.
    /// This encoding is reserved if FEAT_SEL2 is not implemented.
    Secure = 0b1000,
    /// Accesses permitted to Non-secure PA space only.
    NonSecure = 0b1001,
    /// Accesses permitted to Root PA space only.
    Root = 0b1010,
    /// Accesses permitted to Realm PA space only.
    Realm = 0b1011,
    /// Accesses permitted to Non-secure PA space only, by Non-secure or Root Security states.
    /// This encoding is reserved if the Effective value of GPCCR_EL3.NSO is 0, or if FEAT_RME_GPC2
    /// is not implemented.
    NonSecureRoot = 0b1101,
    /// All accesses permitted.
    Any = 0b1111,
}

impl GPIAccessType {
    /// MASK selecting the useful bits from the `u8` used for storing `GPIAccessType`s.
    pub const MASK: u64 = 0b1111;
}

impl TryFrom<u64> for GPIAccessType {
    type Error = Error;

    fn try_from(reg: u64) -> Result<GPIAccessType, Error> {
        GPIAccessType::try_from((reg & GPIAccessType::MASK) as u8)
    }
}

impl From<GPIAccessType> for u64 {
    fn from(gpi: GPIAccessType) -> u64 {
        u8::from(gpi) as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use features::BaseServiceFeaturesRegister1;

    #[test]
    fn service_id() {
        assert_eq!(ServiceId::try_from(0), Ok(ServiceId::Base));
        assert_eq!(ServiceId::try_from(6), Err(Error::InvalidServiceId(6)));
    }

    #[test]
    fn service_version() {
        assert_eq!(
            ServiceVersion::try_from(0u32),
            Ok(ServiceVersion { minor: 0, major: 0 })
        );
        assert_eq!(
            ServiceVersion::try_from(0xFFFFu32),
            Ok(ServiceVersion {
                major: 0,
                minor: 0xFFFF
            })
        );
        assert_eq!(
            ServiceVersion::try_from(0x7FFF_FFFFu32),
            Ok(ServiceVersion {
                major: 0x7FFF,
                minor: 0xFFFF
            })
        );
        assert_eq!(
            ServiceVersion::try_from(0x8000_0000u32),
            Err(Error::MajorVersionOutOfRange)
        );
    }

    #[test]
    fn function_id_service_version() {
        let service_version = 0x0000_0000_C400_0400;
        let regs: [u64; 4] = [service_version, 1, 0, 0];
        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::ServiceVersion {
                service_id: ServiceId::GranuleManagement
            }
        );

        // out-of-range ServiceId getting truncated to an invalid value
        let regs: [u64; 4] = [service_version, 0xFFFF, 0, 0];
        assert_eq!(
            Function::try_from(&regs),
            Err(Error::InvalidServiceId(0xFF))
        );

        // out-of-range ServiceId getting truncated to a valid value
        let regs: [u64; 4] = [service_version, 0xFF02, 0, 0];
        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::ServiceVersion {
                service_id: ServiceId::IDEKeyManagement
            }
        );
    }

    #[test]
    fn service_version_response() {
        // Successful response
        let bits: u64 = ServiceVersion::new(1, 2).into();
        let regs: [u64; 4] = [bits, 0, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::ServiceVersion, &regs)).unwrap(),
            Response::ServiceVersion {
                service_version: ServiceVersion { major: 1, minor: 2 }
            }
        );

        // Unsuccessful response
        let bits = StatusCode::NotSupported as u64;
        let regs: [u64; 4] = [bits, 0, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::ServiceVersion, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::NotSupported))
        );
    }

    #[test]
    fn function_id_service_features() {
        let service_features = 0x0000_0000_C400_0401;
        let regs: [u64; 4] = [service_features, 1, 0, 0];
        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::ServiceFeatures {
                service_id: ServiceId::GranuleManagement,
                feature_reg_index: 0,
            }
        );
        let regs: [u64; 4] = [service_features, 5, 1, 0];
        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::ServiceFeatures {
                service_id: ServiceId::IntegratedDeviceManagement,
                feature_reg_index: 1,
            }
        );
        let regs: [u64; 4] = [service_features, 3, 0xFF, 0];
        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::ServiceFeatures {
                service_id: ServiceId::MECIDManagement,
                feature_reg_index: 0xFF,
            }
        );
    }

    #[test]
    fn service_features_response() {
        // Successful response
        let bits: u64 = (BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT
            | BaseServiceFeaturesRegister1::MECID_MANAGEMENT
            | BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT)
            .bits();
        let regs: [u64; 4] = [StatusCode::Success as u64, bits, 0, 0];
        let resp = Response::try_from((FunctionId::ServiceFeatures, &regs)).unwrap();
        match resp {
            Response::ServiceFeatures { register } => {
                let base = BaseServiceFeaturesRegister1::try_from(register).unwrap();
                assert!(base.contains(BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT));
                assert!(base.contains(BaseServiceFeaturesRegister1::MECID_MANAGEMENT));
                assert!(base.contains(BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT));
            }
            _ => panic!(),
        }

        // Unsuccessful response
        let bits: u64 = (BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT
            | BaseServiceFeaturesRegister1::MECID_MANAGEMENT
            | BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT)
            .bits();
        let regs: [u64; 4] = [StatusCode::NotSupported as u64, bits, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::ServiceFeatures, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::NotSupported)),
        );
    }

    #[test]
    fn gpi_encodings() {
        assert_eq!(
            GPIAccessType::NoAccess,
            GPIAccessType::try_from(0b0000_0000u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::SystemAgent,
            GPIAccessType::try_from(0b0000_0100u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NonSecureProtected,
            GPIAccessType::try_from(0b0000_0101u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NoAccess6,
            GPIAccessType::try_from(0b0000_0110u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NoAccess7,
            GPIAccessType::try_from(0b0000_0111u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::Secure,
            GPIAccessType::try_from(0b0000_1000u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NonSecure,
            GPIAccessType::try_from(0b0000_1001u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::Root,
            GPIAccessType::try_from(0b0000_1010u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::Realm,
            GPIAccessType::try_from(0b0000_1011u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NonSecureRoot,
            GPIAccessType::try_from(0b0000_1101u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::Any,
            GPIAccessType::try_from(0b0000_1111u8).unwrap()
        );
        assert_eq!(
            GPIAccessType::NoAccess,
            GPIAccessType::try_from(0b1_0000_0000u64).unwrap(),
        );
    }

    #[test]
    fn function_id_gm_gpi_set() {
        let gpi_set = 0x0000_0000_C400_0402;
        let regs: [u64; 4] = [gpi_set, 0x8000_0000, 0xABC, 0x0F];

        assert_eq!(
            Function::try_from(&regs).unwrap(),
            Function::GmGPISet {
                base_address: 0x8000_0000,
                granule_count: 0xABC,
                target_gpi: GPIAccessType::Any,
            }
        );
    }

    #[test]
    fn gm_gpi_set_response() {
        // Successful response
        let regs: [u64; 4] = [StatusCode::Success as u64, 2, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGPISet, &regs)).unwrap(),
            Response::GmGPISet {
                response: GpiSetStatus::Complete { granule_count: 2 },
            }
        );

        // Incomplete response
        let regs: [u64; 4] = [StatusCode::Incomplete as u64, 2, 0x123, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGPISet, &regs)).unwrap(),
            Response::GmGPISet {
                response: GpiSetStatus::Incomplete {
                    granule_count: 2,
                    cookie: 0x123,
                }
            }
        );

        // Valid error response
        let regs: [u64; 4] = [StatusCode::Denied as u64, 2, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGPISet, &regs)).unwrap(),
            Response::GmGPISet {
                response: GpiSetStatus::Error {
                    status: StatusCode::Denied,
                    granule_count: 2
                }
            }
        );

        // Other unexpected response
        let regs: [u64; 4] = [StatusCode::AlreadyExists as u64, 2, 0x123, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGPISet, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::AlreadyExists)),
        );
    }
}
