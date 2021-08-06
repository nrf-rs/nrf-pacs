#[doc = "Register `ADDRPTR` reader"]
pub struct R(crate::R<ADDRPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRPTR` writer"]
pub struct W(crate::W<ADDRPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRPTR_SPEC>;
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
impl From<crate::W<ADDRPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRPTR_SPEC>) -> Self {
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
#[doc = "Pointer to the resolvable address (6 bytes).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrptr](index.html) module"]
pub struct ADDRPTR_SPEC;
impl crate::RegisterSpec for ADDRPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrptr::R](R) reader structure"]
impl crate::Readable for ADDRPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrptr::W](W) writer structure"]
impl crate::Writable for ADDRPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRPTR to value 0"]
impl crate::Resettable for ADDRPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
