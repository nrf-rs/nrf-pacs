#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CONFIG")]
pub struct Config {
    mode: Mode,
    rxen: Rxen,
    txen: Txen,
    mcken: Mcken,
    mckfreq: Mckfreq,
    ratio: Ratio,
    swidth: Swidth,
    align: Align,
    format: Format,
    channels: Channels,
    clkconfig: Clkconfig,
}
impl Config {
    #[doc = "0x00 - I2S mode"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - Reception (RX) enable"]
    #[inline(always)]
    pub const fn rxen(&self) -> &Rxen {
        &self.rxen
    }
    #[doc = "0x08 - Transmission (TX) enable"]
    #[inline(always)]
    pub const fn txen(&self) -> &Txen {
        &self.txen
    }
    #[doc = "0x0c - Master clock generator enable"]
    #[inline(always)]
    pub const fn mcken(&self) -> &Mcken {
        &self.mcken
    }
    #[doc = "0x10 - I2S clock generator control"]
    #[inline(always)]
    pub const fn mckfreq(&self) -> &Mckfreq {
        &self.mckfreq
    }
    #[doc = "0x14 - MCK / LRCK ratio"]
    #[inline(always)]
    pub const fn ratio(&self) -> &Ratio {
        &self.ratio
    }
    #[doc = "0x18 - Sample width"]
    #[inline(always)]
    pub const fn swidth(&self) -> &Swidth {
        &self.swidth
    }
    #[doc = "0x1c - Alignment of sample within a frame"]
    #[inline(always)]
    pub const fn align(&self) -> &Align {
        &self.align
    }
    #[doc = "0x20 - Frame format"]
    #[inline(always)]
    pub const fn format(&self) -> &Format {
        &self.format
    }
    #[doc = "0x24 - Enable channels"]
    #[inline(always)]
    pub const fn channels(&self) -> &Channels {
        &self.channels
    }
    #[doc = "0x28 - Clock source selection for the I2S module"]
    #[inline(always)]
    pub const fn clkconfig(&self) -> &Clkconfig {
        &self.clkconfig
    }
}
#[doc = "MODE (rw) register accessor: I2S mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "I2S mode"]
pub mod mode;
#[doc = "RXEN (rw) register accessor: Reception (RX) enable\n\nYou can [`read`](crate::Reg::read) this register and get [`rxen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxen`] module"]
#[doc(alias = "RXEN")]
pub type Rxen = crate::Reg<rxen::RxenSpec>;
#[doc = "Reception (RX) enable"]
pub mod rxen;
#[doc = "TXEN (rw) register accessor: Transmission (TX) enable\n\nYou can [`read`](crate::Reg::read) this register and get [`txen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txen`] module"]
#[doc(alias = "TXEN")]
pub type Txen = crate::Reg<txen::TxenSpec>;
#[doc = "Transmission (TX) enable"]
pub mod txen;
#[doc = "MCKEN (rw) register accessor: Master clock generator enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcken`] module"]
#[doc(alias = "MCKEN")]
pub type Mcken = crate::Reg<mcken::MckenSpec>;
#[doc = "Master clock generator enable"]
pub mod mcken;
#[doc = "MCKFREQ (rw) register accessor: I2S clock generator control\n\nYou can [`read`](crate::Reg::read) this register and get [`mckfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mckfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mckfreq`] module"]
#[doc(alias = "MCKFREQ")]
pub type Mckfreq = crate::Reg<mckfreq::MckfreqSpec>;
#[doc = "I2S clock generator control"]
pub mod mckfreq;
#[doc = "RATIO (rw) register accessor: MCK / LRCK ratio\n\nYou can [`read`](crate::Reg::read) this register and get [`ratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ratio`] module"]
#[doc(alias = "RATIO")]
pub type Ratio = crate::Reg<ratio::RatioSpec>;
#[doc = "MCK / LRCK ratio"]
pub mod ratio;
#[doc = "SWIDTH (rw) register accessor: Sample width\n\nYou can [`read`](crate::Reg::read) this register and get [`swidth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swidth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swidth`] module"]
#[doc(alias = "SWIDTH")]
pub type Swidth = crate::Reg<swidth::SwidthSpec>;
#[doc = "Sample width"]
pub mod swidth;
#[doc = "ALIGN (rw) register accessor: Alignment of sample within a frame\n\nYou can [`read`](crate::Reg::read) this register and get [`align::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`align::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@align`] module"]
#[doc(alias = "ALIGN")]
pub type Align = crate::Reg<align::AlignSpec>;
#[doc = "Alignment of sample within a frame"]
pub mod align;
#[doc = "FORMAT (rw) register accessor: Frame format\n\nYou can [`read`](crate::Reg::read) this register and get [`format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@format`] module"]
#[doc(alias = "FORMAT")]
pub type Format = crate::Reg<format::FormatSpec>;
#[doc = "Frame format"]
pub mod format;
#[doc = "CHANNELS (rw) register accessor: Enable channels\n\nYou can [`read`](crate::Reg::read) this register and get [`channels::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channels::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channels`] module"]
#[doc(alias = "CHANNELS")]
pub type Channels = crate::Reg<channels::ChannelsSpec>;
#[doc = "Enable channels"]
pub mod channels;
#[doc = "CLKCONFIG (rw) register accessor: Clock source selection for the I2S module\n\nYou can [`read`](crate::Reg::read) this register and get [`clkconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkconfig`] module"]
#[doc(alias = "CLKCONFIG")]
pub type Clkconfig = crate::Reg<clkconfig::ClkconfigSpec>;
#[doc = "Clock source selection for the I2S module"]
pub mod clkconfig;
