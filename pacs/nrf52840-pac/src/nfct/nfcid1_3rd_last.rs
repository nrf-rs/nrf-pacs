#[doc = "Register `NFCID1_3RD_LAST` reader"]
pub struct R(crate::R<NFCID1_3RD_LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCID1_3RD_LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCID1_3RD_LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCID1_3RD_LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCID1_3RD_LAST` writer"]
pub struct W(crate::W<NFCID1_3RD_LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCID1_3RD_LAST_SPEC>;
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
impl From<crate::W<NFCID1_3RD_LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCID1_3RD_LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCID1_S` reader - NFCID1 byte S"]
pub struct NFCID1_S_R(crate::FieldReader<u8, u8>);
impl NFCID1_S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_S` writer - NFCID1 byte S"]
pub struct NFCID1_S_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NFCID1_R` reader - NFCID1 byte R"]
pub struct NFCID1_R_R(crate::FieldReader<u8, u8>);
impl NFCID1_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_R` writer - NFCID1 byte R"]
pub struct NFCID1_R_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NFCID1_Q` reader - NFCID1 byte Q"]
pub struct NFCID1_Q_R(crate::FieldReader<u8, u8>);
impl NFCID1_Q_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCID1_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFCID1_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NFCID1_Q` writer - NFCID1 byte Q"]
pub struct NFCID1_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCID1_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&self) -> NFCID1_S_R {
        NFCID1_S_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&self) -> NFCID1_R_R {
        NFCID1_R_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&self) -> NFCID1_Q_R {
        NFCID1_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&mut self) -> NFCID1_S_W {
        NFCID1_S_W { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&mut self) -> NFCID1_R_W {
        NFCID1_R_W { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&mut self) -> NFCID1_Q_W {
        NFCID1_Q_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_3rd_last](index.html) module"]
pub struct NFCID1_3RD_LAST_SPEC;
impl crate::RegisterSpec for NFCID1_3RD_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcid1_3rd_last::R](R) reader structure"]
impl crate::Readable for NFCID1_3RD_LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcid1_3rd_last::W](W) writer structure"]
impl crate::Writable for NFCID1_3RD_LAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NFCID1_3RD_LAST to value 0"]
impl crate::Resettable for NFCID1_3RD_LAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
