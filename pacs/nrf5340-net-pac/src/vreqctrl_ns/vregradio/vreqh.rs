#[doc = "Register `VREQH` reader"]
pub struct R(crate::R<VREQH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREQH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREQH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREQH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREQH` writer"]
pub struct W(crate::W<VREQH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREQH_SPEC>;
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
impl From<crate::W<VREQH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREQH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Request high voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREQH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<VREQH_A> for bool {
    #[inline(always)]
    fn from(variant: VREQH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREQH` reader - Request high voltage"]
pub struct VREQH_R(crate::FieldReader<bool, VREQH_A>);
impl VREQH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREQH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREQH_A {
        match self.bits {
            false => VREQH_A::DISABLED,
            true => VREQH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VREQH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VREQH_A::ENABLED
    }
}
impl core::ops::Deref for VREQH_R {
    type Target = crate::FieldReader<bool, VREQH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREQH` writer - Request high voltage"]
pub struct VREQH_W<'a> {
    w: &'a mut W,
}
impl<'a> VREQH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREQH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREQH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREQH_A::ENABLED)
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
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&self) -> VREQH_R {
        VREQH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&mut self) -> VREQH_W {
        VREQH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreqh](index.html) module"]
pub struct VREQH_SPEC;
impl crate::RegisterSpec for VREQH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreqh::R](R) reader structure"]
impl crate::Readable for VREQH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vreqh::W](W) writer structure"]
impl crate::Writable for VREQH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREQH to value 0"]
impl crate::Resettable for VREQH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
