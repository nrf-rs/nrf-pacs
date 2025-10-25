#[doc = "Register `EVENTS_COLLISION` reader"]
pub type R = crate::R<EventsCollisionSpec>;
#[doc = "Register `EVENTS_COLLISION` writer"]
pub type W = crate::W<EventsCollisionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "NFC Auto collision resolution error reported.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_collision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_collision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCollisionSpec;
impl crate::RegisterSpec for EventsCollisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_collision::R`](R) reader structure"]
impl crate::Readable for EventsCollisionSpec {}
#[doc = "`write(|w| ..)` method takes [`events_collision::W`](W) writer structure"]
impl crate::Writable for EventsCollisionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_COLLISION to value 0"]
impl crate::Resettable for EventsCollisionSpec {}
