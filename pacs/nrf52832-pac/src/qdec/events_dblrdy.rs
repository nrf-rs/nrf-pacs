#[doc = "Register `EVENTS_DBLRDY` reader"]
pub type R = crate::R<EventsDblrdySpec>;
#[doc = "Register `EVENTS_DBLRDY` writer"]
pub type W = crate::W<EventsDblrdySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Double displacement(s) detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_dblrdy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_dblrdy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsDblrdySpec;
impl crate::RegisterSpec for EventsDblrdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_dblrdy::R`](R) reader structure"]
impl crate::Readable for EventsDblrdySpec {}
#[doc = "`write(|w| ..)` method takes [`events_dblrdy::W`](W) writer structure"]
impl crate::Writable for EventsDblrdySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_DBLRDY to value 0"]
impl crate::Resettable for EventsDblrdySpec {}
