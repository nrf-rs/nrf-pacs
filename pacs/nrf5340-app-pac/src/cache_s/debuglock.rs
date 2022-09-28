#[doc = "Register `DEBUGLOCK` reader"]
pub struct R(crate::R<DEBUGLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUGLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUGLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUGLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUGLOCK` writer"]
pub struct W(crate::W<DEBUGLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUGLOCK_SPEC>;
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
impl From<crate::W<DEBUGLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUGLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUGLOCK` reader - Lock debug mode"]
pub type DEBUGLOCK_R = crate::BitReader<DEBUGLOCK_A>;
#[doc = "Lock debug mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGLOCK_A {
    #[doc = "0: Debug mode unlocked"]
    UNLOCKED = 0,
    #[doc = "1: Debug mode locked"]
    LOCKED = 1,
}
impl From<DEBUGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUGLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGLOCK_A {
        match self.bits {
            false => DEBUGLOCK_A::UNLOCKED,
            true => DEBUGLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DEBUGLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DEBUGLOCK_A::LOCKED
    }
}
#[doc = "Field `DEBUGLOCK` writer - Lock debug mode"]
pub type DEBUGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUGLOCK_SPEC, DEBUGLOCK_A, O>;
impl<'a, const O: u8> DEBUGLOCK_W<'a, O> {
    #[doc = "Debug mode unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DEBUGLOCK_A::UNLOCKED)
    }
    #[doc = "Debug mode locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DEBUGLOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&self) -> DEBUGLOCK_R {
        DEBUGLOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&mut self) -> DEBUGLOCK_W<0> {
        DEBUGLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock debug mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debuglock](index.html) module"]
pub struct DEBUGLOCK_SPEC;
impl crate::RegisterSpec for DEBUGLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debuglock::R](R) reader structure"]
impl crate::Readable for DEBUGLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debuglock::W](W) writer structure"]
impl crate::Writable for DEBUGLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUGLOCK to value 0"]
impl crate::Resettable for DEBUGLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
