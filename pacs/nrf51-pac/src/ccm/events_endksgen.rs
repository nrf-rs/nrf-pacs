#[doc = "Register `EVENTS_ENDKSGEN` reader"]
pub type R = crate::R<EventsEndksgenSpec>;
#[doc = "Register `EVENTS_ENDKSGEN` writer"]
pub type W = crate::W<EventsEndksgenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keystream generation completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endksgen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endksgen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndksgenSpec;
impl crate::RegisterSpec for EventsEndksgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endksgen::R`](R) reader structure"]
impl crate::Readable for EventsEndksgenSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endksgen::W`](W) writer structure"]
impl crate::Writable for EventsEndksgenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDKSGEN to value 0"]
impl crate::Resettable for EventsEndksgenSpec {}
