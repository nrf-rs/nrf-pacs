#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08..0x10 - Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    pub tasks_seqstart: [crate::Reg<tasks_seqstart::TASKS_SEQSTART_SPEC>; 2],
    #[doc = "0x10 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    pub tasks_nextstep: crate::Reg<tasks_nextstep::TASKS_NEXTSTEP_SPEC>,
    _reserved3: [u8; 0xf0],
    #[doc = "0x104 - Response to STOP task, emitted when PWM pulses are no longer generated"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    #[doc = "0x108..0x110 - Description collection: First PWM period started on sequence n"]
    pub events_seqstarted: [crate::Reg<events_seqstarted::EVENTS_SEQSTARTED_SPEC>; 2],
    #[doc = "0x110..0x118 - Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    pub events_seqend: [crate::Reg<events_seqend::EVENTS_SEQEND_SPEC>; 2],
    #[doc = "0x118 - Emitted at the end of each PWM period"]
    pub events_pwmperiodend: crate::Reg<events_pwmperiodend::EVENTS_PWMPERIODEND_SPEC>,
    #[doc = "0x11c - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    pub events_loopsdone: crate::Reg<events_loopsdone::EVENTS_LOOPSDONE_SPEC>,
    _reserved8: [u8; 0xe0],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved9: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved12: [u8; 0x01f4],
    #[doc = "0x500 - PWM module enable register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Selects operating mode of the wave counter"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x508 - Value up to which the pulse generator counter counts"]
    pub countertop: crate::Reg<countertop::COUNTERTOP_SPEC>,
    #[doc = "0x50c - Configuration for PWM_CLK"]
    pub prescaler: crate::Reg<prescaler::PRESCALER_SPEC>,
    #[doc = "0x510 - Configuration of the decoder"]
    pub decoder: crate::Reg<decoder::DECODER_SPEC>,
    #[doc = "0x514 - Number of playbacks of a loop"]
    pub loop_: crate::Reg<loop_::LOOP_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x520..0x530 - Unspecified"]
    pub seq0: SEQ,
    _reserved19: [u8; 0x10],
    #[doc = "0x540..0x550 - Unspecified"]
    pub seq1: SEQ,
    _reserved20: [u8; 0x10],
    #[doc = "0x560..0x570 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SEQ {
    #[doc = "0x00 - Description cluster: Beginning address in RAM of this sequence"]
    pub ptr: crate::Reg<self::seq::ptr::PTR_SPEC>,
    #[doc = "0x04 - Description cluster: Number of values (duty cycles) in this sequence"]
    pub cnt: crate::Reg<self::seq::cnt::CNT_SPEC>,
    #[doc = "0x08 - Description cluster: Number of additional PWM periods between samples loaded into compare register"]
    pub refresh: crate::Reg<self::seq::refresh::REFRESH_SPEC>,
    #[doc = "0x0c - Description cluster: Time added after the sequence"]
    pub enddelay: crate::Reg<self::seq::enddelay::ENDDELAY_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod seq;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00..0x10 - Description collection: Output pin select for PWM channel n"]
    pub out: [crate::Reg<self::psel::out::OUT_SPEC>; 4],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub mod tasks_stop;
#[doc = "TASKS_SEQSTART register accessor: an alias for `Reg<TASKS_SEQSTART_SPEC>`"]
pub type TASKS_SEQSTART = crate::Reg<tasks_seqstart::TASKS_SEQSTART_SPEC>;
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub mod tasks_seqstart;
#[doc = "TASKS_NEXTSTEP register accessor: an alias for `Reg<TASKS_NEXTSTEP_SPEC>`"]
pub type TASKS_NEXTSTEP = crate::Reg<tasks_nextstep::TASKS_NEXTSTEP_SPEC>;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub mod tasks_nextstep;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub mod events_stopped;
#[doc = "EVENTS_SEQSTARTED register accessor: an alias for `Reg<EVENTS_SEQSTARTED_SPEC>`"]
pub type EVENTS_SEQSTARTED = crate::Reg<events_seqstarted::EVENTS_SEQSTARTED_SPEC>;
#[doc = "Description collection: First PWM period started on sequence n"]
pub mod events_seqstarted;
#[doc = "EVENTS_SEQEND register accessor: an alias for `Reg<EVENTS_SEQEND_SPEC>`"]
pub type EVENTS_SEQEND = crate::Reg<events_seqend::EVENTS_SEQEND_SPEC>;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub mod events_seqend;
#[doc = "EVENTS_PWMPERIODEND register accessor: an alias for `Reg<EVENTS_PWMPERIODEND_SPEC>`"]
pub type EVENTS_PWMPERIODEND = crate::Reg<events_pwmperiodend::EVENTS_PWMPERIODEND_SPEC>;
#[doc = "Emitted at the end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "EVENTS_LOOPSDONE register accessor: an alias for `Reg<EVENTS_LOOPSDONE_SPEC>`"]
pub type EVENTS_LOOPSDONE = crate::Reg<events_loopsdone::EVENTS_LOOPSDONE_SPEC>;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub mod events_loopsdone;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "PWM module enable register"]
pub mod enable;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Selects operating mode of the wave counter"]
pub mod mode;
#[doc = "COUNTERTOP register accessor: an alias for `Reg<COUNTERTOP_SPEC>`"]
pub type COUNTERTOP = crate::Reg<countertop::COUNTERTOP_SPEC>;
#[doc = "Value up to which the pulse generator counter counts"]
pub mod countertop;
#[doc = "PRESCALER register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "Configuration for PWM_CLK"]
pub mod prescaler;
#[doc = "DECODER register accessor: an alias for `Reg<DECODER_SPEC>`"]
pub type DECODER = crate::Reg<decoder::DECODER_SPEC>;
#[doc = "Configuration of the decoder"]
pub mod decoder;
#[doc = "LOOP register accessor: an alias for `Reg<LOOP_SPEC>`"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Number of playbacks of a loop"]
pub mod loop_;
