#[doc = "Register `EVENTS_SUSPENDED` reader"]
pub type R = crate::R<EventsSuspendedSpec>;
#[doc = "Register `EVENTS_SUSPENDED` writer"]
pub type W = crate::W<EventsSuspendedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_suspended::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_suspended::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSuspendedSpec;
impl crate::RegisterSpec for EventsSuspendedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_suspended::R`](R) reader structure"]
impl crate::Readable for EventsSuspendedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_suspended::W`](W) writer structure"]
impl crate::Writable for EventsSuspendedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SUSPENDED to value 0"]
impl crate::Resettable for EventsSuspendedSpec {}
