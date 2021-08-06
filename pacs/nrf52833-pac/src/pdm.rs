#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stops PDM transfer"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - PDM transfer has started"]
    pub events_started: crate::Reg<events_started::EVENTS_STARTED_SPEC>,
    #[doc = "0x104 - PDM transfer has finished"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    pub events_end: crate::Reg<events_end::EVENTS_END_SPEC>,
    _reserved5: [u8; 0x01f4],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved8: [u8; 0x01f4],
    #[doc = "0x500 - PDM module enable register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - PDM clock generator control"]
    pub pdmclkctrl: crate::Reg<pdmclkctrl::PDMCLKCTRL_SPEC>,
    #[doc = "0x508 - Defines the routing of the connected PDM microphones' signals"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x518 - Left output gain adjustment"]
    pub gainl: crate::Reg<gainl::GAINL_SPEC>,
    #[doc = "0x51c - Right output gain adjustment"]
    pub gainr: crate::Reg<gainr::GAINR_SPEC>,
    #[doc = "0x520 - Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
    pub ratio: crate::Reg<ratio::RATIO_SPEC>,
    _reserved14: [u8; 0x1c],
    #[doc = "0x540..0x548 - Unspecified"]
    pub psel: PSEL,
    _reserved15: [u8; 0x18],
    #[doc = "0x560..0x568 - Unspecified"]
    pub sample: SAMPLE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for PDM CLK signal"]
    pub clk: crate::Reg<self::psel::clk::CLK_SPEC>,
    #[doc = "0x04 - Pin number configuration for PDM DIN signal"]
    pub din: crate::Reg<self::psel::din::DIN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct SAMPLE {
    #[doc = "0x00 - RAM address pointer to write samples to with EasyDMA"]
    pub ptr: crate::Reg<self::sample::ptr::PTR_SPEC>,
    #[doc = "0x04 - Number of samples to allocate memory for in EasyDMA mode"]
    pub maxcnt: crate::Reg<self::sample::maxcnt::MAXCNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod sample;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
#[doc = "EVENTS_STARTED register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "EVENTS_END register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub mod events_end;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "PDMCLKCTRL register accessor: an alias for `Reg<PDMCLKCTRL_SPEC>`"]
pub type PDMCLKCTRL = crate::Reg<pdmclkctrl::PDMCLKCTRL_SPEC>;
#[doc = "PDM clock generator control"]
pub mod pdmclkctrl;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub mod mode;
#[doc = "GAINL register accessor: an alias for `Reg<GAINL_SPEC>`"]
pub type GAINL = crate::Reg<gainl::GAINL_SPEC>;
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "GAINR register accessor: an alias for `Reg<GAINR_SPEC>`"]
pub type GAINR = crate::Reg<gainr::GAINR_SPEC>;
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "RATIO register accessor: an alias for `Reg<RATIO_SPEC>`"]
pub type RATIO = crate::Reg<ratio::RATIO_SPEC>;
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
pub mod ratio;
