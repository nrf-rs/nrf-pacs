#[doc = "Register `CPULOCK` reader"]
pub type R = crate::R<CpulockSpec>;
#[doc = "Register `CPULOCK` writer"]
pub type W = crate::W<CpulockSpec>;
#[doc = "Write '1' to prevent updating the secure interrupt configuration until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locksvtaircr {
    #[doc = "1: Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    Locked = 1,
    #[doc = "0: These registers can be updated"]
    Unlocked = 0,
}
impl From<Locksvtaircr> for bool {
    #[inline(always)]
    fn from(variant: Locksvtaircr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
pub type LocksvtaircrR = crate::BitReader<Locksvtaircr>;
impl LocksvtaircrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locksvtaircr {
        match self.bits {
            true => Locksvtaircr::Locked,
            false => Locksvtaircr::Unlocked,
        }
    }
    #[doc = "Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locksvtaircr::Locked
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Locksvtaircr::Unlocked
    }
}
#[doc = "Field `LOCKSVTAIRCR` writer - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
pub type LocksvtaircrW<'a, REG> = crate::BitWriter1S<'a, REG, Locksvtaircr>;
impl<'a, REG> LocksvtaircrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writes to the VTOR_S, AIRCR.PRIS, and AIRCR.BFHFNMINS registers"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksvtaircr::Locked)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksvtaircr::Unlocked)
    }
}
#[doc = "Write '1' to prevent updating the non-secure vector table base address until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locknsvtor {
    #[doc = "1: The address of the non-secure vector table is locked"]
    Locked = 1,
    #[doc = "0: The address of the non-secure vector table can be updated"]
    Unlocked = 0,
}
impl From<Locknsvtor> for bool {
    #[inline(always)]
    fn from(variant: Locknsvtor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSVTOR` reader - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
pub type LocknsvtorR = crate::BitReader<Locknsvtor>;
impl LocknsvtorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locknsvtor {
        match self.bits {
            true => Locknsvtor::Locked,
            false => Locknsvtor::Unlocked,
        }
    }
    #[doc = "The address of the non-secure vector table is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locknsvtor::Locked
    }
    #[doc = "The address of the non-secure vector table can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Locknsvtor::Unlocked
    }
}
#[doc = "Field `LOCKNSVTOR` writer - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
pub type LocknsvtorW<'a, REG> = crate::BitWriter1S<'a, REG, Locknsvtor>;
impl<'a, REG> LocknsvtorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address of the non-secure vector table is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locknsvtor::Locked)
    }
    #[doc = "The address of the non-secure vector table can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Locknsvtor::Unlocked)
    }
}
#[doc = "Write '1' to prevent updating the secure MPU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locksmpu {
    #[doc = "1: Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    Locked = 1,
    #[doc = "0: These registers can be updated"]
    Unlocked = 0,
}
impl From<Locksmpu> for bool {
    #[inline(always)]
    fn from(variant: Locksmpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSMPU` reader - Write '1' to prevent updating the secure MPU regions until the next reset"]
pub type LocksmpuR = crate::BitReader<Locksmpu>;
impl LocksmpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locksmpu {
        match self.bits {
            true => Locksmpu::Locked,
            false => Locksmpu::Unlocked,
        }
    }
    #[doc = "Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locksmpu::Locked
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Locksmpu::Unlocked
    }
}
#[doc = "Field `LOCKSMPU` writer - Write '1' to prevent updating the secure MPU regions until the next reset"]
pub type LocksmpuW<'a, REG> = crate::BitWriter1S<'a, REG, Locksmpu>;
impl<'a, REG> LocksmpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writes to the MPU_CTRL, MPU_RNR, MPU_RBAR, MPU_RLAR, MPU_RBAR_An and MPU_RLAR_An from software or from a debug agent connected to the processor in Secure state"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksmpu::Locked)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksmpu::Unlocked)
    }
}
#[doc = "Write '1' to prevent updating the Non-secure MPU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locknsmpu {
    #[doc = "1: Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    Locked = 1,
    #[doc = "0: These registers can be updated"]
    Unlocked = 0,
}
impl From<Locknsmpu> for bool {
    #[inline(always)]
    fn from(variant: Locknsmpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKNSMPU` reader - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
pub type LocknsmpuR = crate::BitReader<Locknsmpu>;
impl LocknsmpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locknsmpu {
        match self.bits {
            true => Locknsmpu::Locked,
            false => Locknsmpu::Unlocked,
        }
    }
    #[doc = "Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locknsmpu::Locked
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Locknsmpu::Unlocked
    }
}
#[doc = "Field `LOCKNSMPU` writer - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
pub type LocknsmpuW<'a, REG> = crate::BitWriter1S<'a, REG, Locknsmpu>;
impl<'a, REG> LocknsmpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locknsmpu::Locked)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Locknsmpu::Unlocked)
    }
}
#[doc = "Write '1' to prevent updating the secure SAU regions until the next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locksau {
    #[doc = "1: Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    Locked = 1,
    #[doc = "0: These registers can be updated"]
    Unlocked = 0,
}
impl From<Locksau> for bool {
    #[inline(always)]
    fn from(variant: Locksau) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSAU` reader - Write '1' to prevent updating the secure SAU regions until the next reset"]
pub type LocksauR = crate::BitReader<Locksau>;
impl LocksauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locksau {
        match self.bits {
            true => Locksau::Locked,
            false => Locksau::Unlocked,
        }
    }
    #[doc = "Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locksau::Locked
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Locksau::Unlocked
    }
}
#[doc = "Field `LOCKSAU` writer - Write '1' to prevent updating the secure SAU regions until the next reset"]
pub type LocksauW<'a, REG> = crate::BitWriter1S<'a, REG, Locksau>;
impl<'a, REG> LocksauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writes to the SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers from software or from a debug agent connected to the processor"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksau::Locked)
    }
    #[doc = "These registers can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Locksau::Unlocked)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LocksvtaircrR {
        LocksvtaircrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LocknsvtorR {
        LocknsvtorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LocksmpuR {
        LocksmpuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LocknsmpuR {
        LocknsmpuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub fn locksau(&self) -> LocksauR {
        LocksauR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to prevent updating the secure interrupt configuration until the next reset"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LocksvtaircrW<'_, CpulockSpec> {
        LocksvtaircrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to prevent updating the non-secure vector table base address until the next reset"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LocknsvtorW<'_, CpulockSpec> {
        LocknsvtorW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to prevent updating the secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LocksmpuW<'_, CpulockSpec> {
        LocksmpuW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to prevent updating the Non-secure MPU regions until the next reset"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LocknsmpuW<'_, CpulockSpec> {
        LocknsmpuW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to prevent updating the secure SAU regions until the next reset"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LocksauW<'_, CpulockSpec> {
        LocksauW::new(self, 4)
    }
}
#[doc = "Configure bits to lock down CPU features at runtime\n\nYou can [`read`](crate::Reg::read) this register and get [`cpulock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpulock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpulockSpec;
impl crate::RegisterSpec for CpulockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpulock::R`](R) reader structure"]
impl crate::Readable for CpulockSpec {}
#[doc = "`write(|w| ..)` method takes [`cpulock::W`](W) writer structure"]
impl crate::Writable for CpulockSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
#[doc = "`reset()` method sets CPULOCK to value 0"]
impl crate::Resettable for CpulockSpec {}
