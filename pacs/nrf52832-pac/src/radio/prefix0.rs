#[doc = "Register `PREFIX0` reader"]
pub struct R(crate::R<PREFIX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREFIX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREFIX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREFIX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREFIX0` writer"]
pub struct W(crate::W<PREFIX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREFIX0_SPEC>;
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
impl From<crate::W<PREFIX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREFIX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP0` reader - Address prefix 0."]
pub struct AP0_R(crate::FieldReader<u8, u8>);
impl AP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP0` writer - Address prefix 0."]
pub struct AP0_W<'a> {
    w: &'a mut W,
}
impl<'a> AP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `AP1` reader - Address prefix 1."]
pub struct AP1_R(crate::FieldReader<u8, u8>);
impl AP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP1` writer - Address prefix 1."]
pub struct AP1_W<'a> {
    w: &'a mut W,
}
impl<'a> AP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `AP2` reader - Address prefix 2."]
pub struct AP2_R(crate::FieldReader<u8, u8>);
impl AP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP2` writer - Address prefix 2."]
pub struct AP2_W<'a> {
    w: &'a mut W,
}
impl<'a> AP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `AP3` reader - Address prefix 3."]
pub struct AP3_R(crate::FieldReader<u8, u8>);
impl AP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP3` writer - Address prefix 3."]
pub struct AP3_W<'a> {
    w: &'a mut W,
}
impl<'a> AP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&self) -> AP0_R {
        AP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&self) -> AP1_R {
        AP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&self) -> AP2_R {
        AP2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&self) -> AP3_R {
        AP3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&mut self) -> AP0_W {
        AP0_W { w: self }
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&mut self) -> AP1_W {
        AP1_W { w: self }
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&mut self) -> AP2_W {
        AP2_W { w: self }
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&mut self) -> AP3_W {
        AP3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefixes bytes for logical addresses 0-3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix0](index.html) module"]
pub struct PREFIX0_SPEC;
impl crate::RegisterSpec for PREFIX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prefix0::R](R) reader structure"]
impl crate::Readable for PREFIX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prefix0::W](W) writer structure"]
impl crate::Writable for PREFIX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREFIX0 to value 0"]
impl crate::Resettable for PREFIX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
