#[doc = "Register `DETECTMODE` reader"]
pub struct R(crate::R<DETECTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DETECTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DETECTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DETECTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DETECTMODE` writer"]
pub struct W(crate::W<DETECTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DETECTMODE_SPEC>;
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
impl From<crate::W<DETECTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DETECTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DETECTMODE_A {
    #[doc = "0: DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0,
    #[doc = "1: Use the latched LDETECT behaviour"]
    LDETECT = 1,
}
impl From<DETECTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DETECTMODE` reader - Select between default DETECT signal behaviour and LDETECT mode"]
pub struct DETECTMODE_R(crate::FieldReader<bool, DETECTMODE_A>);
impl DETECTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETECTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECTMODE_A {
        match self.bits {
            false => DETECTMODE_A::DEFAULT,
            true => DETECTMODE_A::LDETECT,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == DETECTMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LDETECT`"]
    #[inline(always)]
    pub fn is_ldetect(&self) -> bool {
        **self == DETECTMODE_A::LDETECT
    }
}
impl core::ops::Deref for DETECTMODE_R {
    type Target = crate::FieldReader<bool, DETECTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETECTMODE` writer - Select between default DETECT signal behaviour and LDETECT mode"]
pub struct DETECTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DETECTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DETECTMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DETECTMODE_A::DEFAULT)
    }
    #[doc = "Use the latched LDETECT behaviour"]
    #[inline(always)]
    pub fn ldetect(self) -> &'a mut W {
        self.variant(DETECTMODE_A::LDETECT)
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
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&self) -> DETECTMODE_R {
        DETECTMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&mut self) -> DETECTMODE_W {
        DETECTMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [detectmode](index.html) module"]
pub struct DETECTMODE_SPEC;
impl crate::RegisterSpec for DETECTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [detectmode::R](R) reader structure"]
impl crate::Readable for DETECTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [detectmode::W](W) writer structure"]
impl crate::Writable for DETECTMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DETECTMODE to value 0"]
impl crate::Resettable for DETECTMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
