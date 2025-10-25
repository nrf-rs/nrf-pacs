#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    led: Led,
    a: A,
    b: B,
}
impl Psel {
    #[doc = "0x00 - Pin select for LED signal"]
    #[inline(always)]
    pub const fn led(&self) -> &Led {
        &self.led
    }
    #[doc = "0x04 - Pin select for A signal"]
    #[inline(always)]
    pub const fn a(&self) -> &A {
        &self.a
    }
    #[doc = "0x08 - Pin select for B signal"]
    #[inline(always)]
    pub const fn b(&self) -> &B {
        &self.b
    }
}
#[doc = "LED (rw) register accessor: Pin select for LED signal\n\nYou can [`read`](crate::Reg::read) this register and get [`led::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led`] module"]
#[doc(alias = "LED")]
pub type Led = crate::Reg<led::LedSpec>;
#[doc = "Pin select for LED signal"]
pub mod led;
#[doc = "A (rw) register accessor: Pin select for A signal\n\nYou can [`read`](crate::Reg::read) this register and get [`a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a`] module"]
pub type A = crate::Reg<a::ASpec>;
#[doc = "Pin select for A signal"]
pub mod a;
#[doc = "B (rw) register accessor: Pin select for B signal\n\nYou can [`read`](crate::Reg::read) this register and get [`b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b`] module"]
pub type B = crate::Reg<b::BSpec>;
#[doc = "Pin select for B signal"]
pub mod b;
