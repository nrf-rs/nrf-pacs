#[doc = "Register `EVENTS_CRCERROR` reader"]
pub type R = crate::R<EventsCrcerrorSpec>;
#[doc = "Register `EVENTS_CRCERROR` writer"]
pub type W = crate::W<EventsCrcerrorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Packet received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrcerrorSpec;
impl crate::RegisterSpec for EventsCrcerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_crcerror::R`](R) reader structure"]
impl crate::Readable for EventsCrcerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_crcerror::W`](W) writer structure"]
impl crate::Writable for EventsCrcerrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CRCERROR to value 0"]
impl crate::Resettable for EventsCrcerrorSpec {}
