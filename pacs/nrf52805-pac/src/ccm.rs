#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_ksgen: TasksKsgen,
    tasks_crypt: TasksCrypt,
    tasks_stop: TasksStop,
    tasks_rateoverride: TasksRateoverride,
    _reserved4: [u8; 0xf0],
    events_endksgen: EventsEndksgen,
    events_endcrypt: EventsEndcrypt,
    events_error: EventsError,
    _reserved7: [u8; 0xf4],
    shorts: Shorts,
    _reserved8: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved10: [u8; 0xf4],
    micstatus: Micstatus,
    _reserved11: [u8; 0xfc],
    enable: Enable,
    mode: Mode,
    cnfptr: Cnfptr,
    inptr: Inptr,
    outptr: Outptr,
    scratchptr: Scratchptr,
    maxpacketsize: Maxpacketsize,
    rateoverride: Rateoverride,
}
impl RegisterBlock {
    #[doc = "0x00 - Start generation of keystream. This operation will stop by itself when completed."]
    #[inline(always)]
    pub const fn tasks_ksgen(&self) -> &TasksKsgen {
        &self.tasks_ksgen
    }
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    #[inline(always)]
    pub const fn tasks_crypt(&self) -> &TasksCrypt {
        &self.tasks_crypt
    }
    #[doc = "0x08 - Stop encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x0c - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    #[inline(always)]
    pub const fn tasks_rateoverride(&self) -> &TasksRateoverride {
        &self.tasks_rateoverride
    }
    #[doc = "0x100 - Keystream generation complete"]
    #[inline(always)]
    pub const fn events_endksgen(&self) -> &EventsEndksgen {
        &self.events_endksgen
    }
    #[doc = "0x104 - Encrypt/decrypt complete"]
    #[inline(always)]
    pub const fn events_endcrypt(&self) -> &EventsEndcrypt {
        &self.events_endcrypt
    }
    #[doc = "0x108 - Deprecated register - CCM error event"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
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
    #[doc = "0x400 - MIC check result"]
    #[inline(always)]
    pub const fn micstatus(&self) -> &Micstatus {
        &self.micstatus
    }
    #[doc = "0x500 - Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504 - Operation mode"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x508 - Pointer to data structure holding AES key and NONCE vector"]
    #[inline(always)]
    pub const fn cnfptr(&self) -> &Cnfptr {
        &self.cnfptr
    }
    #[doc = "0x50c - Input pointer"]
    #[inline(always)]
    pub const fn inptr(&self) -> &Inptr {
        &self.inptr
    }
    #[doc = "0x510 - Output pointer"]
    #[inline(always)]
    pub const fn outptr(&self) -> &Outptr {
        &self.outptr
    }
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    #[inline(always)]
    pub const fn scratchptr(&self) -> &Scratchptr {
        &self.scratchptr
    }
    #[doc = "0x518 - Length of keystream generated when MODE.LENGTH = Extended."]
    #[inline(always)]
    pub const fn maxpacketsize(&self) -> &Maxpacketsize {
        &self.maxpacketsize
    }
    #[doc = "0x51c - Data rate override setting."]
    #[inline(always)]
    pub const fn rateoverride(&self) -> &Rateoverride {
        &self.rateoverride
    }
}
#[doc = "TASKS_KSGEN (w) register accessor: Start generation of keystream. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ksgen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ksgen`] module"]
#[doc(alias = "TASKS_KSGEN")]
pub type TasksKsgen = crate::Reg<tasks_ksgen::TasksKsgenSpec>;
#[doc = "Start generation of keystream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "TASKS_CRYPT (w) register accessor: Start encryption/decryption. This operation will stop by itself when completed.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_crypt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_crypt`] module"]
#[doc(alias = "TASKS_CRYPT")]
pub type TasksCrypt = crate::Reg<tasks_crypt::TasksCryptSpec>;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "TASKS_STOP (w) register accessor: Stop encryption/decryption\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`] module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "TASKS_RATEOVERRIDE (w) register accessor: Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rateoverride::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rateoverride`] module"]
#[doc(alias = "TASKS_RATEOVERRIDE")]
pub type TasksRateoverride = crate::Reg<tasks_rateoverride::TasksRateoverrideSpec>;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub mod tasks_rateoverride;
#[doc = "EVENTS_ENDKSGEN (rw) register accessor: Keystream generation complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endksgen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endksgen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endksgen`] module"]
#[doc(alias = "EVENTS_ENDKSGEN")]
pub type EventsEndksgen = crate::Reg<events_endksgen::EventsEndksgenSpec>;
#[doc = "Keystream generation complete"]
pub mod events_endksgen;
#[doc = "EVENTS_ENDCRYPT (rw) register accessor: Encrypt/decrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endcrypt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endcrypt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endcrypt`] module"]
#[doc(alias = "EVENTS_ENDCRYPT")]
pub type EventsEndcrypt = crate::Reg<events_endcrypt::EventsEndcryptSpec>;
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "EVENTS_ERROR (rw) register accessor: Deprecated register - CCM error event\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`] module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Deprecated register - CCM error event"]
pub mod events_error;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
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
#[doc = "MICSTATUS (r) register accessor: MIC check result\n\nYou can [`read`](crate::Reg::read) this register and get [`micstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micstatus`] module"]
#[doc(alias = "MICSTATUS")]
pub type Micstatus = crate::Reg<micstatus::MicstatusSpec>;
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "ENABLE (rw) register accessor: Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable"]
pub mod enable;
#[doc = "MODE (rw) register accessor: Operation mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Operation mode"]
pub mod mode;
#[doc = "CNFPTR (rw) register accessor: Pointer to data structure holding AES key and NONCE vector\n\nYou can [`read`](crate::Reg::read) this register and get [`cnfptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnfptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnfptr`] module"]
#[doc(alias = "CNFPTR")]
pub type Cnfptr = crate::Reg<cnfptr::CnfptrSpec>;
#[doc = "Pointer to data structure holding AES key and NONCE vector"]
pub mod cnfptr;
#[doc = "INPTR (rw) register accessor: Input pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`inptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inptr`] module"]
#[doc(alias = "INPTR")]
pub type Inptr = crate::Reg<inptr::InptrSpec>;
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "OUTPTR (rw) register accessor: Output pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outptr`] module"]
#[doc(alias = "OUTPTR")]
pub type Outptr = crate::Reg<outptr::OutptrSpec>;
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "SCRATCHPTR (rw) register accessor: Pointer to data area used for temporary storage\n\nYou can [`read`](crate::Reg::read) this register and get [`scratchptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratchptr`] module"]
#[doc(alias = "SCRATCHPTR")]
pub type Scratchptr = crate::Reg<scratchptr::ScratchptrSpec>;
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
#[doc = "MAXPACKETSIZE (rw) register accessor: Length of keystream generated when MODE.LENGTH = Extended.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpacketsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpacketsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxpacketsize`] module"]
#[doc(alias = "MAXPACKETSIZE")]
pub type Maxpacketsize = crate::Reg<maxpacketsize::MaxpacketsizeSpec>;
#[doc = "Length of keystream generated when MODE.LENGTH = Extended."]
pub mod maxpacketsize;
#[doc = "RATEOVERRIDE (rw) register accessor: Data rate override setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`rateoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rateoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rateoverride`] module"]
#[doc(alias = "RATEOVERRIDE")]
pub type Rateoverride = crate::Reg<rateoverride::RateoverrideSpec>;
#[doc = "Data rate override setting."]
pub mod rateoverride;
