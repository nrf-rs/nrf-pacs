#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    aes_key_0: [AesKey0; 8],
    _reserved1: [u8; 0x20],
    aes_iv_0: [AesIv0; 4],
    _reserved2: [u8; 0x10],
    aes_ctr: [AesCtr; 4],
    aes_busy: AesBusy,
    _reserved4: [u8; 0x04],
    aes_sk: AesSk,
    aes_cmac_init: AesCmacInit,
    _reserved6: [u8; 0x3c],
    aes_remaining_bytes: AesRemainingBytes,
    aes_control: AesControl,
    _reserved8: [u8; 0x04],
    aes_hw_flags: AesHwFlags,
    _reserved9: [u8; 0x0c],
    aes_ctr_no_increment: AesCtrNoIncrement,
    _reserved10: [u8; 0x18],
    aes_sw_reset: AesSwReset,
    _reserved11: [u8; 0x2c],
    aes_cmac_size0_kick: AesCmacSize0Kick,
}
impl RegisterBlock {
    #[doc = "0x400..0x420 - Description collection: AES key value to use. The initial AES_KEY_0\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub const fn aes_key_0(&self, n: usize) -> &AesKey0 {
        &self.aes_key_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - Description collection: AES key value to use. The initial AES_KEY_0\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub fn aes_key_0_iter(&self) -> impl Iterator<Item = &AesKey0> {
        self.aes_key_0.iter()
    }
    #[doc = "0x440..0x450 - Description collection: AES Initialization Vector (IV) to use. The initial AES_IV_0\\[0\\] register holds the least significant bits \\[31:0\\] of the IV."]
    #[inline(always)]
    pub const fn aes_iv_0(&self, n: usize) -> &AesIv0 {
        &self.aes_iv_0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440..0x450 - Description collection: AES Initialization Vector (IV) to use. The initial AES_IV_0\\[0\\] register holds the least significant bits \\[31:0\\] of the IV."]
    #[inline(always)]
    pub fn aes_iv_0_iter(&self) -> impl Iterator<Item = &AesIv0> {
        self.aes_iv_0.iter()
    }
    #[doc = "0x460..0x470 - Description collection: AES counter (CTR) to use. The initial AES_CTR\\[0\\] register holds the least significant bits \\[31:0\\] of the CTR."]
    #[inline(always)]
    pub const fn aes_ctr(&self, n: usize) -> &AesCtr {
        &self.aes_ctr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x460..0x470 - Description collection: AES counter (CTR) to use. The initial AES_CTR\\[0\\] register holds the least significant bits \\[31:0\\] of the CTR."]
    #[inline(always)]
    pub fn aes_ctr_iter(&self) -> impl Iterator<Item = &AesCtr> {
        self.aes_ctr.iter()
    }
    #[doc = "0x470 - Status register for AES engine activity."]
    #[inline(always)]
    pub const fn aes_busy(&self) -> &AesBusy {
        &self.aes_busy
    }
    #[doc = "0x478 - Writing to this address trigger sampling of the HW key to the AES_KEY_0 register"]
    #[inline(always)]
    pub const fn aes_sk(&self) -> &AesSk {
        &self.aes_sk
    }
    #[doc = "0x47c - Writing to this address triggers the AES engine to generate K1 and K2 for AES-CMAC operations."]
    #[inline(always)]
    pub const fn aes_cmac_init(&self) -> &AesCmacInit {
        &self.aes_cmac_init
    }
    #[doc = "0x4bc - This register should be set with the amount of remaining bytes until the end of the current AES operation."]
    #[inline(always)]
    pub const fn aes_remaining_bytes(&self) -> &AesRemainingBytes {
        &self.aes_remaining_bytes
    }
    #[doc = "0x4c0 - Control the AES engine behavior."]
    #[inline(always)]
    pub const fn aes_control(&self) -> &AesControl {
        &self.aes_control
    }
    #[doc = "0x4c8 - Hardware configuration of the AES engine. Reset value holds the supported features."]
    #[inline(always)]
    pub const fn aes_hw_flags(&self) -> &AesHwFlags {
        &self.aes_hw_flags
    }
    #[doc = "0x4d8 - This register enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
    #[inline(always)]
    pub const fn aes_ctr_no_increment(&self) -> &AesCtrNoIncrement {
        &self.aes_ctr_no_increment
    }
    #[doc = "0x4f4 - Reset the AES engine."]
    #[inline(always)]
    pub const fn aes_sw_reset(&self) -> &AesSwReset {
        &self.aes_sw_reset
    }
    #[doc = "0x524 - Writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV_0 register."]
    #[inline(always)]
    pub const fn aes_cmac_size0_kick(&self) -> &AesCmacSize0Kick {
        &self.aes_cmac_size0_kick
    }
}
#[doc = "AES_KEY_0 (w) register accessor: Description collection: AES key value to use. The initial AES_KEY_0\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_key_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key_0`] module"]
#[doc(alias = "AES_KEY_0")]
pub type AesKey0 = crate::Reg<aes_key_0::AesKey0Spec>;
#[doc = "Description collection: AES key value to use. The initial AES_KEY_0\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
pub mod aes_key_0;
#[doc = "AES_IV_0 (rw) register accessor: Description collection: AES Initialization Vector (IV) to use. The initial AES_IV_0\\[0\\] register holds the least significant bits \\[31:0\\] of the IV.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_iv_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_iv_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_0`] module"]
#[doc(alias = "AES_IV_0")]
pub type AesIv0 = crate::Reg<aes_iv_0::AesIv0Spec>;
#[doc = "Description collection: AES Initialization Vector (IV) to use. The initial AES_IV_0\\[0\\] register holds the least significant bits \\[31:0\\] of the IV."]
pub mod aes_iv_0;
#[doc = "AES_CTR (rw) register accessor: Description collection: AES counter (CTR) to use. The initial AES_CTR\\[0\\] register holds the least significant bits \\[31:0\\] of the CTR.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctr`] module"]
#[doc(alias = "AES_CTR")]
pub type AesCtr = crate::Reg<aes_ctr::AesCtrSpec>;
#[doc = "Description collection: AES counter (CTR) to use. The initial AES_CTR\\[0\\] register holds the least significant bits \\[31:0\\] of the CTR."]
pub mod aes_ctr;
#[doc = "AES_BUSY (r) register accessor: Status register for AES engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_busy`] module"]
#[doc(alias = "AES_BUSY")]
pub type AesBusy = crate::Reg<aes_busy::AesBusySpec>;
#[doc = "Status register for AES engine activity."]
pub mod aes_busy;
#[doc = "AES_SK (w) register accessor: Writing to this address trigger sampling of the HW key to the AES_KEY_0 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_sk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sk`] module"]
#[doc(alias = "AES_SK")]
pub type AesSk = crate::Reg<aes_sk::AesSkSpec>;
#[doc = "Writing to this address trigger sampling of the HW key to the AES_KEY_0 register"]
pub mod aes_sk;
#[doc = "AES_CMAC_INIT (w) register accessor: Writing to this address triggers the AES engine to generate K1 and K2 for AES-CMAC operations.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cmac_init::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cmac_init`] module"]
#[doc(alias = "AES_CMAC_INIT")]
pub type AesCmacInit = crate::Reg<aes_cmac_init::AesCmacInitSpec>;
#[doc = "Writing to this address triggers the AES engine to generate K1 and K2 for AES-CMAC operations."]
pub mod aes_cmac_init;
#[doc = "AES_REMAINING_BYTES (rw) register accessor: This register should be set with the amount of remaining bytes until the end of the current AES operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_remaining_bytes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_remaining_bytes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_remaining_bytes`] module"]
#[doc(alias = "AES_REMAINING_BYTES")]
pub type AesRemainingBytes = crate::Reg<aes_remaining_bytes::AesRemainingBytesSpec>;
#[doc = "This register should be set with the amount of remaining bytes until the end of the current AES operation."]
pub mod aes_remaining_bytes;
#[doc = "AES_CONTROL (rw) register accessor: Control the AES engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_control`] module"]
#[doc(alias = "AES_CONTROL")]
pub type AesControl = crate::Reg<aes_control::AesControlSpec>;
#[doc = "Control the AES engine behavior."]
pub mod aes_control;
#[doc = "AES_HW_FLAGS (r) register accessor: Hardware configuration of the AES engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_hw_flags::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_hw_flags`] module"]
#[doc(alias = "AES_HW_FLAGS")]
pub type AesHwFlags = crate::Reg<aes_hw_flags::AesHwFlagsSpec>;
#[doc = "Hardware configuration of the AES engine. Reset value holds the supported features."]
pub mod aes_hw_flags;
#[doc = "AES_CTR_NO_INCREMENT (rw) register accessor: This register enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctr_no_increment::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctr_no_increment::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctr_no_increment`] module"]
#[doc(alias = "AES_CTR_NO_INCREMENT")]
pub type AesCtrNoIncrement = crate::Reg<aes_ctr_no_increment::AesCtrNoIncrementSpec>;
#[doc = "This register enables the AES CTR no increment mode in which the counter mode is not incremented between two blocks"]
pub mod aes_ctr_no_increment;
#[doc = "AES_SW_RESET (w) register accessor: Reset the AES engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_sw_reset`] module"]
#[doc(alias = "AES_SW_RESET")]
pub type AesSwReset = crate::Reg<aes_sw_reset::AesSwResetSpec>;
#[doc = "Reset the AES engine."]
pub mod aes_sw_reset;
#[doc = "AES_CMAC_SIZE0_KICK (w) register accessor: Writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV_0 register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cmac_size0_kick::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_cmac_size0_kick`] module"]
#[doc(alias = "AES_CMAC_SIZE0_KICK")]
pub type AesCmacSize0Kick = crate::Reg<aes_cmac_size0_kick::AesCmacSize0KickSpec>;
#[doc = "Writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV_0 register."]
pub mod aes_cmac_size0_kick;
