#[doc = "Register `REGION` reader"]
pub struct R(crate::R<REGION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION` writer"]
pub struct W(crate::W<REGION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_SPEC>;
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
impl From<crate::W<REGION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION` reader - Region number"]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - Region number"]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REGION_SPEC, u8, u8, 5, O>;
#[doc = "Field `LOCK` reader - "]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This register can be updated"]
    UNLOCKED = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCK` writer - "]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGION_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Region number"]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Region number"]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region](index.html) module"]
pub struct REGION_SPEC;
impl crate::RegisterSpec for REGION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region::R](R) reader structure"]
impl crate::Readable for REGION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region::W](W) writer structure"]
impl crate::Writable for REGION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGION to value 0"]
impl crate::Resettable for REGION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
