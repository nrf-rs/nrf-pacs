#[doc = "Register `EDSAMPLE` reader"]
pub struct R(crate::R<EDSAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDSAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDSAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDSAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDSAMPLE` writer"]
pub struct W(crate::W<EDSAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDSAMPLE_SPEC>;
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
impl From<crate::W<EDSAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDSAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDLVL` reader - IEEE 802.15.4 energy detect level"]
pub struct EDLVL_R(crate::FieldReader<u8, u8>);
impl EDLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDLVL` writer - IEEE 802.15.4 energy detect level"]
pub struct EDLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EDLVL_R {
        EDLVL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&mut self) -> EDLVL_W {
        EDLVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 energy detect level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edsample](index.html) module"]
pub struct EDSAMPLE_SPEC;
impl crate::RegisterSpec for EDSAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edsample::R](R) reader structure"]
impl crate::Readable for EDSAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edsample::W](W) writer structure"]
impl crate::Writable for EDSAMPLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDSAMPLE to value 0"]
impl crate::Resettable for EDSAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
