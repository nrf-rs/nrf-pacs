#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0a00],
    irr: Irr,
    imr: Imr,
    icr: Icr,
    endianness: Endianness,
    _reserved4: [u8; 0x14],
    host_signature: HostSignature,
    host_boot: HostBoot,
    _reserved6: [u8; 0x0c],
    host_cryptokey_sel: HostCryptokeySel,
    _reserved7: [u8; 0x10],
    host_iot_kprtl_lock: HostIotKprtlLock,
    host_iot_kdr0: HostIotKdr0,
    host_iot_kdr1: HostIotKdr1,
    host_iot_kdr2: HostIotKdr2,
    host_iot_kdr3: HostIotKdr3,
    host_iot_lcs: HostIotLcs,
}
impl RegisterBlock {
    #[doc = "0xa00 - Interrupt request register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding IMR bit is unmasked, an interrupt is generated."]
    #[inline(always)]
    pub const fn irr(&self) -> &Irr {
        &self.irr
    }
    #[doc = "0xa04 - Interrupt mask register. Each bit of this register holds the mask of a single interrupt source."]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0xa08 - Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in IRR."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0xa0c - This register defines the endianness of the Host-accessible registers, and can only be written once."]
    #[inline(always)]
    pub const fn endianness(&self) -> &Endianness {
        &self.endianness
    }
    #[doc = "0xa24 - This register holds the CRYPTOCELL subsystem signature. See reset value."]
    #[inline(always)]
    pub const fn host_signature(&self) -> &HostSignature {
        &self.host_signature
    }
    #[doc = "0xa28 - Hardware configuration of the CRYPTOCELL subsystem. Reset value holds the supported features."]
    #[inline(always)]
    pub const fn host_boot(&self) -> &HostBoot {
        &self.host_boot
    }
    #[doc = "0xa38 - AES hardware key select."]
    #[inline(always)]
    pub const fn host_cryptokey_sel(&self) -> &HostCryptokeySel {
        &self.host_cryptokey_sel
    }
    #[doc = "0xa4c - This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kprtl_lock(&self) -> &HostIotKprtlLock {
        &self.host_iot_kprtl_lock
    }
    #[doc = "0xa50 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
    #[inline(always)]
    pub const fn host_iot_kdr0(&self) -> &HostIotKdr0 {
        &self.host_iot_kdr0
    }
    #[doc = "0xa54 - This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr1(&self) -> &HostIotKdr1 {
        &self.host_iot_kdr1
    }
    #[doc = "0xa58 - This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr2(&self) -> &HostIotKdr2 {
        &self.host_iot_kdr2
    }
    #[doc = "0xa5c - This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr3(&self) -> &HostIotKdr3 {
        &self.host_iot_kdr3
    }
    #[doc = "0xa60 - Controls life-cycle state (LCS) for CRYPTOCELL subsystem"]
    #[inline(always)]
    pub const fn host_iot_lcs(&self) -> &HostIotLcs {
        &self.host_iot_lcs
    }
}
#[doc = "IRR (r) register accessor: Interrupt request register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding IMR bit is unmasked, an interrupt is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`irr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irr`] module"]
#[doc(alias = "IRR")]
pub type Irr = crate::Reg<irr::IrrSpec>;
#[doc = "Interrupt request register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding IMR bit is unmasked, an interrupt is generated."]
pub mod irr;
#[doc = "IMR (rw) register accessor: Interrupt mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt mask register. Each bit of this register holds the mask of a single interrupt source."]
pub mod imr;
#[doc = "ICR (w) register accessor: Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in IRR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in IRR."]
pub mod icr;
#[doc = "ENDIANNESS (rw) register accessor: This register defines the endianness of the Host-accessible registers, and can only be written once.\n\nYou can [`read`](crate::Reg::read) this register and get [`endianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endianness`] module"]
#[doc(alias = "ENDIANNESS")]
pub type Endianness = crate::Reg<endianness::EndiannessSpec>;
#[doc = "This register defines the endianness of the Host-accessible registers, and can only be written once."]
pub mod endianness;
#[doc = "HOST_SIGNATURE (r) register accessor: This register holds the CRYPTOCELL subsystem signature. See reset value.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_signature::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_signature`] module"]
#[doc(alias = "HOST_SIGNATURE")]
pub type HostSignature = crate::Reg<host_signature::HostSignatureSpec>;
#[doc = "This register holds the CRYPTOCELL subsystem signature. See reset value."]
pub mod host_signature;
#[doc = "HOST_BOOT (r) register accessor: Hardware configuration of the CRYPTOCELL subsystem. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_boot::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_boot`] module"]
#[doc(alias = "HOST_BOOT")]
pub type HostBoot = crate::Reg<host_boot::HostBootSpec>;
#[doc = "Hardware configuration of the CRYPTOCELL subsystem. Reset value holds the supported features."]
pub mod host_boot;
#[doc = "HOST_CRYPTOKEY_SEL (rw) register accessor: AES hardware key select.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cryptokey_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cryptokey_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cryptokey_sel`] module"]
#[doc(alias = "HOST_CRYPTOKEY_SEL")]
pub type HostCryptokeySel = crate::Reg<host_cryptokey_sel::HostCryptokeySelSpec>;
#[doc = "AES hardware key select."]
pub mod host_cryptokey_sel;
#[doc = "HOST_IOT_KPRTL_LOCK (rw) register accessor: This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_kprtl_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kprtl_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kprtl_lock`] module"]
#[doc(alias = "HOST_IOT_KPRTL_LOCK")]
pub type HostIotKprtlLock = crate::Reg<host_iot_kprtl_lock::HostIotKprtlLockSpec>;
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kprtl_lock;
#[doc = "HOST_IOT_KDR0 (rw) register accessor: This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_kdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr0`] module"]
#[doc(alias = "HOST_IOT_KDR0")]
pub type HostIotKdr0 = crate::Reg<host_iot_kdr0::HostIotKdr0Spec>;
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
pub mod host_iot_kdr0;
#[doc = "HOST_IOT_KDR1 (w) register accessor: This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr1`] module"]
#[doc(alias = "HOST_IOT_KDR1")]
pub type HostIotKdr1 = crate::Reg<host_iot_kdr1::HostIotKdr1Spec>;
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr1;
#[doc = "HOST_IOT_KDR2 (w) register accessor: This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr2`] module"]
#[doc(alias = "HOST_IOT_KDR2")]
pub type HostIotKdr2 = crate::Reg<host_iot_kdr2::HostIotKdr2Spec>;
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr2;
#[doc = "HOST_IOT_KDR3 (w) register accessor: This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_kdr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr3`] module"]
#[doc(alias = "HOST_IOT_KDR3")]
pub type HostIotKdr3 = crate::Reg<host_iot_kdr3::HostIotKdr3Spec>;
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr3;
#[doc = "HOST_IOT_LCS (rw) register accessor: Controls life-cycle state (LCS) for CRYPTOCELL subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_lcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_lcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_lcs`] module"]
#[doc(alias = "HOST_IOT_LCS")]
pub type HostIotLcs = crate::Reg<host_iot_lcs::HostIotLcsSpec>;
#[doc = "Controls life-cycle state (LCS) for CRYPTOCELL subsystem"]
pub mod host_iot_lcs;
