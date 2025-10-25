#[doc = "Register `HFCLKAUDIOSTAT` reader"]
pub type R = crate::R<HfclkaudiostatSpec>;
#[doc = "ALWAYSRUN activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alwaysrunning {
    #[doc = "0: Automatic clock control enabled"]
    NotRunning = 0,
    #[doc = "1: Oscillator is always running"]
    Running = 1,
}
impl From<Alwaysrunning> for bool {
    #[inline(always)]
    fn from(variant: Alwaysrunning) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALWAYSRUNNING` reader - ALWAYSRUN activated"]
pub type AlwaysrunningR = crate::BitReader<Alwaysrunning>;
impl AlwaysrunningR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alwaysrunning {
        match self.bits {
            false => Alwaysrunning::NotRunning,
            true => Alwaysrunning::Running,
        }
    }
    #[doc = "Automatic clock control enabled"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == Alwaysrunning::NotRunning
    }
    #[doc = "Oscillator is always running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Alwaysrunning::Running
    }
}
#[doc = "HFCLKAUDIO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: HFCLKAUDIO not running"]
    NotRunning = 0,
    #[doc = "1: HFCLKAUDIO running"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - HFCLKAUDIO state"]
pub type StateR = crate::BitReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            false => State::NotRunning,
            true => State::Running,
        }
    }
    #[doc = "HFCLKAUDIO not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "HFCLKAUDIO running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> AlwaysrunningR {
        AlwaysrunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLKAUDIO state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status indicating which HFCLKAUDIO source is running\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkaudiostat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkaudiostatSpec;
impl crate::RegisterSpec for HfclkaudiostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkaudiostat::R`](R) reader structure"]
impl crate::Readable for HfclkaudiostatSpec {}
#[doc = "`reset()` method sets HFCLKAUDIOSTAT to value 0"]
impl crate::Resettable for HfclkaudiostatSpec {}
