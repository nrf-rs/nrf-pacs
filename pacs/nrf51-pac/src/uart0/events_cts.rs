#[doc = "Register `EVENTS_CTS` reader"]
pub struct R(crate::R<EVENTS_CTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_CTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_CTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_CTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_CTS` writer"]
pub struct W(crate::W<EVENTS_CTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_CTS_SPEC>;
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
impl From<crate::W<EVENTS_CTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_CTS_SPEC>) -> Self {
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
#[doc = "CTS activated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_cts](index.html) module"]
pub struct EVENTS_CTS_SPEC;
impl crate::RegisterSpec for EVENTS_CTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_cts::R](R) reader structure"]
impl crate::Readable for EVENTS_CTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_cts::W](W) writer structure"]
impl crate::Writable for EVENTS_CTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_CTS to value 0"]
impl crate::Resettable for EVENTS_CTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
