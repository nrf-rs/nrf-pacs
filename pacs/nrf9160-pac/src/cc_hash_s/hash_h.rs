#[doc = "Register `HASH_H[%s]` reader"]
pub type R = crate::R<HashHSpec>;
#[doc = "Register `HASH_H[%s]` writer"]
pub type W = crate::W<HashHSpec>;
#[doc = "Field `VALUE` reader - Write the initial hash value before start of digest operation, and read the final hash value result after the digest operation has been completed."]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Write the initial hash value before start of digest operation, and read the final hash value result after the digest operation has been completed."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write the initial hash value before start of digest operation, and read the final hash value result after the digest operation has been completed."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write the initial hash value before start of digest operation, and read the final hash value result after the digest operation has been completed."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, HashHSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Description collection: HASH_H value registers. The initial HASH_H\\[0\\] register holds the least significant bits \\[31:0\\] of the value.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHSpec;
impl crate::RegisterSpec for HashHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_h::R`](R) reader structure"]
impl crate::Readable for HashHSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_h::W`](W) writer structure"]
impl crate::Writable for HashHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_H[%s] to value 0"]
impl crate::Resettable for HashHSpec {}
