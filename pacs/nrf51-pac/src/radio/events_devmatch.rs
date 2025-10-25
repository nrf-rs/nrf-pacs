#[doc = "Register `EVENTS_DEVMATCH` reader"]
pub type R = crate::R<EventsDevmatchSpec>;
#[doc = "Register `EVENTS_DEVMATCH` writer"]
pub type W = crate::W<EventsDevmatchSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "A device address match occurred on the last received packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDevmatchSpec;
impl crate::RegisterSpec for EventsDevmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_devmatch::R`](R) reader structure"]
impl crate::Readable for EventsDevmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`events_devmatch::W`](W) writer structure"]
impl crate::Writable for EventsDevmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DEVMATCH to value 0"]
impl crate::Resettable for EventsDevmatchSpec {}
