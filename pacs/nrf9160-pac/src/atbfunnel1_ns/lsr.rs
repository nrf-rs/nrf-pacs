#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Register `LSR` writer"]
pub type W = crate::W<LsrSpec>;
#[doc = "Indicates that a lock control mechanism exists for this device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Present {
    #[doc = "0: No lock control mechanism exists, writes to the Lock Access Register are ignored."]
    NotImplemented = 0,
    #[doc = "1: Lock control mechanism is present."]
    Implemented = 1,
}
impl From<Present> for bool {
    #[inline(always)]
    fn from(variant: Present) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRESENT` reader - Indicates that a lock control mechanism exists for this device."]
pub type PresentR = crate::BitReader<Present>;
impl PresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Present {
        match self.bits {
            false => Present::NotImplemented,
            true => Present::Implemented,
        }
    }
    #[doc = "No lock control mechanism exists, writes to the Lock Access Register are ignored."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Present::NotImplemented
    }
    #[doc = "Lock control mechanism is present."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Present::Implemented
    }
}
#[doc = "Field `PRESENT` writer - Indicates that a lock control mechanism exists for this device."]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG, Present>;
impl<'a, REG> PresentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No lock control mechanism exists, writes to the Lock Access Register are ignored."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Present::NotImplemented)
    }
    #[doc = "Lock control mechanism is present."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Present::Implemented)
    }
}
#[doc = "Returns the current status of the Lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locked {
    #[doc = "0: Write access is allowed to this device."]
    UnLocked = 0,
    #[doc = "1: Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    Locked = 1,
}
impl From<Locked> for bool {
    #[inline(always)]
    fn from(variant: Locked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKED` reader - Returns the current status of the Lock."]
pub type LockedR = crate::BitReader<Locked>;
impl LockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locked {
        match self.bits {
            false => Locked::UnLocked,
            true => Locked::Locked,
        }
    }
    #[doc = "Write access is allowed to this device."]
    #[inline(always)]
    pub fn is_un_locked(&self) -> bool {
        *self == Locked::UnLocked
    }
    #[doc = "Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Locked::Locked
    }
}
#[doc = "Field `LOCKED` writer - Returns the current status of the Lock."]
pub type LockedW<'a, REG> = crate::BitWriter<'a, REG, Locked>;
impl<'a, REG> LockedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write access is allowed to this device."]
    #[inline(always)]
    pub fn un_locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locked::UnLocked)
    }
    #[doc = "Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Locked::Locked)
    }
}
#[doc = "Indicates if the Lock Access Register is implemented as 8-bit or 32-bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    #[doc = "0: This component implements a 32-bit Lock Access Register."]
    Bits32 = 0,
    #[doc = "1: This component implements an 8-bit Lock Access Register."]
    Bits8 = 1,
}
impl From<Type> for bool {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - Indicates if the Lock Access Register is implemented as 8-bit or 32-bit."]
pub type TypeR = crate::BitReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Type {
        match self.bits {
            false => Type::Bits32,
            true => Type::Bits8,
        }
    }
    #[doc = "This component implements a 32-bit Lock Access Register."]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == Type::Bits32
    }
    #[doc = "This component implements an 8-bit Lock Access Register."]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == Type::Bits8
    }
}
#[doc = "Field `TYPE` writer - Indicates if the Lock Access Register is implemented as 8-bit or 32-bit."]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This component implements a 32-bit Lock Access Register."]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Bits32)
    }
    #[doc = "This component implements an 8-bit Lock Access Register."]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Bits8)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that a lock control mechanism exists for this device."]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Returns the current status of the Lock."]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if the Lock Access Register is implemented as 8-bit or 32-bit."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that a lock control mechanism exists for this device."]
    #[inline(always)]
    pub fn present(&mut self) -> PresentW<'_, LsrSpec> {
        PresentW::new(self, 0)
    }
    #[doc = "Bit 1 - Returns the current status of the Lock."]
    #[inline(always)]
    pub fn locked(&mut self) -> LockedW<'_, LsrSpec> {
        LockedW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates if the Lock Access Register is implemented as 8-bit or 32-bit."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, LsrSpec> {
        TypeW::new(self, 2)
    }
}
#[doc = "This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. Accesses to the extended stimulus port registers are not affected by the lock mechanism. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`write(|w| ..)` method takes [`lsr::W`](W) writer structure"]
impl crate::Writable for LsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {}
