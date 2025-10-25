#[doc = "Register `TXDPTR` reader"]
pub type R = crate::R<TxdptrSpec>;
#[doc = "Register `TXDPTR` writer"]
pub type W = crate::W<TxdptrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TX data pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdptrSpec;
impl crate::RegisterSpec for TxdptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdptr::R`](R) reader structure"]
impl crate::Readable for TxdptrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdptr::W`](W) writer structure"]
impl crate::Writable for TxdptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDPTR to value 0"]
impl crate::Resettable for TxdptrSpec {}
