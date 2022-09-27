#[doc = "Register `HEADERMASK` reader"]
pub struct R(crate::R<HEADERMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HEADERMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HEADERMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HEADERMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HEADERMASK` writer"]
pub struct W(crate::W<HEADERMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HEADERMASK_SPEC>;
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
impl From<crate::W<HEADERMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HEADERMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEADERMASK` reader - Header (S0) mask"]
pub type HEADERMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HEADERMASK` writer - Header (S0) mask"]
pub type HEADERMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HEADERMASK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&self) -> HEADERMASK_R {
        HEADERMASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&mut self) -> HEADERMASK_W<0> {
        HEADERMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Header (S0) mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [headermask](index.html) module"]
pub struct HEADERMASK_SPEC;
impl crate::RegisterSpec for HEADERMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [headermask::R](R) reader structure"]
impl crate::Readable for HEADERMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [headermask::W](W) writer structure"]
impl crate::Writable for HEADERMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HEADERMASK to value 0xe3"]
impl crate::Resettable for HEADERMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe3
    }
}
