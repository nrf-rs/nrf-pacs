#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Current radio state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: RADIO is in the Disabled state"]
    DISABLED = 0,
    #[doc = "1: RADIO is in the RXRU state"]
    RXRU = 1,
    #[doc = "2: RADIO is in the RXIDLE state"]
    RXIDLE = 2,
    #[doc = "3: RADIO is in the RX state"]
    RX = 3,
    #[doc = "4: RADIO is in the RXDISABLED state"]
    RXDISABLE = 4,
    #[doc = "9: RADIO is in the TXRU state"]
    TXRU = 9,
    #[doc = "10: RADIO is in the TXIDLE state"]
    TXIDLE = 10,
    #[doc = "11: RADIO is in the TX state"]
    TX = 11,
    #[doc = "12: RADIO is in the TXDISABLED state"]
    TXDISABLE = 12,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Current radio state"]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::DISABLED),
            1 => Some(STATE_A::RXRU),
            2 => Some(STATE_A::RXIDLE),
            3 => Some(STATE_A::RX),
            4 => Some(STATE_A::RXDISABLE),
            9 => Some(STATE_A::TXRU),
            10 => Some(STATE_A::TXIDLE),
            11 => Some(STATE_A::TX),
            12 => Some(STATE_A::TXDISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RXRU`"]
    #[inline(always)]
    pub fn is_rx_ru(&self) -> bool {
        **self == STATE_A::RXRU
    }
    #[doc = "Checks if the value of the field is `RXIDLE`"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        **self == STATE_A::RXIDLE
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        **self == STATE_A::RX
    }
    #[doc = "Checks if the value of the field is `RXDISABLE`"]
    #[inline(always)]
    pub fn is_rx_disable(&self) -> bool {
        **self == STATE_A::RXDISABLE
    }
    #[doc = "Checks if the value of the field is `TXRU`"]
    #[inline(always)]
    pub fn is_tx_ru(&self) -> bool {
        **self == STATE_A::TXRU
    }
    #[doc = "Checks if the value of the field is `TXIDLE`"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        **self == STATE_A::TXIDLE
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        **self == STATE_A::TX
    }
    #[doc = "Checks if the value of the field is `TXDISABLE`"]
    #[inline(always)]
    pub fn is_tx_disable(&self) -> bool {
        **self == STATE_A::TXDISABLE
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Current radio state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current radio state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
