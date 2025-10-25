#[doc = "Register `HASH_CUR_LEN_0` reader"]
pub type R = crate::R<HashCurLen0Spec>;
#[doc = "Register `HASH_CUR_LEN_0` writer"]
pub type W = crate::W<HashCurLen0Spec>;
#[doc = "Field `VALUE` reader - Bits \\[31:0\\] of current length of digested data in bytes."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Bits \\[31:0\\] of current length of digested data in bytes."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[31:0\\] of current length of digested data in bytes."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\] of current length of digested data in bytes."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, HashCurLen0Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Bits \\[31:0\\] of the number of bytes that have been digested so far.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_cur_len_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_cur_len_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashCurLen0Spec;
impl crate::RegisterSpec for HashCurLen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_cur_len_0::R`](R) reader structure"]
impl crate::Readable for HashCurLen0Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_cur_len_0::W`](W) writer structure"]
impl crate::Writable for HashCurLen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_CUR_LEN_0 to value 0"]
impl crate::Resettable for HashCurLen0Spec {}
