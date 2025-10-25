#[doc = "Register `AES_CTR[%s]` reader"]
pub type R = crate::R<AesCtrSpec>;
#[doc = "Register `AES_CTR[%s]` writer"]
pub type W = crate::W<AesCtrSpec>;
#[doc = "Field `VALUE` reader - AES CTR value."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - AES CTR value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES CTR value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES CTR value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, AesCtrSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: AES counter (CTR) to use. The initial AES_CTR\\[0\\] register holds the least significant bits \\[31:0\\] of the CTR.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtrSpec;
impl crate::RegisterSpec for AesCtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ctr::R`](R) reader structure"]
impl crate::Readable for AesCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctr::W`](W) writer structure"]
impl crate::Writable for AesCtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CTR[%s] to value 0"]
impl crate::Resettable for AesCtrSpec {}
