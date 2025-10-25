#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0640],
    hash_h: [HashH; 8],
    _reserved1: [u8; 0x24],
    hash_pad_auto: HashPadAuto,
    _reserved2: [u8; 0x0c],
    hash_init_state: HashInitState,
    _reserved3: [u8; 0x0118],
    hash_version: HashVersion,
    _reserved4: [u8; 0x0c],
    hash_control: HashControl,
    hash_pad: HashPad,
    hash_pad_force: HashPadForce,
    hash_cur_len_0: HashCurLen0,
    hash_cur_len_1: HashCurLen1,
    _reserved9: [u8; 0x08],
    hash_hw_flags: HashHwFlags,
    _reserved10: [u8; 0x04],
    hash_sw_reset: HashSwReset,
    hash_endianness: HashEndianness,
}
impl RegisterBlock {
    #[doc = "0x640..0x660 - Description collection: HASH_H value registers. The initial HASH_H\\[0\\] register holds the least significant bits \\[31:0\\] of the value."]
    #[inline(always)]
    pub const fn hash_h(&self, n: usize) -> &HashH {
        &self.hash_h[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x640..0x660 - Description collection: HASH_H value registers. The initial HASH_H\\[0\\] register holds the least significant bits \\[31:0\\] of the value."]
    #[inline(always)]
    pub fn hash_h_iter(&self) -> impl Iterator<Item = &HashH> {
        self.hash_h.iter()
    }
    #[doc = "0x684 - Configure the HASH engine to automatically pad data at the end of the DMA transfer to complete the digest operation."]
    #[inline(always)]
    pub const fn hash_pad_auto(&self) -> &HashPadAuto {
        &self.hash_pad_auto
    }
    #[doc = "0x694 - Configure HASH engine initial state registers."]
    #[inline(always)]
    pub const fn hash_init_state(&self) -> &HashInitState {
        &self.hash_init_state
    }
    #[doc = "0x7b0 - HASH engine HW version"]
    #[inline(always)]
    pub const fn hash_version(&self) -> &HashVersion {
        &self.hash_version
    }
    #[doc = "0x7c0 - Control the HASH engine behavior."]
    #[inline(always)]
    pub const fn hash_control(&self) -> &HashControl {
        &self.hash_control
    }
    #[doc = "0x7c4 - Enable the hardware padding feature of the HASH engine."]
    #[inline(always)]
    pub const fn hash_pad(&self) -> &HashPad {
        &self.hash_pad
    }
    #[doc = "0x7c8 - Force the hardware padding operation to trigger if the input data length is zero bytes."]
    #[inline(always)]
    pub const fn hash_pad_force(&self) -> &HashPadForce {
        &self.hash_pad_force
    }
    #[doc = "0x7cc - Bits \\[31:0\\] of the number of bytes that have been digested so far."]
    #[inline(always)]
    pub const fn hash_cur_len_0(&self) -> &HashCurLen0 {
        &self.hash_cur_len_0
    }
    #[doc = "0x7d0 - Bits \\[63:32\\] of the number of bytes that have been digested so far."]
    #[inline(always)]
    pub const fn hash_cur_len_1(&self) -> &HashCurLen1 {
        &self.hash_cur_len_1
    }
    #[doc = "0x7dc - Hardware configuration of the HASH engine. Reset value holds the supported features."]
    #[inline(always)]
    pub const fn hash_hw_flags(&self) -> &HashHwFlags {
        &self.hash_hw_flags
    }
    #[doc = "0x7e4 - Reset the HASH engine."]
    #[inline(always)]
    pub const fn hash_sw_reset(&self) -> &HashSwReset {
        &self.hash_sw_reset
    }
    #[doc = "0x7e8 - Configure the endianness of HASH data and padding generation."]
    #[inline(always)]
    pub const fn hash_endianness(&self) -> &HashEndianness {
        &self.hash_endianness
    }
}
#[doc = "HASH_H (rw) register accessor: Description collection: HASH_H value registers. The initial HASH_H\\[0\\] register holds the least significant bits \\[31:0\\] of the value.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_h`] module"]
#[doc(alias = "HASH_H")]
pub type HashH = crate::Reg<hash_h::HashHSpec>;
#[doc = "Description collection: HASH_H value registers. The initial HASH_H\\[0\\] register holds the least significant bits \\[31:0\\] of the value."]
pub mod hash_h;
#[doc = "HASH_PAD_AUTO (w) register accessor: Configure the HASH engine to automatically pad data at the end of the DMA transfer to complete the digest operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad_auto::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_pad_auto`] module"]
#[doc(alias = "HASH_PAD_AUTO")]
pub type HashPadAuto = crate::Reg<hash_pad_auto::HashPadAutoSpec>;
#[doc = "Configure the HASH engine to automatically pad data at the end of the DMA transfer to complete the digest operation."]
pub mod hash_pad_auto;
#[doc = "HASH_INIT_STATE (w) register accessor: Configure HASH engine initial state registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_init_state::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_init_state`] module"]
#[doc(alias = "HASH_INIT_STATE")]
pub type HashInitState = crate::Reg<hash_init_state::HashInitStateSpec>;
#[doc = "Configure HASH engine initial state registers."]
pub mod hash_init_state;
#[doc = "HASH_VERSION (r) register accessor: HASH engine HW version\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_version`] module"]
#[doc(alias = "HASH_VERSION")]
pub type HashVersion = crate::Reg<hash_version::HashVersionSpec>;
#[doc = "HASH engine HW version"]
pub mod hash_version;
#[doc = "HASH_CONTROL (rw) register accessor: Control the HASH engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_control`] module"]
#[doc(alias = "HASH_CONTROL")]
pub type HashControl = crate::Reg<hash_control::HashControlSpec>;
#[doc = "Control the HASH engine behavior."]
pub mod hash_control;
#[doc = "HASH_PAD (rw) register accessor: Enable the hardware padding feature of the HASH engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_pad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_pad`] module"]
#[doc(alias = "HASH_PAD")]
pub type HashPad = crate::Reg<hash_pad::HashPadSpec>;
#[doc = "Enable the hardware padding feature of the HASH engine."]
pub mod hash_pad;
#[doc = "HASH_PAD_FORCE (rw) register accessor: Force the hardware padding operation to trigger if the input data length is zero bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_pad_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_pad_force`] module"]
#[doc(alias = "HASH_PAD_FORCE")]
pub type HashPadForce = crate::Reg<hash_pad_force::HashPadForceSpec>;
#[doc = "Force the hardware padding operation to trigger if the input data length is zero bytes."]
pub mod hash_pad_force;
#[doc = "HASH_CUR_LEN_0 (rw) register accessor: Bits \\[31:0\\] of the number of bytes that have been digested so far.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_cur_len_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_cur_len_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_cur_len_0`] module"]
#[doc(alias = "HASH_CUR_LEN_0")]
pub type HashCurLen0 = crate::Reg<hash_cur_len_0::HashCurLen0Spec>;
#[doc = "Bits \\[31:0\\] of the number of bytes that have been digested so far."]
pub mod hash_cur_len_0;
#[doc = "HASH_CUR_LEN_1 (rw) register accessor: Bits \\[63:32\\] of the number of bytes that have been digested so far.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_cur_len_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_cur_len_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_cur_len_1`] module"]
#[doc(alias = "HASH_CUR_LEN_1")]
pub type HashCurLen1 = crate::Reg<hash_cur_len_1::HashCurLen1Spec>;
#[doc = "Bits \\[63:32\\] of the number of bytes that have been digested so far."]
pub mod hash_cur_len_1;
#[doc = "HASH_HW_FLAGS (r) register accessor: Hardware configuration of the HASH engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hw_flags::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hw_flags`] module"]
#[doc(alias = "HASH_HW_FLAGS")]
pub type HashHwFlags = crate::Reg<hash_hw_flags::HashHwFlagsSpec>;
#[doc = "Hardware configuration of the HASH engine. Reset value holds the supported features."]
pub mod hash_hw_flags;
#[doc = "HASH_SW_RESET (w) register accessor: Reset the HASH engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_sw_reset`] module"]
#[doc(alias = "HASH_SW_RESET")]
pub type HashSwReset = crate::Reg<hash_sw_reset::HashSwResetSpec>;
#[doc = "Reset the HASH engine."]
pub mod hash_sw_reset;
#[doc = "HASH_ENDIANNESS (rw) register accessor: Configure the endianness of HASH data and padding generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_endianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_endianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_endianness`] module"]
#[doc(alias = "HASH_ENDIANNESS")]
pub type HashEndianness = crate::Reg<hash_endianness::HashEndiannessSpec>;
#[doc = "Configure the endianness of HASH data and padding generation."]
pub mod hash_endianness;
