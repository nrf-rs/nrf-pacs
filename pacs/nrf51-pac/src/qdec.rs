#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the quadrature decoder."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the quadrature decoder."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers."]
    pub tasks_readclracc: TASKS_READCLRACC,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - A new sample is written to the sample register."]
    pub events_samplerdy: EVENTS_SAMPLERDY,
    #[doc = "0x104 - REPORTPER number of samples accumulated in ACC register, and ACC register different than zero."]
    pub events_reportrdy: EVENTS_REPORTRDY,
    #[doc = "0x108 - ACC or ACCDBL register overflow."]
    pub events_accof: EVENTS_ACCOF,
    _reserved6: [u8; 0xf4],
    #[doc = "0x200 - Shortcuts for the QDEC."]
    pub shorts: SHORTS,
    _reserved7: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0x01f4],
    #[doc = "0x500 - Enable the QDEC."]
    pub enable: ENABLE,
    #[doc = "0x504 - LED output pin polarity."]
    pub ledpol: LEDPOL,
    #[doc = "0x508 - Sample period."]
    pub sampleper: SAMPLEPER,
    #[doc = "0x50c - Motion sample value."]
    pub sample: SAMPLE,
    #[doc = "0x510 - Number of samples to generate an EVENT_REPORTRDY."]
    pub reportper: REPORTPER,
    #[doc = "0x514 - Accumulated valid transitions register."]
    pub acc: ACC,
    #[doc = "0x518 - Snapshot of ACC register. Value generated by the TASKS_READCLEACC task."]
    pub accread: ACCREAD,
    #[doc = "0x51c - Pin select for LED output."]
    pub pselled: PSELLED,
    #[doc = "0x520 - Pin select for phase A input."]
    pub psela: PSELA,
    #[doc = "0x524 - Pin select for phase B input."]
    pub pselb: PSELB,
    #[doc = "0x528 - Enable debouncer input filters."]
    pub dbfen: DBFEN,
    _reserved20: [u8; 0x14],
    #[doc = "0x540 - Time LED is switched ON before the sample."]
    pub ledpre: LEDPRE,
    #[doc = "0x544 - Accumulated double (error) transitions register."]
    pub accdbl: ACCDBL,
    #[doc = "0x548 - Snapshot of ACCDBL register. Value generated by the TASKS_READCLEACC task."]
    pub accdblread: ACCDBLREAD,
    _reserved23: [u8; 0x0ab0],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the quadrature decoder."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop the quadrature decoder."]
pub mod tasks_stop;
#[doc = "TASKS_READCLRACC (w) register accessor: an alias for `Reg<TASKS_READCLRACC_SPEC>`"]
pub type TASKS_READCLRACC = crate::Reg<tasks_readclracc::TASKS_READCLRACC_SPEC>;
#[doc = "Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers."]
pub mod tasks_readclracc;
#[doc = "EVENTS_SAMPLERDY (rw) register accessor: an alias for `Reg<EVENTS_SAMPLERDY_SPEC>`"]
pub type EVENTS_SAMPLERDY = crate::Reg<events_samplerdy::EVENTS_SAMPLERDY_SPEC>;
#[doc = "A new sample is written to the sample register."]
pub mod events_samplerdy;
#[doc = "EVENTS_REPORTRDY (rw) register accessor: an alias for `Reg<EVENTS_REPORTRDY_SPEC>`"]
pub type EVENTS_REPORTRDY = crate::Reg<events_reportrdy::EVENTS_REPORTRDY_SPEC>;
#[doc = "REPORTPER number of samples accumulated in ACC register, and ACC register different than zero."]
pub mod events_reportrdy;
#[doc = "EVENTS_ACCOF (rw) register accessor: an alias for `Reg<EVENTS_ACCOF_SPEC>`"]
pub type EVENTS_ACCOF = crate::Reg<events_accof::EVENTS_ACCOF_SPEC>;
#[doc = "ACC or ACCDBL register overflow."]
pub mod events_accof;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for the QDEC."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable the QDEC."]
pub mod enable;
#[doc = "LEDPOL (rw) register accessor: an alias for `Reg<LEDPOL_SPEC>`"]
pub type LEDPOL = crate::Reg<ledpol::LEDPOL_SPEC>;
#[doc = "LED output pin polarity."]
pub mod ledpol;
#[doc = "SAMPLEPER (rw) register accessor: an alias for `Reg<SAMPLEPER_SPEC>`"]
pub type SAMPLEPER = crate::Reg<sampleper::SAMPLEPER_SPEC>;
#[doc = "Sample period."]
pub mod sampleper;
#[doc = "SAMPLE (r) register accessor: an alias for `Reg<SAMPLE_SPEC>`"]
pub type SAMPLE = crate::Reg<sample::SAMPLE_SPEC>;
#[doc = "Motion sample value."]
pub mod sample;
#[doc = "REPORTPER (rw) register accessor: an alias for `Reg<REPORTPER_SPEC>`"]
pub type REPORTPER = crate::Reg<reportper::REPORTPER_SPEC>;
#[doc = "Number of samples to generate an EVENT_REPORTRDY."]
pub mod reportper;
#[doc = "ACC (r) register accessor: an alias for `Reg<ACC_SPEC>`"]
pub type ACC = crate::Reg<acc::ACC_SPEC>;
#[doc = "Accumulated valid transitions register."]
pub mod acc;
#[doc = "ACCREAD (r) register accessor: an alias for `Reg<ACCREAD_SPEC>`"]
pub type ACCREAD = crate::Reg<accread::ACCREAD_SPEC>;
#[doc = "Snapshot of ACC register. Value generated by the TASKS_READCLEACC task."]
pub mod accread;
#[doc = "PSELLED (rw) register accessor: an alias for `Reg<PSELLED_SPEC>`"]
pub type PSELLED = crate::Reg<pselled::PSELLED_SPEC>;
#[doc = "Pin select for LED output."]
pub mod pselled;
#[doc = "PSELA (rw) register accessor: an alias for `Reg<PSELA_SPEC>`"]
pub type PSELA = crate::Reg<psela::PSELA_SPEC>;
#[doc = "Pin select for phase A input."]
pub mod psela;
#[doc = "PSELB (rw) register accessor: an alias for `Reg<PSELB_SPEC>`"]
pub type PSELB = crate::Reg<pselb::PSELB_SPEC>;
#[doc = "Pin select for phase B input."]
pub mod pselb;
#[doc = "DBFEN (rw) register accessor: an alias for `Reg<DBFEN_SPEC>`"]
pub type DBFEN = crate::Reg<dbfen::DBFEN_SPEC>;
#[doc = "Enable debouncer input filters."]
pub mod dbfen;
#[doc = "LEDPRE (rw) register accessor: an alias for `Reg<LEDPRE_SPEC>`"]
pub type LEDPRE = crate::Reg<ledpre::LEDPRE_SPEC>;
#[doc = "Time LED is switched ON before the sample."]
pub mod ledpre;
#[doc = "ACCDBL (r) register accessor: an alias for `Reg<ACCDBL_SPEC>`"]
pub type ACCDBL = crate::Reg<accdbl::ACCDBL_SPEC>;
#[doc = "Accumulated double (error) transitions register."]
pub mod accdbl;
#[doc = "ACCDBLREAD (r) register accessor: an alias for `Reg<ACCDBLREAD_SPEC>`"]
pub type ACCDBLREAD = crate::Reg<accdblread::ACCDBLREAD_SPEC>;
#[doc = "Snapshot of ACCDBL register. Value generated by the TASKS_READCLEACC task."]
pub mod accdblread;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
