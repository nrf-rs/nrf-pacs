#[doc = "Register `EVENTS_FLASHACCERR` reader"]
pub struct R(crate::R<EVENTS_FLASHACCERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_FLASHACCERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_FLASHACCERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_FLASHACCERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_FLASHACCERR` writer"]
pub struct W(crate::W<EVENTS_FLASHACCERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_FLASHACCERR_SPEC>;
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
impl From<crate::W<EVENTS_FLASHACCERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_FLASHACCERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_FLASHACCERR` reader - A security violation has been detected for the flash memory space"]
pub type EVENTS_FLASHACCERR_R = crate::BitReader<EVENTS_FLASHACCERR_A>;
#[doc = "A security violation has been detected for the flash memory space\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_FLASHACCERR_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_FLASHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_FLASHACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_FLASHACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_FLASHACCERR_A {
        match self.bits {
            false => EVENTS_FLASHACCERR_A::NOT_GENERATED,
            true => EVENTS_FLASHACCERR_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_FLASHACCERR_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_FLASHACCERR_A::GENERATED
    }
}
#[doc = "Field `EVENTS_FLASHACCERR` writer - A security violation has been detected for the flash memory space"]
pub type EVENTS_FLASHACCERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_FLASHACCERR_SPEC, EVENTS_FLASHACCERR_A, O>;
impl<'a, const O: u8> EVENTS_FLASHACCERR_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_FLASHACCERR_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_FLASHACCERR_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - A security violation has been detected for the flash memory space"]
    #[inline(always)]
    pub fn events_flashaccerr(&self) -> EVENTS_FLASHACCERR_R {
        EVENTS_FLASHACCERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected for the flash memory space"]
    #[inline(always)]
    pub fn events_flashaccerr(&mut self) -> EVENTS_FLASHACCERR_W<0> {
        EVENTS_FLASHACCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A security violation has been detected for the flash memory space\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_flashaccerr](index.html) module"]
pub struct EVENTS_FLASHACCERR_SPEC;
impl crate::RegisterSpec for EVENTS_FLASHACCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_flashaccerr::R](R) reader structure"]
impl crate::Readable for EVENTS_FLASHACCERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_flashaccerr::W](W) writer structure"]
impl crate::Writable for EVENTS_FLASHACCERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_FLASHACCERR to value 0"]
impl crate::Resettable for EVENTS_FLASHACCERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
