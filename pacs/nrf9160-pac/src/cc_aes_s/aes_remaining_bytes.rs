#[doc = "Register `AES_REMAINING_BYTES` reader"]
pub type R = crate::R<AesRemainingBytesSpec>;
#[doc = "Register `AES_REMAINING_BYTES` writer"]
pub type W = crate::W<AesRemainingBytesSpec>;
#[doc = "Field `VALUE` reader - Remaining bytes util the end of the current AES operation."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Remaining bytes util the end of the current AES operation."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remaining bytes util the end of the current AES operation."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remaining bytes util the end of the current AES operation."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, AesRemainingBytesSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "This register should be set with the amount of remaining bytes until the end of the current AES operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_remaining_bytes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_remaining_bytes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesRemainingBytesSpec;
impl crate::RegisterSpec for AesRemainingBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_remaining_bytes::R`](R) reader structure"]
impl crate::Readable for AesRemainingBytesSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_remaining_bytes::W`](W) writer structure"]
impl crate::Writable for AesRemainingBytesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_REMAINING_BYTES to value 0"]
impl crate::Resettable for AesRemainingBytesSpec {}
