#[doc = "Register `PREFIX1` reader"]
pub struct R(crate::R<PREFIX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREFIX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREFIX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREFIX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREFIX1` writer"]
pub struct W(crate::W<PREFIX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREFIX1_SPEC>;
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
impl From<crate::W<PREFIX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREFIX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP4` reader - Address prefix 4."]
pub struct AP4_R(crate::FieldReader<u8, u8>);
impl AP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP4` writer - Address prefix 4."]
pub struct AP4_W<'a> {
    w: &'a mut W,
}
impl<'a> AP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `AP5` reader - Address prefix 5."]
pub struct AP5_R(crate::FieldReader<u8, u8>);
impl AP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP5` writer - Address prefix 5."]
pub struct AP5_W<'a> {
    w: &'a mut W,
}
impl<'a> AP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `AP6` reader - Address prefix 6."]
pub struct AP6_R(crate::FieldReader<u8, u8>);
impl AP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP6` writer - Address prefix 6."]
pub struct AP6_W<'a> {
    w: &'a mut W,
}
impl<'a> AP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `AP7` reader - Address prefix 7."]
pub struct AP7_R(crate::FieldReader<u8, u8>);
impl AP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP7` writer - Address prefix 7."]
pub struct AP7_W<'a> {
    w: &'a mut W,
}
impl<'a> AP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&self) -> AP4_R {
        AP4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&self) -> AP5_R {
        AP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&self) -> AP6_R {
        AP6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&self) -> AP7_R {
        AP7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&mut self) -> AP4_W {
        AP4_W { w: self }
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&mut self) -> AP5_W {
        AP5_W { w: self }
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&mut self) -> AP6_W {
        AP6_W { w: self }
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&mut self) -> AP7_W {
        AP7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefixes bytes for logical addresses 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix1](index.html) module"]
pub struct PREFIX1_SPEC;
impl crate::RegisterSpec for PREFIX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prefix1::R](R) reader structure"]
impl crate::Readable for PREFIX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prefix1::W](W) writer structure"]
impl crate::Writable for PREFIX1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREFIX1 to value 0"]
impl crate::Resettable for PREFIX1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
