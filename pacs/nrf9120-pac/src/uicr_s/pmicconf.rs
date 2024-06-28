#[doc = "Register `PMICCONF` reader"]
pub struct R(crate::R<PMICCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMICCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMICCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMICCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMICCONF` writer"]
pub struct W(crate::W<PMICCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMICCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMICCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMICCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMICFPWMPOL` reader - Polarity of PMIC_FPWM signal."]
pub type PMICFPWMPOL_R = crate::BitReader<PMICFPWMPOL_A>;
#[doc = "Polarity of PMIC_FPWM signal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMICFPWMPOL_A {
    #[doc = "0: PMIC_FPWM output signal is active-low"]
    ACTIVE_LOW = 0,
    #[doc = "1: PMIC_FPWM output signal is active-high"]
    ACTIVE_HIGH = 1,
}
impl From<PMICFPWMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PMICFPWMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PMICFPWMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMICFPWMPOL_A {
        match self.bits {
            false => PMICFPWMPOL_A::ACTIVE_LOW,
            true => PMICFPWMPOL_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == PMICFPWMPOL_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == PMICFPWMPOL_A::ACTIVE_HIGH
    }
}
#[doc = "Field `PMICFPWMPOL` writer - Polarity of PMIC_FPWM signal."]
pub type PMICFPWMPOL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PMICCONF_SPEC, PMICFPWMPOL_A, O>;
impl<'a, const O: u8> PMICFPWMPOL_W<'a, O> {
    #[doc = "PMIC_FPWM output signal is active-low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(PMICFPWMPOL_A::ACTIVE_LOW)
    }
    #[doc = "PMIC_FPWM output signal is active-high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(PMICFPWMPOL_A::ACTIVE_HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of PMIC_FPWM signal."]
    #[inline(always)]
    pub fn pmicfpwmpol(&self) -> PMICFPWMPOL_R {
        PMICFPWMPOL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of PMIC_FPWM signal."]
    #[inline(always)]
    pub fn pmicfpwmpol(&mut self) -> PMICFPWMPOL_W<0> {
        PMICFPWMPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polarity of PMIC polarity configuration signals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmicconf](index.html) module"]
pub struct PMICCONF_SPEC;
impl crate::RegisterSpec for PMICCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmicconf::R](R) reader structure"]
impl crate::Readable for PMICCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmicconf::W](W) writer structure"]
impl crate::Writable for PMICCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMICCONF to value 0xffff_ffff"]
impl crate::Resettable for PMICCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
