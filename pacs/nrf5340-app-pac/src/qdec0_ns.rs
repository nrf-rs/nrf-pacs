#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task starting the quadrature decoder"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Task stopping the quadrature decoder"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Read and clear ACC and ACCDBL"]
    pub tasks_readclracc: crate::Reg<tasks_readclracc::TASKS_READCLRACC_SPEC>,
    #[doc = "0x0c - Read and clear ACC"]
    pub tasks_rdclracc: crate::Reg<tasks_rdclracc::TASKS_RDCLRACC_SPEC>,
    #[doc = "0x10 - Read and clear ACCDBL"]
    pub tasks_rdclrdbl: crate::Reg<tasks_rdclrdbl::TASKS_RDCLRDBL_SPEC>,
    _reserved5: [u8; 0x6c],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task READCLRACC"]
    pub subscribe_readclracc: crate::Reg<subscribe_readclracc::SUBSCRIBE_READCLRACC_SPEC>,
    #[doc = "0x8c - Subscribe configuration for task RDCLRACC"]
    pub subscribe_rdclracc: crate::Reg<subscribe_rdclracc::SUBSCRIBE_RDCLRACC_SPEC>,
    #[doc = "0x90 - Subscribe configuration for task RDCLRDBL"]
    pub subscribe_rdclrdbl: crate::Reg<subscribe_rdclrdbl::SUBSCRIBE_RDCLRDBL_SPEC>,
    _reserved10: [u8; 0x6c],
    #[doc = "0x100 - Event being generated for every new sample value written to the SAMPLE register"]
    pub events_samplerdy: crate::Reg<events_samplerdy::EVENTS_SAMPLERDY_SPEC>,
    #[doc = "0x104 - Non-null report ready"]
    pub events_reportrdy: crate::Reg<events_reportrdy::EVENTS_REPORTRDY_SPEC>,
    #[doc = "0x108 - ACC or ACCDBL register overflow"]
    pub events_accof: crate::Reg<events_accof::EVENTS_ACCOF_SPEC>,
    #[doc = "0x10c - Double displacement(s) detected"]
    pub events_dblrdy: crate::Reg<events_dblrdy::EVENTS_DBLRDY_SPEC>,
    #[doc = "0x110 - QDEC has been stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved15: [u8; 0x6c],
    #[doc = "0x180 - Publish configuration for event SAMPLERDY"]
    pub publish_samplerdy: crate::Reg<publish_samplerdy::PUBLISH_SAMPLERDY_SPEC>,
    #[doc = "0x184 - Publish configuration for event REPORTRDY"]
    pub publish_reportrdy: crate::Reg<publish_reportrdy::PUBLISH_REPORTRDY_SPEC>,
    #[doc = "0x188 - Publish configuration for event ACCOF"]
    pub publish_accof: crate::Reg<publish_accof::PUBLISH_ACCOF_SPEC>,
    #[doc = "0x18c - Publish configuration for event DBLRDY"]
    pub publish_dblrdy: crate::Reg<publish_dblrdy::PUBLISH_DBLRDY_SPEC>,
    #[doc = "0x190 - Publish configuration for event STOPPED"]
    pub publish_stopped: crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>,
    _reserved20: [u8; 0x6c],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved21: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved23: [u8; 0x01f4],
    #[doc = "0x500 - Enable the quadrature decoder"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - LED output pin polarity"]
    pub ledpol: crate::Reg<ledpol::LEDPOL_SPEC>,
    #[doc = "0x508 - Sample period"]
    pub sampleper: crate::Reg<sampleper::SAMPLEPER_SPEC>,
    #[doc = "0x50c - Motion sample value"]
    pub sample: crate::Reg<sample::SAMPLE_SPEC>,
    #[doc = "0x510 - Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    pub reportper: crate::Reg<reportper::REPORTPER_SPEC>,
    #[doc = "0x514 - Register accumulating the valid transitions"]
    pub acc: crate::Reg<acc::ACC_SPEC>,
    #[doc = "0x518 - Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    pub accread: crate::Reg<accread::ACCREAD_SPEC>,
    #[doc = "0x51c..0x528 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x528 - Enable input debounce filters"]
    pub dbfen: crate::Reg<dbfen::DBFEN_SPEC>,
    _reserved32: [u8; 0x14],
    #[doc = "0x540 - Time period the LED is switched ON prior to sampling"]
    pub ledpre: crate::Reg<ledpre::LEDPRE_SPEC>,
    #[doc = "0x544 - Register accumulating the number of detected double transitions"]
    pub accdbl: crate::Reg<accdbl::ACCDBL_SPEC>,
    #[doc = "0x548 - Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    pub accdblread: crate::Reg<accdblread::ACCDBLREAD_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for LED signal"]
    pub led: crate::Reg<self::psel::led::LED_SPEC>,
    #[doc = "0x04 - Pin select for A signal"]
    pub a: crate::Reg<self::psel::a::A_SPEC>,
    #[doc = "0x08 - Pin select for B signal"]
    pub b: crate::Reg<self::psel::b::B_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Task starting the quadrature decoder"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Task stopping the quadrature decoder"]
