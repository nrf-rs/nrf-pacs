#[doc = "Register `EVENTS_CROSS` reader"]
pub struct R(crate::R<EVENTS_CROSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_CROSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_CROSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_CROSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_CROSS` writer"]
pub struct W(crate::W<EVENTS_CROSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_CROSS_SPEC>;
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
impl From<crate::W<EVENTS_CROSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_CROSS_SPEC>) -> Self {
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
#[doc = "Input voltage crossed the threshold in any direction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_cross](index.html) module"]
pub struct EVENTS_CROSS_SPEC;
impl crate::RegisterSpec for EVENTS_CROSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_cross::R](R) reader structure"]
impl crate::Readable for EVENTS_CROSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_cross::W](W) writer structure"]
impl crate::Writable for EVENTS_CROSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_CROSS to value 0"]
impl crate::Resettable for EVENTS_CROSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
