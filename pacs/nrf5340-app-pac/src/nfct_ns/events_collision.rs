#[doc = "Register `EVENTS_COLLISION` reader"]
pub struct R(crate::R<EVENTS_COLLISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_COLLISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_COLLISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_COLLISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_COLLISION` writer"]
pub struct W(crate::W<EVENTS_COLLISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_COLLISION_SPEC>;
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
impl From<crate::W<EVENTS_COLLISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_COLLISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_COLLISION` reader - NFC auto collision resolution error reported."]
pub type EVENTS_COLLISION_R = crate::BitReader<EVENTS_COLLISION_A>;
#[doc = "NFC auto collision resolution error reported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_COLLISION_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_COLLISION_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_COLLISION_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_COLLISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_COLLISION_A {
        match self.bits {
            false => EVENTS_COLLISION_A::NOT_GENERATED,
            true => EVENTS_COLLISION_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_COLLISION_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_COLLISION_A::GENERATED
    }
}
#[doc = "Field `EVENTS_COLLISION` writer - NFC auto collision resolution error reported."]
pub type EVENTS_COLLISION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_COLLISION_SPEC, EVENTS_COLLISION_A, O>;
impl<'a, const O: u8> EVENTS_COLLISION_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_COLLISION_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_COLLISION_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - NFC auto collision resolution error reported."]
    #[inline(always)]
    pub fn events_collision(&self) -> EVENTS_COLLISION_R {
        EVENTS_COLLISION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NFC auto collision resolution error reported."]
    #[inline(always)]
    pub fn events_collision(&mut self) -> EVENTS_COLLISION_W<0> {
        EVENTS_COLLISION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC auto collision resolution error reported.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_collision](index.html) module"]
pub struct EVENTS_COLLISION_SPEC;
impl crate::RegisterSpec for EVENTS_COLLISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_collision::R](R) reader structure"]
impl crate::Readable for EVENTS_COLLISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_collision::W](W) writer structure"]
impl crate::Writable for EVENTS_COLLISION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_COLLISION to value 0"]
impl crate::Resettable for EVENTS_COLLISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
