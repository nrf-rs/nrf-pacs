#[doc = "Register `EVENTS_RECEIVE[%s]` reader"]
pub struct R(crate::R<EVENTS_RECEIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_RECEIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_RECEIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_RECEIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_RECEIVE[%s]` writer"]
pub struct W(crate::W<EVENTS_RECEIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_RECEIVE_SPEC>;
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
impl From<crate::W<EVENTS_RECEIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_RECEIVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_RECEIVE` reader - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub type EVENTS_RECEIVE_R = crate::BitReader<EVENTS_RECEIVE_A>;
#[doc = "Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RECEIVE_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_RECEIVE_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_RECEIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_RECEIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_RECEIVE_A {
        match self.bits {
            false => EVENTS_RECEIVE_A::NOT_GENERATED,
            true => EVENTS_RECEIVE_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_RECEIVE_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_RECEIVE_A::GENERATED
    }
}
#[doc = "Field `EVENTS_RECEIVE` writer - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub type EVENTS_RECEIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_RECEIVE_SPEC, EVENTS_RECEIVE_A, O>;
impl<'a, const O: u8> EVENTS_RECEIVE_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_RECEIVE_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_RECEIVE_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub fn events_receive(&self) -> EVENTS_RECEIVE_R {
        EVENTS_RECEIVE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub fn events_receive(&mut self) -> EVENTS_RECEIVE_W<0> {
        EVENTS_RECEIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_receive](index.html) module"]
pub struct EVENTS_RECEIVE_SPEC;
impl crate::RegisterSpec for EVENTS_RECEIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_receive::R](R) reader structure"]
impl crate::Readable for EVENTS_RECEIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_receive::W](W) writer structure"]
impl crate::Writable for EVENTS_RECEIVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_RECEIVE[%s]
to value 0"]
impl crate::Resettable for EVENTS_RECEIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
