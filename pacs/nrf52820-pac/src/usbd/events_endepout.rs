#[doc = "Register `EVENTS_ENDEPOUT[%s]` reader"]
pub struct R(crate::R<EVENTS_ENDEPOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_ENDEPOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_ENDEPOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_ENDEPOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_ENDEPOUT[%s]` writer"]
pub struct W(crate::W<EVENTS_ENDEPOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_ENDEPOUT_SPEC>;
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
impl From<crate::W<EVENTS_ENDEPOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_ENDEPOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_ENDEPOUT` reader - The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
pub type EVENTS_ENDEPOUT_R = crate::BitReader<EVENTS_ENDEPOUT_A>;
#[doc = "The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDEPOUT_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ENDEPOUT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ENDEPOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_ENDEPOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ENDEPOUT_A {
        match self.bits {
            false => EVENTS_ENDEPOUT_A::NOT_GENERATED,
            true => EVENTS_ENDEPOUT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ENDEPOUT_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ENDEPOUT_A::GENERATED
    }
}
#[doc = "Field `EVENTS_ENDEPOUT` writer - The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
pub type EVENTS_ENDEPOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_ENDEPOUT_SPEC, EVENTS_ENDEPOUT_A, O>;
impl<'a, const O: u8> EVENTS_ENDEPOUT_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPOUT_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPOUT_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&self) -> EVENTS_ENDEPOUT_R {
        EVENTS_ENDEPOUT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&mut self) -> EVENTS_ENDEPOUT_W<0> {
        EVENTS_ENDEPOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: The whole EPOUT\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endepout](index.html) module"]
pub struct EVENTS_ENDEPOUT_SPEC;
impl crate::RegisterSpec for EVENTS_ENDEPOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_endepout::R](R) reader structure"]
impl crate::Readable for EVENTS_ENDEPOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_endepout::W](W) writer structure"]
impl crate::Writable for EVENTS_ENDEPOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_ENDEPOUT[%s]
to value 0"]
impl crate::Resettable for EVENTS_ENDEPOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
