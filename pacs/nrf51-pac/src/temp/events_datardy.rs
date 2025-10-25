#[doc = "Register `EVENTS_DATARDY` reader"]
pub type R = crate::R<EventsDatardySpec>;
#[doc = "Register `EVENTS_DATARDY` writer"]
pub type W = crate::W<EventsDatardySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Temperature measurement complete, data ready event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_datardy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_datardy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDatardySpec;
impl crate::RegisterSpec for EventsDatardySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_datardy::R`](R) reader structure"]
impl crate::Readable for EventsDatardySpec {}
#[doc = "`write(|w| ..)` method takes [`events_datardy::W`](W) writer structure"]
impl crate::Writable for EventsDatardySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DATARDY to value 0"]
impl crate::Resettable for EventsDatardySpec {}
