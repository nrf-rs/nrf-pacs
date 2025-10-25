#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0504],
    out: Out,
    outset: Outset,
    outclr: Outclr,
    in_: In,
    dir: Dir,
    dirset: Dirset,
    dirclr: Dirclr,
    latch: Latch,
    detectmode: Detectmode,
    _reserved9: [u8; 0x01d8],
    pin_cnf: [PinCnf; 32],
}
impl RegisterBlock {
    #[doc = "0x504 - Write GPIO port"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x508 - Set individual bits in GPIO port"]
    #[inline(always)]
    pub const fn outset(&self) -> &Outset {
        &self.outset
    }
    #[doc = "0x50c - Clear individual bits in GPIO port"]
    #[inline(always)]
    pub const fn outclr(&self) -> &Outclr {
        &self.outclr
    }
    #[doc = "0x510 - Read GPIO port"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x514 - Direction of GPIO pins"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x518 - DIR set register"]
    #[inline(always)]
    pub const fn dirset(&self) -> &Dirset {
        &self.dirset
    }
    #[doc = "0x51c - DIR clear register"]
    #[inline(always)]
    pub const fn dirclr(&self) -> &Dirclr {
        &self.dirclr
    }
    #[doc = "0x520 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    #[inline(always)]
    pub const fn latch(&self) -> &Latch {
        &self.latch
    }
    #[doc = "0x524 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub const fn detectmode(&self) -> &Detectmode {
        &self.detectmode
    }
    #[doc = "0x700..0x780 - Description collection\\[0\\]: Configuration of GPIO pins"]
    #[inline(always)]
    pub const fn pin_cnf(&self, n: usize) -> &PinCnf {
        &self.pin_cnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700..0x780 - Description collection\\[0\\]: Configuration of GPIO pins"]
    #[inline(always)]
    pub fn pin_cnf_iter(&self) -> impl Iterator<Item = &PinCnf> {
        self.pin_cnf.iter()
    }
}
#[doc = "OUT (rw) register accessor: Write GPIO port\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Write GPIO port"]
pub mod out;
#[doc = "OUTSET (rw) register accessor: Set individual bits in GPIO port\n\nYou can [`read`](crate::Reg::read) this register and get [`outset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outset`] module"]
#[doc(alias = "OUTSET")]
pub type Outset = crate::Reg<outset::OutsetSpec>;
#[doc = "Set individual bits in GPIO port"]
pub mod outset;
#[doc = "OUTCLR (rw) register accessor: Clear individual bits in GPIO port\n\nYou can [`read`](crate::Reg::read) this register and get [`outclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outclr`] module"]
#[doc(alias = "OUTCLR")]
pub type Outclr = crate::Reg<outclr::OutclrSpec>;
#[doc = "Clear individual bits in GPIO port"]
pub mod outclr;
#[doc = "IN (r) register accessor: Read GPIO port\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Read GPIO port"]
pub mod in_;
#[doc = "DIR (rw) register accessor: Direction of GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Direction of GPIO pins"]
pub mod dir;
#[doc = "DIRSET (rw) register accessor: DIR set register\n\nYou can [`read`](crate::Reg::read) this register and get [`dirset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`] module"]
#[doc(alias = "DIRSET")]
pub type Dirset = crate::Reg<dirset::DirsetSpec>;
#[doc = "DIR set register"]
pub mod dirset;
#[doc = "DIRCLR (rw) register accessor: DIR clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`] module"]
#[doc(alias = "DIRCLR")]
pub type Dirclr = crate::Reg<dirclr::DirclrSpec>;
#[doc = "DIR clear register"]
pub mod dirclr;
#[doc = "LATCH (rw) register accessor: Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers\n\nYou can [`read`](crate::Reg::read) this register and get [`latch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@latch`] module"]
#[doc(alias = "LATCH")]
pub type Latch = crate::Reg<latch::LatchSpec>;
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
pub mod latch;
#[doc = "DETECTMODE (rw) register accessor: Select between default DETECT signal behaviour and LDETECT mode\n\nYou can [`read`](crate::Reg::read) this register and get [`detectmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`detectmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@detectmode`] module"]
#[doc(alias = "DETECTMODE")]
pub type Detectmode = crate::Reg<detectmode::DetectmodeSpec>;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
pub mod detectmode;
#[doc = "PIN_CNF (rw) register accessor: Description collection\\[0\\]: Configuration of GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_cnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_cnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin_cnf`] module"]
#[doc(alias = "PIN_CNF")]
pub type PinCnf = crate::Reg<pin_cnf::PinCnfSpec>;
#[doc = "Description collection\\[0\\]: Configuration of GPIO pins"]
pub mod pin_cnf;
