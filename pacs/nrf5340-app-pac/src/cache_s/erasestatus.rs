#[doc = "Register `ERASESTATUS` reader"]
pub struct R(crate::R<ERASESTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASESTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASESTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASESTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASESTATUS` writer"]
pub struct W(crate::W<ERASESTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASESTATUS_SPEC>;
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
impl From<crate::W<ERASESTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASESTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASESTATUS` reader - Cache erase status"]
pub type ERASESTATUS_R = crate::BitReader<ERASESTATUS_A>;
#[doc = "Cache erase status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASESTATUS_A {
    #[doc = "0: Erase is not complete or hasn't started"]
    IDLE = 0,
    #[doc = "1: Cache erase is finished"]
    FINISHED = 1,
}
impl From<ERASESTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ERASESTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASESTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASESTATUS_A {
        match self.bits {
            false => ERASESTATUS_A::IDLE,
            true => ERASESTATUS_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ERASESTATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == ERASESTATUS_A::FINISHED
    }
}
#[doc = "Field `ERASESTATUS` writer - Cache erase status"]
pub type ERASESTATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ERASESTATUS_SPEC, ERASESTATUS_A, O>;
impl<'a, const O: u8> ERASESTATUS_W<'a, O> {
    #[doc = "Erase is not complete or hasn't started"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(ERASESTATUS_A::IDLE)
    }
    #[doc = "Cache erase is finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(ERASESTATUS_A::FINISHED)
    }
}
impl R {
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&self) -> ERASESTATUS_R {
        ERASESTATUS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&mut self) -> ERASESTATUS_W<0> {
        ERASESTATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache erase status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasestatus](index.html) module"]
pub struct ERASESTATUS_SPEC;
impl crate::RegisterSpec for ERASESTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erasestatus::R](R) reader structure"]
impl crate::Readable for ERASESTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erasestatus::W](W) writer structure"]
impl crate::Writable for ERASESTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASESTATUS to value 0"]
impl crate::Resettable for ERASESTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
