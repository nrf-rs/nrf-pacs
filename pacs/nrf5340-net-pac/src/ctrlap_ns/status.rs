#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Status bit for UICR part of access port protection at last reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uicrapprotect {
    #[doc = "0: APPROTECT was enabled in UICR"]
    Enabled = 0,
    #[doc = "1: APPROTECT wasdisabled in UICR"]
    Disabled = 1,
}
impl From<Uicrapprotect> for bool {
    #[inline(always)]
    fn from(variant: Uicrapprotect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UICRAPPROTECT` reader - Status bit for UICR part of access port protection at last reset."]
pub type UicrapprotectR = crate::BitReader<Uicrapprotect>;
impl UicrapprotectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uicrapprotect {
        match self.bits {
            false => Uicrapprotect::Enabled,
            true => Uicrapprotect::Disabled,
        }
    }
    #[doc = "APPROTECT was enabled in UICR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Uicrapprotect::Enabled
    }
    #[doc = "APPROTECT wasdisabled in UICR"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Uicrapprotect::Disabled
    }
}
#[doc = "Status bit for device debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgifacemode {
    #[doc = "0: No debugger attached"]
    Disabled = 0,
    #[doc = "1: Debugger is attached and device is in debug interface mode"]
    Enabled = 1,
}
impl From<Dbgifacemode> for bool {
    #[inline(always)]
    fn from(variant: Dbgifacemode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGIFACEMODE` reader - Status bit for device debug interface mode"]
pub type DbgifacemodeR = crate::BitReader<Dbgifacemode>;
impl DbgifacemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgifacemode {
        match self.bits {
            false => Dbgifacemode::Disabled,
            true => Dbgifacemode::Enabled,
        }
    }
    #[doc = "No debugger attached"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dbgifacemode::Disabled
    }
    #[doc = "Debugger is attached and device is in debug interface mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dbgifacemode::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Status bit for UICR part of access port protection at last reset."]
    #[inline(always)]
    pub fn uicrapprotect(&self) -> UicrapprotectR {
        UicrapprotectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Status bit for device debug interface mode"]
    #[inline(always)]
    pub fn dbgifacemode(&self) -> DbgifacemodeR {
        DbgifacemodeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status bits for CTRL-AP peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
