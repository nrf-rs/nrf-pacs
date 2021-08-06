#[doc = "Register `PSELLED` reader"]
pub struct R(crate::R<PSELLED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELLED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELLED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELLED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELLED` writer"]
pub struct W(crate::W<PSELLED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELLED_SPEC>;
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
impl From<crate::W<PSELLED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELLED_SPEC>) -> Self {
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
#[doc = "Pin select for LED output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselled](index.html) module"]
pub struct PSELLED_SPEC;
impl crate::RegisterSpec for PSELLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselled::R](R) reader structure"]
impl crate::Readable for PSELLED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselled::W](W) writer structure"]
impl crate::Writable for PSELLED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELLED to value 0xffff_ffff"]
impl crate::Resettable for PSELLED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
