#[doc = "Register `EVENTS_DEVMISS` reader"]
pub type R = crate::R<EventsDevmissSpec>;
#[doc = "Register `EVENTS_DEVMISS` writer"]
pub type W = crate::W<EventsDevmissSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "No device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmiss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmiss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDevmissSpec;
impl crate::RegisterSpec for EventsDevmissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_devmiss::R`](R) reader structure"]
impl crate::Readable for EventsDevmissSpec {}
#[doc = "`write(|w| ..)` method takes [`events_devmiss::W`](W) writer structure"]
impl crate::Writable for EventsDevmissSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DEVMISS to value 0"]
impl crate::Resettable for EventsDevmissSpec {}
