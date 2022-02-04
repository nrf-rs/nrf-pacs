#[doc = "Register `GAINR` reader"]
pub struct R(crate::R<GAINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINR` writer"]
pub struct W(crate::W<GAINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINR_SPEC>;
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
impl From<crate::W<GAINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINR_A {
    #[doc = "0: -20 dB gain adjustment (minimum)"]
    MINGAIN = 0,
    #[doc = "40: 0 dB gain adjustment"]
    DEFAULTGAIN = 40,
    #[doc = "80: +20 dB gain adjustment (maximum)"]
    MAXGAIN = 80,
}
impl From<GAINR_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAINR` reader - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub struct GAINR_R(crate::FieldReader<u8, GAINR_A>);
impl GAINR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAINR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAINR_A> {
        match self.bits {
            0 => Some(GAINR_A::MINGAIN),
            40 => Some(GAINR_A::DEFAULTGAIN),
            80 => Some(GAINR_A::MAXGAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MINGAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        **self == GAINR_A::MINGAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULTGAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        **self == GAINR_A::DEFAULTGAIN
    }
    #[doc = "Checks if the value of the field is `MAXGAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        **self == GAINR_A::MAXGAIN
    }
}
impl core::ops::Deref for GAINR_R {
    type Target = crate::FieldReader<u8, GAINR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINR` writer - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub struct GAINR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MINGAIN)
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINR_A::DEFAULTGAIN)
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MAXGAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&self) -> GAINR_R {
        GAINR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&mut self) -> GAINR_W {
        GAINR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Right output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainr](index.html) module"]
pub struct GAINR_SPEC;
impl crate::RegisterSpec for GAINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gainr::R](R) reader structure"]
impl crate::Readable for GAINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gainr::W](W) writer structure"]
impl crate::Writable for GAINR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAINR to value 0x28"]
impl crate::Resettable for GAINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x28
    }
}
