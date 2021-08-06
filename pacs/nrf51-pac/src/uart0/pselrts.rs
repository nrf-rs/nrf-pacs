#[doc = "Register `PSELRTS` reader"]
pub struct R(crate::R<PSELRTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELRTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELRTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELRTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELRTS` writer"]
pub struct W(crate::W<PSELRTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELRTS_SPEC>;
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
impl From<crate::W<PSELRTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELRTS_SPEC>) -> Self {
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
#[doc = "Pin select for RTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselrts](index.html) module"]
pub struct PSELRTS_SPEC;
impl crate::RegisterSpec for PSELRTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselrts::R](R) reader structure"]
impl crate::Readable for PSELRTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselrts::W](W) writer structure"]
impl crate::Writable for PSELRTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSELRTS to value 0xffff_ffff"]
impl crate::Resettable for PSELRTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
