#[doc = "Register `EVENTS_COMPARE[%s]` reader"]
pub struct R(crate::R<EVENTS_COMPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_COMPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_COMPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_COMPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_COMPARE[%s]` writer"]
pub struct W(crate::W<EVENTS_COMPARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_COMPARE_SPEC>;
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
impl From<crate::W<EVENTS_COMPARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_COMPARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_COMPARE` reader - "]
pub type EVENTS_COMPARE_R = crate::BitReader<bool>;
#[doc = "Field `EVENTS_COMPARE` writer - "]
pub type EVENTS_COMPARE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_COMPARE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_compare(&self) -> EVENTS_COMPARE_R {
        EVENTS_COMPARE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_compare(&mut self) -> EVENTS_COMPARE_W<0> {
        EVENTS_COMPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection\\[n\\]: Compare event on CC\\[n\\]
match\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_compare](index.html) module"]
pub struct EVENTS_COMPARE_SPEC;
impl crate::RegisterSpec for EVENTS_COMPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_compare::R](R) reader structure"]
impl crate::Readable for EVENTS_COMPARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_compare::W](W) writer structure"]
impl crate::Writable for EVENTS_COMPARE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_COMPARE[%s]
to value 0"]
impl crate::Resettable for EVENTS_COMPARE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
