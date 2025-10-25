#[doc = "Register `EVENTS_FIELDLOST` reader"]
pub type R = crate::R<EventsFieldlostSpec>;
#[doc = "Register `EVENTS_FIELDLOST` writer"]
pub type W = crate::W<EventsFieldlostSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Remote NFC field lost\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fieldlost::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fieldlost::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFieldlostSpec;
impl crate::RegisterSpec for EventsFieldlostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_fieldlost::R`](R) reader structure"]
impl crate::Readable for EventsFieldlostSpec {}
#[doc = "`write(|w| ..)` method takes [`events_fieldlost::W`](W) writer structure"]
impl crate::Writable for EventsFieldlostSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_FIELDLOST to value 0"]
impl crate::Resettable for EventsFieldlostSpec {}
