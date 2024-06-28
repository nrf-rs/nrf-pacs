#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Securemapping {
    #[doc = "0: This peripheral is always accessible as a non-secure peripheral"]
    NonSecure = 0,
    #[doc = "1: This peripheral is always accessible as a secure peripheral"]
    Secure = 1,
    #[doc = "2: Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    UserSelectable = 2,
    #[doc = "3: This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    Split = 3,
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
    pub const fn variant(&self) -> Securemapping {
        match self.bits {
            0 => Securemapping::NonSecure,
            1 => Securemapping::Secure,
            2 => Securemapping::UserSelectable,
            3 => Securemapping::Split,
            _ => unreachable!(),
        }
    }
    #[doc = "This peripheral is always accessible as a non-secure peripheral"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Securemapping::NonSecure
    }
    #[doc = "This peripheral is always accessible as a secure peripheral"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Securemapping::Secure
    }
    #[doc = "Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    #[inline(always)]
    pub fn is_user_selectable(&self) -> bool {
        *self == Securemapping::UserSelectable
    }
    #[doc = "This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    #[inline(always)]
    pub fn is_split(&self) -> bool {
        *self == Securemapping::Split
    }
}
#[doc = "Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dma {
    #[doc = "0: Peripheral has no DMA capability"]
    NoDma = 0,
    #[doc = "1: Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    NoSeparateAttribute = 1,
    #[doc = "2: Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    SeparateAttribute = 2,
}
impl From<Dma> for u8 {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dma {
    type Ux = u8;
}
impl crate::IsEnum for Dma {}
#[doc = "Field `DMA` reader - Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
pub type DmaR = crate::FieldReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dma> {
        match self.bits {
            0 => Some(Dma::NoDma),
            1 => Some(Dma::NoSeparateAttribute),
            2 => Some(Dma::SeparateAttribute),
            _ => None,
        }
    }
    #[doc = "Peripheral has no DMA capability"]
    #[inline(always)]
    pub fn is_no_dma(&self) -> bool {
        *self == Dma::NoDma
    }
    #[doc = "Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    #[inline(always)]
    pub fn is_no_separate_attribute(&self) -> bool {
        *self == Dma::NoSeparateAttribute
    }
    #[doc = "Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    #[inline(always)]
    pub fn is_separate_attribute(&self) -> bool {
        *self == Dma::SeparateAttribute
    }
}
#[doc = "Peripheral security mapping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "1: Peripheral is mapped in secure peripheral address space"]
    Secure = 1,
    #[doc = "0: If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NonSecure = 0,
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
            true => Secattr::Secure,
            false => Secattr::NonSecure,
        }
    }
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
}
#[doc = "Field `SECATTR` writer - Peripheral security mapping"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
}
#[doc = "Security attribution for the DMA transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmasec {
    #[doc = "1: DMA transfers initiated by this peripheral have the secure attribute set"]
    Secure = 1,
    #[doc = "0: DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NonSecure = 0,
}
impl From<Dmasec> for bool {
    #[inline(always)]
    fn from(variant: Dmasec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEC` reader - Security attribution for the DMA transfer"]
pub type DmasecR = crate::BitReader<Dmasec>;
impl DmasecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmasec {
        match self.bits {
            true => Dmasec::Secure,
            false => Dmasec::NonSecure,
        }
    }
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Dmasec::Secure
    }
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Dmasec::NonSecure
    }
}
#[doc = "Field `DMASEC` writer - Security attribution for the DMA transfer"]
pub type DmasecW<'a, REG> = crate::BitWriter<'a, REG, Dmasec>;
impl<'a, REG> DmasecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasec::Secure)
    }
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Dmasec::NonSecure)
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
#[doc = "Indicate if a peripheral is present with ID n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Present {
    #[doc = "0: Peripheral is not present"]
    NotPresent = 0,
    #[doc = "1: Peripheral is present"]
    IsPresent = 1,
}
impl From<Present> for bool {
    #[inline(always)]
    fn from(variant: Present) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRESENT` reader - Indicate if a peripheral is present with ID n"]
pub type PresentR = crate::BitReader<Present>;
impl PresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Present {
        match self.bits {
            false => Present::NotPresent,
            true => Present::IsPresent,
        }
    }
    #[doc = "Peripheral is not present"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Present::NotPresent
    }
    #[doc = "Peripheral is present"]
    #[inline(always)]
    pub fn is_is_present(&self) -> bool {
        *self == Present::IsPresent
    }
}
impl R {
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SecuremappingR {
        SecuremappingR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn dmasec(&self) -> DmasecR {
        DmasecR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicate if a peripheral is present with ID n"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    #[must_use]
    pub fn secattr(&mut self) -> SecattrW<PermSpec> {
        SecattrW::new(self, 4)
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmasec(&mut self) -> DmasecW<PermSpec> {
        DmasecW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<PermSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PERM to value 0x12"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0x12;
}
