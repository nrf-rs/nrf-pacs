#[doc = "Register `EVENTS_DOWN` reader"]
pub type R = crate::R<EventsDownSpec>;
#[doc = "Register `EVENTS_DOWN` writer"]
pub type W = crate::W<EventsDownSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input voltage crossed the threshold going down.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_down::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_down::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDownSpec;
impl crate::RegisterSpec for EventsDownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_down::R`](R) reader structure"]
impl crate::Readable for EventsDownSpec {}
#[doc = "`write(|w| ..)` method takes [`events_down::W`](W) writer structure"]
impl crate::Writable for EventsDownSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DOWN to value 0"]
impl crate::Resettable for EventsDownSpec {}
