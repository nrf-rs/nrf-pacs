#[doc = "Register `TRCCCCTLR` reader"]
pub type R = crate::R<TrcccctlrSpec>;
#[doc = "Register `TRCCCCTLR` writer"]
pub type W = crate::W<TrcccctlrSpec>;
#[doc = "Field `THRESHOLD` reader - Sets the threshold value for instruction trace cycle counting."]
pub type ThresholdR = crate::FieldReader<u16>;
#[doc = "Field `THRESHOLD` writer - Sets the threshold value for instruction trace cycle counting."]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sets the threshold value for instruction trace cycle counting."]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the threshold value for instruction trace cycle counting."]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, TrcccctlrSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "Sets the threshold value for cycle counting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.CCI==1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcccctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcccctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcccctlrSpec;
impl crate::RegisterSpec for TrcccctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcccctlr::R`](R) reader structure"]
impl crate::Readable for TrcccctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcccctlr::W`](W) writer structure"]
impl crate::Writable for TrcccctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCCCCTLR to value 0"]
impl crate::Resettable for TrcccctlrSpec {}
