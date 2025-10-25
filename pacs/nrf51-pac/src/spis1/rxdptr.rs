#[doc = "Register `RXDPTR` reader"]
pub type R = crate::R<RxdptrSpec>;
#[doc = "Register `RXDPTR` writer"]
pub type W = crate::W<RxdptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX data pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdptrSpec;
impl crate::RegisterSpec for RxdptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdptr::R`](R) reader structure"]
impl crate::Readable for RxdptrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdptr::W`](W) writer structure"]
impl crate::Writable for RxdptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDPTR to value 0"]
impl crate::Resettable for RxdptrSpec {}
