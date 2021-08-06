#[doc = "Register `PSELTXD` reader"]
pub struct R(crate::R<PSELTXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELTXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELTXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELTXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELTXD` writer"]
pub struct W(crate::W<PSELTXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELTXD_SPEC>;
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
impl From<crate::W<PSELTXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELTXD_SPEC>) -> Self {
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
#[doc = "Pin select for TXD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pseltxd](index.html) module"]
pub struct PSELTXD_SPEC;
impl crate::RegisterSpec for PSELTXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pseltxd::R](R) reader structure"]
impl crate::Readable for PSELTXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pseltxd::W](W) writer structure"]
impl crate::Writable for PSELTXD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELTXD to value 0xffff_ffff"]
impl crate::Resettable for PSELTXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
