#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0x78],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    _reserved4: [u8; 0x7c],
    events_rxptrupd: EventsRxptrupd,
    events_stopped: EventsStopped,
    _reserved6: [u8; 0x08],
    events_txptrupd: EventsTxptrupd,
    _reserved7: [u8; 0x6c],
    publish_rxptrupd: PublishRxptrupd,
    publish_stopped: PublishStopped,
    _reserved9: [u8; 0x08],
    publish_txptrupd: PublishTxptrupd,
    _reserved10: [u8; 0x0168],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved13: [u8; 0x01f4],
    enable: Enable,
    config: Config,
    _reserved15: [u8; 0x0c],
    rxd: Rxd,
    _reserved16: [u8; 0x04],
    txd: Txd,
    _reserved17: [u8; 0x0c],
    rxtxd: Rxtxd,
    _reserved18: [u8; 0x0c],
    psel: Psel,
}
impl RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x04 - Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x80 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(&self) -> &SubscribeStart {
        &self.subscribe_start
    }
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    #[inline(always)]
    pub const fn events_rxptrupd(&self) -> &EventsRxptrupd {
        &self.events_rxptrupd
    }
    #[doc = "0x108 - I2S transfer stopped."]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub const fn events_txptrupd(&self) -> &EventsTxptrupd {
        &self.events_txptrupd
    }
    #[doc = "0x184 - Publish configuration for event RXPTRUPD"]
    #[inline(always)]
    pub const fn publish_rxptrupd(&self) -> &PublishRxptrupd {
        &self.publish_rxptrupd
    }
    #[doc = "0x188 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x194 - Publish configuration for event TXPTRUPD"]
    #[inline(always)]
    pub const fn publish_txptrupd(&self) -> &PublishTxptrupd {
        &self.publish_txptrupd
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x500 - Enable I2S module."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504..0x52c - Unspecified"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x538 - Unspecified"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x540 - Unspecified"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x550 - Unspecified"]
    #[inline(always)]
    pub const fn rxtxd(&self) -> &Rxtxd {
        &self.rxtxd
    }
    #[doc = "0x560..0x574 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
}
#[doc = "TASKS_START (w) register accessor: Starts continuous I2S transfer. Also starts MCK generator when this is enabled.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the STOPPED event to be generated."]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_start`]
module"]
#[doc(alias = "SUBSCRIBE_START")]
pub type SubscribeStart = crate::Reg<subscribe_start::SubscribeStartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stop`]
module"]
#[doc(alias = "SUBSCRIBE_STOP")]
pub type SubscribeStop = crate::Reg<subscribe_stop::SubscribeStopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_RXPTRUPD (rw) register accessor: The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_rxptrupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_rxptrupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxptrupd`]
module"]
#[doc(alias = "EVENTS_RXPTRUPD")]
pub type EventsRxptrupd = crate::Reg<events_rxptrupd::EventsRxptrupdSpec>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "EVENTS_STOPPED (rw) register accessor: I2S transfer stopped.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "EVENTS_TXPTRUPD (rw) register accessor: The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_txptrupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_txptrupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txptrupd`]
module"]
#[doc(alias = "EVENTS_TXPTRUPD")]
pub type EventsTxptrupd = crate::Reg<events_txptrupd::EventsTxptrupdSpec>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
#[doc = "PUBLISH_RXPTRUPD (rw) register accessor: Publish configuration for event RXPTRUPD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_rxptrupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_rxptrupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxptrupd`]
module"]
#[doc(alias = "PUBLISH_RXPTRUPD")]
pub type PublishRxptrupd = crate::Reg<publish_rxptrupd::PublishRxptrupdSpec>;
#[doc = "Publish configuration for event RXPTRUPD"]
pub mod publish_rxptrupd;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_TXPTRUPD (rw) register accessor: Publish configuration for event TXPTRUPD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_txptrupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_txptrupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txptrupd`]
module"]
#[doc(alias = "PUBLISH_TXPTRUPD")]
pub type PublishTxptrupd = crate::Reg<publish_txptrupd::PublishTxptrupdSpec>;
#[doc = "Publish configuration for event TXPTRUPD"]
pub mod publish_txptrupd;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE (rw) register accessor: Enable I2S module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable I2S module."]
pub mod enable;
#[doc = "Unspecified"]
pub use self::config::Config;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
#[doc = "Unspecified"]
pub use self::rxd::Rxd;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Unspecified"]
pub use self::txd::Txd;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Unspecified"]
pub use self::rxtxd::Rxtxd;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
