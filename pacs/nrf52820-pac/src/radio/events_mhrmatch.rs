#[doc = "Register `EVENTS_MHRMATCH` reader"]
pub struct R(crate::R<EVENTS_MHRMATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_MHRMATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_MHRMATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_MHRMATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_MHRMATCH` writer"]
pub struct W(crate::W<EVENTS_MHRMATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_MHRMATCH_SPEC>;
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
impl From<crate::W<EVENTS_MHRMATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_MHRMATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_MHRMATCH` reader - MAC header match found"]
pub type EVENTS_MHRMATCH_R = crate::BitReader<EVENTS_MHRMATCH_A>;
#[doc = "MAC header match found\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_MHRMATCH_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_MHRMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_MHRMATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_MHRMATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_MHRMATCH_A {
        match self.bits {
            false => EVENTS_MHRMATCH_A::NOT_GENERATED,
            true => EVENTS_MHRMATCH_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_MHRMATCH_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_MHRMATCH_A::GENERATED
    }
}
#[doc = "Field `EVENTS_MHRMATCH` writer - MAC header match found"]
pub type EVENTS_MHRMATCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_MHRMATCH_SPEC, EVENTS_MHRMATCH_A, O>;
impl<'a, const O: u8> EVENTS_MHRMATCH_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_MHRMATCH_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_MHRMATCH_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - MAC header match found"]
    #[inline(always)]
    pub fn events_mhrmatch(&self) -> EVENTS_MHRMATCH_R {
        EVENTS_MHRMATCH_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC header match found"]
    #[inline(always)]
    pub fn events_mhrmatch(&mut self) -> EVENTS_MHRMATCH_W<0> {
        EVENTS_MHRMATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC header match found\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_mhrmatch](index.html) module"]
pub struct EVENTS_MHRMATCH_SPEC;
impl crate::RegisterSpec for EVENTS_MHRMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_mhrmatch::R](R) reader structure"]
impl crate::Readable for EVENTS_MHRMATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_mhrmatch::W](W) writer structure"]
impl crate::Writable for EVENTS_MHRMATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_MHRMATCH to value 0"]
impl crate::Resettable for EVENTS_MHRMATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
