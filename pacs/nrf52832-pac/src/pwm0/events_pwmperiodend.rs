#[doc = "Register `EVENTS_PWMPERIODEND` reader"]
pub type R = crate::R<EventsPwmperiodendSpec>;
#[doc = "Register `EVENTS_PWMPERIODEND` writer"]
pub type W = crate::W<EventsPwmperiodendSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Emitted at the end of each PWM period\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pwmperiodend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pwmperiodend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPwmperiodendSpec;
impl crate::RegisterSpec for EventsPwmperiodendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pwmperiodend::R`](R) reader structure"]
impl crate::Readable for EventsPwmperiodendSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pwmperiodend::W`](W) writer structure"]
impl crate::Writable for EventsPwmperiodendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PWMPERIODEND to value 0"]
impl crate::Resettable for EventsPwmperiodendSpec {}
