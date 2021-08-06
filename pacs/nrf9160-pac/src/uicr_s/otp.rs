#[doc = "Register `OTP[%s]` reader"]
pub struct R(crate::R<OTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTP[%s]` writer"]
pub struct W(crate::W<OTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTP_SPEC>;
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
impl From<crate::W<OTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER` reader - Lower half word"]
pub struct LOWER_R(crate::FieldReader<u16, u16>);
impl LOWER_R {
    pub(crate) fn new(bits: u16) -> Self {
        LOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWER` writer - Lower half word"]
pub struct LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `UPPER` reader - Upper half word"]
pub struct UPPER_R(crate::FieldReader<u16, u16>);
impl UPPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        UPPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPPER` writer - Upper half word"]
pub struct UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower half word"]
    #[inline(always)]
    pub fn lower(&self) -> LOWER_R {
        LOWER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Upper half word"]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower half word"]
    #[inline(always)]
    pub fn lower(&mut self) -> LOWER_W {
        LOWER_W { w: self }
    }
    #[doc = "Bits 16:31 - Upper half word"]
    #[inline(always)]
    pub fn upper(&mut self) -> UPPER_W {
        UPPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: One time programmable memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otp](index.html) module"]
pub struct OTP_SPEC;
impl crate::RegisterSpec for OTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otp::R](R) reader structure"]
impl crate::Readable for OTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otp::W](W) writer structure"]
impl crate::Writable for OTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTP[%s]
to value 0xffff_ffff"]
impl crate::Resettable for OTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
