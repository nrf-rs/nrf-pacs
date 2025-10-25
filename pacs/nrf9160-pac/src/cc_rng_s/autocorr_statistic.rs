#[doc = "Register `AUTOCORR_STATISTIC` reader"]
pub type R = crate::R<AutocorrStatisticSpec>;
#[doc = "Register `AUTOCORR_STATISTIC` writer"]
pub type W = crate::W<AutocorrStatisticSpec>;
#[doc = "Field `AUTOCORR_TRYS` reader - Count each time an autocorrelation test starts. Any write to the field resets the counter."]
pub type AutocorrTrysR = crate::FieldReader<u16>;
#[doc = "Field `AUTOCORR_TRYS` writer - Count each time an autocorrelation test starts. Any write to the field resets the counter."]
pub type AutocorrTrysW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `AUTOCORR_FAILS` reader - Count each time an autocorrelation test fails. Any write to the field resets the counter."]
pub type AutocorrFailsR = crate::FieldReader;
#[doc = "Field `AUTOCORR_FAILS` writer - Count each time an autocorrelation test fails. Any write to the field resets the counter."]
pub type AutocorrFailsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the field resets the counter."]
    #[inline(always)]
    pub fn autocorr_trys(&self) -> AutocorrTrysR {
        AutocorrTrysR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the field resets the counter."]
    #[inline(always)]
    pub fn autocorr_fails(&self) -> AutocorrFailsR {
        AutocorrFailsR::new(((self.bits >> 14) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts. Any write to the field resets the counter."]
    #[inline(always)]
    pub fn autocorr_trys(&mut self) -> AutocorrTrysW<'_, AutocorrStatisticSpec> {
        AutocorrTrysW::new(self, 0)
    }
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails. Any write to the field resets the counter."]
    #[inline(always)]
    pub fn autocorr_fails(&mut self) -> AutocorrFailsW<'_, AutocorrStatisticSpec> {
        AutocorrFailsW::new(self, 14)
    }
}
#[doc = "Statistics counter for autocorrelation test activations. Statistics collection is stopped if one of the counters reach its limit of all ones.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorr_statistic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorr_statistic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutocorrStatisticSpec;
impl crate::RegisterSpec for AutocorrStatisticSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocorr_statistic::R`](R) reader structure"]
impl crate::Readable for AutocorrStatisticSpec {}
#[doc = "`write(|w| ..)` method takes [`autocorr_statistic::W`](W) writer structure"]
impl crate::Writable for AutocorrStatisticSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOCORR_STATISTIC to value 0"]
impl crate::Resettable for AutocorrStatisticSpec {}
