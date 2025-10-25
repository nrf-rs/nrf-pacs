#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0380],
    chacha_control: ChachaControl,
    chacha_version: ChachaVersion,
    chacha_key: [ChachaKey; 8],
    chacha_iv: [ChachaIv; 2],
    chacha_busy: ChachaBusy,
    chacha_hw_flags: ChachaHwFlags,
    chacha_block_cnt_lsb: ChachaBlockCntLsb,
    chacha_block_cnt_msb: ChachaBlockCntMsb,
    chacha_sw_reset: ChachaSwReset,
    chacha_poly1305_key: [ChachaPoly1305Key; 8],
    chacha_endianness: ChachaEndianness,
    chacha_debug: ChachaDebug,
}
impl RegisterBlock {
    #[doc = "0x380 - Control the CHACHA engine behavior."]
    #[inline(always)]
    pub const fn chacha_control(&self) -> &ChachaControl {
        &self.chacha_control
    }
    #[doc = "0x384 - CHACHA engine HW version"]
    #[inline(always)]
    pub const fn chacha_version(&self) -> &ChachaVersion {
        &self.chacha_version
    }
    #[doc = "0x388..0x3a8 - Description collection: CHACHA key value to use. The initial CHACHA_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub const fn chacha_key(&self, n: usize) -> &ChachaKey {
        &self.chacha_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x388..0x3a8 - Description collection: CHACHA key value to use. The initial CHACHA_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub fn chacha_key_iter(&self) -> impl Iterator<Item = &ChachaKey> {
        self.chacha_key.iter()
    }
    #[doc = "0x3a8..0x3b0 - Description collection: CHACHA Initialization Vector (IV) to use. The IV is also known as the nonce."]
    #[inline(always)]
    pub const fn chacha_iv(&self, n: usize) -> &ChachaIv {
        &self.chacha_iv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3a8..0x3b0 - Description collection: CHACHA Initialization Vector (IV) to use. The IV is also known as the nonce."]
    #[inline(always)]
    pub fn chacha_iv_iter(&self) -> impl Iterator<Item = &ChachaIv> {
        self.chacha_iv.iter()
    }
    #[doc = "0x3b0 - Status register for CHACHA engine activity."]
    #[inline(always)]
    pub const fn chacha_busy(&self) -> &ChachaBusy {
        &self.chacha_busy
    }
    #[doc = "0x3b4 - Hardware configuration of the CHACHA engine. Reset value holds the supported features."]
    #[inline(always)]
    pub const fn chacha_hw_flags(&self) -> &ChachaHwFlags {
        &self.chacha_hw_flags
    }
    #[doc = "0x3b8 - Store the LSB value of the block counter, in order to support suspend/resume of operation"]
    #[inline(always)]
    pub const fn chacha_block_cnt_lsb(&self) -> &ChachaBlockCntLsb {
        &self.chacha_block_cnt_lsb
    }
    #[doc = "0x3bc - Store the MSB value of the block counter, in order to support suspend/resume of operation"]
    #[inline(always)]
    pub const fn chacha_block_cnt_msb(&self) -> &ChachaBlockCntMsb {
        &self.chacha_block_cnt_msb
    }
    #[doc = "0x3c0 - Reset the CHACHA engine."]
    #[inline(always)]
    pub const fn chacha_sw_reset(&self) -> &ChachaSwReset {
        &self.chacha_sw_reset
    }
    #[doc = "0x3c4..0x3e4 - Description collection: The auto-generated key to use in Poly1305 MAC calculation. The initial CHACHA_POLY1305_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub const fn chacha_poly1305_key(&self, n: usize) -> &ChachaPoly1305Key {
        &self.chacha_poly1305_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c4..0x3e4 - Description collection: The auto-generated key to use in Poly1305 MAC calculation. The initial CHACHA_POLY1305_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
    #[inline(always)]
    pub fn chacha_poly1305_key_iter(&self) -> impl Iterator<Item = &ChachaPoly1305Key> {
        self.chacha_poly1305_key.iter()
    }
    #[doc = "0x3e4 - CHACHA engine data order configuration."]
    #[inline(always)]
    pub const fn chacha_endianness(&self) -> &ChachaEndianness {
        &self.chacha_endianness
    }
    #[doc = "0x3e8 - Debug register for the CHACHA engine"]
    #[inline(always)]
    pub const fn chacha_debug(&self) -> &ChachaDebug {
        &self.chacha_debug
    }
}
#[doc = "CHACHA_CONTROL (rw) register accessor: Control the CHACHA engine behavior.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_control`] module"]
#[doc(alias = "CHACHA_CONTROL")]
pub type ChachaControl = crate::Reg<chacha_control::ChachaControlSpec>;
#[doc = "Control the CHACHA engine behavior."]
pub mod chacha_control;
#[doc = "CHACHA_VERSION (r) register accessor: CHACHA engine HW version\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_version`] module"]
#[doc(alias = "CHACHA_VERSION")]
pub type ChachaVersion = crate::Reg<chacha_version::ChachaVersionSpec>;
#[doc = "CHACHA engine HW version"]
pub mod chacha_version;
#[doc = "CHACHA_KEY (w) register accessor: Description collection: CHACHA key value to use. The initial CHACHA_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_key`] module"]
#[doc(alias = "CHACHA_KEY")]
pub type ChachaKey = crate::Reg<chacha_key::ChachaKeySpec>;
#[doc = "Description collection: CHACHA key value to use. The initial CHACHA_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
pub mod chacha_key;
#[doc = "CHACHA_IV (rw) register accessor: Description collection: CHACHA Initialization Vector (IV) to use. The IV is also known as the nonce.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_iv`] module"]
#[doc(alias = "CHACHA_IV")]
pub type ChachaIv = crate::Reg<chacha_iv::ChachaIvSpec>;
#[doc = "Description collection: CHACHA Initialization Vector (IV) to use. The IV is also known as the nonce."]
pub mod chacha_iv;
#[doc = "CHACHA_BUSY (r) register accessor: Status register for CHACHA engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_busy`] module"]
#[doc(alias = "CHACHA_BUSY")]
pub type ChachaBusy = crate::Reg<chacha_busy::ChachaBusySpec>;
#[doc = "Status register for CHACHA engine activity."]
pub mod chacha_busy;
#[doc = "CHACHA_HW_FLAGS (r) register accessor: Hardware configuration of the CHACHA engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_hw_flags::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_hw_flags`] module"]
#[doc(alias = "CHACHA_HW_FLAGS")]
pub type ChachaHwFlags = crate::Reg<chacha_hw_flags::ChachaHwFlagsSpec>;
#[doc = "Hardware configuration of the CHACHA engine. Reset value holds the supported features."]
pub mod chacha_hw_flags;
#[doc = "CHACHA_BLOCK_CNT_LSB (rw) register accessor: Store the LSB value of the block counter, in order to support suspend/resume of operation\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_block_cnt_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_block_cnt_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_block_cnt_lsb`] module"]
#[doc(alias = "CHACHA_BLOCK_CNT_LSB")]
pub type ChachaBlockCntLsb = crate::Reg<chacha_block_cnt_lsb::ChachaBlockCntLsbSpec>;
#[doc = "Store the LSB value of the block counter, in order to support suspend/resume of operation"]
pub mod chacha_block_cnt_lsb;
#[doc = "CHACHA_BLOCK_CNT_MSB (rw) register accessor: Store the MSB value of the block counter, in order to support suspend/resume of operation\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_block_cnt_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_block_cnt_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_block_cnt_msb`] module"]
#[doc(alias = "CHACHA_BLOCK_CNT_MSB")]
pub type ChachaBlockCntMsb = crate::Reg<chacha_block_cnt_msb::ChachaBlockCntMsbSpec>;
#[doc = "Store the MSB value of the block counter, in order to support suspend/resume of operation"]
pub mod chacha_block_cnt_msb;
#[doc = "CHACHA_SW_RESET (w) register accessor: Reset the CHACHA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_sw_reset`] module"]
#[doc(alias = "CHACHA_SW_RESET")]
pub type ChachaSwReset = crate::Reg<chacha_sw_reset::ChachaSwResetSpec>;
#[doc = "Reset the CHACHA engine."]
pub mod chacha_sw_reset;
#[doc = "CHACHA_POLY1305_KEY (r) register accessor: Description collection: The auto-generated key to use in Poly1305 MAC calculation. The initial CHACHA_POLY1305_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_poly1305_key::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_poly1305_key`] module"]
#[doc(alias = "CHACHA_POLY1305_KEY")]
pub type ChachaPoly1305Key = crate::Reg<chacha_poly1305_key::ChachaPoly1305KeySpec>;
#[doc = "Description collection: The auto-generated key to use in Poly1305 MAC calculation. The initial CHACHA_POLY1305_KEY\\[0\\] register holds the least significant bits \\[31:0\\] of the key value."]
pub mod chacha_poly1305_key;
#[doc = "CHACHA_ENDIANNESS (rw) register accessor: CHACHA engine data order configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_endianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_endianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_endianness`] module"]
#[doc(alias = "CHACHA_ENDIANNESS")]
pub type ChachaEndianness = crate::Reg<chacha_endianness::ChachaEndiannessSpec>;
#[doc = "CHACHA engine data order configuration."]
pub mod chacha_endianness;
#[doc = "CHACHA_DEBUG (r) register accessor: Debug register for the CHACHA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_debug`] module"]
#[doc(alias = "CHACHA_DEBUG")]
pub type ChachaDebug = crate::Reg<chacha_debug::ChachaDebugSpec>;
#[doc = "Debug register for the CHACHA engine"]
pub mod chacha_debug;
