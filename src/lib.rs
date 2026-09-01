// SPDX-FileCopyrightText: Copyright The arm-firme Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

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
    #[error("invalid GpiAccessType encoding {0}")]
    InvalidGpiEncoding(u8),
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
    /// Operation attempts to create an object that already exists.
    #[error("operation attempts to create an object that already exists")]
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

impl TryFrom<u64> for StatusCode {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::try_from_primitive(value as i32)
    }
}

impl From<StatusCode> for u64 {
    fn from(value: StatusCode) -> Self {
        (i32::from(value) as u32).into()
    }
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::UnrecognisedFunctionId(_)
            | Error::InvalidServiceId(_)
            | Error::InvalidFeatureIndex(_) => Self::NotSupported,
            Error::UnsuccessfulCall(status) => status,
            _ => Self::InvalidParameters,
        }
    }
}

/// FIRME services
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidServiceId))]
#[repr(u8)]
pub enum ServiceId {
    /// Base service
    Base = 0,
    /// Granule management service
    GranuleManagement = 1,
    /// IDE key management service
    IdeKeyManagement = 2,
    /// MECID management service
    MecidManagement = 3,
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
    GmGpiSet = 0xC4000402,
    /// FIRME_GM_GPI_OP_CONTINUE function id
    GmGpiOpContinue = 0xC4000412,
    /// FIRME_GM_L1_GPT_CREATE function id
    GmL1GptCreate = 0xC400040E,
    /// FIRME_GM_L1_GPT_DESTROY function id
    GmL1GptDestroy = 0xC400040F,
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
    GmGpiSet {
        /// Base address
        base_address: u64,
        /// Granule count
        granule_count: u64,
        /// Target GPI
        target_gpi: GpiAccessType,
    },
    /// FIRME_GM_GPI_OP_CONTINUE function
    GmGpiOpContinue {
        /// A value from a previous Granule Management invocation used to identify and
        /// continue an incomplete operation.
        cookie: u64,
    },
    /// FIRME_GM_L1_GPT_CREATE function
    GmL1GptCreate {
        /// Physical base address of a protected address range.
        pa_range_base: u64,
        /// Physical base address of the L1 GPT for the specified protected address range.
        l1_gpt_base: u64,
    },
    /// FIRME_GM_L1_GPT_DESTROY function
    GmL1GptDestroy {
        /// Physical base address of a protected address range.
        pa_range_base: u64,
    },
}

impl Function {
    /// Returns the FunctionId of the current Function.
    pub fn id(&self) -> FunctionId {
        match self {
            Function::ServiceVersion { .. } => FunctionId::ServiceVersion,
            Function::ServiceFeatures { .. } => FunctionId::ServiceFeatures,
            Function::GmGpiSet { .. } => FunctionId::GmGpiSet,
            Function::GmGpiOpContinue { .. } => FunctionId::GmGpiOpContinue,
            Function::GmL1GptCreate { .. } => FunctionId::GmL1GptCreate,
            Function::GmL1GptDestroy { .. } => FunctionId::GmL1GptDestroy,
        }
    }

