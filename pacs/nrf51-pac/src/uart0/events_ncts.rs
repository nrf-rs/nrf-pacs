#[doc = "Register `EVENTS_NCTS` reader"]
pub type R = crate::R<EventsNctsSpec>;
#[doc = "Register `EVENTS_NCTS` writer"]
pub type W = crate::W<EventsNctsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CTS deactivated.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ncts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ncts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsNctsSpec;
impl crate::RegisterSpec for EventsNctsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_ncts::R`](R) reader structure"]
impl crate::Readable for EventsNctsSpec {}
#[doc = "`write(|w| ..)` method takes [`events_ncts::W`](W) writer structure"]
impl crate::Writable for EventsNctsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_NCTS to value 0"]
impl crate::Resettable for EventsNctsSpec {}
