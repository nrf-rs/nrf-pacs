#[doc = "Register `PERM` reader"]
pub struct R(crate::R<PERM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERM` writer"]
pub struct W(crate::W<PERM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERM_SPEC>;
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
impl From<crate::W<PERM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECUREMAPPING` reader - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
pub type SECUREMAPPING_R = crate::FieldReader<u8, SECUREMAPPING_A>;
#[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECUREMAPPING_A {
    #[doc = "0: The bus access from this external domain always have the non-secure attribute set"]
    NON_SECURE = 0,
    #[doc = "1: The bus access from this external domain always have the secure attribute set"]
    SECURE = 1,
    #[doc = "2: Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    USER_SELECTABLE = 2,
}
impl From<SECUREMAPPING_A> for u8 {
    #[inline(always)]
    fn from(variant: SECUREMAPPING_A) -> Self {
        variant as _
    }
}
impl SECUREMAPPING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SECUREMAPPING_A> {
        match self.bits {
            0 => Some(SECUREMAPPING_A::NON_SECURE),
            1 => Some(SECUREMAPPING_A::SECURE),
            2 => Some(SECUREMAPPING_A::USER_SELECTABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECUREMAPPING_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECUREMAPPING_A::SECURE
    }
    #[doc = "Checks if the value of the field is `USER_SELECTABLE`"]
    #[inline(always)]
    pub fn is_user_selectable(&self) -> bool {
        *self == SECUREMAPPING_A::USER_SELECTABLE
    }
}
#[doc = "Field `SECATTR` reader - Peripheral security mapping"]
pub type SECATTR_R = crate::BitReader<SECATTR_A>;
#[doc = "Peripheral security mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "0: Bus accesses from this domain have the non-secure attribute set"]
    NON_SECURE = 0,
    #[doc = "1: Bus accesses from this domain have secure attribute set"]
    SECURE = 1,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SECATTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECATTR_A {
        match self.bits {
            false => SECATTR_A::NON_SECURE,
            true => SECATTR_A::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTR_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECATTR_A::SECURE
    }
}
#[doc = "Field `SECATTR` writer - Peripheral security mapping"]
pub type SECATTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, SECATTR_A, O>;
impl<'a, const O: u8> SECATTR_W<'a, O> {
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NON_SECURE)
    }
    #[doc = "Bus accesses from this domain have secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTR_A::SECURE)
    }
}
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
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, LOCK_A, O>;
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
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SECUREMAPPING_R {
        SECUREMAPPING_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SECATTR_R {
        SECATTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SECATTR_W<4> {
        SECATTR_W::new(self)
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
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
pub struct PERM_SPEC;
impl crate::RegisterSpec for PERM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perm::R](R) reader structure"]
impl crate::Readable for PERM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perm::W](W) writer structure"]
impl crate::Writable for PERM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERM to value 0"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
