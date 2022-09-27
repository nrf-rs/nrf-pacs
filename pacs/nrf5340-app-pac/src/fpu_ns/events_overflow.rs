#[doc = "Register `EVENTS_OVERFLOW` reader"]
pub struct R(crate::R<EVENTS_OVERFLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_OVERFLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_OVERFLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_OVERFLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_OVERFLOW` writer"]
pub struct W(crate::W<EVENTS_OVERFLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_OVERFLOW_SPEC>;
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
impl From<crate::W<EVENTS_OVERFLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_OVERFLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_OVERFLOW` reader - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub type EVENTS_OVERFLOW_R = crate::BitReader<EVENTS_OVERFLOW_A>;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_OVERFLOW_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_OVERFLOW_A {
        match self.bits {
            false => EVENTS_OVERFLOW_A::NOT_GENERATED,
            true => EVENTS_OVERFLOW_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_OVERFLOW_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_OVERFLOW_A::GENERATED
    }
}
#[doc = "Field `EVENTS_OVERFLOW` writer - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub type EVENTS_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_OVERFLOW_SPEC, EVENTS_OVERFLOW_A, O>;
impl<'a, const O: u8> EVENTS_OVERFLOW_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_OVERFLOW_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_OVERFLOW_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_overflow(&self) -> EVENTS_OVERFLOW_R {
        EVENTS_OVERFLOW_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    #[inline(always)]
    pub fn events_overflow(&mut self) -> EVENTS_OVERFLOW_W<0> {
        EVENTS_OVERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_overflow](index.html) module"]
pub struct EVENTS_OVERFLOW_SPEC;
impl crate::RegisterSpec for EVENTS_OVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_overflow::R](R) reader structure"]
impl crate::Readable for EVENTS_OVERFLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_overflow::W](W) writer structure"]
impl crate::Writable for EVENTS_OVERFLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_OVERFLOW to value 0"]
impl crate::Resettable for EVENTS_OVERFLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
