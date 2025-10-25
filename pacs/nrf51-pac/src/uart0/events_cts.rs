#[doc = "Register `EVENTS_CTS` reader"]
pub type R = crate::R<EventsCtsSpec>;
#[doc = "Register `EVENTS_CTS` writer"]
pub type W = crate::W<EventsCtsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CTS activated.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCtsSpec;
impl crate::RegisterSpec for EventsCtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_cts::R`](R) reader structure"]
impl crate::Readable for EventsCtsSpec {}
#[doc = "`write(|w| ..)` method takes [`events_cts::W`](W) writer structure"]
impl crate::Writable for EventsCtsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CTS to value 0"]
impl crate::Resettable for EventsCtsSpec {}
