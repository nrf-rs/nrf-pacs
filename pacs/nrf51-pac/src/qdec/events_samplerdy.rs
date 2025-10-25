#[doc = "Register `EVENTS_SAMPLERDY` reader"]
pub type R = crate::R<EventsSamplerdySpec>;
#[doc = "Register `EVENTS_SAMPLERDY` writer"]
pub type W = crate::W<EventsSamplerdySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "A new sample is written to the sample register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_samplerdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_samplerdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSamplerdySpec;
impl crate::RegisterSpec for EventsSamplerdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_samplerdy::R`](R) reader structure"]
impl crate::Readable for EventsSamplerdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_samplerdy::W`](W) writer structure"]
impl crate::Writable for EventsSamplerdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_SAMPLERDY to value 0"]
impl crate::Resettable for EventsSamplerdySpec {}