    /// Copy members of `self` into `regs`.
    pub fn copy_to_array(&self, regs: &mut [u64; 4]) {
        regs.fill(0);
        regs[0] = u32::from(self.id()).into();

        match *self {
            Self::ServiceVersion { service_id } => {
                regs[1] = u8::from(service_id).into();
            }
            Self::ServiceFeatures {
                service_id,
                feature_reg_index,
            } => {
                regs[1] = u8::from(service_id).into();
                regs[2] = feature_reg_index.into();
            }
            Self::GmGpiSet {
                base_address,
                granule_count,
                target_gpi,
            } => {
                regs[1] = base_address;
                regs[2] = granule_count;
                regs[3] = target_gpi.into();
            }
            Self::GmGpiOpContinue { cookie } => {
                regs[1] = cookie;
            }
            Self::GmL1GptCreate {
                pa_range_base,
                l1_gpt_base,
            } => {
                regs[1] = pa_range_base;
                regs[2] = l1_gpt_base;
            }
            Self::GmL1GptDestroy { pa_range_base } => {
                regs[1] = pa_range_base;
            }
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
            FunctionId::GmGpiSet => Self::GmGpiSet {
                base_address: regs[1],
                granule_count: regs[2],
                target_gpi: GpiAccessType::try_from(regs[3])?,
            },
            FunctionId::GmGpiOpContinue => Self::GmGpiOpContinue { cookie: regs[1] },
            FunctionId::GmL1GptCreate => Self::GmL1GptCreate {
                pa_range_base: regs[1],
                l1_gpt_base: regs[2],
            },
            FunctionId::GmL1GptDestroy => Self::GmL1GptDestroy {
                pa_range_base: regs[1],
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
    GmGpiSet {
        /// Generic response for all defined return values.
        response: GpiSetStatus,
    },
    /// FIRME_GM_GPI_OP_CONTINUE response
    GmGpiOpContinue {
        /// Generic response for all defined return values.
        response: GpiOpContinueStatus,
    },
    /// FIRME_GM_L1_GPT_CREATE response
    GmL1GptCreate,
    /// FIRME_GM_L1_GPT_DESTROY response
    GmL1GptDestroy {
        /// Physical base address of the L1 GPT for the specified protected address range.
        l1_gpt_base: u64,
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

/// Represents the different statuses `FIRME_GM_GPI_OP_CONTINUE` can return
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GpiOpContinueStatus {
    /// `FIRME_GM_GPI_OP_CONTINUE` ABI completed
    Complete {
        /// Count of granules whose GPI encoding was changed in this invocation of
        /// this ABI.
        granule_count: u64,
    },
    /// `FIRME_GM_GPI_OP_CONTINUE` ABI was partially completed
    Incomplete {
        /// Count of granules whose GPI encoding was changed in this invocation of
        /// this ABI.
        granule_count: u64,
        /// Cookie representing the current invocation.
        cookie: u64,
    },
    /// `FIRME_GM_GPI_OP_CONTINUE` returned 'Busy' StatusCode.
    Busy {
        /// Cookie representing the current invocation.
        cookie: u64,
    },
    /// `FIRME_GM_GPI_OP_CONTINUE` ABI returned `Denied`, `OpConflict` or `NotFound` status
    Error {
        /// Exact status code of the response
        status: StatusCode,
        /// Count of granules starting from the first granule at the Base Address
        /// whose GPI encoding was changed
        granule_count: u64,
    },
}

impl Response {
    /// Copy members of `self` into `regs`.
    pub fn copy_to_array(&self, regs: &mut [u64; 4]) {
        regs.fill(0);

        match *self {
            Self::ServiceVersion { service_version } => {
                regs[0] = u64::from(service_version);
            }
            Self::ServiceFeatures { register } => {
                regs[0] = StatusCode::Success.into();
                regs[1] = register.0;
            }
            Self::GmGpiSet { response } => match response {
                GpiSetStatus::Complete { granule_count } => {
                    regs[0] = StatusCode::Success.into();
                    regs[1] = granule_count;
                }
                GpiSetStatus::Incomplete {
                    granule_count,
                    cookie,
                } => {
                    regs[0] = StatusCode::Incomplete.into();
                    regs[1] = granule_count;
                    regs[2] = cookie;
                }
                GpiSetStatus::Error {
                    status,
                    granule_count,
                } => {
                    regs[0] = status.into();
                    regs[1] = granule_count;
                }
            },
            Self::GmGpiOpContinue { response } => match response {
                GpiOpContinueStatus::Complete { granule_count } => {
                    regs[0] = StatusCode::Success.into();
                    regs[1] = granule_count;
                }
                GpiOpContinueStatus::Incomplete {
                    granule_count,
                    cookie,
                } => {
                    regs[0] = StatusCode::Incomplete.into();
                    regs[1] = granule_count;
                    regs[2] = cookie;
                }
                GpiOpContinueStatus::Busy { cookie } => {
                    regs[0] = StatusCode::Busy.into();
                    regs[2] = cookie;
                }
                GpiOpContinueStatus::Error {
                    status,
                    granule_count,
                } => {
                    regs[0] = status.into();
                    regs[1] = granule_count;
                }
            },
            Self::GmL1GptCreate => {
                regs[0] = StatusCode::Success.into();
            }
            Self::GmL1GptDestroy { l1_gpt_base } => {
                regs[0] = StatusCode::Success.into();
                regs[2] = l1_gpt_base;
            }
        }
    }

    /// Copy `status` into `regs`.
    pub fn copy_status_to_array(status: StatusCode, regs: &mut [u64; 4]) {
        regs.fill(0);
        regs[0] = u64::from(status);
    }
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
                    let status: StatusCode = regs[0].try_into()?;
                    Err(Error::UnsuccessfulCall(status))
                }
            }
            FunctionId::ServiceFeatures => {
                let status = StatusCode::try_from(regs[0])?;
                if status == StatusCode::Success {
                    Ok(Self::ServiceFeatures {
                        register: FeatureRegister(regs[1]),
                    })
                } else {
                    Err(Error::UnsuccessfulCall(status))
                }
            }
            FunctionId::GmGpiSet => {
                let status = StatusCode::try_from(regs[0])?;
                let granule_count = regs[1];

                match status {
                    StatusCode::Incomplete => {
                        let cookie = regs[2];
                        Ok(Self::GmGpiSet {
                            response: GpiSetStatus::Incomplete {
                                granule_count,
                                cookie,
                            },
                        })
                    }
                    StatusCode::Success => Ok(Self::GmGpiSet {
                        response: GpiSetStatus::Complete { granule_count },
                    }),
                    StatusCode::Denied | StatusCode::OpConflict | StatusCode::NotFound => {
                        Ok(Self::GmGpiSet {
                            response: GpiSetStatus::Error {
                                status,
                                granule_count,
                            },
                        })
                    }
                    _ => Err(Error::UnsuccessfulCall(status)),
                }
            }
            FunctionId::GmGpiOpContinue => {
                let status = StatusCode::try_from(regs[0])?;
                let granule_count = regs[1];

                match status {
                    StatusCode::Success => Ok(Self::GmGpiOpContinue {
                        response: GpiOpContinueStatus::Complete { granule_count },
                    }),
                    StatusCode::Incomplete => {
                        let cookie = regs[2];
                        Ok(Self::GmGpiOpContinue {
                            response: GpiOpContinueStatus::Incomplete {
                                granule_count,
                                cookie,
                            },
                        })
                    }
                    StatusCode::Busy => {
                        let cookie = regs[2];
                        Ok(Self::GmGpiOpContinue {
                            response: GpiOpContinueStatus::Busy { cookie },
                        })
                    }
                    StatusCode::OpConflict | StatusCode::NotFound | StatusCode::Denied => {
                        Ok(Self::GmGpiOpContinue {
                            response: GpiOpContinueStatus::Error {
                                status,
                                granule_count,
                            },
                        })
                    }
                    _ => Err(Error::UnsuccessfulCall(status)),
                }
            }
            FunctionId::GmL1GptCreate => {
                let status = StatusCode::try_from(regs[0])?;
                if status == StatusCode::Success {
                    Ok(Self::GmL1GptCreate)
                } else {
                    Err(Error::UnsuccessfulCall(status))
                }
            }
            FunctionId::GmL1GptDestroy => {
                let status = StatusCode::try_from(regs[0])?;
                if status == StatusCode::Success {
                    Ok(Self::GmL1GptDestroy {
                        l1_gpt_base: regs[2],
                    })
                } else {
                    Err(Error::UnsuccessfulCall(status))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidGpiEncoding))]
#[repr(u8)]
/// Access control restriction that can be applied to a memory region in the GPT.
pub enum GpiAccessType {
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

impl GpiAccessType {
    /// MASK selecting the useful bits from the `u8` used for storing `GpiAccessType`s.
    pub const MASK: u64 = 0b1111;
}

impl TryFrom<u64> for GpiAccessType {
    type Error = Error;

    fn try_from(reg: u64) -> Result<GpiAccessType, Error> {
        GpiAccessType::try_from((reg & GpiAccessType::MASK) as u8)
    }
}

impl From<GpiAccessType> for u64 {
    fn from(gpi: GpiAccessType) -> u64 {
        u8::from(gpi).into()
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
        let function = Function::try_from(&regs).unwrap();
        assert_eq!(
            function,
            Function::ServiceVersion {
                service_id: ServiceId::GranuleManagement
            }
        );
        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

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
                service_id: ServiceId::IdeKeyManagement
            }
        );
    }

    #[test]
    fn service_version_response() {
        // Successful response
        let regs: [u64; 4] = [0x0001_0002, 0, 0, 0];
        let response = Response::try_from((FunctionId::ServiceVersion, &regs)).unwrap();
        assert_eq!(
            response,
            Response::ServiceVersion {
                service_version: ServiceVersion { major: 1, minor: 2 }
            }
        );
        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Unsuccessful response
        let regs: [u64; 4] = [0xffff_ffff, 0, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::ServiceVersion, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::NotSupported))
        );
    }

