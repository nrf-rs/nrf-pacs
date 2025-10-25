#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "FRAMESTATUS")]
pub struct Framestatus {
    rx: Rx,
}
impl Framestatus {
    #[doc = "0x00 - Result of last incoming frame"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
}
#[doc = "RX (rw) register accessor: Result of last incoming frame\n\nYou can [`read`](crate::Reg::read) this register and get [`rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`] module"]
#[doc(alias = "RX")]
pub type Rx = crate::Reg<rx::RxSpec>;
#[doc = "Result of last incoming frame"]
pub mod rx;
