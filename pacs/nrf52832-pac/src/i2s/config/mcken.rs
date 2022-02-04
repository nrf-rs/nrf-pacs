#[doc = "Register `MCKEN` reader"]
pub struct R(crate::R<MCKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCKEN` writer"]
pub struct W(crate::W<MCKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCKEN_SPEC>;
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
impl From<crate::W<MCKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master clock generator enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKEN_A {
    #[doc = "0: Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    DISABLED = 0,
    #[doc = "1: Master clock generator running and MCK output on PSEL.MCK."]
    ENABLED = 1,
}
impl From<MCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKEN` reader - Master clock generator enable."]
pub struct MCKEN_R(crate::FieldReader<bool, MCKEN_A>);
impl MCKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKEN_A {
        match self.bits {
            false => MCKEN_A::DISABLED,
            true => MCKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MCKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCKEN_A::ENABLED
    }
}
impl core::ops::Deref for MCKEN_R {
    type Target = crate::FieldReader<bool, MCKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKEN` writer - Master clock generator enable."]
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKEN_A::DISABLED)
    }
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKEN_A::ENABLED)
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
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master clock generator enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcken](index.html) module"]
pub struct MCKEN_SPEC;
impl crate::RegisterSpec for MCKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcken::R](R) reader structure"]
impl crate::Readable for MCKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcken::W](W) writer structure"]
impl crate::Writable for MCKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCKEN to value 0x01"]
impl crate::Resettable for MCKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
