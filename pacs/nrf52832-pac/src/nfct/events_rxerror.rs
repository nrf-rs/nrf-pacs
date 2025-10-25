#[doc = "Register `EVENTS_RXERROR` reader"]
pub type R = crate::R<EventsRxerrorSpec>;
#[doc = "Register `EVENTS_RXERROR` writer"]
pub type W = crate::W<EventsRxerrorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRxerrorSpec;
impl crate::RegisterSpec for EventsRxerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rxerror::R`](R) reader structure"]
impl crate::Readable for EventsRxerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rxerror::W`](W) writer structure"]
impl crate::Writable for EventsRxerrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RXERROR to value 0"]
impl crate::Resettable for EventsRxerrorSpec {}
