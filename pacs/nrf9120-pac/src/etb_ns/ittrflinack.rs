#[doc = "Register `ITTRFLINACK` writer"]
pub type W = crate::W<IttrflinackSpec>;
#[doc = "Field `TRIGINACK` writer - Set the value of triginack."]
pub type TriginackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHINACK` writer - Set the value of flushinack."]
pub type FlushinackW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set the value of triginack."]
    #[inline(always)]
    pub fn triginack(&mut self) -> TriginackW<'_, IttrflinackSpec> {
        TriginackW::new(self, 0)
    }
    #[doc = "Bit 1 - Set the value of flushinack."]
    #[inline(always)]
    pub fn flushinack(&mut self) -> FlushinackW<'_, IttrflinackSpec> {
        FlushinackW::new(self, 1)
    }
}
#[doc = "Integration Test Trigger In and Flush In Acknowledge Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrflinack::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrflinackSpec;
impl crate::RegisterSpec for IttrflinackSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ittrflinack::W`](W) writer structure"]
impl crate::Writable for IttrflinackSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITTRFLINACK to value 0"]
impl crate::Resettable for IttrflinackSpec {}
