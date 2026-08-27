// SPDX-FileCopyrightText: Copyright The arm-firme Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Feature discovery

use crate::Error;
use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Generic feature register wrapper type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FeatureRegister(pub(crate) u64);

/// Minimum shared buffer size
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidSharedBufferMinSize))]
#[repr(u8)]
pub enum SharedBufferMinSize {
    /// 4 KB
    KB4 = 0b00,
    /// 64 KB
    KB64 = 0b01,
    /// 16 KB
    KB16 = 0b10,
}

impl SharedBufferMinSize {
    /// Size in KB
    pub fn size_kb(self) -> usize {
        match self {
            Self::KB4 => 4,
            Self::KB64 => 64,
            Self::KB16 => 16,
        }
    }
}

bitflags! {
    /// Base service features register 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct BaseServiceFeaturesRegister0 : u64 {
        /// FIRME_SERVICE_FEATURES ABI is present.
        const SERVICE_FEATURES = 1 << 1;
        /// FIRME_SERVICE_VERSION ABI is present.
        const SERVICE_VERSION = 1 << 0;
    }
}

impl From<FeatureRegister> for BaseServiceFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<BaseServiceFeaturesRegister0> for FeatureRegister {
    fn from(value: BaseServiceFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

bitflags! {
    /// Base service features register 1
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct BaseServiceFeaturesRegister1 : u64 {
        /// INTEGRATED_DEVICE_MANAGEMENT bit
        const INTEGRATED_DEVICE_MANAGEMENT = 1 << 20;
        /// ATTESTATION bit
        const ATTESTATION = 1 << 19;
        /// MECID_MANAGEMENT bit
        const MECID_MANAGEMENT = 1 << 18;
        /// IDE_KEY_MANAGEMENT bit
        const IDE_KEY_MANAGEMENT = 1 << 17;
        /// GRANULE_MANAGEMENT bit
        const GRANULE_MANAGEMENT = 1 << 16;
    }
}

impl From<FeatureRegister> for BaseServiceFeaturesRegister1 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<BaseServiceFeaturesRegister1> for FeatureRegister {
    fn from(value: BaseServiceFeaturesRegister1) -> Self {
        Self(value.bits())
    }
}

impl BaseServiceFeaturesRegister1 {
    const MAX_SH_BUF_PG_CNT_SHIFT: u32 = 2;
    const MAX_SH_BUF_PG_CNT_MASK: u64 = 0b0011_1111_1111_1111;
    const SH_BUF_MIN_SZ_SHIFT: u32 = 0;
    const SH_BUF_MIN_SZ_MASK: u64 = 0b11;

    /// Return the maximum size of the shared buffer in KB
    pub fn shared_buffer_max_size_kb(&self) -> Result<usize, Error> {
        Ok(((self.max_sh_buf_pg_cnt() + 1) as usize) * self.shared_buffer_min_size()?.size_kb())
    }

    /// Return the maximum shared buffer page count
    pub const fn max_sh_buf_pg_cnt(&self) -> u64 {
        (self.bits() >> Self::MAX_SH_BUF_PG_CNT_SHIFT) & Self::MAX_SH_BUF_PG_CNT_MASK
    }

    /// Set the maximum shared buffer page count
    pub const fn set_max_sh_buf_pg_cnt(&mut self, value: u64) {
        let offset = Self::MAX_SH_BUF_PG_CNT_SHIFT;
        assert!(value & Self::MAX_SH_BUF_PG_CNT_MASK == value);
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::MAX_SH_BUF_PG_CNT_MASK << offset)) | (value << offset),
        );
    }

    /// Set the maximum shared buffer page count
    pub const fn with_max_sh_buf_pg_cnt(mut self, value: u64) -> Self {
        self.set_max_sh_buf_pg_cnt(value);
        self
    }

    /// Return the minimum shared buffer size
    pub fn shared_buffer_min_size(&self) -> Result<SharedBufferMinSize, Error> {
        let bits: u8 =
            ((self.bits() >> Self::SH_BUF_MIN_SZ_SHIFT) & Self::SH_BUF_MIN_SZ_MASK) as u8;
        SharedBufferMinSize::try_from(bits)
    }

    /// Set the minimum shared buffer size
    pub fn set_shared_buffer_min_size(&mut self, value: SharedBufferMinSize) {
        let offset = Self::SH_BUF_MIN_SZ_SHIFT;
        let raw: u8 = value.into();
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::SH_BUF_MIN_SZ_MASK << offset)) | (u64::from(raw) << offset),
        );
    }

    /// Set the minimum shared buffer size
    pub fn with_shared_buffer_min_size(mut self, value: SharedBufferMinSize) -> Self {
        self.set_shared_buffer_min_size(value);
        self
    }
}

