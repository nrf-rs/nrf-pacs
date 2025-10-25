#[doc = "Register `NONCE1` writer"]
pub type W = crate::W<Nonce1Spec>;
#[doc = "Field `NONCE1` writer - Bits 63:32 of DMA NONCE"]
pub type Nonce1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of DMA NONCE"]
    #[inline(always)]
    pub fn nonce1(&mut self) -> Nonce1W<'_, Nonce1Spec> {
        Nonce1W::new(self, 0)
    }
}
#[doc = "Bits 63:32 of DMA NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nonce1Spec;
impl crate::RegisterSpec for Nonce1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nonce1::W`](W) writer structure"]
impl crate::Writable for Nonce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NONCE1 to value 0"]
impl crate::Resettable for Nonce1Spec {}
