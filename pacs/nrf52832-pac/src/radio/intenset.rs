#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Write '1' to Enable interrupt for READY event"]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "Write '1' to Enable interrupt for READY event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for READY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to Enable interrupt for READY event"]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, READY_AW, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(READY_AW::SET)
    }
}
#[doc = "Field `ADDRESS` reader - Write '1' to Enable interrupt for ADDRESS event"]
pub type ADDRESS_R = crate::BitReader<ADDRESS_A>;
#[doc = "Write '1' to Enable interrupt for ADDRESS event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for ADDRESS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ADDRESS_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRESS` writer - Write '1' to Enable interrupt for ADDRESS event"]
pub type ADDRESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ADDRESS_AW, O>;
impl<'a, const O: u8> ADDRESS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ADDRESS_AW::SET)
    }
}
#[doc = "Field `PAYLOAD` reader - Write '1' to Enable interrupt for PAYLOAD event"]
pub type PAYLOAD_R = crate::BitReader<PAYLOAD_A>;
#[doc = "Write '1' to Enable interrupt for PAYLOAD event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for PAYLOAD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PAYLOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAYLOAD` writer - Write '1' to Enable interrupt for PAYLOAD event"]
pub type PAYLOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, PAYLOAD_AW, O>;
impl<'a, const O: u8> PAYLOAD_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PAYLOAD_AW::SET)
    }
}
#[doc = "Field `END` reader - Write '1' to Enable interrupt for END event"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to Enable interrupt for END event"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, END_AW, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
    }
}
#[doc = "Field `DISABLED` reader - Write '1' to Enable interrupt for DISABLED event"]
pub type DISABLED_R = crate::BitReader<DISABLED_A>;
#[doc = "Write '1' to Enable interrupt for DISABLED event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for DISABLED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DISABLED_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLED` writer - Write '1' to Enable interrupt for DISABLED event"]
pub type DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DISABLED_AW, O>;
impl<'a, const O: u8> DISABLED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DISABLED_AW::SET)
    }
}
#[doc = "Field `DEVMATCH` reader - Write '1' to Enable interrupt for DEVMATCH event"]
pub type DEVMATCH_R = crate::BitReader<DEVMATCH_A>;
#[doc = "Write '1' to Enable interrupt for DEVMATCH event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for DEVMATCH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DEVMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMATCH` writer - Write '1' to Enable interrupt for DEVMATCH event"]
pub type DEVMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DEVMATCH_AW, O>;
impl<'a, const O: u8> DEVMATCH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DEVMATCH_AW::SET)
    }
}
#[doc = "Field `DEVMISS` reader - Write '1' to Enable interrupt for DEVMISS event"]
pub type DEVMISS_R = crate::BitReader<DEVMISS_A>;
#[doc = "Write '1' to Enable interrupt for DEVMISS event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for DEVMISS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DEVMISS_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEVMISS` writer - Write '1' to Enable interrupt for DEVMISS event"]
pub type DEVMISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DEVMISS_AW, O>;
impl<'a, const O: u8> DEVMISS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DEVMISS_AW::SET)
    }
}
#[doc = "Field `RSSIEND` reader - Write '1' to Enable interrupt for RSSIEND event"]
pub type RSSIEND_R = crate::BitReader<RSSIEND_A>;
#[doc = "Write '1' to Enable interrupt for RSSIEND event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for RSSIEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RSSIEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSSIEND` writer - Write '1' to Enable interrupt for RSSIEND event"]
pub type RSSIEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RSSIEND_AW, O>;
impl<'a, const O: u8> RSSIEND_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RSSIEND_AW::SET)
    }
}
#[doc = "Field `BCMATCH` reader - Write '1' to Enable interrupt for BCMATCH event"]
pub type BCMATCH_R = crate::BitReader<BCMATCH_A>;
#[doc = "Write '1' to Enable interrupt for BCMATCH event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for BCMATCH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<BCMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCMATCH` writer - Write '1' to Enable interrupt for BCMATCH event"]
pub type BCMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, BCMATCH_AW, O>;
impl<'a, const O: u8> BCMATCH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCMATCH_AW::SET)
    }
}
#[doc = "Field `CRCOK` reader - Write '1' to Enable interrupt for CRCOK event"]
pub type CRCOK_R = crate::BitReader<CRCOK_A>;
#[doc = "Write '1' to Enable interrupt for CRCOK event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for CRCOK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCOK_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CRCOK_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCOK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCOK` writer - Write '1' to Enable interrupt for CRCOK event"]
pub type CRCOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CRCOK_AW, O>;
impl<'a, const O: u8> CRCOK_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CRCOK_AW::SET)
    }
}
#[doc = "Field `CRCERROR` reader - Write '1' to Enable interrupt for CRCERROR event"]
pub type CRCERROR_R = crate::BitReader<CRCERROR_A>;
#[doc = "Write '1' to Enable interrupt for CRCERROR event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for CRCERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CRCERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERROR` writer - Write '1' to Enable interrupt for CRCERROR event"]
pub type CRCERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CRCERROR_AW, O>;
impl<'a, const O: u8> CRCERROR_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CRCERROR_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for READY event"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ADDRESS event"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for PAYLOAD event"]
    #[inline(always)]
    pub fn payload(&self) -> PAYLOAD_R {
        PAYLOAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for DISABLED event"]
    #[inline(always)]
    pub fn disabled(&self) -> DISABLED_R {
        DISABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for DEVMATCH event"]
    #[inline(always)]
    pub fn devmatch(&self) -> DEVMATCH_R {
        DEVMATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for DEVMISS event"]
    #[inline(always)]
    pub fn devmiss(&self) -> DEVMISS_R {
        DEVMISS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for RSSIEND event"]
    #[inline(always)]
    pub fn rssiend(&self) -> RSSIEND_R {
        RSSIEND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for BCMATCH event"]
    #[inline(always)]
    pub fn bcmatch(&self) -> BCMATCH_R {
        BCMATCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for CRCOK event"]
    #[inline(always)]
    pub fn crcok(&self) -> CRCOK_R {
        CRCOK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to Enable interrupt for CRCERROR event"]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for READY event"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W<0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ADDRESS event"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<1> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for PAYLOAD event"]
    #[inline(always)]
    pub fn payload(&mut self) -> PAYLOAD_W<2> {
        PAYLOAD_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<3> {
        END_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for DISABLED event"]
    #[inline(always)]
    pub fn disabled(&mut self) -> DISABLED_W<4> {
        DISABLED_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for DEVMATCH event"]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DEVMATCH_W<5> {
        DEVMATCH_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for DEVMISS event"]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DEVMISS_W<6> {
        DEVMISS_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for RSSIEND event"]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RSSIEND_W<7> {
        RSSIEND_W::new(self)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for BCMATCH event"]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BCMATCH_W<10> {
        BCMATCH_W::new(self)
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for CRCOK event"]
    #[inline(always)]
    pub fn crcok(&mut self) -> CRCOK_W<12> {
        CRCOK_W::new(self)
    }
    #[doc = "Bit 13 - Write '1' to Enable interrupt for CRCERROR event"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CRCERROR_W<13> {
        CRCERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
