#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_ramaccerr: EventsRamaccerr,
    events_flashaccerr: EventsFlashaccerr,
    events_periphaccerr: EventsPeriphaccerr,
    _reserved3: [u8; 0x74],
    publish_ramaccerr: PublishRamaccerr,
    publish_flashaccerr: PublishFlashaccerr,
    publish_periphaccerr: PublishPeriphaccerr,
    _reserved6: [u8; 0x0174],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved9: [u8; 0xf4],
    cap: Cap,
    _reserved10: [u8; 0x3c],
    extdomain: [Extdomain; 1],
    _reserved11: [u8; 0x3c],
    dppi: [Dppi; 1],
    _reserved12: [u8; 0x38],
    gpioport: [Gpioport; 1],
    _reserved13: [u8; 0x38],
    flashnsc: [Flashnsc; 2],
    _reserved14: [u8; 0x30],
    ramnsc: [Ramnsc; 2],
    _reserved15: [u8; 0xb0],
    flashregion: [Flashregion; 32],
    _reserved16: [u8; 0x80],
    ramregion: [Ramregion; 32],
    _reserved17: [u8; 0x80],
    periphid: [Periphid; 67],
}
impl RegisterBlock {
    #[doc = "0x100 - A security violation has been detected for the RAM memory space"]
    #[inline(always)]
    pub const fn events_ramaccerr(&self) -> &EventsRamaccerr {
        &self.events_ramaccerr
    }
    #[doc = "0x104 - A security violation has been detected for the flash memory space"]
    #[inline(always)]
    pub const fn events_flashaccerr(&self) -> &EventsFlashaccerr {
        &self.events_flashaccerr
    }
    #[doc = "0x108 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub const fn events_periphaccerr(&self) -> &EventsPeriphaccerr {
        &self.events_periphaccerr
    }
    #[doc = "0x180 - Publish configuration for event RAMACCERR"]
    #[inline(always)]
    pub const fn publish_ramaccerr(&self) -> &PublishRamaccerr {
        &self.publish_ramaccerr
    }
    #[doc = "0x184 - Publish configuration for event FLASHACCERR"]
    #[inline(always)]
    pub const fn publish_flashaccerr(&self) -> &PublishFlashaccerr {
        &self.publish_flashaccerr
    }
    #[doc = "0x188 - Publish configuration for event PERIPHACCERR"]
    #[inline(always)]
    pub const fn publish_periphaccerr(&self) -> &PublishPeriphaccerr {
        &self.publish_periphaccerr
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
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
    #[doc = "0x400 - Show implemented features for the current device"]
    #[inline(always)]
    pub const fn cap(&self) -> &Cap {
        &self.cap
    }
    #[doc = "0x440 - Unspecified"]
    #[inline(always)]
    pub const fn extdomain(&self, n: usize) -> &Extdomain {
        &self.extdomain[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440 - Unspecified"]
    #[inline(always)]
    pub fn extdomain_iter(&self) -> impl Iterator<Item = &Extdomain> {
        self.extdomain.iter()
    }
    #[doc = "0x480..0x488 - Unspecified"]
    #[inline(always)]
    pub const fn dppi(&self, n: usize) -> &Dppi {
        &self.dppi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x480..0x488 - Unspecified"]
    #[inline(always)]
    pub fn dppi_iter(&self) -> impl Iterator<Item = &Dppi> {
        self.dppi.iter()
    }
    #[doc = "0x4c0..0x4c8 - Unspecified"]
    #[inline(always)]
    pub const fn gpioport(&self, n: usize) -> &Gpioport {
        &self.gpioport[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c0..0x4c8 - Unspecified"]
    #[inline(always)]
    pub fn gpioport_iter(&self) -> impl Iterator<Item = &Gpioport> {
        self.gpioport.iter()
    }
    #[doc = "0x500..0x510 - Unspecified"]
    #[inline(always)]
    pub const fn flashnsc(&self, n: usize) -> &Flashnsc {
        &self.flashnsc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x510 - Unspecified"]
    #[inline(always)]
    pub fn flashnsc_iter(&self) -> impl Iterator<Item = &Flashnsc> {
        self.flashnsc.iter()
    }
    #[doc = "0x540..0x550 - Unspecified"]
    #[inline(always)]
    pub const fn ramnsc(&self, n: usize) -> &Ramnsc {
        &self.ramnsc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x550 - Unspecified"]
    #[inline(always)]
    pub fn ramnsc_iter(&self) -> impl Iterator<Item = &Ramnsc> {
        self.ramnsc.iter()
    }
    #[doc = "0x600..0x680 - Unspecified"]
    #[inline(always)]
    pub const fn flashregion(&self, n: usize) -> &Flashregion {
        &self.flashregion[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x680 - Unspecified"]
    #[inline(always)]
    pub fn flashregion_iter(&self) -> impl Iterator<Item = &Flashregion> {
        self.flashregion.iter()
    }
    #[doc = "0x700..0x780 - Unspecified"]
    #[inline(always)]
    pub const fn ramregion(&self, n: usize) -> &Ramregion {
        &self.ramregion[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700..0x780 - Unspecified"]
    #[inline(always)]
    pub fn ramregion_iter(&self) -> impl Iterator<Item = &Ramregion> {
        self.ramregion.iter()
    }
    #[doc = "0x800..0x90c - Unspecified"]
    #[inline(always)]
    pub const fn periphid(&self, n: usize) -> &Periphid {
        &self.periphid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x90c - Unspecified"]
    #[inline(always)]
    pub fn periphid_iter(&self) -> impl Iterator<Item = &Periphid> {
        self.periphid.iter()
    }
}
#[doc = "EVENTS_RAMACCERR (rw) register accessor: A security violation has been detected for the RAM memory space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_ramaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_ramaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ramaccerr`]
module"]
#[doc(alias = "EVENTS_RAMACCERR")]
pub type EventsRamaccerr = crate::Reg<events_ramaccerr::EventsRamaccerrSpec>;
#[doc = "A security violation has been detected for the RAM memory space"]
pub mod events_ramaccerr;
#[doc = "EVENTS_FLASHACCERR (rw) register accessor: A security violation has been detected for the flash memory space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_flashaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_flashaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_flashaccerr`]
module"]
#[doc(alias = "EVENTS_FLASHACCERR")]
pub type EventsFlashaccerr = crate::Reg<events_flashaccerr::EventsFlashaccerrSpec>;
#[doc = "A security violation has been detected for the flash memory space"]
pub mod events_flashaccerr;
#[doc = "EVENTS_PERIPHACCERR (rw) register accessor: A security violation has been detected on one or several peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_periphaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_periphaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_periphaccerr`]
module"]
#[doc(alias = "EVENTS_PERIPHACCERR")]
pub type EventsPeriphaccerr = crate::Reg<events_periphaccerr::EventsPeriphaccerrSpec>;
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
#[doc = "PUBLISH_RAMACCERR (rw) register accessor: Publish configuration for event RAMACCERR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_ramaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_ramaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ramaccerr`]
module"]
#[doc(alias = "PUBLISH_RAMACCERR")]
pub type PublishRamaccerr = crate::Reg<publish_ramaccerr::PublishRamaccerrSpec>;
#[doc = "Publish configuration for event RAMACCERR"]
pub mod publish_ramaccerr;
#[doc = "PUBLISH_FLASHACCERR (rw) register accessor: Publish configuration for event FLASHACCERR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_flashaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_flashaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_flashaccerr`]
module"]
#[doc(alias = "PUBLISH_FLASHACCERR")]
pub type PublishFlashaccerr = crate::Reg<publish_flashaccerr::PublishFlashaccerrSpec>;
#[doc = "Publish configuration for event FLASHACCERR"]
pub mod publish_flashaccerr;
#[doc = "PUBLISH_PERIPHACCERR (rw) register accessor: Publish configuration for event PERIPHACCERR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_periphaccerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_periphaccerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_periphaccerr`]
module"]
#[doc(alias = "PUBLISH_PERIPHACCERR")]
pub type PublishPeriphaccerr = crate::Reg<publish_periphaccerr::PublishPeriphaccerrSpec>;
#[doc = "Publish configuration for event PERIPHACCERR"]
pub mod publish_periphaccerr;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CAP (r) register accessor: Show implemented features for the current device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "CAP")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Show implemented features for the current device"]
pub mod cap;
#[doc = "Unspecified"]
pub use self::extdomain::Extdomain;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod extdomain;
#[doc = "Unspecified"]
pub use self::dppi::Dppi;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dppi;
#[doc = "Unspecified"]
pub use self::gpioport::Gpioport;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod gpioport;
#[doc = "Unspecified"]
pub use self::flashnsc::Flashnsc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod flashnsc;
#[doc = "Unspecified"]
pub use self::ramnsc::Ramnsc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ramnsc;
#[doc = "Unspecified"]
pub use self::flashregion::Flashregion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod flashregion;
#[doc = "Unspecified"]
pub use self::ramregion::Ramregion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ramregion;
#[doc = "Unspecified"]
pub use self::periphid::Periphid;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod periphid;
