#[doc = "Register `EVENTS_SELECTED` reader"]
pub type R = crate::R<EventsSelectedSpec>;
#[doc = "Register `EVENTS_SELECTED` writer"]
pub type W = crate::W<EventsSelectedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "NFC Auto collision resolution successfully completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_selected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_selected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSelectedSpec;
impl crate::RegisterSpec for EventsSelectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_selected::R`](R) reader structure"]
impl crate::Readable for EventsSelectedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_selected::W`](W) writer structure"]
impl crate::Writable for EventsSelectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SELECTED to value 0"]
impl crate::Resettable for EventsSelectedSpec {}
