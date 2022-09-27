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
pub type LOWER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOWER` writer - Lower half word"]
pub type LOWER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTP_SPEC, u16, u16, 16, O>;
#[doc = "Field `UPPER` reader - Upper half word"]
pub type UPPER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UPPER` writer - Upper half word"]
pub type UPPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTP_SPEC, u16, u16, 16, O>;
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
    pub fn lower(&mut self) -> LOWER_W<0> {
        LOWER_W::new(self)
    }
    #[doc = "Bits 16:31 - Upper half word"]
    #[inline(always)]
    pub fn upper(&mut self) -> UPPER_W<16> {
        UPPER_W::new(self)
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
