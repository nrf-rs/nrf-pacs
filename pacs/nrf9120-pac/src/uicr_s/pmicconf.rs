#[doc = "Register `PMICCONF` reader"]
pub type R = crate::R<PmicconfSpec>;
#[doc = "Register `PMICCONF` writer"]
pub type W = crate::W<PmicconfSpec>;
#[doc = "Polarity of PMIC_FPWM signal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmicfpwmpol {
    #[doc = "0: PMIC_FPWM output signal is active-low"]
    ActiveLow = 0,
    #[doc = "1: PMIC_FPWM output signal is active-high"]
    ActiveHigh = 1,
}
impl From<Pmicfpwmpol> for bool {
    #[inline(always)]
    fn from(variant: Pmicfpwmpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMICFPWMPOL` reader - Polarity of PMIC_FPWM signal."]
pub type PmicfpwmpolR = crate::BitReader<Pmicfpwmpol>;
impl PmicfpwmpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmicfpwmpol {
        match self.bits {
            false => Pmicfpwmpol::ActiveLow,
            true => Pmicfpwmpol::ActiveHigh,
        }
    }
    #[doc = "PMIC_FPWM output signal is active-low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Pmicfpwmpol::ActiveLow
    }
    #[doc = "PMIC_FPWM output signal is active-high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Pmicfpwmpol::ActiveHigh
    }
}
#[doc = "Field `PMICFPWMPOL` writer - Polarity of PMIC_FPWM signal."]
pub type PmicfpwmpolW<'a, REG> = crate::BitWriter<'a, REG, Pmicfpwmpol>;
impl<'a, REG> PmicfpwmpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMIC_FPWM output signal is active-low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Pmicfpwmpol::ActiveLow)
    }
    #[doc = "PMIC_FPWM output signal is active-high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Pmicfpwmpol::ActiveHigh)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of PMIC_FPWM signal."]
    #[inline(always)]
    pub fn pmicfpwmpol(&self) -> PmicfpwmpolR {
        PmicfpwmpolR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of PMIC_FPWM signal."]
    #[inline(always)]
    #[must_use]
    pub fn pmicfpwmpol(&mut self) -> PmicfpwmpolW<PmicconfSpec> {
        PmicfpwmpolW::new(self, 0)
    }
}
#[doc = "Polarity of PMIC polarity configuration signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmicconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmicconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmicconfSpec;
impl crate::RegisterSpec for PmicconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmicconf::R`](R) reader structure"]
impl crate::Readable for PmicconfSpec {}
#[doc = "`write(|w| ..)` method takes [`pmicconf::W`](W) writer structure"]
impl crate::Writable for PmicconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMICCONF to value 0xffff_ffff"]
impl crate::Resettable for PmicconfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
