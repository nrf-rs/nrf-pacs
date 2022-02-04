#[doc = "Register `CTIV` reader"]
pub struct R(crate::R<CTIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIV` writer"]
pub struct W(crate::W<CTIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIV_SPEC>;
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
impl From<crate::W<CTIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIV` reader - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub struct CTIV_R(crate::FieldReader<u8, u8>);
impl CTIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIV` writer - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub struct CTIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&self) -> CTIV_R {
        CTIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&mut self) -> CTIV_W {
        CTIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration timer interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiv](index.html) module"]
pub struct CTIV_SPEC;
impl crate::RegisterSpec for CTIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctiv::R](R) reader structure"]
impl crate::Readable for CTIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctiv::W](W) writer structure"]
impl crate::Writable for CTIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIV to value 0"]
impl crate::Resettable for CTIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
