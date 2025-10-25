#[doc = "Register `NONCE2` writer"]
pub type W = crate::W<Nonce2Spec>;
#[doc = "Field `NONCE2` writer - Bits 95:64 of XIP NONCE"]
pub type Nonce2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits 95:64 of XIP NONCE"]
    #[inline(always)]
    pub fn nonce2(&mut self) -> Nonce2W<'_, Nonce2Spec> {
        Nonce2W::new(self, 0)
    }
}
#[doc = "Bits 95:64 of XIP NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nonce2Spec;
impl crate::RegisterSpec for Nonce2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nonce2::W`](W) writer structure"]
impl crate::Writable for Nonce2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NONCE2 to value 0"]
impl crate::Resettable for Nonce2Spec {}
