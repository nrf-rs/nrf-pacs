#[doc = "Register `CHACHA_IV[%s]` reader"]
pub type R = crate::R<ChachaIvSpec>;
#[doc = "Register `CHACHA_IV[%s]` writer"]
pub type W = crate::W<ChachaIvSpec>;
#[doc = "Field `VALUE` reader - CHACHA IV value."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - CHACHA IV value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CHACHA IV value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHACHA IV value."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ChachaIvSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: CHACHA Initialization Vector (IV) to use. The IV is also known as the nonce.\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaIvSpec;
impl crate::RegisterSpec for ChachaIvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_iv::R`](R) reader structure"]
impl crate::Readable for ChachaIvSpec {}
#[doc = "`write(|w| ..)` method takes [`chacha_iv::W`](W) writer structure"]
impl crate::Writable for ChachaIvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHACHA_IV[%s] to value 0"]
impl crate::Resettable for ChachaIvSpec {}
