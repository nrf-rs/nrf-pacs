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
#[doc = "Field `EXECUTE` reader - Configure instruction fetch permissions from flash region n"]
pub type EXECUTE_R = crate::BitReader<EXECUTE_A>;
#[doc = "Configure instruction fetch permissions from flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXECUTE_A {
    #[doc = "1: Allow instruction fetches from flash region n"]
    ENABLE = 1,
    #[doc = "0: Block instruction fetches from flash region n"]
    DISABLE = 0,
}
impl From<EXECUTE_A> for bool {
    #[inline(always)]
    fn from(variant: EXECUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl EXECUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXECUTE_A {
        match self.bits {
            true => EXECUTE_A::ENABLE,
            false => EXECUTE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXECUTE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXECUTE_A::DISABLE
    }
}
#[doc = "Field `EXECUTE` writer - Configure instruction fetch permissions from flash region n"]
pub type EXECUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, EXECUTE_A, O>;
impl<'a, const O: u8> EXECUTE_W<'a, O> {
    #[doc = "Allow instruction fetches from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXECUTE_A::ENABLE)
    }
    #[doc = "Block instruction fetches from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXECUTE_A::DISABLE)
    }
}
#[doc = "Field `WRITE` reader - Configure write permission for flash region n"]
pub type WRITE_R = crate::BitReader<WRITE_A>;
#[doc = "Configure write permission for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "1: Allow write operation to region n"]
    ENABLE = 1,
    #[doc = "0: Block write operation to region n"]
    DISABLE = 0,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            true => WRITE_A::ENABLE,
            false => WRITE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WRITE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WRITE_A::DISABLE
    }
}
#[doc = "Field `WRITE` writer - Configure write permission for flash region n"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, WRITE_A, O>;
impl<'a, const O: u8> WRITE_W<'a, O> {
    #[doc = "Allow write operation to region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLE)
    }
    #[doc = "Block write operation to region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLE)
    }
}
#[doc = "Field `READ` reader - Configure read permissions for flash region n"]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "Configure read permissions for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "1: Allow read operation from flash region n"]
    ENABLE = 1,
    #[doc = "0: Block read operation from flash region n"]
    DISABLE = 0,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            true => READ_A::ENABLE,
            false => READ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == READ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == READ_A::DISABLE
    }
}
#[doc = "Field `READ` writer - Configure read permissions for flash region n"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, READ_A, O>;
impl<'a, const O: u8> READ_W<'a, O> {
    #[doc = "Allow read operation from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(READ_A::ENABLE)
    }
    #[doc = "Block read operation from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(READ_A::DISABLE)
    }
}
#[doc = "Field `SECATTR` reader - Security attribute for flash region n"]
pub type SECATTR_R = crate::BitReader<SECATTR_A>;
#[doc = "Security attribute for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "0: Flash region n security attribute is non-secure"]
    NON_SECURE = 0,
    #[doc = "1: Flash region n security attribute is secure"]
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
#[doc = "Field `SECATTR` writer - Security attribute for flash region n"]
pub type SECATTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERM_SPEC, SECATTR_A, O>;
impl<'a, const O: u8> SECATTR_W<'a, O> {
    #[doc = "Flash region n security attribute is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NON_SECURE)
    }
    #[doc = "Flash region n security attribute is secure"]
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
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn execute(&self) -> EXECUTE_R {
        EXECUTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
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
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn execute(&mut self) -> EXECUTE_W<0> {
        EXECUTE_W::new(self)
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<2> {
        READ_W::new(self)
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
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
#[doc = "Description cluster: Access permissions for flash region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](index.html) module"]
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
#[doc = "`reset()` method sets PERM to value 0x17"]
impl crate::Resettable for PERM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
