#[doc = "Register `MUTEX[%s]` reader"]
pub struct R(crate::R<MUTEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUTEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUTEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUTEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUTEX[%s]` writer"]
pub struct W(crate::W<MUTEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUTEX_SPEC>;
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
impl From<crate::W<MUTEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUTEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mutex register n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEX_A {
    #[doc = "0: Mutex n is in unlocked state"]
    UNLOCKED = 0,
    #[doc = "1: Mutex n is in locked state"]
    LOCKED = 1,
}
impl From<MUTEX_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEX` reader - Mutex register n"]
pub struct MUTEX_R(crate::FieldReader<bool, MUTEX_A>);
impl MUTEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MUTEX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEX_A {
        match self.bits {
            false => MUTEX_A::UNLOCKED,
            true => MUTEX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == MUTEX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == MUTEX_A::LOCKED
    }
}
impl core::ops::Deref for MUTEX_R {
    type Target = crate::FieldReader<bool, MUTEX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEX` writer - Mutex register n"]
pub struct MUTEX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTEX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mutex n is in unlocked state"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(MUTEX_A::UNLOCKED)
    }
    #[doc = "Mutex n is in locked state"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(MUTEX_A::LOCKED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&self) -> MUTEX_R {
        MUTEX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&mut self) -> MUTEX_W {
        MUTEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Mutex register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mutex](index.html) module"]
pub struct MUTEX_SPEC;
impl crate::RegisterSpec for MUTEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mutex::R](R) reader structure"]
impl crate::Readable for MUTEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mutex::W](W) writer structure"]
impl crate::Writable for MUTEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUTEX[%s]
to value 0"]
impl crate::Resettable for MUTEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
