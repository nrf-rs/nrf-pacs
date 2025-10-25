#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `LOCKEXIST` reader - Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockexistR = crate::BitReader;
#[doc = "Field `LOCKGRANT` reader - Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
pub type LockgrantR = crate::BitReader;
#[doc = "Field `LOCKTYPE` reader - Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
pub type LocktypeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that a lock control mechanism exists for this device. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    pub fn lockexist(&self) -> LockexistR {
        LockexistR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Returns the current status of the Lock. This bit reads as 0 when read from an external debugger (paddrdbg31 = 1) since external debugger accesses are not subject to Lock Registers."]
    #[inline(always)]
    pub fn lockgrant(&self) -> LockgrantR {
        LockgrantR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if the Lock Access Register (0xFB0) is implemented as 8-bit or 32-bit"]
    #[inline(always)]
    pub fn locktype(&self) -> LocktypeR {
        LocktypeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0x03"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
