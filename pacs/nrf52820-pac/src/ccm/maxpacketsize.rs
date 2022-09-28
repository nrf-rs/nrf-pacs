#[doc = "Register `MAXPACKETSIZE` reader"]
pub struct R(crate::R<MAXPACKETSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXPACKETSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXPACKETSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXPACKETSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXPACKETSIZE` writer"]
pub struct W(crate::W<MAXPACKETSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXPACKETSIZE_SPEC>;
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
impl From<crate::W<MAXPACKETSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXPACKETSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXPACKETSIZE` reader - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
pub type MAXPACKETSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXPACKETSIZE` writer - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
pub type MAXPACKETSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAXPACKETSIZE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MAXPACKETSIZE_R {
        MAXPACKETSIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Length of keystream generated when MODE.LENGTH = Extended. This value must be greater or equal to the subsequent packet payload to be encrypted/decrypted."]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MAXPACKETSIZE_W<0> {
        MAXPACKETSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length of keystream generated when MODE.LENGTH = Extended.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpacketsize](index.html) module"]
pub struct MAXPACKETSIZE_SPEC;
impl crate::RegisterSpec for MAXPACKETSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxpacketsize::R](R) reader structure"]
impl crate::Readable for MAXPACKETSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxpacketsize::W](W) writer structure"]
impl crate::Writable for MAXPACKETSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXPACKETSIZE to value 0xfb"]
impl crate::Resettable for MAXPACKETSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfb
    }
}
