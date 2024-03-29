#[doc = "Register `EVENTS_RESULTDONE` reader"]
pub struct R(crate::R<EVENTS_RESULTDONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_RESULTDONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_RESULTDONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_RESULTDONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_RESULTDONE` writer"]
pub struct W(crate::W<EVENTS_RESULTDONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_RESULTDONE_SPEC>;
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
impl From<crate::W<EVENTS_RESULTDONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_RESULTDONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_RESULTDONE` reader - "]
pub type EVENTS_RESULTDONE_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_RESULTDONE` writer - "]
pub type EVENTS_RESULTDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_RESULTDONE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_resultdone(&self) -> EVENTS_RESULTDONE_R {
        EVENTS_RESULTDONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_resultdone(&mut self) -> EVENTS_RESULTDONE_W<0> {
        EVENTS_RESULTDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result ready for transfer to RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_resultdone](index.html) module"]
pub struct EVENTS_RESULTDONE_SPEC;
impl crate::RegisterSpec for EVENTS_RESULTDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_resultdone::R](R) reader structure"]
impl crate::Readable for EVENTS_RESULTDONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_resultdone::W](W) writer structure"]
impl crate::Writable for EVENTS_RESULTDONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_RESULTDONE to value 0"]
impl crate::Resettable for EVENTS_RESULTDONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
