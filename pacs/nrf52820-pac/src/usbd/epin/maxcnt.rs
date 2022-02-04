#[doc = "Register `MAXCNT` reader"]
pub struct R(crate::R<MAXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXCNT` writer"]
pub struct W(crate::W<MAXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXCNT_SPEC>;
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
impl From<crate::W<MAXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXCNT` reader - Maximum number of bytes to transfer"]
pub struct MAXCNT_R(crate::FieldReader<u8, u8>);
impl MAXCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXCNT` writer - Maximum number of bytes to transfer"]
pub struct MAXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W {
        MAXCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Maximum number of bytes to transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxcnt](index.html) module"]
pub struct MAXCNT_SPEC;
impl crate::RegisterSpec for MAXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxcnt::R](R) reader structure"]
impl crate::Readable for MAXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxcnt::W](W) writer structure"]
impl crate::Writable for MAXCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXCNT to value 0"]
impl crate::Resettable for MAXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