pub mod tasks_stop;
#[doc = "TASKS_READCLRACC register accessor: an alias for `Reg<TASKS_READCLRACC_SPEC>`"]
pub type TASKS_READCLRACC = crate::Reg<tasks_readclracc::TASKS_READCLRACC_SPEC>;
#[doc = "Read and clear ACC and ACCDBL"]
pub mod tasks_readclracc;
#[doc = "TASKS_RDCLRACC register accessor: an alias for `Reg<TASKS_RDCLRACC_SPEC>`"]
pub type TASKS_RDCLRACC = crate::Reg<tasks_rdclracc::TASKS_RDCLRACC_SPEC>;
#[doc = "Read and clear ACC"]
pub mod tasks_rdclracc;
#[doc = "TASKS_RDCLRDBL register accessor: an alias for `Reg<TASKS_RDCLRDBL_SPEC>`"]
pub type TASKS_RDCLRDBL = crate::Reg<tasks_rdclrdbl::TASKS_RDCLRDBL_SPEC>;
#[doc = "Read and clear ACCDBL"]
pub mod tasks_rdclrdbl;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_READCLRACC register accessor: an alias for `Reg<SUBSCRIBE_READCLRACC_SPEC>`"]
pub type SUBSCRIBE_READCLRACC = crate::Reg<subscribe_readclracc::SUBSCRIBE_READCLRACC_SPEC>;
#[doc = "Subscribe configuration for task READCLRACC"]
pub mod subscribe_readclracc;
#[doc = "SUBSCRIBE_RDCLRACC register accessor: an alias for `Reg<SUBSCRIBE_RDCLRACC_SPEC>`"]
pub type SUBSCRIBE_RDCLRACC = crate::Reg<subscribe_rdclracc::SUBSCRIBE_RDCLRACC_SPEC>;
#[doc = "Subscribe configuration for task RDCLRACC"]
pub mod subscribe_rdclracc;
#[doc = "SUBSCRIBE_RDCLRDBL register accessor: an alias for `Reg<SUBSCRIBE_RDCLRDBL_SPEC>`"]
pub type SUBSCRIBE_RDCLRDBL = crate::Reg<subscribe_rdclrdbl::SUBSCRIBE_RDCLRDBL_SPEC>;
#[doc = "Subscribe configuration for task RDCLRDBL"]
pub mod subscribe_rdclrdbl;
#[doc = "EVENTS_SAMPLERDY register accessor: an alias for `Reg<EVENTS_SAMPLERDY_SPEC>`"]
pub type EVENTS_SAMPLERDY = crate::Reg<events_samplerdy::EVENTS_SAMPLERDY_SPEC>;
#[doc = "Event being generated for every new sample value written to the SAMPLE register"]
pub mod events_samplerdy;
#[doc = "EVENTS_REPORTRDY register accessor: an alias for `Reg<EVENTS_REPORTRDY_SPEC>`"]
pub type EVENTS_REPORTRDY = crate::Reg<events_reportrdy::EVENTS_REPORTRDY_SPEC>;
#[doc = "Non-null report ready"]
pub mod events_reportrdy;
#[doc = "EVENTS_ACCOF register accessor: an alias for `Reg<EVENTS_ACCOF_SPEC>`"]
pub type EVENTS_ACCOF = crate::Reg<events_accof::EVENTS_ACCOF_SPEC>;
#[doc = "ACC or ACCDBL register overflow"]
pub mod events_accof;
#[doc = "EVENTS_DBLRDY register accessor: an alias for `Reg<EVENTS_DBLRDY_SPEC>`"]
pub type EVENTS_DBLRDY = crate::Reg<events_dblrdy::EVENTS_DBLRDY_SPEC>;
#[doc = "Double displacement(s) detected"]
pub mod events_dblrdy;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "QDEC has been stopped"]
pub mod events_stopped;
#[doc = "PUBLISH_SAMPLERDY register accessor: an alias for `Reg<PUBLISH_SAMPLERDY_SPEC>`"]
pub type PUBLISH_SAMPLERDY = crate::Reg<publish_samplerdy::PUBLISH_SAMPLERDY_SPEC>;
#[doc = "Publish configuration for event SAMPLERDY"]
pub mod publish_samplerdy;
#[doc = "PUBLISH_REPORTRDY register accessor: an alias for `Reg<PUBLISH_REPORTRDY_SPEC>`"]
pub type PUBLISH_REPORTRDY = crate::Reg<publish_reportrdy::PUBLISH_REPORTRDY_SPEC>;
#[doc = "Publish configuration for event REPORTRDY"]
pub mod publish_reportrdy;
#[doc = "PUBLISH_ACCOF register accessor: an alias for `Reg<PUBLISH_ACCOF_SPEC>`"]
pub type PUBLISH_ACCOF = crate::Reg<publish_accof::PUBLISH_ACCOF_SPEC>;
#[doc = "Publish configuration for event ACCOF"]
pub mod publish_accof;
#[doc = "PUBLISH_DBLRDY register accessor: an alias for `Reg<PUBLISH_DBLRDY_SPEC>`"]
pub type PUBLISH_DBLRDY = crate::Reg<publish_dblrdy::PUBLISH_DBLRDY_SPEC>;
#[doc = "Publish configuration for event DBLRDY"]
pub mod publish_dblrdy;
#[doc = "PUBLISH_STOPPED register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "Enable the quadrature decoder"]
pub mod enable;
#[doc = "LEDPOL register accessor: an alias for `Reg<LEDPOL_SPEC>`"]
pub type LEDPOL = crate::Reg<ledpol::LEDPOL_SPEC>;
#[doc = "LED output pin polarity"]
pub mod ledpol;
#[doc = "SAMPLEPER register accessor: an alias for `Reg<SAMPLEPER_SPEC>`"]
pub type SAMPLEPER = crate::Reg<sampleper::SAMPLEPER_SPEC>;
#[doc = "Sample period"]
pub mod sampleper;
#[doc = "SAMPLE register accessor: an alias for `Reg<SAMPLE_SPEC>`"]
pub type SAMPLE = crate::Reg<sample::SAMPLE_SPEC>;
#[doc = "Motion sample value"]
pub mod sample;
#[doc = "REPORTPER register accessor: an alias for `Reg<REPORTPER_SPEC>`"]
pub type REPORTPER = crate::Reg<reportper::REPORTPER_SPEC>;
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
pub mod reportper;
#[doc = "ACC register accessor: an alias for `Reg<ACC_SPEC>`"]
pub type ACC = crate::Reg<acc::ACC_SPEC>;
#[doc = "Register accumulating the valid transitions"]
pub mod acc;
#[doc = "ACCREAD register accessor: an alias for `Reg<ACCREAD_SPEC>`"]
pub type ACCREAD = crate::Reg<accread::ACCREAD_SPEC>;
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
pub mod accread;
#[doc = "DBFEN register accessor: an alias for `Reg<DBFEN_SPEC>`"]
pub type DBFEN = crate::Reg<dbfen::DBFEN_SPEC>;
#[doc = "Enable input debounce filters"]
pub mod dbfen;
#[doc = "LEDPRE register accessor: an alias for `Reg<LEDPRE_SPEC>`"]
pub type LEDPRE = crate::Reg<ledpre::LEDPRE_SPEC>;
#[doc = "Time period the LED is switched ON prior to sampling"]
pub mod ledpre;
#[doc = "ACCDBL register accessor: an alias for `Reg<ACCDBL_SPEC>`"]
pub type ACCDBL = crate::Reg<accdbl::ACCDBL_SPEC>;
#[doc = "Register accumulating the number of detected double transitions"]
pub mod accdbl;
#[doc = "ACCDBLREAD register accessor: an alias for `Reg<ACCDBLREAD_SPEC>`"]
pub type ACCDBLREAD = crate::Reg<accdblread::ACCDBLREAD_SPEC>;
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
pub mod accdblread;
