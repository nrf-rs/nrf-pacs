#[doc = "Register `EVENTS_BB` reader"]
pub struct R(crate::R<EVENTS_BB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_BB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_BB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_BB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_BB` writer"]
pub struct W(crate::W<EVENTS_BB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_BB_SPEC>;
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
impl From<crate::W<EVENTS_BB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_BB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_BB` reader - TWI byte boundary, generated before each byte that is sent or received"]
pub type EVENTS_BB_R = crate::BitReader<EVENTS_BB_A>;
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_BB_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_BB_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_BB_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_BB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_BB_A {
        match self.bits {
            false => EVENTS_BB_A::NOT_GENERATED,
            true => EVENTS_BB_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_BB_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_BB_A::GENERATED
    }
}
#[doc = "Field `EVENTS_BB` writer - TWI byte boundary, generated before each byte that is sent or received"]
pub type EVENTS_BB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENTS_BB_SPEC, EVENTS_BB_A, O>;
impl<'a, const O: u8> EVENTS_BB_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_BB_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_BB_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub fn events_bb(&self) -> EVENTS_BB_R {
        EVENTS_BB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub fn events_bb(&mut self) -> EVENTS_BB_W<0> {
        EVENTS_BB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI byte boundary, generated before each byte that is sent or received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_bb](index.html) module"]
pub struct EVENTS_BB_SPEC;
impl crate::RegisterSpec for EVENTS_BB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_bb::R](R) reader structure"]
impl crate::Readable for EVENTS_BB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_bb::W](W) writer structure"]
impl crate::Writable for EVENTS_BB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_BB to value 0"]
impl crate::Resettable for EVENTS_BB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
