#[doc = "Register `EVENTS_END` reader"]
pub struct R(crate::R<EVENTS_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_END` writer"]
pub struct W(crate::W<EVENTS_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_END_SPEC>;
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
impl From<crate::W<EVENTS_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_END` reader - "]
pub type EVENTS_END_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_END` writer - "]
pub type EVENTS_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTS_END_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_end(&self) -> EVENTS_END_R {
        EVENTS_END_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_end(&mut self) -> EVENTS_END_W<0> {
        EVENTS_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_end](index.html) module"]
pub struct EVENTS_END_SPEC;
impl crate::RegisterSpec for EVENTS_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_end::R](R) reader structure"]
impl crate::Readable for EVENTS_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_end::W](W) writer structure"]
impl crate::Writable for EVENTS_END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_END to value 0"]
impl crate::Resettable for EVENTS_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
