#[doc = "Register `HFCLKSTAT` reader"]
pub type R = crate::R<HfclkstatSpec>;
#[doc = "Active clock source for the HF clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: Internal 16MHz RC oscillator running and generating the HFCLK clock."]
    Rc = 0,
    #[doc = "1: External 16MHz/32MHz crystal oscillator running and generating the HFCLK clock."]
    Xtal = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Active clock source for the HF clock."]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            false => Src::Rc,
            true => Src::Xtal,
        }
    }
    #[doc = "Internal 16MHz RC oscillator running and generating the HFCLK clock."]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Src::Rc
    }
    #[doc = "External 16MHz/32MHz crystal oscillator running and generating the HFCLK clock."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Src::Xtal
    }
}
#[doc = "State for the HFCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: HFCLK clock not running."]
    NotRunning = 0,
    #[doc = "1: HFCLK clock running."]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - State for the HFCLK."]
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
    #[doc = "HFCLK clock not running."]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "HFCLK clock running."]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bit 0 - Active clock source for the HF clock."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - State for the HFCLK."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "High frequency clock status.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclkstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkstatSpec;
impl crate::RegisterSpec for HfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkstat::R`](R) reader structure"]
impl crate::Readable for HfclkstatSpec {}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HfclkstatSpec {}