    #[test]
    fn function_id_service_features() {
        let service_features = 0x0000_0000_C400_0401;
        let regs: [u64; 4] = [service_features, 1, 0, 0];
        let function = Function::try_from(&regs).unwrap();
        assert_eq!(
            function,
            Function::ServiceFeatures {
                service_id: ServiceId::GranuleManagement,
                feature_reg_index: 0,
            }
        );
        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);
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
                service_id: ServiceId::MecidManagement,
                feature_reg_index: 0xFF,
            }
        );

        let regs = [service_features, 6, 0, 0];
        assert_eq!(Function::try_from(&regs), Err(Error::InvalidServiceId(6)));
    }

    #[test]
    fn invalid_function_id() {
        let mut regs = [0, 0, 0, 0];
        let err = Function::try_from(&regs).err().unwrap();
        assert_eq!(err, Error::UnrecognisedFunctionId(0));
        let status: StatusCode = err.into();
        assert_eq!(status, StatusCode::NotSupported);
        Response::copy_status_to_array(status, &mut regs);
        assert_eq!(regs, [0xffff_ffff, 0, 0, 0]);
    }

    #[test]
    fn service_features_response() {
        // Successful response
        let regs: [u64; 4] = [0, 0b10101 << 16, 0, 0];
        let resp = Response::try_from((FunctionId::ServiceFeatures, &regs)).unwrap();
        match resp {
            Response::ServiceFeatures { register } => {
                let base = BaseServiceFeaturesRegister1::from(register);
                assert!(base.contains(BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT));
                assert!(base.contains(BaseServiceFeaturesRegister1::MECID_MANAGEMENT));
                assert!(base.contains(BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT));
            }
            _ => panic!(),
        }

        let mut regs2 = [u64::MAX; 4];
        resp.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Unsuccessful response
        let regs: [u64; 4] = [0xffff_ffff, 0b10101 << 16, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::ServiceFeatures, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::NotSupported)),
        );
    }

    #[test]
    fn gpi_encodings() {
        assert_eq!(
            GpiAccessType::NoAccess,
            GpiAccessType::try_from(0b0000_0000u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::SystemAgent,
            GpiAccessType::try_from(0b0000_0100u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NonSecureProtected,
            GpiAccessType::try_from(0b0000_0101u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NoAccess6,
            GpiAccessType::try_from(0b0000_0110u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NoAccess7,
            GpiAccessType::try_from(0b0000_0111u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::Secure,
            GpiAccessType::try_from(0b0000_1000u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NonSecure,
            GpiAccessType::try_from(0b0000_1001u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::Root,
            GpiAccessType::try_from(0b0000_1010u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::Realm,
            GpiAccessType::try_from(0b0000_1011u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NonSecureRoot,
            GpiAccessType::try_from(0b0000_1101u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::Any,
            GpiAccessType::try_from(0b0000_1111u8).unwrap()
        );
        assert_eq!(
            GpiAccessType::NoAccess,
            GpiAccessType::try_from(0b1_0000_0000u64).unwrap(),
        );
    }

    #[test]
    fn function_id_gm_gpi_set() {
        let gpi_set = 0x0000_0000_C400_0402;
        let regs: [u64; 4] = [gpi_set, 0x8000_0000, 0xABC, 0x0F];

        let function = Function::try_from(&regs).unwrap();
        assert_eq!(
            function,
            Function::GmGpiSet {
                base_address: 0x8000_0000,
                granule_count: 0xABC,
                target_gpi: GpiAccessType::Any,
            }
        );

        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        let regs = [gpi_set, 0, 0, 1];
        assert_eq!(Function::try_from(&regs), Err(Error::InvalidGpiEncoding(1)));
    }

    #[test]
    fn gm_gpi_set_response() {
        // Successful response
        let regs: [u64; 4] = [0, 2, 0, 0];
        let response = Response::try_from((FunctionId::GmGpiSet, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmGpiSet {
                response: GpiSetStatus::Complete { granule_count: 2 },
            }
        );

        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Incomplete response
        let regs: [u64; 4] = [0xffff_fffc, 2, 0x123, 0];
        let response = Response::try_from((FunctionId::GmGpiSet, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmGpiSet {
                response: GpiSetStatus::Incomplete {
                    granule_count: 2,
                    cookie: 0x123,
                },
            }
        );

        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Valid error responses
        for status in [
            StatusCode::Denied,
            StatusCode::OpConflict,
            StatusCode::NotFound,
        ] {
            let regs: [u64; 4] = [status.into(), 2, 0, 0];
            let response = Response::try_from((FunctionId::GmGpiSet, &regs)).unwrap();
            assert_eq!(
                response,
                Response::GmGpiSet {
                    response: GpiSetStatus::Error {
                        status,
                        granule_count: 2
                    }
                }
            );
            let mut regs2 = [u64::MAX; 4];
            response.copy_to_array(&mut regs2);
            assert_eq!(regs, regs2);
        }

        // Other unexpected response
        let regs: [u64; 4] = [0xffff_fff8, 2, 0x123, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGpiSet, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::AlreadyExists)),
        );
    }

    #[test]
    fn function_id_gm_gpi_op_continue() {
        let id = 0x0000_0000_C400_0412;
        let regs: [u64; 4] = [id, 0x123, 0, 0];

        let function = Function::try_from(&regs).unwrap();
        assert_eq!(function, Function::GmGpiOpContinue { cookie: 0x123 });

        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);
    }

    #[test]
    fn gm_gpi_op_continue_response() {
        // Successful response
        let regs: [u64; 4] = [0, 2, 0, 0];
        let response = Response::try_from((FunctionId::GmGpiOpContinue, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmGpiOpContinue {
                response: GpiOpContinueStatus::Complete { granule_count: 2 },
            }
        );
        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Incomplete response
        let regs: [u64; 4] = [0xffff_fffc, 2, 0x123, 0];
        let response = Response::try_from((FunctionId::GmGpiOpContinue, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmGpiOpContinue {
                response: GpiOpContinueStatus::Incomplete {
                    granule_count: 2,
                    cookie: 0x123,
                }
            }
        );
        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Busy response
        let regs: [u64; 4] = [0xffff_fffa, 0, 0x123, 0];
        let response = Response::try_from((FunctionId::GmGpiOpContinue, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmGpiOpContinue {
                response: GpiOpContinueStatus::Busy { cookie: 0x123 }
            }
        );
        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Valid error responses
        for status in [
            StatusCode::Denied,
            StatusCode::OpConflict,
            StatusCode::NotFound,
        ] {
            let regs: [u64; 4] = [status.into(), 2, 0, 0];
            let response = Response::try_from((FunctionId::GmGpiOpContinue, &regs)).unwrap();
            assert_eq!(
                response,
                Response::GmGpiOpContinue {
                    response: GpiOpContinueStatus::Error {
                        status,
                        granule_count: 2,
                    }
                }
            );
            let mut regs2 = [u64::MAX; 4];
            response.copy_to_array(&mut regs2);
            assert_eq!(regs, regs2);
        }

        // Other unexpected error
        let regs: [u64; 4] = [0xffff_fff6, 2, 0x123, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmGpiOpContinue, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::NoMemory)),
        );
    }

    #[test]
    fn function_id_gm_l1_gpt_create() {
        let id = 0x0000_0000_C400_040E;
        let regs: [u64; 4] = [id, 0xabcd_0000, 0xabcd_ffff, 0];

        let function = Function::try_from(&regs).unwrap();
        assert_eq!(
            function,
            Function::GmL1GptCreate {
                pa_range_base: 0xabcd_0000,
                l1_gpt_base: 0xabcd_ffff
            }
        );
        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);
    }

    #[test]
    fn gm_l1_gpt_create_response() {
        // Successful response
        let regs: [u64; 4] = [0, 0, 0, 0];
        let response = Response::try_from((FunctionId::GmL1GptCreate, &regs)).unwrap();
        assert_eq!(response, Response::GmL1GptCreate,);

        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Unsuccessful response
        let regs: [u64; 4] = [0xffff_fffb, 0, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmL1GptCreate, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::Denied)),
        );
    }

    #[test]
    fn function_id_gm_l1_gpt_destroy() {
        let id = 0x0000_0000_C400_040F;
        let regs: [u64; 4] = [id, 0xabcd_0000, 0, 0];

        let function = Function::try_from(&regs).unwrap();
        assert_eq!(
            function,
            Function::GmL1GptDestroy {
                pa_range_base: 0xabcd_0000,
            }
        );
        let mut regs2 = [u64::MAX; 4];
        function.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);
    }

    #[test]
    fn gm_l1_gpt_destroy_response() {
        // Successful response
        let regs: [u64; 4] = [0, 0, 0xabcd_ffff, 0];
        let response = Response::try_from((FunctionId::GmL1GptDestroy, &regs)).unwrap();
        assert_eq!(
            response,
            Response::GmL1GptDestroy {
                l1_gpt_base: 0xabcd_ffff,
            },
        );
        let mut regs2 = [u64::MAX; 4];
        response.copy_to_array(&mut regs2);
        assert_eq!(regs, regs2);

        // Unsuccessful response
        let regs: [u64; 4] = [0xffff_fffb, 0, 0, 0];
        assert_eq!(
            Response::try_from((FunctionId::GmL1GptDestroy, &regs)),
            Err(Error::UnsuccessfulCall(StatusCode::Denied)),
        );
    }
}
