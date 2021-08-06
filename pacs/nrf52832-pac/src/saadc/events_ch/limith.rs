#[doc = "Register `LIMITH` reader"]
pub struct R(crate::R<LIMITH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMITH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMITH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMITH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMITH` writer"]
pub struct W(crate::W<LIMITH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMITH_SPEC>;
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
impl From<crate::W<LIMITH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMITH_SPEC>) -> Self {
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
#[doc = "Description cluster\\[0\\]: Last results is equal or above CH\\[0\\].LIMIT.HIGH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limith](index.html) module"]
pub struct LIMITH_SPEC;
impl crate::RegisterSpec for LIMITH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limith::R](R) reader structure"]
impl crate::Readable for LIMITH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limith::W](W) writer structure"]
impl crate::Writable for LIMITH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMITH to value 0"]
impl crate::Resettable for LIMITH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
