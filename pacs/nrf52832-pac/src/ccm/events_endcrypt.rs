#[doc = "Register `EVENTS_ENDCRYPT` reader"]
pub type R = crate::R<EventsEndcryptSpec>;
#[doc = "Register `EVENTS_ENDCRYPT` writer"]
pub type W = crate::W<EventsEndcryptSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Encrypt/decrypt complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_endcrypt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_endcrypt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEndcryptSpec;
impl crate::RegisterSpec for EventsEndcryptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_endcrypt::R`](R) reader structure"]
impl crate::Readable for EventsEndcryptSpec {}
#[doc = "`write(|w| ..)` method takes [`events_endcrypt::W`](W) writer structure"]
impl crate::Writable for EventsEndcryptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_ENDCRYPT to value 0"]
impl crate::Resettable for EventsEndcryptSpec {}
