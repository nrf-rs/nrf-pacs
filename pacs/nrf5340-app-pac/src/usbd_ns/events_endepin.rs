#[doc = "Register `EVENTS_ENDEPIN[%s]` reader"]
pub struct R(crate::R<EVENTS_ENDEPIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_ENDEPIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_ENDEPIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_ENDEPIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_ENDEPIN[%s]` writer"]
pub struct W(crate::W<EVENTS_ENDEPIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_ENDEPIN_SPEC>;
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
impl From<crate::W<EVENTS_ENDEPIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_ENDEPIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_ENDEPIN` reader - The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
pub type EVENTS_ENDEPIN_R = crate::BitReader<EVENTS_ENDEPIN_A>;
#[doc = "The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDEPIN_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ENDEPIN_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ENDEPIN_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_ENDEPIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ENDEPIN_A {
        match self.bits {
            false => EVENTS_ENDEPIN_A::NOT_GENERATED,
            true => EVENTS_ENDEPIN_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ENDEPIN_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ENDEPIN_A::GENERATED
    }
}
#[doc = "Field `EVENTS_ENDEPIN` writer - The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
pub type EVENTS_ENDEPIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_ENDEPIN_SPEC, EVENTS_ENDEPIN_A, O>;
impl<'a, const O: u8> EVENTS_ENDEPIN_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPIN_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPIN_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin(&self) -> EVENTS_ENDEPIN_R {
        EVENTS_ENDEPIN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin(&mut self) -> EVENTS_ENDEPIN_W<0> {
        EVENTS_ENDEPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: The whole EPIN\\[n\\]
buffer has been consumed. The buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endepin](index.html) module"]
pub struct EVENTS_ENDEPIN_SPEC;
impl crate::RegisterSpec for EVENTS_ENDEPIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_endepin::R](R) reader structure"]
impl crate::Readable for EVENTS_ENDEPIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_endepin::W](W) writer structure"]
impl crate::Writable for EVENTS_ENDEPIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_ENDEPIN[%s]
to value 0"]
impl crate::Resettable for EVENTS_ENDEPIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
