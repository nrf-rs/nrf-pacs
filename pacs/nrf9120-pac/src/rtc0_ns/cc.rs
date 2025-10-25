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
    pub fn compare(&mut self) -> CompareW<'_, CcSpec> {
        CompareW::new(self, 0)
    }
}
#[doc = "Description collection: Compare register n\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC[%s] to value 0"]
impl crate::Resettable for CcSpec {}
