#[doc = "Register `EVENTS_SLEEPENTER` reader"]
pub struct R(crate::R<EVENTS_SLEEPENTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_SLEEPENTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_SLEEPENTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_SLEEPENTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_SLEEPENTER` writer"]
pub struct W(crate::W<EVENTS_SLEEPENTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_SLEEPENTER_SPEC>;
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
impl From<crate::W<EVENTS_SLEEPENTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_SLEEPENTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_SLEEPENTER` reader - "]
pub type EVENTS_SLEEPENTER_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_SLEEPENTER` writer - "]
pub type EVENTS_SLEEPENTER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_SLEEPENTER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_sleepenter(&self) -> EVENTS_SLEEPENTER_R {
        EVENTS_SLEEPENTER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_sleepenter(&mut self) -> EVENTS_SLEEPENTER_W<0> {
        EVENTS_SLEEPENTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU entered WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sleepenter](index.html) module"]
pub struct EVENTS_SLEEPENTER_SPEC;
impl crate::RegisterSpec for EVENTS_SLEEPENTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_sleepenter::R](R) reader structure"]
impl crate::Readable for EVENTS_SLEEPENTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_sleepenter::W](W) writer structure"]
impl crate::Writable for EVENTS_SLEEPENTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_SLEEPENTER to value 0"]
impl crate::Resettable for EVENTS_SLEEPENTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
