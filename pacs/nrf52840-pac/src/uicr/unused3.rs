#[doc = "Register `UNUSED3` reader"]
pub struct R(crate::R<UNUSED3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNUSED3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNUSED3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNUSED3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNUSED3` writer"]
pub struct W(crate::W<UNUSED3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNUSED3_SPEC>;
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
impl From<crate::W<UNUSED3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNUSED3_SPEC>) -> Self {
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
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused3](index.html) module"]
pub struct UNUSED3_SPEC;
impl crate::RegisterSpec for UNUSED3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unused3::R](R) reader structure"]
impl crate::Readable for UNUSED3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unused3::W](W) writer structure"]
impl crate::Writable for UNUSED3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNUSED3 to value 0"]
impl crate::Resettable for UNUSED3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
