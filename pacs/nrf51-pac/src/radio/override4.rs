#[doc = "Register `OVERRIDE4` reader"]
pub struct R(crate::R<OVERRIDE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERRIDE4` writer"]
pub struct W(crate::W<OVERRIDE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERRIDE4_SPEC>;
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
impl From<crate::W<OVERRIDE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERRIDE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE4` reader - Trim value override 4."]
pub struct OVERRIDE4_R(crate::FieldReader<u32, u32>);
impl OVERRIDE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OVERRIDE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE4` writer - Trim value override 4."]
pub struct OVERRIDE4_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Enable or disable override of default trim values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Override trim values disabled."]
    DISABLED = 0,
    #[doc = "1: Override trim values enabled."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable or disable override of default trim values."]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
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
#[doc = "Field `ENABLE` writer - Enable or disable override of default trim values."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Override trim values disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Override trim values enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&self) -> OVERRIDE4_R {
        OVERRIDE4_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&mut self) -> OVERRIDE4_W {
        OVERRIDE4_W { w: self }
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
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
#[doc = "Trim value override register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [override4](index.html) module"]
pub struct OVERRIDE4_SPEC;
impl crate::RegisterSpec for OVERRIDE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [override4::R](R) reader structure"]
impl crate::Readable for OVERRIDE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [override4::W](W) writer structure"]
impl crate::Writable for OVERRIDE4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERRIDE4 to value 0"]
impl crate::Resettable for OVERRIDE4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
