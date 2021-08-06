#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "I2S mode."]
pub mod mode;
#[doc = "RXEN register accessor: an alias for `Reg<RXEN_SPEC>`"]
pub type RXEN = crate::Reg<rxen::RXEN_SPEC>;
#[doc = "Reception (RX) enable."]
pub mod rxen;
#[doc = "TXEN register accessor: an alias for `Reg<TXEN_SPEC>`"]
pub type TXEN = crate::Reg<txen::TXEN_SPEC>;
#[doc = "Transmission (TX) enable."]
pub mod txen;
#[doc = "MCKEN register accessor: an alias for `Reg<MCKEN_SPEC>`"]
pub type MCKEN = crate::Reg<mcken::MCKEN_SPEC>;
#[doc = "Master clock generator enable."]
pub mod mcken;
#[doc = "MCKFREQ register accessor: an alias for `Reg<MCKFREQ_SPEC>`"]
pub type MCKFREQ = crate::Reg<mckfreq::MCKFREQ_SPEC>;
#[doc = "Master clock generator frequency."]
pub mod mckfreq;
#[doc = "RATIO register accessor: an alias for `Reg<RATIO_SPEC>`"]
pub type RATIO = crate::Reg<ratio::RATIO_SPEC>;
#[doc = "MCK / LRCK ratio."]
pub mod ratio;
#[doc = "SWIDTH register accessor: an alias for `Reg<SWIDTH_SPEC>`"]
pub type SWIDTH = crate::Reg<swidth::SWIDTH_SPEC>;
#[doc = "Sample width."]
pub mod swidth;
#[doc = "ALIGN register accessor: an alias for `Reg<ALIGN_SPEC>`"]
pub type ALIGN = crate::Reg<align::ALIGN_SPEC>;
#[doc = "Alignment of sample within a frame."]
pub mod align;
#[doc = "FORMAT register accessor: an alias for `Reg<FORMAT_SPEC>`"]
pub type FORMAT = crate::Reg<format::FORMAT_SPEC>;
#[doc = "Frame format."]
pub mod format;
#[doc = "CHANNELS register accessor: an alias for `Reg<CHANNELS_SPEC>`"]
pub type CHANNELS = crate::Reg<channels::CHANNELS_SPEC>;
#[doc = "Enable channels."]
pub mod channels;
