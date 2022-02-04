#[doc = "Register `RATIO` reader"]
pub struct R(crate::R<RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATIO` writer"]
pub struct W(crate::W<RATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATIO_SPEC>;
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
impl From<crate::W<RATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIO_A {
    #[doc = "0: Ratio of 64"]
    RATIO64 = 0,
    #[doc = "1: Ratio of 80"]
    RATIO80 = 1,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATIO` reader - Selects the ratio between PDM_CLK and output sample rate"]
pub struct RATIO_R(crate::FieldReader<bool, RATIO_A>);
impl RATIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RATIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            false => RATIO_A::RATIO64,
            true => RATIO_A::RATIO80,
        }
    }
    #[doc = "Checks if the value of the field is `RATIO64`"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        **self == RATIO_A::RATIO64
    }
    #[doc = "Checks if the value of the field is `RATIO80`"]
    #[inline(always)]
    pub fn is_ratio80(&self) -> bool {
        **self == RATIO_A::RATIO80
    }
}
impl core::ops::Deref for RATIO_R {
    type Target = crate::FieldReader<bool, RATIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RATIO` writer - Selects the ratio between PDM_CLK and output sample rate"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO64)
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn ratio80(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO80)
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
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio](index.html) module"]
pub struct RATIO_SPEC;
impl crate::RegisterSpec for RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratio::R](R) reader structure"]
impl crate::Readable for RATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratio::W](W) writer structure"]
impl crate::Writable for RATIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RATIO to value 0"]
impl crate::Resettable for RATIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
