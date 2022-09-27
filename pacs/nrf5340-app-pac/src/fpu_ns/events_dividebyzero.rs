#[doc = "Register `EVENTS_DIVIDEBYZERO` reader"]
pub struct R(crate::R<EVENTS_DIVIDEBYZERO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_DIVIDEBYZERO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_DIVIDEBYZERO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_DIVIDEBYZERO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_DIVIDEBYZERO` writer"]
pub struct W(crate::W<EVENTS_DIVIDEBYZERO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_DIVIDEBYZERO_SPEC>;
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
impl From<crate::W<EVENTS_DIVIDEBYZERO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_DIVIDEBYZERO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_DIVIDEBYZERO` reader - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub type EVENTS_DIVIDEBYZERO_R = crate::BitReader<EVENTS_DIVIDEBYZERO_A>;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_DIVIDEBYZERO_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_DIVIDEBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_DIVIDEBYZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_DIVIDEBYZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_DIVIDEBYZERO_A {
        match self.bits {
            false => EVENTS_DIVIDEBYZERO_A::NOT_GENERATED,
            true => EVENTS_DIVIDEBYZERO_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_DIVIDEBYZERO_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_DIVIDEBYZERO_A::GENERATED
    }
}
#[doc = "Field `EVENTS_DIVIDEBYZERO` writer - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub type EVENTS_DIVIDEBYZERO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_DIVIDEBYZERO_SPEC, EVENTS_DIVIDEBYZERO_A, O>;
impl<'a, const O: u8> EVENTS_DIVIDEBYZERO_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_DIVIDEBYZERO_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_DIVIDEBYZERO_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&self) -> EVENTS_DIVIDEBYZERO_R {
        EVENTS_DIVIDEBYZERO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&mut self) -> EVENTS_DIVIDEBYZERO_W<0> {
        EVENTS_DIVIDEBYZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_dividebyzero](index.html) module"]
pub struct EVENTS_DIVIDEBYZERO_SPEC;
impl crate::RegisterSpec for EVENTS_DIVIDEBYZERO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_dividebyzero::R](R) reader structure"]
impl crate::Readable for EVENTS_DIVIDEBYZERO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_dividebyzero::W](W) writer structure"]
impl crate::Writable for EVENTS_DIVIDEBYZERO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_DIVIDEBYZERO to value 0"]
impl crate::Resettable for EVENTS_DIVIDEBYZERO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
