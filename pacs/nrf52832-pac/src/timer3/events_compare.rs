#[doc = "Register `EVENTS_COMPARE[%s]` reader"]
pub type R = crate::R<EventsCompareSpec>;
#[doc = "Register `EVENTS_COMPARE[%s]` writer"]
pub type W = crate::W<EventsCompareSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Description collection\\[0\\]: Compare event on CC\\[0\\] match\n\nYou can [`read`](crate::Reg::read) this register and get [`events_compare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_compare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCompareSpec;
impl crate::RegisterSpec for EventsCompareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_compare::R`](R) reader structure"]
impl crate::Readable for EventsCompareSpec {}
#[doc = "`write(|w| ..)` method takes [`events_compare::W`](W) writer structure"]
impl crate::Writable for EventsCompareSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_COMPARE[%s] to value 0"]
impl crate::Resettable for EventsCompareSpec {}
