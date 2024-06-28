#[doc = "Register `CC[%s]` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC[%s]` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Field `COMPARE` reader - Compare value"]
pub type CompareR = crate::FieldReader<u32>;
#[doc = "Field `COMPARE` writer - Compare value"]
pub type CompareW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare value"]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> CompareW<CcSpec> {
        CompareW::new(self, 0)
    }
}
#[doc = "Description collection: Compare register n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC[%s]
to value 0"]
impl crate::Resettable for CcSpec {
    const RESET_VALUE: u32 = 0;
}
