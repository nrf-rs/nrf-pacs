#[doc = "Register `EVENTS_ERROR` reader"]
pub struct R(crate::R<EVENTS_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_ERROR` writer"]
pub struct W(crate::W<EVENTS_ERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_ERROR_SPEC>;
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
impl From<crate::W<EVENTS_ERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_ERROR_SPEC>) -> Self {
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
#[doc = "Error happened.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](index.html) module"]
pub struct EVENTS_ERROR_SPEC;
impl crate::RegisterSpec for EVENTS_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_error::R](R) reader structure"]
impl crate::Readable for EVENTS_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_error::W](W) writer structure"]
impl crate::Writable for EVENTS_ERROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_ERROR to value 0"]
impl crate::Resettable for EVENTS_ERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
