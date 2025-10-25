#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "READ")]
pub struct Read {
    src: Src,
    dst: Dst,
    cnt: Cnt,
}
impl Read {
    #[doc = "0x00 - Flash memory source address"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x04 - RAM destination address"]
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    #[doc = "0x08 - Read transfer length"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
#[doc = "SRC (rw) register accessor: Flash memory source address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`] module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Flash memory source address"]
pub mod src;
#[doc = "DST (rw) register accessor: RAM destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`] module"]
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DstSpec>;
#[doc = "RAM destination address"]
pub mod dst;
#[doc = "CNT (rw) register accessor: Read transfer length\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Read transfer length"]
pub mod cnt;
