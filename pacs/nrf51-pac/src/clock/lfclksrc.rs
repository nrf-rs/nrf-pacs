#[doc = "Register `LFCLKSRC` reader"]
pub struct R(crate::R<LFCLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKSRC` writer"]
pub struct W(crate::W<LFCLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKSRC_SPEC>;
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
impl From<crate::W<LFCLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Internal 32KiHz RC oscillator."]
    RC = 0,
    #[doc = "1: External 32KiHz crystal."]
    XTAL = 1,
    #[doc = "2: Internal 32KiHz synthesizer from HFCLK system clock."]
    SYNTH = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - Clock source."]
pub struct SRC_R(crate::FieldReader<u8, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::RC),
            1 => Some(SRC_A::XTAL),
            2 => Some(SRC_A::SYNTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        **self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        **self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        **self == SRC_A::SYNTH
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u8, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Clock source."]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 32KiHz RC oscillator."]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRC_A::RC)
    }
    #[doc = "External 32KiHz crystal."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRC_A::XTAL)
    }
    #[doc = "Internal 32KiHz synthesizer from HFCLK system clock."]
    #[inline(always)]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRC_A::SYNTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source for the LFCLK clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrc](index.html) module"]
pub struct LFCLKSRC_SPEC;
impl crate::RegisterSpec for LFCLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksrc::R](R) reader structure"]
impl crate::Readable for LFCLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](W) writer structure"]
impl crate::Writable for LFCLKSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0"]
impl crate::Resettable for LFCLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
