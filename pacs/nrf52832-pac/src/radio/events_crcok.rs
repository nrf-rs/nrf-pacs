#[doc = "Register `EVENTS_CRCOK` reader"]
pub type R = crate::R<EventsCrcokSpec>;
#[doc = "Register `EVENTS_CRCOK` writer"]
pub type W = crate::W<EventsCrcokSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Packet received with CRC ok\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcok::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcok::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCrcokSpec;
impl crate::RegisterSpec for EventsCrcokSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_crcok::R`](R) reader structure"]
impl crate::Readable for EventsCrcokSpec {}
#[doc = "`write(|w| ..)` method takes [`events_crcok::W`](W) writer structure"]
impl crate::Writable for EventsCrcokSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CRCOK to value 0"]
impl crate::Resettable for EventsCrcokSpec {}
