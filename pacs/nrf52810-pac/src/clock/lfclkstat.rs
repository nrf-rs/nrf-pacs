#[doc = "Register `LFCLKSTAT` reader"]
pub type R = crate::R<LfclkstatSpec>;
#[doc = "Source of LFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: 32.768 kHz RC oscillator"]
    Rc = 0,
    #[doc = "1: 32.768 kHz crystal oscillator"]
    Xtal = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK"]
    Synth = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Source of LFCLK"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Rc),
            1 => Some(Src::Xtal),
            2 => Some(Src::Synth),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == Src::Rc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Src::Xtal
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == Src::Synth
    }
}
#[doc = "LFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: LFCLK not running"]
    NotRunning = 0,
    #[doc = "1: LFCLK running"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - LFCLK state"]
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
    #[doc = "LFCLK not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "LFCLK running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bits 0:1 - Source of LFCLK"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "LFCLK status\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkstatSpec;
impl crate::RegisterSpec for LfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkstat::R`](R) reader structure"]
impl crate::Readable for LfclkstatSpec {}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LfclkstatSpec {}
