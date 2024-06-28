#[doc = "Register `ERASEPAGEPARTIALCFG` reader"]
pub type R = crate::R<ErasepagepartialcfgSpec>;
#[doc = "Register `ERASEPAGEPARTIALCFG` writer"]
pub type W = crate::W<ErasepagepartialcfgSpec>;
#[doc = "Field `DURATION` reader - Duration of the partial erase in milliseconds"]
pub type DurationR = crate::FieldReader;
#[doc = "Field `DURATION` writer - Duration of the partial erase in milliseconds"]
pub type DurationW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    pub fn duration(&self) -> DurationR {
        DurationR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Duration of the partial erase in milliseconds"]
    #[inline(always)]
    #[must_use]
    pub fn duration(&mut self) -> DurationW<ErasepagepartialcfgSpec> {
        DurationW::new(self, 0)
    }
}
#[doc = "Register for partial erase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erasepagepartialcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`erasepagepartialcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErasepagepartialcfgSpec;
impl crate::RegisterSpec for ErasepagepartialcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erasepagepartialcfg::R`](R) reader structure"]
impl crate::Readable for ErasepagepartialcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`erasepagepartialcfg::W`](W) writer structure"]
impl crate::Writable for ErasepagepartialcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERASEPAGEPARTIALCFG to value 0x0a"]
impl crate::Resettable for ErasepagepartialcfgSpec {
    const RESET_VALUE: u32 = 0x0a;
}
