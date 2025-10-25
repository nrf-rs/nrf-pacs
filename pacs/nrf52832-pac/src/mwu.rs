#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_region: [EventsRegion; 4],
    _reserved1: [u8; 0x40],
    events_pregion: [EventsPregion; 2],
    _reserved2: [u8; 0x0190],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved5: [u8; 0x14],
    nmien: Nmien,
    nmienset: Nmienset,
    nmienclr: Nmienclr,
    _reserved8: [u8; 0xd4],
    perregion: [Perregion; 2],
    _reserved9: [u8; 0x0100],
    regionen: Regionen,
    regionenset: Regionenset,
    regionenclr: Regionenclr,
    _reserved12: [u8; 0xe4],
    region: (),
    _reserved13: [u8; 0xc0],
    pregion: (),
}
impl RegisterBlock {
    #[doc = "0x100..0x120 - Unspecified"]
    #[inline(always)]
    pub const fn events_region(&self, n: usize) -> &EventsRegion {
        &self.events_region[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Unspecified"]
    #[inline(always)]
    pub fn events_region_iter(&self) -> impl Iterator<Item = &EventsRegion> {
        self.events_region.iter()
    }
    #[doc = "0x160..0x170 - Unspecified"]
    #[inline(always)]
    pub const fn events_pregion(&self, n: usize) -> &EventsPregion {
        &self.events_pregion[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - Unspecified"]
    #[inline(always)]
    pub fn events_pregion_iter(&self) -> impl Iterator<Item = &EventsPregion> {
        self.events_pregion.iter()
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
    #[doc = "0x320 - Enable or disable non-maskable interrupt"]
    #[inline(always)]
    pub const fn nmien(&self) -> &Nmien {
        &self.nmien
    }
    #[doc = "0x324 - Enable non-maskable interrupt"]
    #[inline(always)]
    pub const fn nmienset(&self) -> &Nmienset {
        &self.nmienset
    }
    #[doc = "0x328 - Disable non-maskable interrupt"]
    #[inline(always)]
    pub const fn nmienclr(&self) -> &Nmienclr {
        &self.nmienclr
    }
    #[doc = "0x400..0x410 - Unspecified"]
    #[inline(always)]
    pub const fn perregion(&self, n: usize) -> &Perregion {
        &self.perregion[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x410 - Unspecified"]
    #[inline(always)]
    pub fn perregion_iter(&self) -> impl Iterator<Item = &Perregion> {
        self.perregion.iter()
    }
    #[doc = "0x510 - Enable/disable regions watch"]
    #[inline(always)]
    pub const fn regionen(&self) -> &Regionen {
        &self.regionen
    }
    #[doc = "0x514 - Enable regions watch"]
    #[inline(always)]
    pub const fn regionenset(&self) -> &Regionenset {
        &self.regionenset
    }
    #[doc = "0x518 - Disable regions watch"]
    #[inline(always)]
    pub const fn regionenclr(&self) -> &Regionenclr {
        &self.regionenclr
    }
    #[doc = "0x600..0x620 - Unspecified"]
    #[inline(always)]
    pub const fn region(&self, n: usize) -> &Region {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x620 - Unspecified"]
    #[inline(always)]
    pub fn region_iter(&self) -> impl Iterator<Item = &Region> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x6c0..0x6d8 - Unspecified"]
    #[inline(always)]
    pub const fn pregion(&self, n: usize) -> &Pregion {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1728)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6c0..0x6d8 - Unspecified"]
    #[inline(always)]
    pub fn pregion_iter(&self) -> impl Iterator<Item = &Pregion> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1728)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "Unspecified"]
pub use self::events_region::EventsRegion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod events_region;
#[doc = "Unspecified"]
pub use self::events_pregion::EventsPregion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod events_pregion;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "NMIEN (rw) register accessor: Enable or disable non-maskable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nmien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmien`] module"]
#[doc(alias = "NMIEN")]
pub type Nmien = crate::Reg<nmien::NmienSpec>;
#[doc = "Enable or disable non-maskable interrupt"]
pub mod nmien;
#[doc = "NMIENSET (rw) register accessor: Enable non-maskable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nmienset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmienset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmienset`] module"]
#[doc(alias = "NMIENSET")]
pub type Nmienset = crate::Reg<nmienset::NmiensetSpec>;
#[doc = "Enable non-maskable interrupt"]
pub mod nmienset;
#[doc = "NMIENCLR (rw) register accessor: Disable non-maskable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nmienclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmienclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmienclr`] module"]
#[doc(alias = "NMIENCLR")]
pub type Nmienclr = crate::Reg<nmienclr::NmienclrSpec>;
#[doc = "Disable non-maskable interrupt"]
pub mod nmienclr;
#[doc = "Unspecified"]
pub use self::perregion::Perregion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod perregion;
#[doc = "REGIONEN (rw) register accessor: Enable/disable regions watch\n\nYou can [`read`](crate::Reg::read) this register and get [`regionen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regionen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regionen`] module"]
#[doc(alias = "REGIONEN")]
pub type Regionen = crate::Reg<regionen::RegionenSpec>;
#[doc = "Enable/disable regions watch"]
pub mod regionen;
#[doc = "REGIONENSET (rw) register accessor: Enable regions watch\n\nYou can [`read`](crate::Reg::read) this register and get [`regionenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regionenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regionenset`] module"]
#[doc(alias = "REGIONENSET")]
pub type Regionenset = crate::Reg<regionenset::RegionensetSpec>;
#[doc = "Enable regions watch"]
pub mod regionenset;
#[doc = "REGIONENCLR (rw) register accessor: Disable regions watch\n\nYou can [`read`](crate::Reg::read) this register and get [`regionenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regionenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regionenclr`] module"]
#[doc(alias = "REGIONENCLR")]
pub type Regionenclr = crate::Reg<regionenclr::RegionenclrSpec>;
#[doc = "Disable regions watch"]
pub mod regionenclr;
#[doc = "Unspecified"]
pub use self::region::Region;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod region;
#[doc = "Unspecified"]
pub use self::pregion::Pregion;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod pregion;
