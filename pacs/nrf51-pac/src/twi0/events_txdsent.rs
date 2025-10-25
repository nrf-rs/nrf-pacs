#[doc = "Register `EVENTS_TXDSENT` reader"]
pub type R = crate::R<EventsTxdsentSpec>;
#[doc = "Register `EVENTS_TXDSENT` writer"]
pub type W = crate::W<EventsTxdsentSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Two-wire finished sending last TXD byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdsent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdsent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTxdsentSpec;
impl crate::RegisterSpec for EventsTxdsentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_txdsent::R`](R) reader structure"]
impl crate::Readable for EventsTxdsentSpec {}
#[doc = "`write(|w| ..)` method takes [`events_txdsent::W`](W) writer structure"]
impl crate::Writable for EventsTxdsentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_TXDSENT to value 0"]
impl crate::Resettable for EventsTxdsentSpec {}
