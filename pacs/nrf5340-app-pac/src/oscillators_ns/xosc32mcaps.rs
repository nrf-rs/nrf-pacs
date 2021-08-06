#[doc = "Register `XOSC32MCAPS` reader"]
pub struct R(crate::R<XOSC32MCAPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32MCAPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32MCAPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32MCAPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32MCAPS` writer"]
pub struct W(crate::W<XOSC32MCAPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32MCAPS_SPEC>;
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
impl From<crate::W<XOSC32MCAPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32MCAPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPVALUE` reader - Value representing capacitance, calculated using provided equation"]
pub struct CAPVALUE_R(crate::FieldReader<u8, u8>);
impl CAPVALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPVALUE` writer - Value representing capacitance, calculated using provided equation"]
pub struct CAPVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Enable on-chip capacitors on XC1 and XC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Capacitor disabled (use external caps)"]
    DISABLED = 0,
    #[doc = "1: Capacitor enabled"]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable on-chip capacitors on XC1 and XC2"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable on-chip capacitors on XC1 and XC2"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capacitor disabled (use external caps)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Capacitor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&self) -> CAPVALUE_R {
        CAPVALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&mut self) -> CAPVALUE_W {
        CAPVALUE_W { w: self }
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable capacitance of XC1 and XC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32mcaps](index.html) module"]
pub struct XOSC32MCAPS_SPEC;
impl crate::RegisterSpec for XOSC32MCAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xosc32mcaps::R](R) reader structure"]
impl crate::Readable for XOSC32MCAPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32mcaps::W](W) writer structure"]
impl crate::Writable for XOSC32MCAPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSC32MCAPS to value 0"]
impl crate::Resettable for XOSC32MCAPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
