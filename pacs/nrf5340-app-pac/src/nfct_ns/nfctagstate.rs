#[doc = "Register `NFCTAGSTATE` reader"]
pub type R = crate::R<NfctagstateSpec>;
#[doc = "NfcTag state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfctagstate {
    #[doc = "0: Disabled or sense"]
    Disabled = 0,
    #[doc = "2: RampUp"]
    RampUp = 2,
    #[doc = "3: Idle"]
    Idle = 3,
    #[doc = "4: Receive"]
    Receive = 4,
    #[doc = "5: FrameDelay"]
    FrameDelay = 5,
    #[doc = "6: Transmit"]
    Transmit = 6,
}
impl From<Nfctagstate> for u8 {
    #[inline(always)]
    fn from(variant: Nfctagstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfctagstate {
    type Ux = u8;
}
impl crate::IsEnum for Nfctagstate {}
#[doc = "Field `NFCTAGSTATE` reader - NfcTag state"]
pub type NfctagstateR = crate::FieldReader<Nfctagstate>;
impl NfctagstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nfctagstate> {
        match self.bits {
            0 => Some(Nfctagstate::Disabled),
            2 => Some(Nfctagstate::RampUp),
            3 => Some(Nfctagstate::Idle),
            4 => Some(Nfctagstate::Receive),
            5 => Some(Nfctagstate::FrameDelay),
            6 => Some(Nfctagstate::Transmit),
            _ => None,
        }
    }
    #[doc = "Disabled or sense"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nfctagstate::Disabled
    }
    #[doc = "RampUp"]
    #[inline(always)]
    pub fn is_ramp_up(&self) -> bool {
        *self == Nfctagstate::RampUp
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Nfctagstate::Idle
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == Nfctagstate::Receive
    }
    #[doc = "FrameDelay"]
    #[inline(always)]
    pub fn is_frame_delay(&self) -> bool {
        *self == Nfctagstate::FrameDelay
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == Nfctagstate::Transmit
    }
}
impl R {
    #[doc = "Bits 0:2 - NfcTag state"]
    #[inline(always)]
    pub fn nfctagstate(&self) -> NfctagstateR {
        NfctagstateR::new((self.bits & 7) as u8)
    }
}
#[doc = "Current operating state of NFC tag\n\nYou can [`read`](crate::Reg::read) this register and get [`nfctagstate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NfctagstateSpec;
impl crate::RegisterSpec for NfctagstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfctagstate::R`](R) reader structure"]
impl crate::Readable for NfctagstateSpec {}
#[doc = "`reset()` method sets NFCTAGSTATE to value 0"]
impl crate::Resettable for NfctagstateSpec {}
