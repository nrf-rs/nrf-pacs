#[doc = "Register `CLKCONFIG` reader"]
pub struct R(crate::R<CLKCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCONFIG` writer"]
pub struct W(crate::W<CLKCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCONFIG_SPEC>;
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
impl From<crate::W<CLKCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: 32MHz peripheral clock"]
    PCLK32M = 0,
    #[doc = "1: Audio PLL clock"]
    ACLK = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Clock source selection"]
pub struct CLKSRC_R(crate::FieldReader<bool, CLKSRC_A>);
impl CLKSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::PCLK32M,
            true => CLKSRC_A::ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK32M`"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        **self == CLKSRC_A::PCLK32M
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        **self == CLKSRC_A::ACLK
    }
}
impl core::ops::Deref for CLKSRC_R {
    type Target = crate::FieldReader<bool, CLKSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSRC` writer - Clock source selection"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut W {
        self.variant(CLKSRC_A::PCLK32M)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(CLKSRC_A::ACLK)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable bypass"]
    DISABLE = 0,
    #[doc = "1: Enable bypass"]
    ENABLE = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
pub struct BYPASS_R(crate::FieldReader<bool, BYPASS_A>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLE,
            true => BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BYPASS_A::ENABLE
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, BYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable bypass"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLE)
    }
    #[doc = "Enable bypass"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLE)
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
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source selection for the I2S module\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkconfig](index.html) module"]
pub struct CLKCONFIG_SPEC;
impl crate::RegisterSpec for CLKCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkconfig::R](R) reader structure"]
impl crate::Readable for CLKCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkconfig::W](W) writer structure"]
impl crate::Writable for CLKCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCONFIG to value 0"]
impl crate::Resettable for CLKCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
