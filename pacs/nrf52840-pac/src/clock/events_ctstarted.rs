#[doc = "Register `EVENTS_CTSTARTED` reader"]
pub struct R(crate::R<EVENTS_CTSTARTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_CTSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_CTSTARTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_CTSTARTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_CTSTARTED` writer"]
pub struct W(crate::W<EVENTS_CTSTARTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_CTSTARTED_SPEC>;
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
impl From<crate::W<EVENTS_CTSTARTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_CTSTARTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_CTSTARTED` reader - "]
pub type EVENTS_CTSTARTED_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_CTSTARTED` writer - "]
pub type EVENTS_CTSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_CTSTARTED_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_ctstarted(&self) -> EVENTS_CTSTARTED_R {
        EVENTS_CTSTARTED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_ctstarted(&mut self) -> EVENTS_CTSTARTED_W<0> {
        EVENTS_CTSTARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration timer has been started and is ready to process new tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ctstarted](index.html) module"]
pub struct EVENTS_CTSTARTED_SPEC;
impl crate::RegisterSpec for EVENTS_CTSTARTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_ctstarted::R](R) reader structure"]
impl crate::Readable for EVENTS_CTSTARTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_ctstarted::W](W) writer structure"]
impl crate::Writable for EVENTS_CTSTARTED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_CTSTARTED to value 0"]
impl crate::Resettable for EVENTS_CTSTARTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
