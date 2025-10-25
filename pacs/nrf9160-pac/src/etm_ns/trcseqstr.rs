#[doc = "Register `TRCSEQSTR` reader"]
pub type R = crate::R<TrcseqstrSpec>;
#[doc = "Register `TRCSEQSTR` writer"]
pub type W = crate::W<TrcseqstrSpec>;
#[doc = "Sets or returns the state of the sequencer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: The sequencer is in state 0."]
    State0 = 0,
    #[doc = "1: The sequencer is in state 1."]
    State1 = 1,
    #[doc = "2: The sequencer is in state 2."]
    State2 = 2,
    #[doc = "3: The sequencer is in state 3."]
    State3 = 3,
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
#[doc = "Field `STATE` reader - Sets or returns the state of the sequencer."]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            0 => State::State0,
            1 => State::State1,
            2 => State::State2,
            3 => State::State3,
            _ => unreachable!(),
        }
    }
    #[doc = "The sequencer is in state 0."]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        *self == State::State0
    }
    #[doc = "The sequencer is in state 1."]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        *self == State::State1
    }
    #[doc = "The sequencer is in state 2."]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        *self == State::State2
    }
    #[doc = "The sequencer is in state 3."]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        *self == State::State3
    }
}
#[doc = "Field `STATE` writer - Sets or returns the state of the sequencer."]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 2, State, crate::Safe>;
impl<'a, REG> StateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The sequencer is in state 0."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut crate::W<REG> {
        self.variant(State::State0)
    }
    #[doc = "The sequencer is in state 1."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut crate::W<REG> {
        self.variant(State::State1)
    }
    #[doc = "The sequencer is in state 2."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut crate::W<REG> {
        self.variant(State::State2)
    }
    #[doc = "The sequencer is in state 3."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut crate::W<REG> {
        self.variant(State::State3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets or returns the state of the sequencer."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets or returns the state of the sequencer."]
    #[inline(always)]
    pub fn state(&mut self) -> StateW<'_, TrcseqstrSpec> {
        StateW::new(self, 0)
    }
}
#[doc = "Use this to set, or read, the sequencer state. Might ignore writes when the trace unit is enabled or not idle. Only returns stable data when TRCSTATR.PMSTABLE == 1. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcseqstrSpec;
impl crate::RegisterSpec for TrcseqstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcseqstr::R`](R) reader structure"]
impl crate::Readable for TrcseqstrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcseqstr::W`](W) writer structure"]
impl crate::Writable for TrcseqstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSEQSTR to value 0"]
impl crate::Resettable for TrcseqstrSpec {}
