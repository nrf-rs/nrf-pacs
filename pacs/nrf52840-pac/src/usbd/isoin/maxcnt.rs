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
pub type MAXCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAXCNT` writer - Maximum number of bytes to transfer"]
pub type MAXCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAXCNT_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W<0> {
        MAXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum number of bytes to transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxcnt](index.html) module"]
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
