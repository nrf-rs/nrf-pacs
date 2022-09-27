#[doc = "Register `EVENTS_NCTS` reader"]
pub struct R(crate::R<EVENTS_NCTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_NCTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_NCTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_NCTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_NCTS` writer"]
pub struct W(crate::W<EVENTS_NCTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_NCTS_SPEC>;
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
impl From<crate::W<EVENTS_NCTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_NCTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_NCTS` reader - "]
pub type EVENTS_NCTS_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_NCTS` writer - "]
pub type EVENTS_NCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTS_NCTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_ncts(&self) -> EVENTS_NCTS_R {
        EVENTS_NCTS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_ncts(&mut self) -> EVENTS_NCTS_W<0> {
        EVENTS_NCTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ncts](index.html) module"]
pub struct EVENTS_NCTS_SPEC;
impl crate::RegisterSpec for EVENTS_NCTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_ncts::R](R) reader structure"]
impl crate::Readable for EVENTS_NCTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_ncts::W](W) writer structure"]
impl crate::Writable for EVENTS_NCTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_NCTS to value 0"]
impl crate::Resettable for EVENTS_NCTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