/// Physical Granule size as encoded in the GPCCR_EL3.PGS field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidPhysicalGranuleSize))]
#[repr(u8)]
pub enum PhysicalGranuleSize {
    /// 4KB
    KB4 = 0b00,
    /// 64KB
    KB64 = 0b01,
    /// 16KB
    KB16 = 0b10,
}

/// Level 0 GPT Size
///
/// Size of the memory corresponding to one L0 GPT entry as encoded in the GPCCR_EL3.L0GPTSZ field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidLevel0GptSize))]
#[repr(u8)]
pub enum Level0GptSize {
    /// 1GB
    GB1 = 0b0000,
    /// 16GB
    GB16 = 0b0100,
    /// 64GB
    GB64 = 0b0110,
    /// 512GB
    GB512 = 0b1001,
}

/// Protected Physical Address Size.
///
/// Size of the total memory protected by the GPT, as encoded in the GPCCR_EL3.PPS field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidProtectedPhysicalAddressSize))]
#[repr(u8)]
pub enum ProtectedPhysicalAddressSize {
    /// 4GB
    GB4 = 0b000,
    /// 64GB
    GB64 = 0b001,
    /// 1TB
    TB1 = 0b010,
    /// 4TB
    TB4 = 0b011,
    /// 16TB
    TB16 = 0b100,
    /// 256TB
    TB256 = 0b101,
    /// 4PB
    PB4 = 0b110,
}

bitflags! {
    /// Feature register 0 for the Granule Management Service
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct GranuleManagementFeaturesRegister0 : u64 {
        /// L1_GPT_DESTROY bit
        const L1_GPT_DESTROY = 1 << 3;
        /// L1_GPT_CREATE bit
        const L1_GPT_CREATE = 1 << 2;
        /// GPI_OP_CONTINUE bit
        const GPI_OP_CONTINUE = 1 << 1;
        /// GPI_SET bit
        const GPI_SET = 1 << 0;
    }
}

