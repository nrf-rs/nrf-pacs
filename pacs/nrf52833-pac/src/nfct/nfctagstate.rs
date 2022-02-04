#[doc = "Register `NFCTAGSTATE` reader"]
pub struct R(crate::R<NFCTAGSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCTAGSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCTAGSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCTAGSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "NfcTag state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NFCTAGSTATE_A {
    #[doc = "0: Disabled or sense"]
    DISABLED = 0,
    #[doc = "2: RampUp"]
    RAMPUP = 2,
    #[doc = "3: Idle"]
    IDLE = 3,
    #[doc = "4: Receive"]
    RECEIVE = 4,
    #[doc = "5: FrameDelay"]
    FRAMEDELAY = 5,
    #[doc = "6: Transmit"]
    TRANSMIT = 6,
}
impl From<NFCTAGSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCTAGSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NFCTAGSTATE` reader - NfcTag state"]
pub struct NFCTAGSTATE_R(crate::FieldReader<u8, NFCTAGSTATE_A>);
impl NFCTAGSTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NFCTAGSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFCTAGSTATE_A> {
        match self.bits {
            0 => Some(NFCTAGSTATE_A::DISABLED),
            2 => Some(NFCTAGSTATE_A::RAMPUP),
            3 => Some(NFCTAGSTATE_A::IDLE),
            4 => Some(NFCTAGSTATE_A::RECEIVE),
            5 => Some(NFCTAGSTATE_A::FRAMEDELAY),
            6 => Some(NFCTAGSTATE_A::TRANSMIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NFCTAGSTATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RAMPUP`"]
    #[inline(always)]
    pub fn is_ramp_up(&self) -> bool {
        **self == NFCTAGSTATE_A::RAMPUP
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == NFCTAGSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        **self == NFCTAGSTATE_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `FRAMEDELAY`"]
    #[inline(always)]
    pub fn is_frame_delay(&self) -> bool {
        **self == NFCTAGSTATE_A::FRAMEDELAY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        **self == NFCTAGSTATE_A::TRANSMIT
    }
}
impl core::ops::Deref for NFCTAGSTATE_R {
    type Target = crate::FieldReader<u8, NFCTAGSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - NfcTag state"]
    #[inline(always)]
    pub fn nfctagstate(&self) -> NFCTAGSTATE_R {
        NFCTAGSTATE_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "NfcTag state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfctagstate](index.html) module"]
pub struct NFCTAGSTATE_SPEC;
impl crate::RegisterSpec for NFCTAGSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfctagstate::R](R) reader structure"]
impl crate::Readable for NFCTAGSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NFCTAGSTATE to value 0"]
impl crate::Resettable for NFCTAGSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
