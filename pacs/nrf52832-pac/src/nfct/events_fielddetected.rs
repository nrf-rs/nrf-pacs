#[doc = "Register `EVENTS_FIELDDETECTED` reader"]
pub type R = crate::R<EventsFielddetectedSpec>;
#[doc = "Register `EVENTS_FIELDDETECTED` writer"]
pub type W = crate::W<EventsFielddetectedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Remote NFC field detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fielddetected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fielddetected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFielddetectedSpec;
impl crate::RegisterSpec for EventsFielddetectedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fielddetected::R`](R) reader structure"]
impl crate::Readable for EventsFielddetectedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fielddetected::W`](W) writer structure"]
impl crate::Writable for EventsFielddetectedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FIELDDETECTED to value 0"]
impl crate::Resettable for EventsFielddetectedSpec {}
