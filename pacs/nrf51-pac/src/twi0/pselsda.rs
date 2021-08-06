#[doc = "Register `PSELSDA` reader"]
pub struct R(crate::R<PSELSDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELSDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELSDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELSDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELSDA` writer"]
pub struct W(crate::W<PSELSDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELSDA_SPEC>;
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
impl From<crate::W<PSELSDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELSDA_SPEC>) -> Self {
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
#[doc = "Pin select for SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselsda](index.html) module"]
pub struct PSELSDA_SPEC;
impl crate::RegisterSpec for PSELSDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselsda::R](R) reader structure"]
impl crate::Readable for PSELSDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselsda::W](W) writer structure"]
impl crate::Writable for PSELSDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELSDA to value 0xffff_ffff"]
impl crate::Resettable for PSELSDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
