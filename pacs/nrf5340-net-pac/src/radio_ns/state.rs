#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Current radio state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: RADIO is in the Disabled state"]
    Disabled = 0,
    #[doc = "1: RADIO is in the RXRU state"]
    RxRu = 1,
    #[doc = "2: RADIO is in the RXIDLE state"]
    RxIdle = 2,
    #[doc = "3: RADIO is in the RX state"]
    Rx = 3,
    #[doc = "4: RADIO is in the RXDISABLED state"]
    RxDisable = 4,
    #[doc = "9: RADIO is in the TXRU state"]
    TxRu = 9,
    #[doc = "10: RADIO is in the TXIDLE state"]
    TxIdle = 10,
    #[doc = "11: RADIO is in the TX state"]
    Tx = 11,
    #[doc = "12: RADIO is in the TXDISABLED state"]
    TxDisable = 12,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - Current radio state"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Disabled),
            1 => Some(State::RxRu),
            2 => Some(State::RxIdle),
            3 => Some(State::Rx),
            4 => Some(State::RxDisable),
            9 => Some(State::TxRu),
            10 => Some(State::TxIdle),
            11 => Some(State::Tx),
            12 => Some(State::TxDisable),
            _ => None,
        }
    }
    #[doc = "RADIO is in the Disabled state"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == State::Disabled
    }
    #[doc = "RADIO is in the RXRU state"]
    #[inline(always)]
    pub fn is_rx_ru(&self) -> bool {
        *self == State::RxRu
    }
    #[doc = "RADIO is in the RXIDLE state"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        *self == State::RxIdle
    }
    #[doc = "RADIO is in the RX state"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == State::Rx
    }
    #[doc = "RADIO is in the RXDISABLED state"]
    #[inline(always)]
    pub fn is_rx_disable(&self) -> bool {
        *self == State::RxDisable
    }
    #[doc = "RADIO is in the TXRU state"]
    #[inline(always)]
    pub fn is_tx_ru(&self) -> bool {
        *self == State::TxRu
    }
    #[doc = "RADIO is in the TXIDLE state"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        *self == State::TxIdle
    }
    #[doc = "RADIO is in the TX state"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == State::Tx
    }
    #[doc = "RADIO is in the TXDISABLED state"]
    #[inline(always)]
    pub fn is_tx_disable(&self) -> bool {
        *self == State::TxDisable
    }
}
impl R {
    #[doc = "Bits 0:3 - Current radio state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current radio state\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
