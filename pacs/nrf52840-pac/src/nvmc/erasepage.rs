#[doc = "Register `ERASEPAGE` writer"]
pub type W = crate::W<ErasepageSpec>;
#[doc = "Field `ERASEPAGE` writer - Register for starting erase of a page in code area"]
pub type ErasepageW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in code area"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ErasepageW<'_, ErasepageSpec> {
        ErasepageW::new(self, 0)
    }
}
#[doc = "Register for erasing a page in code area\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erasepage::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErasepageSpec;
impl crate::RegisterSpec for ErasepageSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`erasepage::W`](W) writer structure"]
impl crate::Writable for ErasepageSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERASEPAGE to value 0"]
impl crate::Resettable for ErasepageSpec {}
