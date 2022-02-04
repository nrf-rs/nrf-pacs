#[doc = "Register `GAINL` reader"]
pub struct R(crate::R<GAINL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAINL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAINL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINL` writer"]
pub struct W(crate::W<GAINL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINL_SPEC>;
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
impl From<crate::W<GAINL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAINL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINL_A {
    #[doc = "0: -20 dB gain adjustment (minimum)"]
    MINGAIN = 0,
    #[doc = "40: 0 dB gain adjustment"]
    DEFAULTGAIN = 40,
    #[doc = "80: +20 dB gain adjustment (maximum)"]
    MAXGAIN = 80,
}
impl From<GAINL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GAINL` reader - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub struct GAINL_R(crate::FieldReader<u8, GAINL_A>);
impl GAINL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAINL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAINL_A> {
        match self.bits {
            0 => Some(GAINL_A::MINGAIN),
            40 => Some(GAINL_A::DEFAULTGAIN),
            80 => Some(GAINL_A::MAXGAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MINGAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        **self == GAINL_A::MINGAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULTGAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        **self == GAINL_A::DEFAULTGAIN
    }
    #[doc = "Checks if the value of the field is `MAXGAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        **self == GAINL_A::MAXGAIN
    }
}
impl core::ops::Deref for GAINL_R {
    type Target = crate::FieldReader<u8, GAINL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAINL` writer - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub struct GAINL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-20 dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MINGAIN)
    }
    #[doc = "0 dB gain adjustment"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINL_A::DEFAULTGAIN)
    }
    #[doc = "+20 dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MAXGAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&self) -> GAINL_R {
        GAINL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&mut self) -> GAINL_W {
        GAINL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Left output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainl](index.html) module"]
pub struct GAINL_SPEC;
impl crate::RegisterSpec for GAINL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gainl::R](R) reader structure"]
impl crate::Readable for GAINL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gainl::W](W) writer structure"]
impl crate::Writable for GAINL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAINL to value 0x28"]
impl crate::Resettable for GAINL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x28
    }
}
