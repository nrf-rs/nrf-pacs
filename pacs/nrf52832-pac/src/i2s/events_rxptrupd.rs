#[doc = "Register `EVENTS_RXPTRUPD` reader"]
pub type R = crate::R<EventsRxptrupdSpec>;
#[doc = "Register `EVENTS_RXPTRUPD` writer"]
pub type W = crate::W<EventsRxptrupdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxptrupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxptrupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxptrupdSpec;
impl crate::RegisterSpec for EventsRxptrupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxptrupd::R`](R) reader structure"]
impl crate::Readable for EventsRxptrupdSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxptrupd::W`](W) writer structure"]
impl crate::Writable for EventsRxptrupdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXPTRUPD to value 0"]
impl crate::Resettable for EventsRxptrupdSpec {}
