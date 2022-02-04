#[doc = "Register `LFCLKALWAYSRUN` reader"]
pub struct R(crate::R<LFCLKALWAYSRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKALWAYSRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKALWAYSRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKALWAYSRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKALWAYSRUN` writer"]
pub struct W(crate::W<LFCLKALWAYSRUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKALWAYSRUN_SPEC>;
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
impl From<crate::W<LFCLKALWAYSRUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKALWAYSRUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Ensure clock is always running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALWAYSRUN_A {
    #[doc = "0: Use automatic clock control"]
    AUTOMATIC = 0,
    #[doc = "1: Ensure clock is always running"]
    ALWAYSRUN = 1,
}
impl From<ALWAYSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSRUN` reader - Ensure clock is always running"]
pub struct ALWAYSRUN_R(crate::FieldReader<bool, ALWAYSRUN_A>);
impl ALWAYSRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALWAYSRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSRUN_A {
        match self.bits {
            false => ALWAYSRUN_A::AUTOMATIC,
            true => ALWAYSRUN_A::ALWAYSRUN,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == ALWAYSRUN_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `ALWAYSRUN`"]
    #[inline(always)]
    pub fn is_always_run(&self) -> bool {
        **self == ALWAYSRUN_A::ALWAYSRUN
    }
}
impl core::ops::Deref for ALWAYSRUN_R {
    type Target = crate::FieldReader<bool, ALWAYSRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWAYSRUN` writer - Ensure clock is always running"]
pub struct ALWAYSRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYSRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALWAYSRUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use automatic clock control"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(ALWAYSRUN_A::AUTOMATIC)
    }
    #[doc = "Ensure clock is always running"]
    #[inline(always)]
    pub fn always_run(self) -> &'a mut W {
        self.variant(ALWAYSRUN_A::ALWAYSRUN)
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
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&self) -> ALWAYSRUN_R {
        ALWAYSRUN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&mut self) -> ALWAYSRUN_W {
        ALWAYSRUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automatic or manual control of LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkalwaysrun](index.html) module"]
pub struct LFCLKALWAYSRUN_SPEC;
impl crate::RegisterSpec for LFCLKALWAYSRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclkalwaysrun::R](R) reader structure"]
impl crate::Readable for LFCLKALWAYSRUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclkalwaysrun::W](W) writer structure"]
impl crate::Writable for LFCLKALWAYSRUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCLKALWAYSRUN to value 0"]
impl crate::Resettable for LFCLKALWAYSRUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
