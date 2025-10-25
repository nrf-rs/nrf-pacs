#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    clk: Clk,
    din: Din,
}
impl Psel {
    #[doc = "0x00 - Pin number configuration for PDM CLK signal"]
    #[inline(always)]
    pub const fn clk(&self) -> &Clk {
        &self.clk
    }
    #[doc = "0x04 - Pin number configuration for PDM DIN signal"]
    #[inline(always)]
    pub const fn din(&self) -> &Din {
        &self.din
    }
}
#[doc = "CLK (rw) register accessor: Pin number configuration for PDM CLK signal\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
#[doc(alias = "CLK")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "Pin number configuration for PDM CLK signal"]
pub mod clk;
#[doc = "DIN (rw) register accessor: Pin number configuration for PDM DIN signal\n\nYou can [`read`](crate::Reg::read) this register and get [`din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din`] module"]
#[doc(alias = "DIN")]
pub type Din = crate::Reg<din::DinSpec>;
#[doc = "Pin number configuration for PDM DIN signal"]
pub mod din;
