#[doc = "Register `DFESTATUS` reader"]
pub type R = crate::R<DfestatusSpec>;
#[doc = "Internal state of switching state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Switchingstate {
    #[doc = "0: Switching state Idle"]
    Idle = 0,
    #[doc = "1: Switching state Offset"]
    Offset = 1,
    #[doc = "2: Switching state Guard"]
    Guard = 2,
    #[doc = "3: Switching state Ref"]
    Ref = 3,
    #[doc = "4: Switching state Switching"]
    Switching = 4,
    #[doc = "5: Switching state Ending"]
    Ending = 5,
}
impl From<Switchingstate> for u8 {
    #[inline(always)]
    fn from(variant: Switchingstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Switchingstate {
    type Ux = u8;
}
impl crate::IsEnum for Switchingstate {}
#[doc = "Field `SWITCHINGSTATE` reader - Internal state of switching state machine"]
pub type SwitchingstateR = crate::FieldReader<Switchingstate>;
impl SwitchingstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Switchingstate> {
        match self.bits {
            0 => Some(Switchingstate::Idle),
            1 => Some(Switchingstate::Offset),
            2 => Some(Switchingstate::Guard),
            3 => Some(Switchingstate::Ref),
            4 => Some(Switchingstate::Switching),
            5 => Some(Switchingstate::Ending),
            _ => None,
        }
    }
    #[doc = "Switching state Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Switchingstate::Idle
    }
    #[doc = "Switching state Offset"]
    #[inline(always)]
    pub fn is_offset(&self) -> bool {
        *self == Switchingstate::Offset
    }
    #[doc = "Switching state Guard"]
    #[inline(always)]
    pub fn is_guard(&self) -> bool {
        *self == Switchingstate::Guard
    }
    #[doc = "Switching state Ref"]
    #[inline(always)]
    pub fn is_ref(&self) -> bool {
        *self == Switchingstate::Ref
    }
    #[doc = "Switching state Switching"]
    #[inline(always)]
    pub fn is_switching(&self) -> bool {
        *self == Switchingstate::Switching
    }
    #[doc = "Switching state Ending"]
    #[inline(always)]
    pub fn is_ending(&self) -> bool {
        *self == Switchingstate::Ending
    }
}
#[doc = "Internal state of sampling state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samplingstate {
    #[doc = "0: Sampling state Idle"]
    Idle = 0,
    #[doc = "1: Sampling state Sampling"]
    Sampling = 1,
}
impl From<Samplingstate> for bool {
    #[inline(always)]
    fn from(variant: Samplingstate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLINGSTATE` reader - Internal state of sampling state machine"]
pub type SamplingstateR = crate::BitReader<Samplingstate>;
impl SamplingstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samplingstate {
        match self.bits {
            false => Samplingstate::Idle,
            true => Samplingstate::Sampling,
        }
    }
    #[doc = "Sampling state Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Samplingstate::Idle
    }
    #[doc = "Sampling state Sampling"]
    #[inline(always)]
    pub fn is_sampling(&self) -> bool {
        *self == Samplingstate::Sampling
    }
}
impl R {
    #[doc = "Bits 0:2 - Internal state of switching state machine"]
    #[inline(always)]
    pub fn switchingstate(&self) -> SwitchingstateR {
        SwitchingstateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Internal state of sampling state machine"]
    #[inline(always)]
    pub fn samplingstate(&self) -> SamplingstateR {
        SamplingstateR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "DFE status information\n\nYou can [`read`](crate::Reg::read) this register and get [`dfestatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfestatusSpec;
impl crate::RegisterSpec for DfestatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfestatus::R`](R) reader structure"]
impl crate::Readable for DfestatusSpec {}
#[doc = "`reset()` method sets DFESTATUS to value 0"]
impl crate::Resettable for DfestatusSpec {}
