#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_readclracc: TasksReadclracc,
    tasks_rdclracc: TasksRdclracc,
    tasks_rdclrdbl: TasksRdclrdbl,
    _reserved5: [u8; 0xec],
    events_samplerdy: EventsSamplerdy,
    events_reportrdy: EventsReportrdy,
    events_accof: EventsAccof,
    events_dblrdy: EventsDblrdy,
    events_stopped: EventsStopped,
    _reserved10: [u8; 0xec],
    shorts: Shorts,
    _reserved11: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved13: [u8; 0x01f4],
    enable: Enable,
    ledpol: Ledpol,
    sampleper: Sampleper,
    sample: Sample,
    reportper: Reportper,
    acc: Acc,
    accread: Accread,
    psel: Psel,
    dbfen: Dbfen,
    _reserved22: [u8; 0x14],
    ledpre: Ledpre,
    accdbl: Accdbl,
    accdblread: Accdblread,
}
impl RegisterBlock {
    #[doc = "0x00 - Task starting the quadrature decoder"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Task stopping the quadrature decoder"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x08 - Read and clear ACC and ACCDBL"]
    #[inline(always)]
    pub const fn tasks_readclracc(&self) -> &TasksReadclracc {
        &self.tasks_readclracc
    }
    #[doc = "0x0c - Read and clear ACC"]
    #[inline(always)]
    pub const fn tasks_rdclracc(&self) -> &TasksRdclracc {
        &self.tasks_rdclracc
    }
    #[doc = "0x10 - Read and clear ACCDBL"]
    #[inline(always)]
    pub const fn tasks_rdclrdbl(&self) -> &TasksRdclrdbl {
        &self.tasks_rdclrdbl
    }
    #[doc = "0x100 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub const fn events_samplerdy(&self) -> &EventsSamplerdy {
        &self.events_samplerdy
    }
    #[doc = "0x104 - Non-null report ready"]
    #[inline(always)]
    pub const fn events_reportrdy(&self) -> &EventsReportrdy {
        &self.events_reportrdy
    }
    #[doc = "0x108 - ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub const fn events_accof(&self) -> &EventsAccof {
        &self.events_accof
    }
    #[doc = "0x10c - Double displacement(s) detected"]
    #[inline(always)]
    pub const fn events_dblrdy(&self) -> &EventsDblrdy {
        &self.events_dblrdy
    }
    #[doc = "0x110 - QDEC has been stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x200 - Shortcut register"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x500 - Enable the quadrature decoder"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - LED output pin polarity"]
    #[inline(always)]
    pub const fn ledpol(&self) -> &Ledpol {
        &self.ledpol
    }
    #[doc = "0x508 - Sample period"]
    #[inline(always)]
    pub const fn sampleper(&self) -> &Sampleper {
        &self.sampleper
    }
    #[doc = "0x50c - Motion sample value"]
    #[inline(always)]
    pub const fn sample(&self) -> &Sample {
        &self.sample
    }
    #[doc = "0x510 - Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    #[inline(always)]
    pub const fn reportper(&self) -> &Reportper {
        &self.reportper
    }
    #[doc = "0x514 - Register accumulating the valid transitions"]
    #[inline(always)]
    pub const fn acc(&self) -> &Acc {
        &self.acc
    }
    #[doc = "0x518 - Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    #[inline(always)]
    pub const fn accread(&self) -> &Accread {
        &self.accread
    }
    #[doc = "0x51c..0x528 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x528 - Enable input debounce filters"]
    #[inline(always)]
    pub const fn dbfen(&self) -> &Dbfen {
        &self.dbfen
    }
    #[doc = "0x540 - Time period the LED is switched ON prior to sampling"]
    #[inline(always)]
    pub const fn ledpre(&self) -> &Ledpre {
        &self.ledpre
    }
    #[doc = "0x544 - Register accumulating the number of detected double transitions"]
    #[inline(always)]
    pub const fn accdbl(&self) -> &Accdbl {
        &self.accdbl
    }
    #[doc = "0x548 - Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    #[inline(always)]
    pub const fn accdblread(&self) -> &Accdblread {
        &self.accdblread
    }
}
#[doc = "TASKS_START (w) register accessor: Task starting the quadrature decoder\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`] module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Task starting the quadrature decoder"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Task stopping the quadrature decoder\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Task stopping the quadrature decoder"]
pub mod tasks_stop;
#[doc = "TASKS_READCLRACC (w) register accessor: Read and clear ACC and ACCDBL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readclracc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_readclracc`] module"]
#[doc(alias = "TASKS_READCLRACC")]
pub type TasksReadclracc = crate::Reg<tasks_readclracc::TasksReadclraccSpec>;
#[doc = "Read and clear ACC and ACCDBL"]
pub mod tasks_readclracc;
#[doc = "TASKS_RDCLRACC (w) register accessor: Read and clear ACC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclracc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rdclracc`] module"]
#[doc(alias = "TASKS_RDCLRACC")]
pub type TasksRdclracc = crate::Reg<tasks_rdclracc::TasksRdclraccSpec>;
#[doc = "Read and clear ACC"]
pub mod tasks_rdclracc;
#[doc = "TASKS_RDCLRDBL (w) register accessor: Read and clear ACCDBL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rdclrdbl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rdclrdbl`] module"]
#[doc(alias = "TASKS_RDCLRDBL")]
pub type TasksRdclrdbl = crate::Reg<tasks_rdclrdbl::TasksRdclrdblSpec>;
#[doc = "Read and clear ACCDBL"]
pub mod tasks_rdclrdbl;
#[doc = "EVENTS_SAMPLERDY (rw) register accessor: Event being generated for every new sample value written to the SAMPLE register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_samplerdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_samplerdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_samplerdy`] module"]
#[doc(alias = "EVENTS_SAMPLERDY")]
pub type EventsSamplerdy = crate::Reg<events_samplerdy::EventsSamplerdySpec>;
#[doc = "Event being generated for every new sample value written to the SAMPLE register"]
pub mod events_samplerdy;
#[doc = "EVENTS_REPORTRDY (rw) register accessor: Non-null report ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_reportrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_reportrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_reportrdy`] module"]
#[doc(alias = "EVENTS_REPORTRDY")]
pub type EventsReportrdy = crate::Reg<events_reportrdy::EventsReportrdySpec>;
#[doc = "Non-null report ready"]
pub mod events_reportrdy;
#[doc = "EVENTS_ACCOF (rw) register accessor: ACC or ACCDBL register overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_accof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_accof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_accof`] module"]
#[doc(alias = "EVENTS_ACCOF")]
pub type EventsAccof = crate::Reg<events_accof::EventsAccofSpec>;
#[doc = "ACC or ACCDBL register overflow"]
pub mod events_accof;
#[doc = "EVENTS_DBLRDY (rw) register accessor: Double displacement(s) detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_dblrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_dblrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_dblrdy`] module"]
#[doc(alias = "EVENTS_DBLRDY")]
pub type EventsDblrdy = crate::Reg<events_dblrdy::EventsDblrdySpec>;
#[doc = "Double displacement(s) detected"]
pub mod events_dblrdy;
#[doc = "EVENTS_STOPPED (rw) register accessor: QDEC has been stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_stopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`] module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "QDEC has been stopped"]
pub mod events_stopped;
#[doc = "SHORTS (rw) register accessor: Shortcut register\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE (rw) register accessor: Enable the quadrature decoder\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable the quadrature decoder"]
pub mod enable;
#[doc = "LEDPOL (rw) register accessor: LED output pin polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`ledpol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledpol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledpol`] module"]
#[doc(alias = "LEDPOL")]
pub type Ledpol = crate::Reg<ledpol::LedpolSpec>;
#[doc = "LED output pin polarity"]
pub mod ledpol;
#[doc = "SAMPLEPER (rw) register accessor: Sample period\n\nYou can [`read`](crate::Reg::read) this register and get [`sampleper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampleper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampleper`] module"]
#[doc(alias = "SAMPLEPER")]
pub type Sampleper = crate::Reg<sampleper::SampleperSpec>;
#[doc = "Sample period"]
pub mod sampleper;
#[doc = "SAMPLE (r) register accessor: Motion sample value\n\nYou can [`read`](crate::Reg::read) this register and get [`sample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample`] module"]
#[doc(alias = "SAMPLE")]
pub type Sample = crate::Reg<sample::SampleSpec>;
#[doc = "Motion sample value"]
pub mod sample;
#[doc = "REPORTPER (rw) register accessor: Number of samples to be taken before REPORTRDY and DBLRDY events can be generated\n\nYou can [`read`](crate::Reg::read) this register and get [`reportper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reportper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reportper`] module"]
#[doc(alias = "REPORTPER")]
pub type Reportper = crate::Reg<reportper::ReportperSpec>;
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
pub mod reportper;
#[doc = "ACC (r) register accessor: Register accumulating the valid transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`acc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc`] module"]
#[doc(alias = "ACC")]
pub type Acc = crate::Reg<acc::AccSpec>;
#[doc = "Register accumulating the valid transitions"]
pub mod acc;
#[doc = "ACCREAD (r) register accessor: Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task\n\nYou can [`read`](crate::Reg::read) this register and get [`accread::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accread`] module"]
#[doc(alias = "ACCREAD")]
pub type Accread = crate::Reg<accread::AccreadSpec>;
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
pub mod accread;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "DBFEN (rw) register accessor: Enable input debounce filters\n\nYou can [`read`](crate::Reg::read) this register and get [`dbfen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbfen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbfen`] module"]
#[doc(alias = "DBFEN")]
pub type Dbfen = crate::Reg<dbfen::DbfenSpec>;
#[doc = "Enable input debounce filters"]
pub mod dbfen;
#[doc = "LEDPRE (rw) register accessor: Time period the LED is switched ON prior to sampling\n\nYou can [`read`](crate::Reg::read) this register and get [`ledpre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledpre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledpre`] module"]
#[doc(alias = "LEDPRE")]
pub type Ledpre = crate::Reg<ledpre::LedpreSpec>;
#[doc = "Time period the LED is switched ON prior to sampling"]
pub mod ledpre;
#[doc = "ACCDBL (r) register accessor: Register accumulating the number of detected double transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`accdbl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accdbl`] module"]
#[doc(alias = "ACCDBL")]
pub type Accdbl = crate::Reg<accdbl::AccdblSpec>;
#[doc = "Register accumulating the number of detected double transitions"]
pub mod accdbl;
#[doc = "ACCDBLREAD (r) register accessor: Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task\n\nYou can [`read`](crate::Reg::read) this register and get [`accdblread::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@accdblread`] module"]
#[doc(alias = "ACCDBLREAD")]
pub type Accdblread = crate::Reg<accdblread::AccdblreadSpec>;
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
pub mod accdblread;
