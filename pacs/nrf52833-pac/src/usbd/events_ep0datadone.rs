#[doc = "Register `EVENTS_EP0DATADONE` reader"]
pub struct R(crate::R<EVENTS_EP0DATADONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_EP0DATADONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_EP0DATADONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_EP0DATADONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_EP0DATADONE` writer"]
pub struct W(crate::W<EVENTS_EP0DATADONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_EP0DATADONE_SPEC>;
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
impl From<crate::W<EVENTS_EP0DATADONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_EP0DATADONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_EP0DATADONE` reader - An acknowledged data transfer has taken place on the control endpoint"]
pub type EVENTS_EP0DATADONE_R = crate::BitReader<EVENTS_EP0DATADONE_A>;
#[doc = "An acknowledged data transfer has taken place on the control endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_EP0DATADONE_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_EP0DATADONE_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_EP0DATADONE_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_EP0DATADONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_EP0DATADONE_A {
        match self.bits {
            false => EVENTS_EP0DATADONE_A::NOT_GENERATED,
            true => EVENTS_EP0DATADONE_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_EP0DATADONE_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_EP0DATADONE_A::GENERATED
    }
}
#[doc = "Field `EVENTS_EP0DATADONE` writer - An acknowledged data transfer has taken place on the control endpoint"]
pub type EVENTS_EP0DATADONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_EP0DATADONE_SPEC, EVENTS_EP0DATADONE_A, O>;
impl<'a, const O: u8> EVENTS_EP0DATADONE_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_EP0DATADONE_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_EP0DATADONE_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0datadone(&self) -> EVENTS_EP0DATADONE_R {
        EVENTS_EP0DATADONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0datadone(&mut self) -> EVENTS_EP0DATADONE_W<0> {
        EVENTS_EP0DATADONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "An acknowledged data transfer has taken place on the control endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ep0datadone](index.html) module"]
pub struct EVENTS_EP0DATADONE_SPEC;
impl crate::RegisterSpec for EVENTS_EP0DATADONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_ep0datadone::R](R) reader structure"]
impl crate::Readable for EVENTS_EP0DATADONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_ep0datadone::W](W) writer structure"]
impl crate::Writable for EVENTS_EP0DATADONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_EP0DATADONE to value 0"]
impl crate::Resettable for EVENTS_EP0DATADONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
