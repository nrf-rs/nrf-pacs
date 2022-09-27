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
#[doc = "Field `SRC` reader - Select which LFCLK source is started by the LFCLKSTART task"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "Select which LFCLK source is started by the LFCLKSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "1: 32.768 kHz RC oscillator"]
    LFRC = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    LFXO = 2,
    #[doc = "3: 32.768 kHz synthesized from HFCLK"]
    LFSYNT = 3,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            1 => Some(SRC_A::LFRC),
            2 => Some(SRC_A::LFXO),
            3 => Some(SRC_A::LFSYNT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SRC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFSYNT`"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == SRC_A::LFSYNT
    }
}
#[doc = "Field `SRC` writer - Select which LFCLK source is started by the LFCLKSTART task"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFCLKSRC_SPEC, u8, SRC_A, 2, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(SRC_A::LFRC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(SRC_A::LFXO)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn lfsynt(self) -> &'a mut W {
        self.variant(SRC_A::LFSYNT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source for LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrc](index.html) module"]
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
#[doc = "`reset()` method sets LFCLKSRC to value 0x01"]
impl crate::Resettable for LFCLKSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
