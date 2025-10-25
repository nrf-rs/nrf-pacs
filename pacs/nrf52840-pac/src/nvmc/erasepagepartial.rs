#[doc = "Register `ERASEPAGEPARTIAL` writer"]
pub type W = crate::W<ErasepagepartialSpec>;
#[doc = "Field `ERASEPAGEPARTIAL` writer - Register for starting partial erase of a page in code area"]
pub type ErasepagepartialW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for starting partial erase of a page in code area"]
    #[inline(always)]
    pub fn erasepagepartial(&mut self) -> ErasepagepartialW<'_, ErasepagepartialSpec> {
        ErasepagepartialW::new(self, 0)
    }
}
#[doc = "Register for partial erase of a page in code area\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepagepartial::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErasepagepartialSpec;
impl crate::RegisterSpec for ErasepagepartialSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`erasepagepartial::W`](W) writer structure"]
impl crate::Writable for ErasepagepartialSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPAGEPARTIAL to value 0"]
impl crate::Resettable for ErasepagepartialSpec {}
