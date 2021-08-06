#[doc = "Register `TXDPTR` reader"]
pub struct R(crate::R<TXDPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDPTR` writer"]
pub struct W(crate::W<TXDPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDPTR_SPEC>;
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
impl From<crate::W<TXDPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDPTR_SPEC>) -> Self {
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
#[doc = "TX data pointer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdptr](index.html) module"]
pub struct TXDPTR_SPEC;
impl crate::RegisterSpec for TXDPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdptr::R](R) reader structure"]
impl crate::Readable for TXDPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdptr::W](W) writer structure"]
impl crate::Writable for TXDPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDPTR to value 0"]
impl crate::Resettable for TXDPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