impl From<FeatureRegister> for GranuleManagementFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<GranuleManagementFeaturesRegister0> for FeatureRegister {
    fn from(value: GranuleManagementFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

bitflags! {
    /// Feature register 1 for the Granule Management Service
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GranuleManagementFeaturesRegister1 : u64 {}
}

impl From<FeatureRegister> for GranuleManagementFeaturesRegister1 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<GranuleManagementFeaturesRegister1> for FeatureRegister {
    fn from(value: GranuleManagementFeaturesRegister1) -> Self {
        Self(value.bits())
    }
}

impl GranuleManagementFeaturesRegister1 {
    const PPS_SHIFT: u32 = 6;
    const PPS_MASK: u64 = 0b111;
    const L0GPTSZ_SHIFT: u32 = 2;
    const L0GPTSZ_MASK: u64 = 0b1111;
    const PGS_SHIFT: u32 = 0;
    const PGS_MASK: u64 = 0b11;

    /// Return the Granule Protection Table's protected physical address size.
    pub fn protected_physical_address_size(&self) -> Result<ProtectedPhysicalAddressSize, Error> {
        let bits = ((self.bits() >> Self::PPS_SHIFT) & Self::PPS_MASK) as u8;
        ProtectedPhysicalAddressSize::try_from(bits)
    }

    /// Set the protected physical address size
    pub fn set_protected_physical_address_size(&mut self, value: ProtectedPhysicalAddressSize) {
        let offset = Self::PPS_SHIFT;
        let raw: u8 = value.into();
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PPS_MASK << offset)) | (u64::from(raw) << offset),
        );
    }

    /// Set the protected physical address size
    pub fn with_protected_physical_address_size(
        mut self,
        value: ProtectedPhysicalAddressSize,
    ) -> Self {
        self.set_protected_physical_address_size(value);
        self
    }

    /// Return the Granule Protection Table's level 0 GPT size.
    pub fn level0_gpt_size(&self) -> Result<Level0GptSize, Error> {
        let bits = ((self.bits() >> Self::L0GPTSZ_SHIFT) & Self::L0GPTSZ_MASK) as u8;
        Level0GptSize::try_from(bits)
    }

    /// Set the level 0 GPT size
    pub fn set_level0_gpt_size(&mut self, value: Level0GptSize) {
        let offset = Self::L0GPTSZ_SHIFT;
        let raw: u8 = value.into();
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::L0GPTSZ_MASK << offset)) | (u64::from(raw) << offset),
        );
    }

    /// Set the level 0 GPT size
    pub fn with_level0_gpt_size(mut self, value: Level0GptSize) -> Self {
        self.set_level0_gpt_size(value);
        self
    }

    /// Return the physical granule size.
    pub fn physical_granule_size(&self) -> Result<PhysicalGranuleSize, Error> {
        let bits = ((self.bits() >> Self::PGS_SHIFT) & Self::PGS_MASK) as u8;
        PhysicalGranuleSize::try_from(bits)
    }

    /// Set the physical granule size
    pub fn set_physical_granule_size(&mut self, value: PhysicalGranuleSize) {
        let offset = Self::PGS_SHIFT;
        let raw: u8 = value.into();
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::PGS_MASK << offset)) | (u64::from(raw) << offset),
        );
    }

    /// Set the physical granule size
    pub fn with_physical_granule_size(mut self, value: PhysicalGranuleSize) -> Self {
        self.set_physical_granule_size(value);
        self
    }
}

bitflags! {
    /// IDE key management service's feature register 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct IdeKeyManagementFeaturesRegister0 : u64 {
        /// KEYSET_POLL bit
        const KEYSET_POLL = 1 << 3;
        /// KEYSET_STOP bit
        const KEYSET_STOP = 1 << 2;
        /// KEYSET_GO bit
        const KEYSET_GO = 1 << 1;
        /// KEYSET_PROG bit
        const KEYSET_PROG = 1 << 0;
    }
}

impl From<FeatureRegister> for IdeKeyManagementFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<IdeKeyManagementFeaturesRegister0> for FeatureRegister {
    fn from(value: IdeKeyManagementFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

bitflags! {
    /// MECID management service's feature register 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct MecidManagementFeaturesRegister0 : u64 {
        /// MEC_REFRESH bit
        const MEC_REFRESH = 1 << 0;
    }
}

impl From<FeatureRegister> for MecidManagementFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<MecidManagementFeaturesRegister0> for FeatureRegister {
    fn from(value: MecidManagementFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

/// Describes how a portion of the Realm attestation key is encoded in the shared buffer
/// in an invocation of the FIRME_ATTEST_RAK_GET ABI.
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidRakFormat))]
#[repr(u8)]
pub enum AttestationKeyFormat {
    /// Implementation defined encoding.
    ImpDef = 0b00,
    /// COSE key structure.
    Cose = 0b01,
}

bitflags! {
    /// Attestation service's feature register 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct AttestationFeaturesRegister0 : u64 {
        /// PAT_EXT_CLAIMS_CLEAR bit
        const PAT_EXT_CLAIMS_CLEAR = 1 << 5;
        /// PAT_EXT_CLAIMS_FINALISE bit
        const PAT_EXT_CLAIMS_FINALISE = 1 << 4;
        /// PAT_EXT_CLAIMS_STAGE bit
        const PAT_EXT_CLAIMS_STAGE = 1 << 3;
        /// RAT_SIGN bit
        const RAT_SIGN = 1 << 2;
        /// RAK_GET bit
        const RAK_GET = 1 << 1;
        /// PAT_GET bit
        const PAT_GET = 1 << 0;
    }
}

