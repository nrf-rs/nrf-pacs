#[doc = "Register `NONCE0` writer"]
pub type W = crate::W<Nonce0Spec>;
#[doc = "Field `NONCE0` writer - Bits 31:0 of XIP NONCE"]
pub type Nonce0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits 31:0 of XIP NONCE"]
    #[inline(always)]
    pub fn nonce0(&mut self) -> Nonce0W<'_, Nonce0Spec> {
        Nonce0W::new(self, 0)
    }
}
#[doc = "Bits 31:0 of XIP NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nonce0Spec;
impl crate::RegisterSpec for Nonce0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nonce0::W`](W) writer structure"]
impl crate::Writable for Nonce0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NONCE0 to value 0"]
impl crate::Resettable for Nonce0Spec {}
