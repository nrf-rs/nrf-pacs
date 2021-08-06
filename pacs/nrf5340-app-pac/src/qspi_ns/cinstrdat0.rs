#[doc = "Register `CINSTRDAT0` reader"]
pub struct R(crate::R<CINSTRDAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CINSTRDAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CINSTRDAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CINSTRDAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CINSTRDAT0` writer"]
pub struct W(crate::W<CINSTRDAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CINSTRDAT0_SPEC>;
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
impl From<crate::W<CINSTRDAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CINSTRDAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE0` reader - Data byte 0"]
pub struct BYTE0_R(crate::FieldReader<u8, u8>);
impl BYTE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE0` writer - Data byte 0"]
pub struct BYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BYTE1` reader - Data byte 1"]
pub struct BYTE1_R(crate::FieldReader<u8, u8>);
impl BYTE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE1` writer - Data byte 1"]
pub struct BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE2` reader - Data byte 2"]
pub struct BYTE2_R(crate::FieldReader<u8, u8>);
impl BYTE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE2` writer - Data byte 2"]
pub struct BYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `BYTE3` reader - Data byte 3"]
pub struct BYTE3_R(crate::FieldReader<u8, u8>);
impl BYTE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE3` writer - Data byte 3"]
pub struct BYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W {
        BYTE2_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W {
        BYTE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Custom instruction data register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrdat0](index.html) module"]
pub struct CINSTRDAT0_SPEC;
impl crate::RegisterSpec for CINSTRDAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cinstrdat0::R](R) reader structure"]
impl crate::Readable for CINSTRDAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cinstrdat0::W](W) writer structure"]
impl crate::Writable for CINSTRDAT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CINSTRDAT0 to value 0"]
impl crate::Resettable for CINSTRDAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
