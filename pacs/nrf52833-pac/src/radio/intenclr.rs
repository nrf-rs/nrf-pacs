#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Write '1' to disable interrupt for event READY"]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to disable interrupt for event READY"]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, READY_AW, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(READY_AW::CLEAR)
    }
}
#[doc = "Field `ADDRESS` reader - Write '1' to disable interrupt for event ADDRESS"]
pub type ADDRESS_R = crate::BitReader<ADDRESS_A>;
#[doc = "Write '1' to disable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_A {
        match self.bits {
            false => ADDRESS_A::DISABLED,
            true => ADDRESS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ADDRESS_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Write '1' to disable interrupt for event ADDRESS"]
pub type ADDRESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ADDRESS_AW, O>;
impl<'a, const O: u8> ADDRESS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRESS_AW::CLEAR)
    }
}
#[doc = "Field `PAYLOAD` reader - Write '1' to disable interrupt for event PAYLOAD"]
pub type PAYLOAD_R = crate::BitReader<PAYLOAD_A>;
#[doc = "Write '1' to disable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PAYLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl PAYLOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAYLOAD_A {
        match self.bits {
            false => PAYLOAD_A::DISABLED,
            true => PAYLOAD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAYLOAD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAYLOAD_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PAYLOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Write '1' to disable interrupt for event PAYLOAD"]
pub type PAYLOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, PAYLOAD_AW, O>;
impl<'a, const O: u8> PAYLOAD_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PAYLOAD_AW::CLEAR)
    }
}
#[doc = "Field `END` reader - Write '1' to disable interrupt for event END"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
impl END_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to disable interrupt for event END"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, END_AW, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(END_AW::CLEAR)
    }
}
#[doc = "Field `DISABLED` reader - Write '1' to disable interrupt for event DISABLED"]
pub type DISABLED_R = crate::BitReader<DISABLED_A>;
#[doc = "Write '1' to disable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DISABLED_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_A {
        match self.bits {
            false => DISABLED_A::DISABLED,
            true => DISABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DISABLED_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Write '1' to disable interrupt for event DISABLED"]
pub type DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, DISABLED_AW, O>;
impl<'a, const O: u8> DISABLED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DISABLED_AW::CLEAR)
    }
}
#[doc = "Field `DEVMATCH` reader - Write '1' to disable interrupt for event DEVMATCH"]
pub type DEVMATCH_R = crate::BitReader<DEVMATCH_A>;
#[doc = "Write '1' to disable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DEVMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl DEVMATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMATCH_A {
        match self.bits {
            false => DEVMATCH_A::DISABLED,
            true => DEVMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DEVMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Write '1' to disable interrupt for event DEVMATCH"]
pub type DEVMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, DEVMATCH_AW, O>;
impl<'a, const O: u8> DEVMATCH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMATCH_AW::CLEAR)
    }
}
#[doc = "Field `DEVMISS` reader - Write '1' to disable interrupt for event DEVMISS"]
pub type DEVMISS_R = crate::BitReader<DEVMISS_A>;
#[doc = "Write '1' to disable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DEVMISS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_A) -> Self {
        variant as u8 != 0
    }
}
impl DEVMISS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMISS_A {
        match self.bits {
            false => DEVMISS_A::DISABLED,
            true => DEVMISS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMISS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMISS_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DEVMISS_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Write '1' to disable interrupt for event DEVMISS"]
pub type DEVMISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, DEVMISS_AW, O>;
impl<'a, const O: u8> DEVMISS_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DEVMISS_AW::CLEAR)
    }
}
#[doc = "Field `RSSIEND` reader - Write '1' to disable interrupt for event RSSIEND"]
pub type RSSIEND_R = crate::BitReader<RSSIEND_A>;
#[doc = "Write '1' to disable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RSSIEND_A> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_A) -> Self {
        variant as u8 != 0
    }
}
impl RSSIEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSSIEND_A {
        match self.bits {
            false => RSSIEND_A::DISABLED,
            true => RSSIEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSSIEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSSIEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RSSIEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Write '1' to disable interrupt for event RSSIEND"]
pub type RSSIEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RSSIEND_AW, O>;
impl<'a, const O: u8> RSSIEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSSIEND_AW::CLEAR)
    }
}
#[doc = "Field `BCMATCH` reader - Write '1' to disable interrupt for event BCMATCH"]
pub type BCMATCH_R = crate::BitReader<BCMATCH_A>;
#[doc = "Write '1' to disable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<BCMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl BCMATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCMATCH_A {
        match self.bits {
            false => BCMATCH_A::DISABLED,
            true => BCMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<BCMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Write '1' to disable interrupt for event BCMATCH"]
pub type BCMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, BCMATCH_AW, O>;
impl<'a, const O: u8> BCMATCH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCMATCH_AW::CLEAR)
    }
}
#[doc = "Field `CRCOK` reader - Write '1' to disable interrupt for event CRCOK"]
pub type CRCOK_R = crate::BitReader<CRCOK_A>;
#[doc = "Write '1' to disable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCOK_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CRCOK_A> for bool {
    #[inline(always)]
    fn from(variant: CRCOK_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCOK_A {
        match self.bits {
            false => CRCOK_A::DISABLED,
            true => CRCOK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCOK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCOK_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCOK_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CRCOK_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCOK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCOK` writer - Write '1' to disable interrupt for event CRCOK"]
pub type CRCOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CRCOK_AW, O>;
impl<'a, const O: u8> CRCOK_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRCOK_AW::CLEAR)
    }
}
#[doc = "Field `CRCERROR` reader - Write '1' to disable interrupt for event CRCERROR"]
pub type CRCERROR_R = crate::BitReader<CRCERROR_A>;
#[doc = "Write '1' to disable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CRCERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERROR_A {
        match self.bits {
            false => CRCERROR_A::DISABLED,
            true => CRCERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCERROR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CRCERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` writer - Write '1' to disable interrupt for event CRCERROR"]
pub type CRCERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CRCERROR_AW, O>;
impl<'a, const O: u8> CRCERROR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRCERROR_AW::CLEAR)
    }
}
#[doc = "Field `FRAMESTART` reader - Write '1' to disable interrupt for event FRAMESTART"]
pub type FRAMESTART_R = crate::BitReader<FRAMESTART_A>;
#[doc = "Write '1' to disable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAMESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMESTART_A {
        match self.bits {
            false => FRAMESTART_A::DISABLED,
            true => FRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMESTART_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<FRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMESTART` writer - Write '1' to disable interrupt for event FRAMESTART"]
pub type FRAMESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, FRAMESTART_AW, O>;
impl<'a, const O: u8> FRAMESTART_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRAMESTART_AW::CLEAR)
    }
}
#[doc = "Field `EDEND` reader - Write '1' to disable interrupt for event EDEND"]
pub type EDEND_R = crate::BitReader<EDEND_A>;
#[doc = "Write '1' to disable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EDEND_A> for bool {
    #[inline(always)]
    fn from(variant: EDEND_A) -> Self {
        variant as u8 != 0
    }
}
impl EDEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDEND_A {
        match self.bits {
            false => EDEND_A::DISABLED,
            true => EDEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<EDEND_AW> for bool {
    #[inline(always)]
    fn from(variant: EDEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDEND` writer - Write '1' to disable interrupt for event EDEND"]
pub type EDEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, EDEND_AW, O>;
impl<'a, const O: u8> EDEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EDEND_AW::CLEAR)
    }
}
#[doc = "Field `EDSTOPPED` reader - Write '1' to disable interrupt for event EDSTOPPED"]
pub type EDSTOPPED_R = crate::BitReader<EDSTOPPED_A>;
#[doc = "Write '1' to disable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EDSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: EDSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
impl EDSTOPPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSTOPPED_A {
        match self.bits {
            false => EDSTOPPED_A::DISABLED,
            true => EDSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDSTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSTOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<EDSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: EDSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSTOPPED` writer - Write '1' to disable interrupt for event EDSTOPPED"]
pub type EDSTOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, EDSTOPPED_AW, O>;
impl<'a, const O: u8> EDSTOPPED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EDSTOPPED_AW::CLEAR)
    }
}
#[doc = "Field `CCAIDLE` reader - Write '1' to disable interrupt for event CCAIDLE"]
pub type CCAIDLE_R = crate::BitReader<CCAIDLE_A>;
#[doc = "Write '1' to disable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCAIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCAIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCAIDLE_A {
        match self.bits {
            false => CCAIDLE_A::DISABLED,
            true => CCAIDLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCAIDLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCAIDLE_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CCAIDLE_AW> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCAIDLE` writer - Write '1' to disable interrupt for event CCAIDLE"]
pub type CCAIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CCAIDLE_AW, O>;
impl<'a, const O: u8> CCAIDLE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCAIDLE_AW::CLEAR)
    }
}
#[doc = "Field `CCABUSY` reader - Write '1' to disable interrupt for event CCABUSY"]
pub type CCABUSY_R = crate::BitReader<CCABUSY_A>;
#[doc = "Write '1' to disable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABUSY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCABUSY_A> for bool {
    #[inline(always)]
    fn from(variant: CCABUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CCABUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCABUSY_A {
        match self.bits {
            false => CCABUSY_A::DISABLED,
            true => CCABUSY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCABUSY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCABUSY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABUSY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CCABUSY_AW> for bool {
    #[inline(always)]
    fn from(variant: CCABUSY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCABUSY` writer - Write '1' to disable interrupt for event CCABUSY"]
pub type CCABUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CCABUSY_AW, O>;
impl<'a, const O: u8> CCABUSY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCABUSY_AW::CLEAR)
    }
}
#[doc = "Field `CCASTOPPED` reader - Write '1' to disable interrupt for event CCASTOPPED"]
pub type CCASTOPPED_R = crate::BitReader<CCASTOPPED_A>;
#[doc = "Write '1' to disable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCASTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCASTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: CCASTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
impl CCASTOPPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCASTOPPED_A {
        match self.bits {
            false => CCASTOPPED_A::DISABLED,
            true => CCASTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCASTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCASTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCASTOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CCASTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: CCASTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCASTOPPED` writer - Write '1' to disable interrupt for event CCASTOPPED"]
pub type CCASTOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CCASTOPPED_AW, O>;
impl<'a, const O: u8> CCASTOPPED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCASTOPPED_AW::CLEAR)
    }
}
#[doc = "Field `RATEBOOST` reader - Write '1' to disable interrupt for event RATEBOOST"]
pub type RATEBOOST_R = crate::BitReader<RATEBOOST_A>;
#[doc = "Write '1' to disable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATEBOOST_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RATEBOOST_A> for bool {
    #[inline(always)]
    fn from(variant: RATEBOOST_A) -> Self {
        variant as u8 != 0
    }
}
impl RATEBOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATEBOOST_A {
        match self.bits {
            false => RATEBOOST_A::DISABLED,
            true => RATEBOOST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RATEBOOST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RATEBOOST_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATEBOOST_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RATEBOOST_AW> for bool {
    #[inline(always)]
    fn from(variant: RATEBOOST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATEBOOST` writer - Write '1' to disable interrupt for event RATEBOOST"]
pub type RATEBOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RATEBOOST_AW, O>;
impl<'a, const O: u8> RATEBOOST_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RATEBOOST_AW::CLEAR)
    }
}
#[doc = "Field `TXREADY` reader - Write '1' to disable interrupt for event TXREADY"]
pub type TXREADY_R = crate::BitReader<TXREADY_A>;
#[doc = "Write '1' to disable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXREADY_A> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_A) -> Self {
        variant as u8 != 0
    }
}
impl TXREADY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREADY_A {
        match self.bits {
            false => TXREADY_A::DISABLED,
            true => TXREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXREADY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXREADY` writer - Write '1' to disable interrupt for event TXREADY"]
pub type TXREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, TXREADY_AW, O>;
impl<'a, const O: u8> TXREADY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXREADY_AW::CLEAR)
    }
}
#[doc = "Field `RXREADY` reader - Write '1' to disable interrupt for event RXREADY"]
pub type RXREADY_R = crate::BitReader<RXREADY_A>;
#[doc = "Write '1' to disable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXREADY_A> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXREADY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXREADY_A {
        match self.bits {
            false => RXREADY_A::DISABLED,
            true => RXREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXREADY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXREADY` writer - Write '1' to disable interrupt for event RXREADY"]
pub type RXREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RXREADY_AW, O>;
impl<'a, const O: u8> RXREADY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXREADY_AW::CLEAR)
    }
}
#[doc = "Field `MHRMATCH` reader - Write '1' to disable interrupt for event MHRMATCH"]
pub type MHRMATCH_R = crate::BitReader<MHRMATCH_A>;
#[doc = "Write '1' to disable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MHRMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<MHRMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: MHRMATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl MHRMATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MHRMATCH_A {
        match self.bits {
            false => MHRMATCH_A::DISABLED,
            true => MHRMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MHRMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MHRMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MHRMATCH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<MHRMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: MHRMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MHRMATCH` writer - Write '1' to disable interrupt for event MHRMATCH"]
pub type MHRMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, MHRMATCH_AW, O>;
impl<'a, const O: u8> MHRMATCH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MHRMATCH_AW::CLEAR)
    }
}
#[doc = "Field `SYNC` reader - Write '1' to disable interrupt for event SYNC"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Write '1' to disable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLED,
            true => SYNC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNC_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SYNC_AW> for bool {
    #[inline(always)]
    fn from(variant: SYNC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` writer - Write '1' to disable interrupt for event SYNC"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SYNC_AW, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SYNC_AW::CLEAR)
    }
}
#[doc = "Field `PHYEND` reader - Write '1' to disable interrupt for event PHYEND"]
pub type PHYEND_R = crate::BitReader<PHYEND_A>;
#[doc = "Write '1' to disable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PHYEND_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_A {
        match self.bits {
            false => PHYEND_A::DISABLED,
            true => PHYEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PHYEND_AW> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYEND` writer - Write '1' to disable interrupt for event PHYEND"]
pub type PHYEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, PHYEND_AW, O>;
impl<'a, const O: u8> PHYEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PHYEND_AW::CLEAR)
    }
}
#[doc = "Field `CTEPRESENT` reader - Write '1' to disable interrupt for event CTEPRESENT"]
pub type CTEPRESENT_R = crate::BitReader<CTEPRESENT_A>;
#[doc = "Write '1' to disable interrupt for event CTEPRESENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEPRESENT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTEPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: CTEPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEPRESENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEPRESENT_A {
        match self.bits {
            false => CTEPRESENT_A::DISABLED,
            true => CTEPRESENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTEPRESENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTEPRESENT_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event CTEPRESENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEPRESENT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTEPRESENT_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEPRESENT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEPRESENT` writer - Write '1' to disable interrupt for event CTEPRESENT"]
pub type CTEPRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, CTEPRESENT_AW, O>;
impl<'a, const O: u8> CTEPRESENT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEPRESENT_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&self) -> PAYLOAD_R {
        PAYLOAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&self) -> DISABLED_R {
        DISABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&self) -> DEVMATCH_R {
        DEVMATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&self) -> DEVMISS_R {
        DEVMISS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&self) -> RSSIEND_R {
        RSSIEND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&self) -> BCMATCH_R {
        BCMATCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&self) -> CRCOK_R {
        CRCOK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&self) -> FRAMESTART_R {
        FRAMESTART_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&self) -> EDEND_R {
        EDEND_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&self) -> EDSTOPPED_R {
        EDSTOPPED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&self) -> CCAIDLE_R {
        CCAIDLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&self) -> CCABUSY_R {
        CCABUSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&self) -> CCASTOPPED_R {
        CCASTOPPED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&self) -> RATEBOOST_R {
        RATEBOOST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&self) -> TXREADY_R {
        TXREADY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RXREADY_R {
        RXREADY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&self) -> MHRMATCH_R {
        MHRMATCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&self) -> PHYEND_R {
        PHYEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write '1' to disable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub fn ctepresent(&self) -> CTEPRESENT_R {
        CTEPRESENT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<1> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&mut self) -> PAYLOAD_W<2> {
        PAYLOAD_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<3> {
        END_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&mut self) -> DISABLED_W<4> {
        DISABLED_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DEVMATCH_W<5> {
        DEVMATCH_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DEVMISS_W<6> {
        DEVMISS_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RSSIEND_W<7> {
        RSSIEND_W::new(self)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BCMATCH_W<10> {
        BCMATCH_W::new(self)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&mut self) -> CRCOK_W<12> {
        CRCOK_W::new(self)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CRCERROR_W<13> {
        CRCERROR_W::new(self)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&mut self) -> FRAMESTART_W<14> {
        FRAMESTART_W::new(self)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&mut self) -> EDEND_W<15> {
        EDEND_W::new(self)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&mut self) -> EDSTOPPED_W<16> {
        EDSTOPPED_W::new(self)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&mut self) -> CCAIDLE_W<17> {
        CCAIDLE_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&mut self) -> CCABUSY_W<18> {
        CCABUSY_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&mut self) -> CCASTOPPED_W<19> {
        CCASTOPPED_W::new(self)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&mut self) -> RATEBOOST_W<20> {
        RATEBOOST_W::new(self)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&mut self) -> TXREADY_W<21> {
        TXREADY_W::new(self)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RXREADY_W<22> {
        RXREADY_W::new(self)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&mut self) -> MHRMATCH_W<23> {
        MHRMATCH_W::new(self)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<26> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&mut self) -> PHYEND_W<27> {
        PHYEND_W::new(self)
    }
    #[doc = "Bit 28 - Write '1' to disable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub fn ctepresent(&mut self) -> CTEPRESENT_W<28> {
        CTEPRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
