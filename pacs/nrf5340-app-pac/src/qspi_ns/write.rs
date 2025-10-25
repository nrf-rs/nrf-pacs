#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "WRITE")]
pub struct Write {
    dst: Dst,
    src: Src,
    cnt: Cnt,
}
impl Write {
    #[doc = "0x00 - Flash destination address"]
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    #[doc = "0x04 - RAM source address"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x08 - Write transfer length"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
#[doc = "DST (rw) register accessor: Flash destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`] module"]
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DstSpec>;
#[doc = "Flash destination address"]
pub mod dst;
#[doc = "SRC (rw) register accessor: RAM source address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`] module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "RAM source address"]
pub mod src;
#[doc = "CNT (rw) register accessor: Write transfer length\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Write transfer length"]
pub mod cnt;
