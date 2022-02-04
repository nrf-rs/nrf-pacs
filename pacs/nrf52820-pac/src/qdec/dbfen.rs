#[doc = "Register `DBFEN` reader"]
pub struct R(crate::R<DBFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBFEN` writer"]
pub struct W(crate::W<DBFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBFEN_SPEC>;
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
impl From<crate::W<DBFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable input debounce filters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBFEN_A {
    #[doc = "0: Debounce input filters disabled"]
    DISABLED = 0,
    #[doc = "1: Debounce input filters enabled"]
    ENABLED = 1,
}
impl From<DBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBFEN` reader - Enable input debounce filters"]
pub struct DBFEN_R(crate::FieldReader<bool, DBFEN_A>);
impl DBFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBFEN_A {
        match self.bits {
            false => DBFEN_A::DISABLED,
            true => DBFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBFEN_A::ENABLED
    }
}
impl core::ops::Deref for DBFEN_R {
    type Target = crate::FieldReader<bool, DBFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBFEN` writer - Enable input debounce filters"]
pub struct DBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Debounce input filters disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBFEN_A::DISABLED)
    }
    #[doc = "Debounce input filters enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBFEN_A::ENABLED)
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
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&self) -> DBFEN_R {
        DBFEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&mut self) -> DBFEN_W {
        DBFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable input debounce filters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbfen](index.html) module"]
pub struct DBFEN_SPEC;
impl crate::RegisterSpec for DBFEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbfen::R](R) reader structure"]
impl crate::Readable for DBFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbfen::W](W) writer structure"]
impl crate::Writable for DBFEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBFEN to value 0"]
impl crate::Resettable for DBFEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
