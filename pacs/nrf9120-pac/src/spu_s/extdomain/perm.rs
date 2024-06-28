#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Securemapping {
    #[doc = "0: The bus access from this external domain always have the non-secure attribute set"]
    NonSecure = 0,
    #[doc = "1: The bus access from this external domain always have the secure attribute set"]
    Secure = 1,
    #[doc = "2: Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    UserSelectable = 2,
}
impl From<Securemapping> for u8 {
    #[inline(always)]
    fn from(variant: Securemapping) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Securemapping {
    type Ux = u8;
}
impl crate::IsEnum for Securemapping {}
#[doc = "Field `SECUREMAPPING` reader - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
pub type SecuremappingR = crate::FieldReader<Securemapping>;
impl SecuremappingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Securemapping> {
        match self.bits {
            0 => Some(Securemapping::NonSecure),
            1 => Some(Securemapping::Secure),
            2 => Some(Securemapping::UserSelectable),
            _ => None,
        }
    }
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Securemapping::NonSecure
    }
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Securemapping::Secure
    }
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    #[inline(always)]
    pub fn is_user_selectable(&self) -> bool {
        *self == Securemapping::UserSelectable
    }
}
#[doc = "Peripheral security mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "0: Bus accesses from this domain have the non-secure attribute set"]
    NonSecure = 0,
    #[doc = "1: Bus accesses from this domain have secure attribute set"]
    Secure = 1,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Peripheral security mapping"]
pub type SecattrR = crate::BitReader<Secattr>;
impl SecattrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secattr {
        match self.bits {
            false => Secattr::NonSecure,
            true => Secattr::Secure,
        }
    }
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
    #[doc = "Bus accesses from this domain have secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
}
#[doc = "Field `SECATTR` writer - Peripheral security mapping"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
    #[doc = "Bus accesses from this domain have secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: This register can be updated"]
    Unlocked = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
impl R {
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SecuremappingR {
        SecuremappingR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    #[must_use]
    pub fn secattr(&mut self) -> SecattrW<PermSpec> {
        SecattrW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<PermSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERM to value 0"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0;
}
