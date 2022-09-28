#[doc = "Register `EVENTS_EPDATA` reader"]
pub struct R(crate::R<EVENTS_EPDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_EPDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_EPDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_EPDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_EPDATA` writer"]
pub struct W(crate::W<EVENTS_EPDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_EPDATA_SPEC>;
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
impl From<crate::W<EVENTS_EPDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_EPDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_EPDATA` reader - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub type EVENTS_EPDATA_R = crate::BitReader<EVENTS_EPDATA_A>;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_EPDATA_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_EPDATA_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_EPDATA_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_EPDATA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_EPDATA_A {
        match self.bits {
            false => EVENTS_EPDATA_A::NOT_GENERATED,
            true => EVENTS_EPDATA_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_EPDATA_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_EPDATA_A::GENERATED
    }
}
#[doc = "Field `EVENTS_EPDATA` writer - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub type EVENTS_EPDATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_EPDATA_SPEC, EVENTS_EPDATA_A, O>;
impl<'a, const O: u8> EVENTS_EPDATA_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_EPDATA_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_EPDATA_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub fn events_epdata(&self) -> EVENTS_EPDATA_R {
        EVENTS_EPDATA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub fn events_epdata(&mut self) -> EVENTS_EPDATA_W<0> {
        EVENTS_EPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_epdata](index.html) module"]
pub struct EVENTS_EPDATA_SPEC;
impl crate::RegisterSpec for EVENTS_EPDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_epdata::R](R) reader structure"]
impl crate::Readable for EVENTS_EPDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_epdata::W](W) writer structure"]
impl crate::Writable for EVENTS_EPDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_EPDATA to value 0"]
impl crate::Resettable for EVENTS_EPDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
