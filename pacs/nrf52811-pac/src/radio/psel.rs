#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    dfegpio: [Dfegpio; 8],
}
impl Psel {
    #[doc = "0x00..0x20 - Description collection: Pin select for DFE pin n"]
    #[inline(always)]
    pub const fn dfegpio(&self, n: usize) -> &Dfegpio {
        &self.dfegpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: Pin select for DFE pin n"]
    #[inline(always)]
    pub fn dfegpio_iter(&self) -> impl Iterator<Item = &Dfegpio> {
        self.dfegpio.iter()
    }
}
#[doc = "DFEGPIO (rw) register accessor: Description collection: Pin select for DFE pin n\n\nYou can [`read`](crate::Reg::read) this register and get [`dfegpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfegpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfegpio`] module"]
#[doc(alias = "DFEGPIO")]
pub type Dfegpio = crate::Reg<dfegpio::DfegpioSpec>;
#[doc = "Description collection: Pin select for DFE pin n"]
pub mod dfegpio;
