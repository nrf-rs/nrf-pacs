#[doc = "Register `EVENTS_CALIBRATEDONE` reader"]
pub type R = crate::R<EventsCalibratedoneSpec>;
#[doc = "Register `EVENTS_CALIBRATEDONE` writer"]
pub type W = crate::W<EventsCalibratedoneSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Calibration is complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_calibratedone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_calibratedone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCalibratedoneSpec;
impl crate::RegisterSpec for EventsCalibratedoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_calibratedone::R`](R) reader structure"]
impl crate::Readable for EventsCalibratedoneSpec {}
#[doc = "`write(|w| ..)` method takes [`events_calibratedone::W`](W) writer structure"]
impl crate::Writable for EventsCalibratedoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CALIBRATEDONE to value 0"]
impl crate::Resettable for EventsCalibratedoneSpec {}
