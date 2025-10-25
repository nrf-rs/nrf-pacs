#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    trcprgctlr: Trcprgctlr,
    trcprocselr: Trcprocselr,
    trcstatr: Trcstatr,
    trcconfigr: Trcconfigr,
    _reserved4: [u8; 0x0c],
    trceventctl0r: Trceventctl0r,
    trceventctl1r: Trceventctl1r,
    _reserved6: [u8; 0x04],
    trcstallctlr: Trcstallctlr,
    trctsctlr: Trctsctlr,
    trcsyncpr: Trcsyncpr,
    trcccctlr: Trcccctlr,
    trcbbctlr: Trcbbctlr,
    trctraceidr: Trctraceidr,
    trcqctlr: Trcqctlr,
    _reserved13: [u8; 0x38],
    trcvictlr: Trcvictlr,
    trcviiectlr: Trcviiectlr,
    trcvissctlr: Trcvissctlr,
    trcvipcssctlr: Trcvipcssctlr,
    _reserved17: [u8; 0x10],
    trcvdctlr: Trcvdctlr,
    trcvdsacctlr: Trcvdsacctlr,
    trcvdarcctlr: Trcvdarcctlr,
    _reserved20: [u8; 0x54],
    trcseqevr: [Trcseqevr; 3],
    _reserved21: [u8; 0x0c],
    trcseqrstevr: Trcseqrstevr,
    trcseqstr: Trcseqstr,
    trcextinselr: Trcextinselr,
    _reserved24: [u8; 0x1c],
    trccntrldvr: [Trccntrldvr; 4],
    trccntctlr: [Trccntctlr; 4],
    trccntvr: [Trccntvr; 4],
    _reserved27: [u8; 0x90],
    trcrsctlr: [Trcrsctlr; 30],
    _reserved28: [u8; 0x08],
    trcssccr0: Trcssccr0,
    _reserved29: [u8; 0x1c],
    trcsscsr0: Trcsscsr0,
    _reserved30: [u8; 0x1c],
    trcsspcicr0: Trcsspcicr0,
    _reserved31: [u8; 0x4c],
    trcpdcr: Trcpdcr,
    trcpdsr: Trcpdsr,
    _reserved33: [u8; 0x0bcc],
    trcitatbidr: Trcitatbidr,
    _reserved34: [u8; 0x0c],
    trcitiatbinr: Trcitiatbinr,
    _reserved35: [u8; 0x04],
    trcitiatboutr: Trcitiatboutr,
    trcitctrl: Trcitctrl,
    _reserved37: [u8; 0x9c],
    trcclaimset: Trcclaimset,
    trcclaimclr: Trcclaimclr,
    _reserved39: [u8; 0x10],
    trcauthstatus: Trcauthstatus,
    trcdevarch: Trcdevarch,
    _reserved41: [u8; 0x0c],
    trcdevtype: Trcdevtype,
    trcpidr: [Trcpidr; 8],
    trccidr: [Trccidr; 4],
}
impl RegisterBlock {
    #[doc = "0x04 - Enables the trace unit."]
    #[inline(always)]
    pub const fn trcprgctlr(&self) -> &Trcprgctlr {
        &self.trcprgctlr
    }
    #[doc = "0x08 - Controls which PE to trace. Might ignore writes when the trace unit is enabled or not idle. Before writing to this register, ensure that TRCSTATR.IDLE == 1 so that the trace unit can synchronize with the chosen PE. Implemented if TRCIDR3.NUMPROC is greater than zero."]
    #[inline(always)]
    pub const fn trcprocselr(&self) -> &Trcprocselr {
        &self.trcprocselr
    }
    #[doc = "0x0c - Idle status bit"]
    #[inline(always)]
    pub const fn trcstatr(&self) -> &Trcstatr {
        &self.trcstatr
    }
    #[doc = "0x10 - Controls the tracing options This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trcconfigr(&self) -> &Trcconfigr {
        &self.trcconfigr
    }
    #[doc = "0x20 - Controls the tracing of arbitrary events. If the selected event occurs a trace element is generated in the trace stream according to the settings in TRCEVENTCTL1R.DATAEN and TRCEVENTCTL1R.INSTEN."]
    #[inline(always)]
    pub const fn trceventctl0r(&self) -> &Trceventctl0r {
        &self.trceventctl0r
    }
    #[doc = "0x24 - Controls the behavior of the events that TRCEVENTCTL0R selects. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trceventctl1r(&self) -> &Trceventctl1r {
        &self.trceventctl1r
    }
    #[doc = "0x2c - Enables trace unit functionality that prevents trace unit buffer overflows. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCIDR3.STALLCTL == 1."]
    #[inline(always)]
    pub const fn trcstallctlr(&self) -> &Trcstallctlr {
        &self.trcstallctlr
    }
    #[doc = "0x30 - Controls the insertion of global timestamps in the trace streams. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.TS == 1."]
    #[inline(always)]
    pub const fn trctsctlr(&self) -> &Trctsctlr {
        &self.trctsctlr
    }
    #[doc = "0x34 - Controls how often trace synchronization requests occur. Might ignore writes when the trace unit is enabled or not idle. If writes are permitted then the register must be programmed."]
    #[inline(always)]
    pub const fn trcsyncpr(&self) -> &Trcsyncpr {
        &self.trcsyncpr
    }
    #[doc = "0x38 - Sets the threshold value for cycle counting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.CCI==1."]
    #[inline(always)]
    pub const fn trcccctlr(&self) -> &Trcccctlr {
        &self.trcccctlr
    }
    #[doc = "0x3c - Controls which regions in the memory map are enabled to use branch broadcasting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.BB == 1."]
    #[inline(always)]
    pub const fn trcbbctlr(&self) -> &Trcbbctlr {
        &self.trcbbctlr
    }
    #[doc = "0x40 - Sets the trace ID for instruction trace. If data trace is enabled then it also sets the trace ID for data trace, to (trace ID for instruction trace) + 1. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trctraceidr(&self) -> &Trctraceidr {
        &self.trctraceidr
    }
    #[doc = "0x44 - Controls when Q elements are enabled. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed if it is implemented and TRCCONFIGR.QE is set to any value other than 0b00."]
    #[inline(always)]
    pub const fn trcqctlr(&self) -> &Trcqctlr {
        &self.trcqctlr
    }
    #[doc = "0x80 - Controls instruction trace filtering. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. Must be programmed, particularly to set the value of the SSSTATUS bit, which sets the state of the start/stop logic."]
    #[inline(always)]
    pub const fn trcvictlr(&self) -> &Trcvictlr {
        &self.trcvictlr
    }
    #[doc = "0x84 - ViewInst exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
    #[inline(always)]
    pub const fn trcviiectlr(&self) -> &Trcviiectlr {
        &self.trcviiectlr
    }
    #[doc = "0x88 - Use this to set, or read, the single address comparators that control the ViewInst start/stop logic. The start/stop logic is active for an instruction which causes a start and remains active up to and including an instruction which causes a stop, and then the start/stop logic becomes inactive. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed."]
    #[inline(always)]
    pub const fn trcvissctlr(&self) -> &Trcvissctlr {
        &self.trcvissctlr
    }
    #[doc = "0x8c - Use this to set, or read, which PE comparator inputs can control the ViewInst start/stop logic. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed."]
    #[inline(always)]
    pub const fn trcvipcssctlr(&self) -> &Trcvipcssctlr {
        &self.trcvipcssctlr
    }
    #[doc = "0xa0 - Controls data trace filtering. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when data tracing is enabled, that is, when either TRCCONFIGR.DA == 1 or TRCCONFIGR.DV == 1."]
    #[inline(always)]
    pub const fn trcvdctlr(&self) -> &Trcvdctlr {
        &self.trcvdctlr
    }
    #[doc = "0xa4 - ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
    #[inline(always)]
    pub const fn trcvdsacctlr(&self) -> &Trcvdsacctlr {
        &self.trcvdsacctlr
    }
    #[doc = "0xa8 - ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
    #[inline(always)]
    pub const fn trcvdarcctlr(&self) -> &Trcvdarcctlr {
        &self.trcvdarcctlr
    }
    #[doc = "0x100..0x10c - Description collection: Moves the sequencer state according to programmed events. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
    #[inline(always)]
    pub const fn trcseqevr(&self, n: usize) -> &Trcseqevr {
        &self.trcseqevr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x10c - Description collection: Moves the sequencer state according to programmed events. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
    #[inline(always)]
    pub fn trcseqevr_iter(&self) -> impl Iterator<Item = &Trcseqevr> {
        self.trcseqevr.iter()
    }
    #[doc = "0x118 - Moves the sequencer to state 0 when a programmed event occurs. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
    #[inline(always)]
    pub const fn trcseqrstevr(&self) -> &Trcseqrstevr {
        &self.trcseqrstevr
    }
    #[doc = "0x11c - Use this to set, or read, the sequencer state. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
    #[inline(always)]
    pub const fn trcseqstr(&self) -> &Trcseqstr {
        &self.trcseqstr
    }
    #[doc = "0x120 - Use this to set, or read, which external inputs are resources to the trace unit. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
    #[inline(always)]
    pub const fn trcextinselr(&self) -> &Trcextinselr {
        &self.trcextinselr
    }
    #[doc = "0x140..0x150 - Description collection: This sets or returns the reload count value for counter n. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trccntrldvr(&self, n: usize) -> &Trccntrldvr {
        &self.trccntrldvr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - Description collection: This sets or returns the reload count value for counter n. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub fn trccntrldvr_iter(&self) -> impl Iterator<Item = &Trccntrldvr> {
        self.trccntrldvr.iter()
    }
    #[doc = "0x150..0x160 - Description collection: Controls the operation of counter n. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trccntctlr(&self, n: usize) -> &Trccntctlr {
        &self.trccntctlr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - Description collection: Controls the operation of counter n. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub fn trccntctlr_iter(&self) -> impl Iterator<Item = &Trccntctlr> {
        self.trccntctlr.iter()
    }
    #[doc = "0x160..0x170 - Description collection: This sets or returns the value of counter n. The count value is only stable when TRCSTATR.PMSTABLE == 1. If software uses counter n then it must write to this register to set the initial counter value. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub const fn trccntvr(&self, n: usize) -> &Trccntvr {
        &self.trccntvr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - Description collection: This sets or returns the value of counter n. The count value is only stable when TRCSTATR.PMSTABLE == 1. If software uses counter n then it must write to this register to set the initial counter value. Might ignore writes when the trace unit is enabled or not idle."]
    #[inline(always)]
    pub fn trccntvr_iter(&self) -> impl Iterator<Item = &Trccntvr> {
        self.trccntvr.iter()
    }
    #[doc = "0x200..0x278 - Description collection: Controls the selection of the resources in the trace unit. Might ignore writes when the trace unit is enabled or not idle. If software selects a non-implemented resource then CONSTRAINED UNPREDICTABLE behavior of the resource selector occurs, so the resource selector might fire unexpectedly or might not fire. Reads of the TRCRSCTLRn might return UNKNOWN."]
    #[inline(always)]
    pub const fn trcrsctlr(&self, n: usize) -> &Trcrsctlr {
        &self.trcrsctlr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x278 - Description collection: Controls the selection of the resources in the trace unit. Might ignore writes when the trace unit is enabled or not idle. If software selects a non-implemented resource then CONSTRAINED UNPREDICTABLE behavior of the resource selector occurs, so the resource selector might fire unexpectedly or might not fire. Reads of the TRCRSCTLRn might return UNKNOWN."]
    #[inline(always)]
    pub fn trcrsctlr_iter(&self) -> impl Iterator<Item = &Trcrsctlr> {
        self.trcrsctlr.iter()
    }
    #[doc = "0x280 - Controls the single-shot comparator."]
    #[inline(always)]
    pub const fn trcssccr0(&self) -> &Trcssccr0 {
        &self.trcssccr0
    }
    #[doc = "0x2a0 - Indicates the status of the single-shot comparators. TRCSSCSR0 is sensitive toinstruction addresses."]
    #[inline(always)]
    pub const fn trcsscsr0(&self) -> &Trcsscsr0 {
        &self.trcsscsr0
    }
    #[doc = "0x2c0 - Selects the processor comparator inputs for Single-shot control."]
    #[inline(always)]
    pub const fn trcsspcicr0(&self) -> &Trcsspcicr0 {
        &self.trcsspcicr0
    }
    #[doc = "0x310 - Controls the single-shot comparator."]
    #[inline(always)]
    pub const fn trcpdcr(&self) -> &Trcpdcr {
        &self.trcpdcr
    }
    #[doc = "0x314 - Indicates the power down status of the ETM."]
    #[inline(always)]
    pub const fn trcpdsr(&self) -> &Trcpdsr {
        &self.trcpdsr
    }
    #[doc = "0xee4 - Sets the state of output pins."]
    #[inline(always)]
    pub const fn trcitatbidr(&self) -> &Trcitatbidr {
        &self.trcitatbidr
    }
    #[doc = "0xef4 - Reads the state of the input pins."]
    #[inline(always)]
    pub const fn trcitiatbinr(&self) -> &Trcitiatbinr {
        &self.trcitiatbinr
    }
    #[doc = "0xefc - Sets the state of the output pins."]
    #[inline(always)]
    pub const fn trcitiatboutr(&self) -> &Trcitiatboutr {
        &self.trcitiatboutr
    }
    #[doc = "0xf00 - Enables topology detection or integration testing, by putting ETM-M33 into integration mode."]
    #[inline(always)]
    pub const fn trcitctrl(&self) -> &Trcitctrl {
        &self.trcitctrl
    }
    #[doc = "0xfa0 - Sets bits in the claim tag and determines the number of claim tag bits implemented."]
    #[inline(always)]
    pub const fn trcclaimset(&self) -> &Trcclaimset {
        &self.trcclaimset
    }
    #[doc = "0xfa4 - Clears bits in the claim tag and determines the current value of the claim tag."]
    #[inline(always)]
    pub const fn trcclaimclr(&self) -> &Trcclaimclr {
        &self.trcclaimclr
    }
    #[doc = "0xfb8 - Indicates the current level of tracing permitted by the system"]
    #[inline(always)]
    pub const fn trcauthstatus(&self) -> &Trcauthstatus {
        &self.trcauthstatus
    }
    #[doc = "0xfbc - The TRCDEVARCH identifies ETM-M33 as an ETMv4.2 component"]
    #[inline(always)]
    pub const fn trcdevarch(&self) -> &Trcdevarch {
        &self.trcdevarch
    }
    #[doc = "0xfcc - Controls the single-shot comparator."]
    #[inline(always)]
    pub const fn trcdevtype(&self) -> &Trcdevtype {
        &self.trcdevtype
    }
    #[doc = "0xfd0..0xff0 - Description collection: Coresight peripheral identification registers."]
    #[inline(always)]
    pub const fn trcpidr(&self, n: usize) -> &Trcpidr {
        &self.trcpidr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfd0..0xff0 - Description collection: Coresight peripheral identification registers."]
    #[inline(always)]
    pub fn trcpidr_iter(&self) -> impl Iterator<Item = &Trcpidr> {
        self.trcpidr.iter()
    }
    #[doc = "0xff0..0x1000 - Description collection: Coresight component identification registers."]
    #[inline(always)]
    pub const fn trccidr(&self, n: usize) -> &Trccidr {
        &self.trccidr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xff0..0x1000 - Description collection: Coresight component identification registers."]
    #[inline(always)]
    pub fn trccidr_iter(&self) -> impl Iterator<Item = &Trccidr> {
        self.trccidr.iter()
    }
}
#[doc = "TRCPRGCTLR (rw) register accessor: Enables the trace unit.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcprgctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprgctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcprgctlr`] module"]
#[doc(alias = "TRCPRGCTLR")]
pub type Trcprgctlr = crate::Reg<trcprgctlr::TrcprgctlrSpec>;
#[doc = "Enables the trace unit."]
pub mod trcprgctlr;
#[doc = "TRCPROCSELR (rw) register accessor: Controls which PE to trace. Might ignore writes when the trace unit is enabled or not idle. Before writing to this register, ensure that TRCSTATR.IDLE == 1 so that the trace unit can synchronize with the chosen PE. Implemented if TRCIDR3.NUMPROC is greater than zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcprocselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprocselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcprocselr`] module"]
#[doc(alias = "TRCPROCSELR")]
pub type Trcprocselr = crate::Reg<trcprocselr::TrcprocselrSpec>;
#[doc = "Controls which PE to trace. Might ignore writes when the trace unit is enabled or not idle. Before writing to this register, ensure that TRCSTATR.IDLE == 1 so that the trace unit can synchronize with the chosen PE. Implemented if TRCIDR3.NUMPROC is greater than zero."]
pub mod trcprocselr;
#[doc = "TRCSTATR (rw) register accessor: Idle status bit\n\nYou can [`read`](crate::Reg::read) this register and get [`trcstatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcstatr`] module"]
#[doc(alias = "TRCSTATR")]
pub type Trcstatr = crate::Reg<trcstatr::TrcstatrSpec>;
#[doc = "Idle status bit"]
pub mod trcstatr;
#[doc = "TRCCONFIGR (rw) register accessor: Controls the tracing options This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcconfigr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcconfigr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcconfigr`] module"]
#[doc(alias = "TRCCONFIGR")]
pub type Trcconfigr = crate::Reg<trcconfigr::TrcconfigrSpec>;
#[doc = "Controls the tracing options This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trcconfigr;
#[doc = "TRCEVENTCTL0R (rw) register accessor: Controls the tracing of arbitrary events. If the selected event occurs a trace element is generated in the trace stream according to the settings in TRCEVENTCTL1R.DATAEN and TRCEVENTCTL1R.INSTEN.\n\nYou can [`read`](crate::Reg::read) this register and get [`trceventctl0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trceventctl0r`] module"]
#[doc(alias = "TRCEVENTCTL0R")]
pub type Trceventctl0r = crate::Reg<trceventctl0r::Trceventctl0rSpec>;
#[doc = "Controls the tracing of arbitrary events. If the selected event occurs a trace element is generated in the trace stream according to the settings in TRCEVENTCTL1R.DATAEN and TRCEVENTCTL1R.INSTEN."]
pub mod trceventctl0r;
#[doc = "TRCEVENTCTL1R (rw) register accessor: Controls the behavior of the events that TRCEVENTCTL0R selects. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trceventctl1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trceventctl1r`] module"]
#[doc(alias = "TRCEVENTCTL1R")]
pub type Trceventctl1r = crate::Reg<trceventctl1r::Trceventctl1rSpec>;
#[doc = "Controls the behavior of the events that TRCEVENTCTL0R selects. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trceventctl1r;
#[doc = "TRCSTALLCTLR (rw) register accessor: Enables trace unit functionality that prevents trace unit buffer overflows. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCIDR3.STALLCTL == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcstallctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstallctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcstallctlr`] module"]
#[doc(alias = "TRCSTALLCTLR")]
pub type Trcstallctlr = crate::Reg<trcstallctlr::TrcstallctlrSpec>;
#[doc = "Enables trace unit functionality that prevents trace unit buffer overflows. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCIDR3.STALLCTL == 1."]
pub mod trcstallctlr;
#[doc = "TRCTSCTLR (rw) register accessor: Controls the insertion of global timestamps in the trace streams. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.TS == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trctsctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctsctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trctsctlr`] module"]
#[doc(alias = "TRCTSCTLR")]
pub type Trctsctlr = crate::Reg<trctsctlr::TrctsctlrSpec>;
#[doc = "Controls the insertion of global timestamps in the trace streams. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.TS == 1."]
pub mod trctsctlr;
#[doc = "TRCSYNCPR (rw) register accessor: Controls how often trace synchronization requests occur. Might ignore writes when the trace unit is enabled or not idle. If writes are permitted then the register must be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsyncpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsyncpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcsyncpr`] module"]
#[doc(alias = "TRCSYNCPR")]
pub type Trcsyncpr = crate::Reg<trcsyncpr::TrcsyncprSpec>;
#[doc = "Controls how often trace synchronization requests occur. Might ignore writes when the trace unit is enabled or not idle. If writes are permitted then the register must be programmed."]
pub mod trcsyncpr;
#[doc = "TRCCCCTLR (rw) register accessor: Sets the threshold value for cycle counting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.CCI==1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcccctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcccctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcccctlr`] module"]
#[doc(alias = "TRCCCCTLR")]
pub type Trcccctlr = crate::Reg<trcccctlr::TrcccctlrSpec>;
#[doc = "Sets the threshold value for cycle counting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.CCI==1."]
pub mod trcccctlr;
#[doc = "TRCBBCTLR (rw) register accessor: Controls which regions in the memory map are enabled to use branch broadcasting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.BB == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcbbctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcbbctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcbbctlr`] module"]
#[doc(alias = "TRCBBCTLR")]
pub type Trcbbctlr = crate::Reg<trcbbctlr::TrcbbctlrSpec>;
#[doc = "Controls which regions in the memory map are enabled to use branch broadcasting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.BB == 1."]
pub mod trcbbctlr;
#[doc = "TRCTRACEIDR (rw) register accessor: Sets the trace ID for instruction trace. If data trace is enabled then it also sets the trace ID for data trace, to (trace ID for instruction trace) + 1. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trctraceidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trctraceidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trctraceidr`] module"]
#[doc(alias = "TRCTRACEIDR")]
pub type Trctraceidr = crate::Reg<trctraceidr::TrctraceidrSpec>;
#[doc = "Sets the trace ID for instruction trace. If data trace is enabled then it also sets the trace ID for data trace, to (trace ID for instruction trace) + 1. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trctraceidr;
#[doc = "TRCQCTLR (rw) register accessor: Controls when Q elements are enabled. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed if it is implemented and TRCCONFIGR.QE is set to any value other than 0b00.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcqctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcqctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcqctlr`] module"]
#[doc(alias = "TRCQCTLR")]
pub type Trcqctlr = crate::Reg<trcqctlr::TrcqctlrSpec>;
#[doc = "Controls when Q elements are enabled. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed if it is implemented and TRCCONFIGR.QE is set to any value other than 0b00."]
pub mod trcqctlr;
#[doc = "TRCVICTLR (rw) register accessor: Controls instruction trace filtering. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. Must be programmed, particularly to set the value of the SSSTATUS bit, which sets the state of the start/stop logic.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvictlr`] module"]
#[doc(alias = "TRCVICTLR")]
pub type Trcvictlr = crate::Reg<trcvictlr::TrcvictlrSpec>;
#[doc = "Controls instruction trace filtering. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. Must be programmed, particularly to set the value of the SSSTATUS bit, which sets the state of the start/stop logic."]
pub mod trcvictlr;
#[doc = "TRCVIIECTLR (rw) register accessor: ViewInst exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcviiectlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcviiectlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcviiectlr`] module"]
#[doc(alias = "TRCVIIECTLR")]
pub type Trcviiectlr = crate::Reg<trcviiectlr::TrcviiectlrSpec>;
#[doc = "ViewInst exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
pub mod trcviiectlr;
#[doc = "TRCVISSCTLR (rw) register accessor: Use this to set, or read, the single address comparators that control the ViewInst start/stop logic. The start/stop logic is active for an instruction which causes a start and remains active up to and including an instruction which causes a stop, and then the start/stop logic becomes inactive. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvissctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvissctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvissctlr`] module"]
#[doc(alias = "TRCVISSCTLR")]
pub type Trcvissctlr = crate::Reg<trcvissctlr::TrcvissctlrSpec>;
#[doc = "Use this to set, or read, the single address comparators that control the ViewInst start/stop logic. The start/stop logic is active for an instruction which causes a start and remains active up to and including an instruction which causes a stop, and then the start/stop logic becomes inactive. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed."]
pub mod trcvissctlr;
#[doc = "TRCVIPCSSCTLR (rw) register accessor: Use this to set, or read, which PE comparator inputs can control the ViewInst start/stop logic. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvipcssctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvipcssctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvipcssctlr`] module"]
#[doc(alias = "TRCVIPCSSCTLR")]
pub type Trcvipcssctlr = crate::Reg<trcvipcssctlr::TrcvipcssctlrSpec>;
#[doc = "Use this to set, or read, which PE comparator inputs can control the ViewInst start/stop logic. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed."]
pub mod trcvipcssctlr;
#[doc = "TRCVDCTLR (rw) register accessor: Controls data trace filtering. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when data tracing is enabled, that is, when either TRCCONFIGR.DA == 1 or TRCCONFIGR.DV == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvdctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvdctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvdctlr`] module"]
#[doc(alias = "TRCVDCTLR")]
pub type Trcvdctlr = crate::Reg<trcvdctlr::TrcvdctlrSpec>;
#[doc = "Controls data trace filtering. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when data tracing is enabled, that is, when either TRCCONFIGR.DA == 1 or TRCCONFIGR.DV == 1."]
pub mod trcvdctlr;
#[doc = "TRCVDSACCTLR (rw) register accessor: ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvdsacctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvdsacctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvdsacctlr`] module"]
#[doc(alias = "TRCVDSACCTLR")]
pub type Trcvdsacctlr = crate::Reg<trcvdsacctlr::TrcvdsacctlrSpec>;
#[doc = "ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
pub mod trcvdsacctlr;
#[doc = "TRCVDARCCTLR (rw) register accessor: ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvdarcctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvdarcctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcvdarcctlr`] module"]
#[doc(alias = "TRCVDARCCTLR")]
pub type Trcvdarcctlr = crate::Reg<trcvdarcctlr::TrcvdarcctlrSpec>;
#[doc = "ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented."]
pub mod trcvdarcctlr;
#[doc = "TRCSEQEVR (rw) register accessor: Description collection: Moves the sequencer state according to programmed events. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcseqevr`] module"]
#[doc(alias = "TRCSEQEVR")]
pub type Trcseqevr = crate::Reg<trcseqevr::TrcseqevrSpec>;
#[doc = "Description collection: Moves the sequencer state according to programmed events. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
pub mod trcseqevr;
#[doc = "TRCSEQRSTEVR (rw) register accessor: Moves the sequencer to state 0 when a programmed event occurs. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqrstevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqrstevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcseqrstevr`] module"]
#[doc(alias = "TRCSEQRSTEVR")]
pub type Trcseqrstevr = crate::Reg<trcseqrstevr::TrcseqrstevrSpec>;
#[doc = "Moves the sequencer to state 0 when a programmed event occurs. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
pub mod trcseqrstevr;
#[doc = "TRCSEQSTR (rw) register accessor: Use this to set, or read, the sequencer state. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcseqstr`] module"]
#[doc(alias = "TRCSEQSTR")]
pub type Trcseqstr = crate::Reg<trcseqstr::TrcseqstrSpec>;
#[doc = "Use this to set, or read, the sequencer state. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
pub mod trcseqstr;
#[doc = "TRCEXTINSELR (rw) register accessor: Use this to set, or read, which external inputs are resources to the trace unit. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcextinselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcextinselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcextinselr`] module"]
#[doc(alias = "TRCEXTINSELR")]
pub type Trcextinselr = crate::Reg<trcextinselr::TrcextinselrSpec>;
#[doc = "Use this to set, or read, which external inputs are resources to the trace unit. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event."]
pub mod trcextinselr;
#[doc = "TRCCNTRLDVR (rw) register accessor: Description collection: This sets or returns the reload count value for counter n. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntrldvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntrldvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trccntrldvr`] module"]
#[doc(alias = "TRCCNTRLDVR")]
pub type Trccntrldvr = crate::Reg<trccntrldvr::TrccntrldvrSpec>;
#[doc = "Description collection: This sets or returns the reload count value for counter n. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trccntrldvr;
#[doc = "TRCCNTCTLR (rw) register accessor: Description collection: Controls the operation of counter n. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trccntctlr`] module"]
#[doc(alias = "TRCCNTCTLR")]
pub type Trccntctlr = crate::Reg<trccntctlr::TrccntctlrSpec>;
#[doc = "Description collection: Controls the operation of counter n. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trccntctlr;
#[doc = "TRCCNTVR (rw) register accessor: Description collection: This sets or returns the value of counter n. The count value is only stable when TRCSTATR.PMSTABLE == 1. If software uses counter n then it must write to this register to set the initial counter value. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccntvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccntvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trccntvr`] module"]
#[doc(alias = "TRCCNTVR")]
pub type Trccntvr = crate::Reg<trccntvr::TrccntvrSpec>;
#[doc = "Description collection: This sets or returns the value of counter n. The count value is only stable when TRCSTATR.PMSTABLE == 1. If software uses counter n then it must write to this register to set the initial counter value. Might ignore writes when the trace unit is enabled or not idle."]
pub mod trccntvr;
#[doc = "TRCRSCTLR (rw) register accessor: Description collection: Controls the selection of the resources in the trace unit. Might ignore writes when the trace unit is enabled or not idle. If software selects a non-implemented resource then CONSTRAINED UNPREDICTABLE behavior of the resource selector occurs, so the resource selector might fire unexpectedly or might not fire. Reads of the TRCRSCTLRn might return UNKNOWN.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcrsctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcrsctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcrsctlr`] module"]
#[doc(alias = "TRCRSCTLR")]
pub type Trcrsctlr = crate::Reg<trcrsctlr::TrcrsctlrSpec>;
#[doc = "Description collection: Controls the selection of the resources in the trace unit. Might ignore writes when the trace unit is enabled or not idle. If software selects a non-implemented resource then CONSTRAINED UNPREDICTABLE behavior of the resource selector occurs, so the resource selector might fire unexpectedly or might not fire. Reads of the TRCRSCTLRn might return UNKNOWN."]
pub mod trcrsctlr;
#[doc = "TRCSSCCR0 (rw) register accessor: Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcssccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcssccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcssccr0`] module"]
#[doc(alias = "TRCSSCCR0")]
pub type Trcssccr0 = crate::Reg<trcssccr0::Trcssccr0Spec>;
#[doc = "Controls the single-shot comparator."]
pub mod trcssccr0;
#[doc = "TRCSSCSR0 (rw) register accessor: Indicates the status of the single-shot comparators. TRCSSCSR0 is sensitive toinstruction addresses.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsscsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsscsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcsscsr0`] module"]
#[doc(alias = "TRCSSCSR0")]
pub type Trcsscsr0 = crate::Reg<trcsscsr0::Trcsscsr0Spec>;
#[doc = "Indicates the status of the single-shot comparators. TRCSSCSR0 is sensitive toinstruction addresses."]
pub mod trcsscsr0;
#[doc = "TRCSSPCICR0 (rw) register accessor: Selects the processor comparator inputs for Single-shot control.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsspcicr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsspcicr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcsspcicr0`] module"]
#[doc(alias = "TRCSSPCICR0")]
pub type Trcsspcicr0 = crate::Reg<trcsspcicr0::Trcsspcicr0Spec>;
#[doc = "Selects the processor comparator inputs for Single-shot control."]
pub mod trcsspcicr0;
#[doc = "TRCPDCR (rw) register accessor: Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcpdcr`] module"]
#[doc(alias = "TRCPDCR")]
pub type Trcpdcr = crate::Reg<trcpdcr::TrcpdcrSpec>;
#[doc = "Controls the single-shot comparator."]
pub mod trcpdcr;
#[doc = "TRCPDSR (rw) register accessor: Indicates the power down status of the ETM.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcpdsr`] module"]
#[doc(alias = "TRCPDSR")]
pub type Trcpdsr = crate::Reg<trcpdsr::TrcpdsrSpec>;
#[doc = "Indicates the power down status of the ETM."]
pub mod trcpdsr;
#[doc = "TRCITATBIDR (rw) register accessor: Sets the state of output pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitatbidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitatbidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcitatbidr`] module"]
#[doc(alias = "TRCITATBIDR")]
pub type Trcitatbidr = crate::Reg<trcitatbidr::TrcitatbidrSpec>;
#[doc = "Sets the state of output pins."]
pub mod trcitatbidr;
#[doc = "TRCITIATBINR (rw) register accessor: Reads the state of the input pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitiatbinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatbinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcitiatbinr`] module"]
#[doc(alias = "TRCITIATBINR")]
pub type Trcitiatbinr = crate::Reg<trcitiatbinr::TrcitiatbinrSpec>;
#[doc = "Reads the state of the input pins."]
pub mod trcitiatbinr;
#[doc = "TRCITIATBOUTR (rw) register accessor: Sets the state of the output pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitiatboutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatboutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcitiatboutr`] module"]
#[doc(alias = "TRCITIATBOUTR")]
pub type Trcitiatboutr = crate::Reg<trcitiatboutr::TrcitiatboutrSpec>;
#[doc = "Sets the state of the output pins."]
pub mod trcitiatboutr;
#[doc = "TRCITCTRL (rw) register accessor: Enables topology detection or integration testing, by putting ETM-M33 into integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcitctrl`] module"]
#[doc(alias = "TRCITCTRL")]
pub type Trcitctrl = crate::Reg<trcitctrl::TrcitctrlSpec>;
#[doc = "Enables topology detection or integration testing, by putting ETM-M33 into integration mode."]
pub mod trcitctrl;
#[doc = "TRCCLAIMSET (rw) register accessor: Sets bits in the claim tag and determines the number of claim tag bits implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclaimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclaimset`] module"]
#[doc(alias = "TRCCLAIMSET")]
pub type Trcclaimset = crate::Reg<trcclaimset::TrcclaimsetSpec>;
#[doc = "Sets bits in the claim tag and determines the number of claim tag bits implemented."]
pub mod trcclaimset;
#[doc = "TRCCLAIMCLR (rw) register accessor: Clears bits in the claim tag and determines the current value of the claim tag.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcclaimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcclaimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcclaimclr`] module"]
#[doc(alias = "TRCCLAIMCLR")]
pub type Trcclaimclr = crate::Reg<trcclaimclr::TrcclaimclrSpec>;
#[doc = "Clears bits in the claim tag and determines the current value of the claim tag."]
pub mod trcclaimclr;
#[doc = "TRCAUTHSTATUS (rw) register accessor: Indicates the current level of tracing permitted by the system\n\nYou can [`read`](crate::Reg::read) this register and get [`trcauthstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcauthstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcauthstatus`] module"]
#[doc(alias = "TRCAUTHSTATUS")]
pub type Trcauthstatus = crate::Reg<trcauthstatus::TrcauthstatusSpec>;
#[doc = "Indicates the current level of tracing permitted by the system"]
pub mod trcauthstatus;
#[doc = "TRCDEVARCH (r) register accessor: The TRCDEVARCH identifies ETM-M33 as an ETMv4.2 component\n\nYou can [`read`](crate::Reg::read) this register and get [`trcdevarch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcdevarch`] module"]
#[doc(alias = "TRCDEVARCH")]
pub type Trcdevarch = crate::Reg<trcdevarch::TrcdevarchSpec>;
#[doc = "The TRCDEVARCH identifies ETM-M33 as an ETMv4.2 component"]
pub mod trcdevarch;
#[doc = "TRCDEVTYPE (r) register accessor: Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcdevtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcdevtype`] module"]
#[doc(alias = "TRCDEVTYPE")]
pub type Trcdevtype = crate::Reg<trcdevtype::TrcdevtypeSpec>;
#[doc = "Controls the single-shot comparator."]
pub mod trcdevtype;
#[doc = "TRCPIDR (rw) register accessor: Description collection: Coresight peripheral identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcpidr`] module"]
#[doc(alias = "TRCPIDR")]
pub type Trcpidr = crate::Reg<trcpidr::TrcpidrSpec>;
#[doc = "Description collection: Coresight peripheral identification registers."]
pub mod trcpidr;
#[doc = "TRCCIDR (rw) register accessor: Description collection: Coresight component identification registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`trccidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trccidr`] module"]
#[doc(alias = "TRCCIDR")]
pub type Trccidr = crate::Reg<trccidr::TrccidrSpec>;
#[doc = "Description collection: Coresight component identification registers."]
pub mod trccidr;
