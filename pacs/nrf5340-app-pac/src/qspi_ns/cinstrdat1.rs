#[doc = "Register `CINSTRDAT1` reader"]
pub struct R(crate::R<CINSTRDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CINSTRDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CINSTRDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CINSTRDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CINSTRDAT1` writer"]
pub struct W(crate::W<CINSTRDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CINSTRDAT1_SPEC>;
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
impl From<crate::W<CINSTRDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CINSTRDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE4` reader - Data byte 4"]
pub struct BYTE4_R(crate::FieldReader<u8, u8>);
impl BYTE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE4` writer - Data byte 4"]
pub struct BYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BYTE5` reader - Data byte 5"]
pub struct BYTE5_R(crate::FieldReader<u8, u8>);
impl BYTE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE5` writer - Data byte 5"]
pub struct BYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE6` reader - Data byte 6"]
pub struct BYTE6_R(crate::FieldReader<u8, u8>);
impl BYTE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE6` writer - Data byte 6"]
pub struct BYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `BYTE7` reader - Data byte 7"]
pub struct BYTE7_R(crate::FieldReader<u8, u8>);
impl BYTE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE7` writer - Data byte 7"]
pub struct BYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> BYTE4_W {
        BYTE4_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> BYTE5_W {
        BYTE5_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> BYTE6_W {
        BYTE6_W { w: self }
    }
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> BYTE7_W {
        BYTE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Custom instruction data register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrdat1](index.html) module"]
pub struct CINSTRDAT1_SPEC;
impl crate::RegisterSpec for CINSTRDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cinstrdat1::R](R) reader structure"]
impl crate::Readable for CINSTRDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cinstrdat1::W](W) writer structure"]
impl crate::Writable for CINSTRDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CINSTRDAT1 to value 0"]
impl crate::Resettable for CINSTRDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
