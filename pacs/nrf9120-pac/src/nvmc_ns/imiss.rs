#[doc = "Register `IMISS` reader"]
pub type R = crate::R<ImissSpec>;
#[doc = "Register `IMISS` writer"]
pub type W = crate::W<ImissSpec>;
#[doc = "Field `MISSES` reader - Number of cache misses Write zero to clear"]
pub type MissesR = crate::FieldReader<u32>;
#[doc = "Field `MISSES` writer - Number of cache misses Write zero to clear"]
pub type MissesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cache misses Write zero to clear"]
    #[inline(always)]
    pub fn misses(&self) -> MissesR {
        MissesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache misses Write zero to clear"]
    #[inline(always)]
    #[must_use]
    pub fn misses(&mut self) -> MissesW<ImissSpec> {
        MissesW::new(self, 0)
    }
}
#[doc = "I-code cache miss counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imiss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imiss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImissSpec;
impl crate::RegisterSpec for ImissSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imiss::R`](R) reader structure"]
impl crate::Readable for ImissSpec {}
#[doc = "`write(|w| ..)` method takes [`imiss::W`](W) writer structure"]
impl crate::Writable for ImissSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMISS to value 0"]
impl crate::Resettable for ImissSpec {
    const RESET_VALUE: u32 = 0;
}
