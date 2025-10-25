#[doc = "Register `EVENTS_UP` reader"]
pub type R = crate::R<EventsUpSpec>;
#[doc = "Register `EVENTS_UP` writer"]
pub type W = crate::W<EventsUpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Upward crossing\n\nYou can [`read`](crate::Reg::read) this register and get [`events_up::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_up::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsUpSpec;
impl crate::RegisterSpec for EventsUpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_up::R`](R) reader structure"]
impl crate::Readable for EventsUpSpec {}
#[doc = "`write(|w| ..)` method takes [`events_up::W`](W) writer structure"]
impl crate::Writable for EventsUpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_UP to value 0"]
impl crate::Resettable for EventsUpSpec {}
