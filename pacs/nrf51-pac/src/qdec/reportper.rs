#[doc = "Register `REPORTPER` reader"]
pub struct R(crate::R<REPORTPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPORTPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPORTPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPORTPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REPORTPER` writer"]
pub struct W(crate::W<REPORTPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPORTPER_SPEC>;
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
impl From<crate::W<REPORTPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPORTPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Number of samples to generate an EVENT_REPORTRDY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REPORTPER_A {
    #[doc = "0: 10 samples per report."]
    _10SMPL = 0,
    #[doc = "1: 40 samples per report."]
    _40SMPL = 1,
    #[doc = "2: 80 samples per report."]
    _80SMPL = 2,
    #[doc = "3: 120 samples per report."]
    _120SMPL = 3,
    #[doc = "4: 160 samples per report."]
    _160SMPL = 4,
    #[doc = "5: 200 samples per report."]
    _200SMPL = 5,
    #[doc = "6: 240 samples per report."]
    _240SMPL = 6,
    #[doc = "7: 280 samples per report."]
    _280SMPL = 7,
}
impl From<REPORTPER_A> for u8 {
    #[inline(always)]
    fn from(variant: REPORTPER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REPORTPER` reader - Number of samples to generate an EVENT_REPORTRDY."]
pub struct REPORTPER_R(crate::FieldReader<u8, REPORTPER_A>);
impl REPORTPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REPORTPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTPER_A {
        match self.bits {
            0 => REPORTPER_A::_10SMPL,
            1 => REPORTPER_A::_40SMPL,
            2 => REPORTPER_A::_80SMPL,
            3 => REPORTPER_A::_120SMPL,
            4 => REPORTPER_A::_160SMPL,
            5 => REPORTPER_A::_200SMPL,
            6 => REPORTPER_A::_240SMPL,
            7 => REPORTPER_A::_280SMPL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_10SMPL`"]
    #[inline(always)]
    pub fn is_10smpl(&self) -> bool {
        **self == REPORTPER_A::_10SMPL
    }
    #[doc = "Checks if the value of the field is `_40SMPL`"]
    #[inline(always)]
    pub fn is_40smpl(&self) -> bool {
        **self == REPORTPER_A::_40SMPL
    }
    #[doc = "Checks if the value of the field is `_80SMPL`"]
    #[inline(always)]
    pub fn is_80smpl(&self) -> bool {
        **self == REPORTPER_A::_80SMPL
    }
    #[doc = "Checks if the value of the field is `_120SMPL`"]
    #[inline(always)]
    pub fn is_120smpl(&self) -> bool {
        **self == REPORTPER_A::_120SMPL
    }
    #[doc = "Checks if the value of the field is `_160SMPL`"]
    #[inline(always)]
    pub fn is_160smpl(&self) -> bool {
        **self == REPORTPER_A::_160SMPL
    }
    #[doc = "Checks if the value of the field is `_200SMPL`"]
    #[inline(always)]
    pub fn is_200smpl(&self) -> bool {
        **self == REPORTPER_A::_200SMPL
    }
    #[doc = "Checks if the value of the field is `_240SMPL`"]
    #[inline(always)]
    pub fn is_240smpl(&self) -> bool {
        **self == REPORTPER_A::_240SMPL
    }
    #[doc = "Checks if the value of the field is `_280SMPL`"]
    #[inline(always)]
    pub fn is_280smpl(&self) -> bool {
        **self == REPORTPER_A::_280SMPL
    }
}
impl core::ops::Deref for REPORTPER_R {
    type Target = crate::FieldReader<u8, REPORTPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPORTPER` writer - Number of samples to generate an EVENT_REPORTRDY."]
pub struct REPORTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTPER_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "10 samples per report."]
    #[inline(always)]
    pub fn _10smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_10SMPL)
    }
    #[doc = "40 samples per report."]
    #[inline(always)]
    pub fn _40smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_40SMPL)
    }
    #[doc = "80 samples per report."]
    #[inline(always)]
    pub fn _80smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_80SMPL)
    }
    #[doc = "120 samples per report."]
    #[inline(always)]
    pub fn _120smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_120SMPL)
    }
    #[doc = "160 samples per report."]
    #[inline(always)]
    pub fn _160smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_160SMPL)
    }
    #[doc = "200 samples per report."]
    #[inline(always)]
    pub fn _200smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_200SMPL)
    }
    #[doc = "240 samples per report."]
    #[inline(always)]
    pub fn _240smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_240SMPL)
    }
    #[doc = "280 samples per report."]
    #[inline(always)]
    pub fn _280smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_280SMPL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline(always)]
    pub fn reportper(&self) -> REPORTPER_R {
        REPORTPER_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline(always)]
    pub fn reportper(&mut self) -> REPORTPER_W {
        REPORTPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of samples to generate an EVENT_REPORTRDY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reportper](index.html) module"]
pub struct REPORTPER_SPEC;
impl crate::RegisterSpec for REPORTPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reportper::R](R) reader structure"]
impl crate::Readable for REPORTPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reportper::W](W) writer structure"]
impl crate::Writable for REPORTPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REPORTPER to value 0"]
impl crate::Resettable for REPORTPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
