#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    out: [Out; 4],
}
impl Psel {
    #[doc = "0x00..0x10 - Description collection: Output pin select for PWM channel n"]
    #[inline(always)]
    pub const fn out(&self, n: usize) -> &Out {
        &self.out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Description collection: Output pin select for PWM channel n"]
    #[inline(always)]
    pub fn out_iter(&self) -> impl Iterator<Item = &Out> {
        self.out.iter()
    }
}
#[doc = "OUT (rw) register accessor: Description collection: Output pin select for PWM channel n\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Description collection: Output pin select for PWM channel n"]
pub mod out;
