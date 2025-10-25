#[doc = "Register `AES_IV_0[%s]` reader"]
pub type R = crate::R<AesIv0Spec>;
#[doc = "Register `AES_IV_0[%s]` writer"]
pub type W = crate::W<AesIv0Spec>;
#[doc = "Field `VALUE` reader - AES non-tunneling or first tunnel stage IV value."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - AES non-tunneling or first tunnel stage IV value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES non-tunneling or first tunnel stage IV value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES non-tunneling or first tunnel stage IV value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, AesIv0Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: AES Initialization Vector (IV) to use. The initial AES_IV_0\\[0\\] register holds the least significant bits \\[31:0\\] of the IV.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_iv_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_iv_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIv0Spec;
impl crate::RegisterSpec for AesIv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_iv_0::R`](R) reader structure"]
impl crate::Readable for AesIv0Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_iv_0::W`](W) writer structure"]
impl crate::Writable for AesIv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_IV_0[%s] to value 0"]
impl crate::Resettable for AesIv0Spec {}
