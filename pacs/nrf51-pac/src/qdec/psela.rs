#[doc = "Register `PSELA` reader"]
pub struct R(crate::R<PSELA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELA` writer"]
pub struct W(crate::W<PSELA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELA_SPEC>;
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
impl From<crate::W<PSELA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELA_SPEC>) -> Self {
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
#[doc = "Pin select for phase A input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psela](index.html) module"]
pub struct PSELA_SPEC;
impl crate::RegisterSpec for PSELA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psela::R](R) reader structure"]
impl crate::Readable for PSELA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psela::W](W) writer structure"]
impl crate::Writable for PSELA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELA to value 0xffff_ffff"]
impl crate::Resettable for PSELA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
