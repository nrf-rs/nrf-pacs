#[doc = "Register `EVENTS_DONE` reader"]
pub struct R(crate::R<EVENTS_DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_DONE` writer"]
pub struct W(crate::W<EVENTS_DONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_DONE_SPEC>;
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
impl From<crate::W<EVENTS_DONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_DONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_DONE` reader - "]
pub type EVENTS_DONE_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_DONE` writer - "]
pub type EVENTS_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTS_DONE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_done(&self) -> EVENTS_DONE_R {
        EVENTS_DONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_done(&mut self) -> EVENTS_DONE_W<0> {
        EVENTS_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_done](index.html) module"]
pub struct EVENTS_DONE_SPEC;
impl crate::RegisterSpec for EVENTS_DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_done::R](R) reader structure"]
impl crate::Readable for EVENTS_DONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_done::W](W) writer structure"]
impl crate::Writable for EVENTS_DONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_DONE to value 0"]
impl crate::Resettable for EVENTS_DONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
