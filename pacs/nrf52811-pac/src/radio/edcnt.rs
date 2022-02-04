#[doc = "Register `EDCNT` reader"]
pub struct R(crate::R<EDCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDCNT` writer"]
pub struct W(crate::W<EDCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDCNT_SPEC>;
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
impl From<crate::W<EDCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDCNT` reader - IEEE 802.15.4 energy detect loop count"]
pub struct EDCNT_R(crate::FieldReader<u32, u32>);
impl EDCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EDCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDCNT` writer - IEEE 802.15.4 energy detect loop count"]
pub struct EDCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | (value as u32 & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&self) -> EDCNT_R {
        EDCNT_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(&mut self) -> EDCNT_W {
        EDCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 energy detect loop count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edcnt](index.html) module"]
pub struct EDCNT_SPEC;
impl crate::RegisterSpec for EDCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edcnt::R](R) reader structure"]
impl crate::Readable for EDCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edcnt::W](W) writer structure"]
impl crate::Writable for EDCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EDCNT to value 0"]
impl crate::Resettable for EDCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