impl From<FeatureRegister> for AttestationFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<AttestationFeaturesRegister0> for FeatureRegister {
    fn from(value: AttestationFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

bitflags! {
    /// Attestation service's Feature Register1
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct AttestationFeaturesRegister1 : u64 {
        /// RAK_PUB_POR bit
        const RAK_PUB_POR = 1 << 12;
    }
}

impl From<FeatureRegister> for AttestationFeaturesRegister1 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<AttestationFeaturesRegister1> for FeatureRegister {
    fn from(value: AttestationFeaturesRegister1) -> Self {
        Self(value.bits())
    }
}

impl AttestationFeaturesRegister1 {
    const RAK_FORMAT_SHIFT: u32 = 10;
    const RAK_FORMAT_MASK: u64 = 0b11;
    const MAX_PAT_EXT_BUF_PG_CNT_SHIFT: u32 = 8;
    const MAX_PAT_EXT_BUF_PG_CNT_MASK: u8 = 0b11;
    const MAX_PAT_PG_CNT_SHIFT: u32 = 0;
    const MAX_PAT_PG_CNT_MASK: u8 = 0b1111_1111;

    /// Return the Realm attestation key encoding format.
    pub fn rak_format(&self) -> Result<AttestationKeyFormat, Error> {
        let bits = ((self.bits() >> Self::RAK_FORMAT_SHIFT) & Self::RAK_FORMAT_MASK) as u8;
        AttestationKeyFormat::try_from(bits)
    }

    /// Set the realm attestation key format
    pub fn set_rak_format(&mut self, value: AttestationKeyFormat) {
        let offset = Self::RAK_FORMAT_SHIFT;
        let raw: u8 = value.into();
        *self = Self::from_bits_retain(
            (self.bits() & !(Self::RAK_FORMAT_MASK << offset)) | (u64::from(raw) << offset),
        );
    }

    /// Set the realm attestation key format
    pub fn with_rak_format(mut self, rak_format: AttestationKeyFormat) -> Self {
        self.set_rak_format(rak_format);
        self
    }

    /// Return the maximum PAT extension-buffer page count field
    pub const fn max_pat_ext_buf_pg_cnt(&self) -> u8 {
        ((self.bits() >> Self::MAX_PAT_EXT_BUF_PG_CNT_SHIFT)
            & Self::MAX_PAT_EXT_BUF_PG_CNT_MASK as u64) as u8
    }

    /// Set the maximum PAT extension-buffer page count field.
    pub fn set_max_pat_ext_buf_pg_cnt(&mut self, value: u8) {
        assert!(value & Self::MAX_PAT_EXT_BUF_PG_CNT_MASK == value);
        let offset = Self::MAX_PAT_EXT_BUF_PG_CNT_SHIFT;
        *self = Self::from_bits_retain(
            (self.bits() & !((Self::MAX_PAT_EXT_BUF_PG_CNT_MASK as u64) << offset))
                | (u64::from(value) << offset),
        );
    }

    /// Set the maximum PAT extension-buffer page count field.
    pub fn with_max_pat_ext_buf_pg_cnt(mut self, value: u8) -> Self {
        self.set_max_pat_ext_buf_pg_cnt(value);
        self
    }

    /// Return the maximum PAT page count field.
    pub const fn max_pat_pg_cnt(&self) -> u8 {
        ((self.bits() >> Self::MAX_PAT_PG_CNT_SHIFT) & Self::MAX_PAT_PG_CNT_MASK as u64) as u8
    }

    /// Set the maximum PAT page count field.
    pub fn set_max_pat_pg_cnt(&mut self, value: u8) {
        assert!(value & Self::MAX_PAT_PG_CNT_MASK == value);
        let offset = Self::MAX_PAT_PG_CNT_SHIFT;
        *self = Self::from_bits_retain(
            (self.bits() & !((Self::MAX_PAT_PG_CNT_MASK as u64) << offset))
                | (u64::from(value) << offset),
        );
    }

    /// Set the maximum PAT page count field.
    pub fn with_max_pat_pg_cnt(mut self, value: u8) -> Self {
        self.set_max_pat_pg_cnt(value);
        self
    }
}

bitflags! {
    /// Integrated device management service's feature register 0
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct IntegratedDeviceManagementFeaturesRegister0 : u64 {
        /// OP_CONTINUE bit
        const OP_CONTINUE = 1 << 1;
        /// OP_START bit
        const OP_START = 1 << 0;
    }
}

impl From<FeatureRegister> for IntegratedDeviceManagementFeaturesRegister0 {
    fn from(value: FeatureRegister) -> Self {
        Self::from_bits_retain(value.0)
    }
}

impl From<IntegratedDeviceManagementFeaturesRegister0> for FeatureRegister {
    fn from(value: IntegratedDeviceManagementFeaturesRegister0) -> Self {
        Self(value.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn ide_key_management_feat() {
        let reg0 = IdeKeyManagementFeaturesRegister0::KEYSET_POLL
            | IdeKeyManagementFeaturesRegister0::KEYSET_STOP
            | IdeKeyManagementFeaturesRegister0::KEYSET_GO
            | IdeKeyManagementFeaturesRegister0::KEYSET_PROG;
        assert_eq!(reg0.bits(), 0x0000_000F);

        let reg1: IdeKeyManagementFeaturesRegister0 = FeatureRegister(reg0.bits()).into();
        assert!(reg1.contains(IdeKeyManagementFeaturesRegister0::KEYSET_POLL));
        assert!(reg1.contains(IdeKeyManagementFeaturesRegister0::KEYSET_STOP));
        assert!(reg1.contains(IdeKeyManagementFeaturesRegister0::KEYSET_GO));
        assert!(reg1.contains(IdeKeyManagementFeaturesRegister0::KEYSET_PROG));
    }

    #[test]
    fn granule_management_feat() {
        let reg0 = GranuleManagementFeaturesRegister0::L1_GPT_DESTROY
            | GranuleManagementFeaturesRegister0::L1_GPT_CREATE
            | GranuleManagementFeaturesRegister0::GPI_SET;
        assert_eq!(reg0.bits(), 0x0000_000D);

        let reg01: GranuleManagementFeaturesRegister0 = FeatureRegister(reg0.bits()).into();
        assert!(reg01.contains(GranuleManagementFeaturesRegister0::L1_GPT_DESTROY));
        assert!(reg01.contains(GranuleManagementFeaturesRegister0::L1_GPT_CREATE));
        assert!(reg01.contains(GranuleManagementFeaturesRegister0::GPI_SET));

        let reg1 = GranuleManagementFeaturesRegister1::empty()
            .with_protected_physical_address_size(ProtectedPhysicalAddressSize::TB4)
            .with_level0_gpt_size(Level0GptSize::GB64)
            .with_physical_granule_size(PhysicalGranuleSize::KB16);
        assert_eq!(reg1.bits(), 0x0000_00DA);

        let reg11: GranuleManagementFeaturesRegister1 = FeatureRegister(reg1.bits()).into();
        assert_eq!(
            reg11.protected_physical_address_size(),
            Ok(ProtectedPhysicalAddressSize::TB4)
        );
        assert_eq!(reg11.level0_gpt_size(), Ok(Level0GptSize::GB64));
        assert_eq!(reg11.physical_granule_size(), Ok(PhysicalGranuleSize::KB16));
    }

    #[test]
    fn invalid_protected_physical_size() {
        assert_eq!(
            GranuleManagementFeaturesRegister1::from_bits_retain(0x1C0)
                .protected_physical_address_size(),
            Err(Error::InvalidProtectedPhysicalAddressSize(7))
        );
    }

    #[test]
    fn invalid_level0_gpt_size() {
        assert_eq!(
            GranuleManagementFeaturesRegister1::from_bits_retain(0x8).level0_gpt_size(),
            Err(Error::InvalidLevel0GptSize(2))
        );
    }

    #[test]
    fn invalid_physical_granule_size() {
        assert_eq!(
            GranuleManagementFeaturesRegister1::from_bits_retain(0x3).physical_granule_size(),
            Err(Error::InvalidPhysicalGranuleSize(3))
        );
    }

    #[test]
    fn base_service_feat() {
        let reg0 = BaseServiceFeaturesRegister0::SERVICE_FEATURES
            | BaseServiceFeaturesRegister0::SERVICE_VERSION;
        assert_eq!(reg0.bits(), 0x0000_0003);

        let reg01: BaseServiceFeaturesRegister0 = FeatureRegister(reg0.bits()).into();
        assert!(reg01.contains(BaseServiceFeaturesRegister0::SERVICE_FEATURES));
        assert!(reg01.contains(BaseServiceFeaturesRegister0::SERVICE_VERSION));

        let reg1 = (BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT
            | BaseServiceFeaturesRegister1::MECID_MANAGEMENT
            | BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT)
            .with_max_sh_buf_pg_cnt(0x123)
            .with_shared_buffer_min_size(SharedBufferMinSize::KB16);
        assert_eq!(reg1.bits(), 0x0015_048E);
        assert_eq!(reg1.shared_buffer_max_size_kb(), Ok(0x124 * 16));

        let reg11: BaseServiceFeaturesRegister1 = FeatureRegister(reg1.bits()).into();
        assert!(reg11.contains(BaseServiceFeaturesRegister1::INTEGRATED_DEVICE_MANAGEMENT));
        assert!(reg11.contains(BaseServiceFeaturesRegister1::MECID_MANAGEMENT));
        assert!(reg11.contains(BaseServiceFeaturesRegister1::GRANULE_MANAGEMENT));
        assert_eq!(reg11.max_sh_buf_pg_cnt(), 0x123);
        assert_eq!(
            reg11.shared_buffer_min_size(),
            Ok(SharedBufferMinSize::KB16)
        );
    }

    #[test]
    fn invalid_shared_buffer_min_size() {
        assert_eq!(
            BaseServiceFeaturesRegister1::from_bits_retain(0x3).shared_buffer_min_size(),
            Err(Error::InvalidSharedBufferMinSize(3))
        );
    }

    #[test]
    fn attestation_feat() {
        let reg0 = AttestationFeaturesRegister0::PAT_EXT_CLAIMS_CLEAR
            | AttestationFeaturesRegister0::PAT_EXT_CLAIMS_STAGE
            | AttestationFeaturesRegister0::RAT_SIGN
            | AttestationFeaturesRegister0::PAT_GET;
        assert_eq!(reg0.bits(), 0x0000_002D);

        let reg01: AttestationFeaturesRegister0 = FeatureRegister(reg0.bits()).into();
        assert!(reg01.contains(AttestationFeaturesRegister0::PAT_EXT_CLAIMS_CLEAR));
        assert!(reg01.contains(AttestationFeaturesRegister0::PAT_EXT_CLAIMS_STAGE));
        assert!(reg01.contains(AttestationFeaturesRegister0::RAT_SIGN));
        assert!(reg01.contains(AttestationFeaturesRegister0::PAT_GET));

        let reg1 = AttestationFeaturesRegister1::RAK_PUB_POR
            .with_rak_format(AttestationKeyFormat::Cose)
            .with_max_pat_ext_buf_pg_cnt(0x02)
            .with_max_pat_pg_cnt(0xAB);
        assert_eq!(reg1.bits(), 0x0000_16AB);

        let reg11: AttestationFeaturesRegister1 = FeatureRegister(reg1.bits()).into();
        assert_eq!(reg11.rak_format(), Ok(AttestationKeyFormat::Cose));
        assert_eq!(reg11.max_pat_ext_buf_pg_cnt(), 0x02);
        assert_eq!(reg11.max_pat_pg_cnt(), 0xAB);
    }

    #[test]
    fn invalid_rak_format() {
        assert_eq!(
            AttestationFeaturesRegister1::from_bits_retain(0xC00).rak_format(),
            Err(Error::InvalidRakFormat(3))
        );
    }

    #[test]
    fn idev_feat() {
        let reg0 = IntegratedDeviceManagementFeaturesRegister0::OP_CONTINUE;
        assert_eq!(reg0.bits(), 0x0000_0002);
    }

    #[test]
    fn mecid_feat() {
        let reg0 = MecidManagementFeaturesRegister0::MEC_REFRESH;
        assert_eq!(reg0.bits(), 0x0000_0001);
    }
}
