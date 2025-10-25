#[doc = "Register `CHACHA_DEBUG` reader"]
pub type R = crate::R<ChachaDebugSpec>;
#[doc = "Reflects the debug state of the CHACHA FSM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FsmState {
    #[doc = "0: CHACHA FSM is in idle state"]
    IdleState = 0,
    #[doc = "1: CHACHA FSM is in init state"]
    InitState = 1,
    #[doc = "2: CHACHA FSM is in rounds state"]
    RoundsState = 2,
    #[doc = "3: CHACHA FSM is in final state"]
    FinalState = 3,
}
impl From<FsmState> for u8 {
    #[inline(always)]
    fn from(variant: FsmState) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FsmState {
    type Ux = u8;
}
impl crate::IsEnum for FsmState {}
#[doc = "Field `FSM_STATE` reader - Reflects the debug state of the CHACHA FSM."]
pub type FsmStateR = crate::FieldReader<FsmState>;
impl FsmStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FsmState {
        match self.bits {
            0 => FsmState::IdleState,
            1 => FsmState::InitState,
            2 => FsmState::RoundsState,
            3 => FsmState::FinalState,
            _ => unreachable!(),
        }
    }
    #[doc = "CHACHA FSM is in idle state"]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == FsmState::IdleState
    }
    #[doc = "CHACHA FSM is in init state"]
    #[inline(always)]
    pub fn is_init_state(&self) -> bool {
        *self == FsmState::InitState
    }
    #[doc = "CHACHA FSM is in rounds state"]
    #[inline(always)]
    pub fn is_rounds_state(&self) -> bool {
        *self == FsmState::RoundsState
    }
    #[doc = "CHACHA FSM is in final state"]
    #[inline(always)]
    pub fn is_final_state(&self) -> bool {
        *self == FsmState::FinalState
    }
}
impl R {
    #[doc = "Bits 0:1 - Reflects the debug state of the CHACHA FSM."]
    #[inline(always)]
    pub fn fsm_state(&self) -> FsmStateR {
        FsmStateR::new((self.bits & 3) as u8)
    }
}
#[doc = "Debug register for the CHACHA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chacha_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaDebugSpec;
impl crate::RegisterSpec for ChachaDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chacha_debug::R`](R) reader structure"]
impl crate::Readable for ChachaDebugSpec {}
#[doc = "`reset()` method sets CHACHA_DEBUG to value 0"]
impl crate::Resettable for ChachaDebugSpec {}
