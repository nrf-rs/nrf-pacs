#[doc = "Register `EVENTS_RSSIEND` reader"]
pub type R = crate::R<EventsRssiendSpec>;
#[doc = "Register `EVENTS_RSSIEND` writer"]
pub type W = crate::W<EventsRssiendSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rssiend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rssiend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsRssiendSpec;
impl crate::RegisterSpec for EventsRssiendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_rssiend::R`](R) reader structure"]
impl crate::Readable for EventsRssiendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_rssiend::W`](W) writer structure"]
impl crate::Writable for EventsRssiendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RSSIEND to value 0"]
impl crate::Resettable for EventsRssiendSpec {}
