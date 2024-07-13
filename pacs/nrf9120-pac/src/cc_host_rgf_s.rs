#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1a38],
    #[doc = "0x1a38 - AES hardware key select"]
    pub host_cryptokey_sel: HOST_CRYPTOKEY_SEL,
    _reserved1: [u8; 0x10],
    #[doc = "0x1a4c - This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kprtl_lock: HOST_IOT_KPRTL_LOCK,
    #[doc = "0x1a50 - This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
    pub host_iot_kdr0: HOST_IOT_KDR0,
    #[doc = "0x1a54 - This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr1: HOST_IOT_KDR1,
    #[doc = "0x1a58 - This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr2: HOST_IOT_KDR2,
    #[doc = "0x1a5c - This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    pub host_iot_kdr3: HOST_IOT_KDR3,
    #[doc = "0x1a60 - Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
    pub host_iot_lcs: HOST_IOT_LCS,
}
#[doc = "HOST_CRYPTOKEY_SEL (rw) register accessor: an alias for `Reg<HOST_CRYPTOKEY_SEL_SPEC>`"]
pub type HOST_CRYPTOKEY_SEL = crate::Reg<host_cryptokey_sel::HOST_CRYPTOKEY_SEL_SPEC>;
#[doc = "AES hardware key select"]
pub mod host_cryptokey_sel;
#[doc = "HOST_IOT_KPRTL_LOCK (rw) register accessor: an alias for `Reg<HOST_IOT_KPRTL_LOCK_SPEC>`"]
pub type HOST_IOT_KPRTL_LOCK = crate::Reg<host_iot_kprtl_lock::HOST_IOT_KPRTL_LOCK_SPEC>;
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL cannot be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kprtl_lock;
#[doc = "HOST_IOT_KDR0 (rw) register accessor: an alias for `Reg<HOST_IOT_KDR0_SPEC>`"]
pub type HOST_IOT_KDR0 = crate::Reg<host_iot_kdr0::HOST_IOT_KDR0_SPEC>;
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
pub mod host_iot_kdr0;
#[doc = "HOST_IOT_KDR1 (w) register accessor: an alias for `Reg<HOST_IOT_KDR1_SPEC>`"]
pub type HOST_IOT_KDR1 = crate::Reg<host_iot_kdr1::HOST_IOT_KDR1_SPEC>;
#[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr1;
#[doc = "HOST_IOT_KDR2 (w) register accessor: an alias for `Reg<HOST_IOT_KDR2_SPEC>`"]
pub type HOST_IOT_KDR2 = crate::Reg<host_iot_kdr2::HOST_IOT_KDR2_SPEC>;
#[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr2;
#[doc = "HOST_IOT_KDR3 (w) register accessor: an alias for `Reg<HOST_IOT_KDR3_SPEC>`"]
pub type HOST_IOT_KDR3 = crate::Reg<host_iot_kdr3::HOST_IOT_KDR3_SPEC>;
#[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
pub mod host_iot_kdr3;
#[doc = "HOST_IOT_LCS (rw) register accessor: an alias for `Reg<HOST_IOT_LCS_SPEC>`"]
pub type HOST_IOT_LCS = crate::Reg<host_iot_lcs::HOST_IOT_LCS_SPEC>;
#[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
pub mod host_iot_lcs;
