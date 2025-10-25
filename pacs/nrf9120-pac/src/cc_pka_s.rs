#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memory_map: [MemoryMap; 32],
    opcode: Opcode,
    n_np_t0_t1_addr: NNpT0T1Addr,
    pka_status: PkaStatus,
    pka_sw_reset: PkaSwReset,
    pka_l: [PkaL; 8],
    pka_pipe: PkaPipe,
    pka_done: PkaDone,
    _reserved8: [u8; 0x0c],
    pka_version: PkaVersion,
    _reserved9: [u8; 0x0c],
    pka_sram_waddr: PkaSramWaddr,
    pka_sram_wdata: PkaSramWdata,
    pka_sram_rdata: PkaSramRdata,
    pka_sram_wclear: PkaSramWclear,
    pka_sram_raddr: PkaSramRaddr,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - Description collection: Register for mapping the virtual register R\\[n\\] to a physical address in the PKA SRAM."]
    #[inline(always)]
    pub const fn memory_map(&self, n: usize) -> &MemoryMap {
        &self.memory_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Description collection: Register for mapping the virtual register R\\[n\\] to a physical address in the PKA SRAM."]
    #[inline(always)]
    pub fn memory_map_iter(&self) -> impl Iterator<Item = &MemoryMap> {
        self.memory_map.iter()
    }
    #[doc = "0x80 - Operation code to be executed by the PKA engine. Writing to this register triggers the PKA operation."]
    #[inline(always)]
    pub const fn opcode(&self) -> &Opcode {
        &self.opcode
    }
    #[doc = "0x84 - This register defines the N, Np, T0, and T1 virtual register index."]
    #[inline(always)]
    pub const fn n_np_t0_t1_addr(&self) -> &NNpT0T1Addr {
        &self.n_np_t0_t1_addr
    }
    #[doc = "0x88 - This register holds the status for the PKA pipeline."]
    #[inline(always)]
    pub const fn pka_status(&self) -> &PkaStatus {
        &self.pka_status
    }
    #[doc = "0x8c - Reset the PKA engine."]
    #[inline(always)]
    pub const fn pka_sw_reset(&self) -> &PkaSwReset {
        &self.pka_sw_reset
    }
    #[doc = "0x90..0xb0 - Description collection: This register holds the operands bit size."]
    #[inline(always)]
    pub const fn pka_l(&self, n: usize) -> &PkaL {
        &self.pka_l[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0xb0 - Description collection: This register holds the operands bit size."]
    #[inline(always)]
    pub fn pka_l_iter(&self) -> impl Iterator<Item = &PkaL> {
        self.pka_l.iter()
    }
    #[doc = "0xb0 - Status register indicating if the PKA pipeline is ready to receive a new OPCODE."]
    #[inline(always)]
    pub const fn pka_pipe(&self) -> &PkaPipe {
        &self.pka_pipe
    }
    #[doc = "0xb4 - Status register indicating if the PKA operation has been completed."]
    #[inline(always)]
    pub const fn pka_done(&self) -> &PkaDone {
        &self.pka_done
    }
    #[doc = "0xc4 - PKA engine HW version. Reset value holds the version."]
    #[inline(always)]
    pub const fn pka_version(&self) -> &PkaVersion {
        &self.pka_version
    }
    #[doc = "0xd4 - Start address in PKA SRAM for subsequent write transactions."]
    #[inline(always)]
    pub const fn pka_sram_waddr(&self) -> &PkaSramWaddr {
        &self.pka_sram_waddr
    }
    #[doc = "0xd8 - Write data to PKA SRAM. Writing to this register triggers a DMA transaction writing data into PKA SRAM. The DMA address offset is automatically incremented during write."]
    #[inline(always)]
    pub const fn pka_sram_wdata(&self) -> &PkaSramWdata {
        &self.pka_sram_wdata
    }
    #[doc = "0xdc - Read data from PKA SRAM. Reading from this register triggers a DMA transaction read data from PKA SRAM. The DMA address offset is automatically incremented during read."]
    #[inline(always)]
    pub const fn pka_sram_rdata(&self) -> &PkaSramRdata {
        &self.pka_sram_rdata
    }
    #[doc = "0xe0 - Register for clearing PKA SRAM write buffer."]
    #[inline(always)]
    pub const fn pka_sram_wclear(&self) -> &PkaSramWclear {
        &self.pka_sram_wclear
    }
    #[doc = "0xe4 - Start address in PKA SRAM for subsequent read transactions."]
    #[inline(always)]
    pub const fn pka_sram_raddr(&self) -> &PkaSramRaddr {
        &self.pka_sram_raddr
    }
}
#[doc = "MEMORY_MAP (rw) register accessor: Description collection: Register for mapping the virtual register R\\[n\\] to a physical address in the PKA SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`memory_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memory_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memory_map`] module"]
#[doc(alias = "MEMORY_MAP")]
pub type MemoryMap = crate::Reg<memory_map::MemoryMapSpec>;
#[doc = "Description collection: Register for mapping the virtual register R\\[n\\] to a physical address in the PKA SRAM."]
pub mod memory_map;
#[doc = "OPCODE (rw) register accessor: Operation code to be executed by the PKA engine. Writing to this register triggers the PKA operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`opcode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcode`] module"]
#[doc(alias = "OPCODE")]
pub type Opcode = crate::Reg<opcode::OpcodeSpec>;
#[doc = "Operation code to be executed by the PKA engine. Writing to this register triggers the PKA operation."]
pub mod opcode;
#[doc = "N_NP_T0_T1_ADDR (rw) register accessor: This register defines the N, Np, T0, and T1 virtual register index.\n\nYou can [`read`](crate::Reg::read) this register and get [`n_np_t0_t1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_np_t0_t1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_np_t0_t1_addr`] module"]
#[doc(alias = "N_NP_T0_T1_ADDR")]
pub type NNpT0T1Addr = crate::Reg<n_np_t0_t1_addr::NNpT0T1AddrSpec>;
#[doc = "This register defines the N, Np, T0, and T1 virtual register index."]
pub mod n_np_t0_t1_addr;
#[doc = "PKA_STATUS (r) register accessor: This register holds the status for the PKA pipeline.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_status`] module"]
#[doc(alias = "PKA_STATUS")]
pub type PkaStatus = crate::Reg<pka_status::PkaStatusSpec>;
#[doc = "This register holds the status for the PKA pipeline."]
pub mod pka_status;
#[doc = "PKA_SW_RESET (w) register accessor: Reset the PKA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sw_reset`] module"]
#[doc(alias = "PKA_SW_RESET")]
pub type PkaSwReset = crate::Reg<pka_sw_reset::PkaSwResetSpec>;
#[doc = "Reset the PKA engine."]
pub mod pka_sw_reset;
#[doc = "PKA_L (rw) register accessor: Description collection: This register holds the operands bit size.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_l`] module"]
#[doc(alias = "PKA_L")]
pub type PkaL = crate::Reg<pka_l::PkaLSpec>;
#[doc = "Description collection: This register holds the operands bit size."]
pub mod pka_l;
#[doc = "PKA_PIPE (r) register accessor: Status register indicating if the PKA pipeline is ready to receive a new OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_pipe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_pipe`] module"]
#[doc(alias = "PKA_PIPE")]
pub type PkaPipe = crate::Reg<pka_pipe::PkaPipeSpec>;
#[doc = "Status register indicating if the PKA pipeline is ready to receive a new OPCODE."]
pub mod pka_pipe;
#[doc = "PKA_DONE (r) register accessor: Status register indicating if the PKA operation has been completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_done::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_done`] module"]
#[doc(alias = "PKA_DONE")]
pub type PkaDone = crate::Reg<pka_done::PkaDoneSpec>;
#[doc = "Status register indicating if the PKA operation has been completed."]
pub mod pka_done;
#[doc = "PKA_VERSION (r) register accessor: PKA engine HW version. Reset value holds the version.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_version`] module"]
#[doc(alias = "PKA_VERSION")]
pub type PkaVersion = crate::Reg<pka_version::PkaVersionSpec>;
#[doc = "PKA engine HW version. Reset value holds the version."]
pub mod pka_version;
#[doc = "PKA_SRAM_WADDR (w) register accessor: Start address in PKA SRAM for subsequent write transactions.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_waddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sram_waddr`] module"]
#[doc(alias = "PKA_SRAM_WADDR")]
pub type PkaSramWaddr = crate::Reg<pka_sram_waddr::PkaSramWaddrSpec>;
#[doc = "Start address in PKA SRAM for subsequent write transactions."]
pub mod pka_sram_waddr;
#[doc = "PKA_SRAM_WDATA (w) register accessor: Write data to PKA SRAM. Writing to this register triggers a DMA transaction writing data into PKA SRAM. The DMA address offset is automatically incremented during write.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sram_wdata`] module"]
#[doc(alias = "PKA_SRAM_WDATA")]
pub type PkaSramWdata = crate::Reg<pka_sram_wdata::PkaSramWdataSpec>;
#[doc = "Write data to PKA SRAM. Writing to this register triggers a DMA transaction writing data into PKA SRAM. The DMA address offset is automatically incremented during write."]
pub mod pka_sram_wdata;
#[doc = "PKA_SRAM_RDATA (r) register accessor: Read data from PKA SRAM. Reading from this register triggers a DMA transaction read data from PKA SRAM. The DMA address offset is automatically incremented during read.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_sram_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sram_rdata`] module"]
#[doc(alias = "PKA_SRAM_RDATA")]
pub type PkaSramRdata = crate::Reg<pka_sram_rdata::PkaSramRdataSpec>;
#[doc = "Read data from PKA SRAM. Reading from this register triggers a DMA transaction read data from PKA SRAM. The DMA address offset is automatically incremented during read."]
pub mod pka_sram_rdata;
#[doc = "PKA_SRAM_WCLEAR (w) register accessor: Register for clearing PKA SRAM write buffer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_wclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sram_wclear`] module"]
#[doc(alias = "PKA_SRAM_WCLEAR")]
pub type PkaSramWclear = crate::Reg<pka_sram_wclear::PkaSramWclearSpec>;
#[doc = "Register for clearing PKA SRAM write buffer."]
pub mod pka_sram_wclear;
#[doc = "PKA_SRAM_RADDR (w) register accessor: Start address in PKA SRAM for subsequent read transactions.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_sram_raddr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_sram_raddr`] module"]
#[doc(alias = "PKA_SRAM_RADDR")]
pub type PkaSramRaddr = crate::Reg<pka_sram_raddr::PkaSramRaddrSpec>;
#[doc = "Start address in PKA SRAM for subsequent read transactions."]
pub mod pka_sram_raddr;
