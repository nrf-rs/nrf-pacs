#[doc = "Register `CODEPAGESIZE` reader"]
pub struct R(crate::R<CODEPAGESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEPAGESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEPAGESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEPAGESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CODEPAGESIZE` writer"]
pub struct W(crate::W<CODEPAGESIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEPAGESIZE_SPEC>;
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
impl From<crate::W<CODEPAGESIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEPAGESIZE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Code memory page size in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codepagesize](index.html) module"]
pub struct CODEPAGESIZE_SPEC;
impl crate::RegisterSpec for CODEPAGESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codepagesize::R](R) reader structure"]
impl crate::Readable for CODEPAGESIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codepagesize::W](W) writer structure"]
impl crate::Writable for CODEPAGESIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0xffff_ffff"]
impl crate::Resettable for CODEPAGESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
