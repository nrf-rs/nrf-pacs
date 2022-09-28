#[doc = "Register `EVENTS_PERIPHACCERR` reader"]
pub struct R(crate::R<EVENTS_PERIPHACCERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_PERIPHACCERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_PERIPHACCERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_PERIPHACCERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_PERIPHACCERR` writer"]
pub struct W(crate::W<EVENTS_PERIPHACCERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_PERIPHACCERR_SPEC>;
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
impl From<crate::W<EVENTS_PERIPHACCERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_PERIPHACCERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_PERIPHACCERR` reader - A security violation has been detected on one or several peripherals"]
pub type EVENTS_PERIPHACCERR_R = crate::BitReader<EVENTS_PERIPHACCERR_A>;
#[doc = "A security violation has been detected on one or several peripherals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_PERIPHACCERR_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_PERIPHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_PERIPHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_PERIPHACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_PERIPHACCERR_A {
        match self.bits {
            false => EVENTS_PERIPHACCERR_A::NOT_GENERATED,
            true => EVENTS_PERIPHACCERR_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_PERIPHACCERR_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_PERIPHACCERR_A::GENERATED
    }
}
#[doc = "Field `EVENTS_PERIPHACCERR` writer - A security violation has been detected on one or several peripherals"]
pub type EVENTS_PERIPHACCERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_PERIPHACCERR_SPEC, EVENTS_PERIPHACCERR_A, O>;
impl<'a, const O: u8> EVENTS_PERIPHACCERR_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_PERIPHACCERR_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_PERIPHACCERR_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&self) -> EVENTS_PERIPHACCERR_R {
        EVENTS_PERIPHACCERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&mut self) -> EVENTS_PERIPHACCERR_W<0> {
        EVENTS_PERIPHACCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A security violation has been detected on one or several peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_periphaccerr](index.html) module"]
pub struct EVENTS_PERIPHACCERR_SPEC;
impl crate::RegisterSpec for EVENTS_PERIPHACCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_periphaccerr::R](R) reader structure"]
impl crate::Readable for EVENTS_PERIPHACCERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_periphaccerr::W](W) writer structure"]
impl crate::Writable for EVENTS_PERIPHACCERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_PERIPHACCERR to value 0"]
impl crate::Resettable for EVENTS_PERIPHACCERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
