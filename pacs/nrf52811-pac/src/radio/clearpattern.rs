#[doc = "Register `CLEARPATTERN` reader"]
pub struct R(crate::R<CLEARPATTERN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARPATTERN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARPATTERN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARPATTERN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARPATTERN` writer"]
pub struct W(crate::W<CLEARPATTERN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARPATTERN_SPEC>;
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
impl From<crate::W<CLEARPATTERN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARPATTERN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clears GPIO pattern array for antenna control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARPATTERN_A {
    #[doc = "1: Clear the GPIO pattern"]
    CLEAR = 1,
}
impl From<CLEARPATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEARPATTERN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEARPATTERN` reader - Clears GPIO pattern array for antenna control"]
pub struct CLEARPATTERN_R(crate::FieldReader<bool, CLEARPATTERN_A>);
impl CLEARPATTERN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLEARPATTERN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLEARPATTERN_A> {
        match self.bits {
            true => Some(CLEARPATTERN_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CLEARPATTERN_A::CLEAR
    }
}
impl core::ops::Deref for CLEARPATTERN_R {
    type Target = crate::FieldReader<bool, CLEARPATTERN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEARPATTERN` writer - Clears GPIO pattern array for antenna control"]
pub struct CLEARPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEARPATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEARPATTERN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the GPIO pattern"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEARPATTERN_A::CLEAR)
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
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&self) -> CLEARPATTERN_R {
        CLEARPATTERN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&mut self) -> CLEARPATTERN_W {
        CLEARPATTERN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear the GPIO pattern array for antenna control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearpattern](index.html) module"]
pub struct CLEARPATTERN_SPEC;
impl crate::RegisterSpec for CLEARPATTERN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearpattern::R](R) reader structure"]
impl crate::Readable for CLEARPATTERN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearpattern::W](W) writer structure"]
impl crate::Writable for CLEARPATTERN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEARPATTERN to value 0"]
impl crate::Resettable for CLEARPATTERN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
