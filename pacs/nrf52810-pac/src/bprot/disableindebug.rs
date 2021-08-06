#[doc = "Register `DISABLEINDEBUG` reader"]
pub struct R(crate::R<DISABLEINDEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLEINDEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLEINDEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLEINDEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLEINDEBUG` writer"]
pub struct W(crate::W<DISABLEINDEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLEINDEBUG_SPEC>;
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
impl From<crate::W<DISABLEINDEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLEINDEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLEINDEBUG_A {
    #[doc = "1: Disabled in debug"]
    DISABLED = 1,
    #[doc = "0: Enabled in debug"]
    ENABLED = 0,
}
impl From<DISABLEINDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLEINDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLEINDEBUG` reader - Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
pub struct DISABLEINDEBUG_R(crate::FieldReader<bool, DISABLEINDEBUG_A>);
impl DISABLEINDEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISABLEINDEBUG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLEINDEBUG_A {
        match self.bits {
            true => DISABLEINDEBUG_A::DISABLED,
            false => DISABLEINDEBUG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISABLEINDEBUG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISABLEINDEBUG_A::ENABLED
    }
}
impl core::ops::Deref for DISABLEINDEBUG_R {
    type Target = crate::FieldReader<bool, DISABLEINDEBUG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLEINDEBUG` writer - Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
pub struct DISABLEINDEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLEINDEBUG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLEINDEBUG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled in debug"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::DISABLED)
    }
    #[doc = "Enabled in debug"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
    #[inline(always)]
    pub fn disableindebug(&self) -> DISABLEINDEBUG_R {
        DISABLEINDEBUG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug mode. This register will only disable the protection mechanism if the device is in debug mode."]
    #[inline(always)]
    pub fn disableindebug(&mut self) -> DISABLEINDEBUG_W {
        DISABLEINDEBUG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable protection mechanism in debug mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disableindebug](index.html) module"]
pub struct DISABLEINDEBUG_SPEC;
impl crate::RegisterSpec for DISABLEINDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disableindebug::R](R) reader structure"]
impl crate::Readable for DISABLEINDEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disableindebug::W](W) writer structure"]
impl crate::Writable for DISABLEINDEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLEINDEBUG to value 0x01"]
impl crate::Resettable for DISABLEINDEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
