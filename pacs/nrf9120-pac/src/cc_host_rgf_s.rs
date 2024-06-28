#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1a38],
    host_cryptokey_sel: HostCryptokeySel,
    _reserved1: [u8; 0x10],
    host_iot_kprtl_lock: HostIotKprtlLock,
    host_iot_kdr0: HostIotKdr0,
    host_iot_kdr1: HostIotKdr1,
    host_iot_kdr2: HostIotKdr2,
    host_iot_kdr3: HostIotKdr3,
    host_iot_lcs: HostIotLcs,
}
impl RegisterBlock {
    #[doc = "0x1a38 - AES hardware key select"]
    #[inline(always)]
    pub const fn host_cryptokey_sel(&self) -> &HostCryptokeySel {
        &self.host_cryptokey_sel
    }
    #[doc = "0x1a4c - This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kprtl_lock(&self) -> &HostIotKprtlLock {
        &self.host_iot_kprtl_lock
    }
    #[doc = "0x1a50 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
    #[inline(always)]
    pub const fn host_iot_kdr0(&self) -> &HostIotKdr0 {
        &self.host_iot_kdr0
    }
    #[doc = "0x1a54 - This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr1(&self) -> &HostIotKdr1 {
        &self.host_iot_kdr1
    }
    #[doc = "0x1a58 - This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr2(&self) -> &HostIotKdr2 {
        &self.host_iot_kdr2
    }
    #[doc = "0x1a5c - This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kdr3(&self) -> &HostIotKdr3 {
        &self.host_iot_kdr3
    }
    #[doc = "0x1a60 - Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
    #[inline(always)]
    pub const fn host_iot_lcs(&self) -> &HostIotLcs {
        &self.host_iot_lcs
    }
}
#[doc = "HOST_CRYPTOKEY_SEL (rw) register accessor: AES hardware key select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_cryptokey_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_cryptokey_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cryptokey_sel`]
module"]
#[doc(alias = "HOST_CRYPTOKEY_SEL")]
pub type HostCryptokeySel = crate::Reg<host_cryptokey_sel::HostCryptokeySelSpec>;
#[doc = "AES hardware key select"]
pub mod host_cryptokey_sel;
#[doc = "HOST_IOT_KPRTL_LOCK (rw) register accessor: This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_iot_kprtl_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kprtl_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kprtl_lock`]
module"]
#[doc(alias = "HOST_IOT_KPRTL_LOCK")]
pub type HostIotKprtlLock = crate::Reg<host_iot_kprtl_lock::HostIotKprtlLockSpec>;
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kprtl_lock;
#[doc = "HOST_IOT_KDR0 (rw) register accessor: This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_iot_kdr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr0`]
module"]
#[doc(alias = "HOST_IOT_KDR0")]
pub type HostIotKdr0 = crate::Reg<host_iot_kdr0::HostIotKdr0Spec>;
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
pub mod host_iot_kdr0;
#[doc = "HOST_IOT_KDR1 (w) register accessor: This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kdr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr1`]
module"]
#[doc(alias = "HOST_IOT_KDR1")]
pub type HostIotKdr1 = crate::Reg<host_iot_kdr1::HostIotKdr1Spec>;
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr1;
#[doc = "HOST_IOT_KDR2 (w) register accessor: This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kdr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr2`]
module"]
#[doc(alias = "HOST_IOT_KDR2")]
pub type HostIotKdr2 = crate::Reg<host_iot_kdr2::HostIotKdr2Spec>;
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr2;
#[doc = "HOST_IOT_KDR3 (w) register accessor: This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kdr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_kdr3`]
module"]
#[doc(alias = "HOST_IOT_KDR3")]
pub type HostIotKdr3 = crate::Reg<host_iot_kdr3::HostIotKdr3Spec>;
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr3;
#[doc = "HOST_IOT_LCS (rw) register accessor: Controls lifecycle state (LCS) for CRYPTOCELL subsystem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_iot_lcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_lcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_iot_lcs`]
module"]
#[doc(alias = "HOST_IOT_LCS")]
pub type HostIotLcs = crate::Reg<host_iot_lcs::HostIotLcsSpec>;
#[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
pub mod host_iot_lcs;
