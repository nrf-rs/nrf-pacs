#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Deep power-down mode (DPM) status of external flash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpm {
    #[doc = "0: External flash is not in DPM."]
    Disabled = 0,
    #[doc = "1: External flash is in DPM."]
    Enabled = 1,
}
impl From<Dpm> for bool {
    #[inline(always)]
    fn from(variant: Dpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPM` reader - Deep power-down mode (DPM) status of external flash."]
pub type DpmR = crate::BitReader<Dpm>;
impl DpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpm {
        match self.bits {
            false => Dpm::Disabled,
            true => Dpm::Enabled,
        }
    }
    #[doc = "External flash is not in DPM."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dpm::Disabled
    }
    #[doc = "External flash is in DPM."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dpm::Enabled
    }
}
#[doc = "Ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "1: QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    Ready = 1,
    #[doc = "0: QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    Busy = 0,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Ready status."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            true => Ready::Ready,
            false => Ready::Busy,
        }
    }
    #[doc = "QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
    #[doc = "QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ready::Busy
    }
}
#[doc = "Field `SREG` reader - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
pub type SregR = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Deep power-down mode (DPM) status of external flash."]
    #[inline(always)]
    pub fn dpm(&self) -> DpmR {
        DpmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ready status."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline(always)]
    pub fn sreg(&self) -> SregR {
        SregR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
