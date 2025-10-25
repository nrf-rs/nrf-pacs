#[doc = "Register `EVENTS_ACCOF` reader"]
pub type R = crate::R<EventsAccofSpec>;
#[doc = "Register `EVENTS_ACCOF` writer"]
pub type W = crate::W<EventsAccofSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ACC or ACCDBL register overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`events_accof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_accof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsAccofSpec;
impl crate::RegisterSpec for EventsAccofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_accof::R`](R) reader structure"]
impl crate::Readable for EventsAccofSpec {}
#[doc = "`write(|w| ..)` method takes [`events_accof::W`](W) writer structure"]
impl crate::Writable for EventsAccofSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ACCOF to value 0"]
impl crate::Resettable for EventsAccofSpec {}
