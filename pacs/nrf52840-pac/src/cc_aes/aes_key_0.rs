#[doc = "Register `AES_KEY_0[%s]` writer"]
pub type W = crate::W<AesKey0Spec>;
#[doc = "Field `VALUE` writer - AES key value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES key value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, AesKey0Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: AES key value to use. The initial AES_KEY_0\\[0\\] register holds the least significant bits \\[31:0\\] of the key value.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_key_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey0Spec;
impl crate::RegisterSpec for AesKey0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_key_0::W`](W) writer structure"]
impl crate::Writable for AesKey0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_KEY_0[%s] to value 0"]
impl crate::Resettable for AesKey0Spec {}
