#[doc = "Register `LFCLKSTAT` reader"]
pub type R = crate::R<LfclkstatSpec>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "1: 32.768 kHz RC oscillator"]
    Lfrc = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    Lfxo = 2,
    #[doc = "3: 32.768 kHz synthesized from HFCLK"]
    Lfsynt = 3,
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
#[doc = "Field `SRC` reader - Active clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            1 => Some(Src::Lfrc),
            2 => Some(Src::Lfxo),
            3 => Some(Src::Lfsynt),
            _ => None,
        }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == Src::Lfsynt
    }
}
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
    #[doc = "Bits 0:1 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> AlwaysrunningR {
        AlwaysrunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status indicating which LFCLK source is running This register value in any CLOCK instance reflects status only due to configurations/actions in that CLOCK instance.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkstatSpec;
impl crate::RegisterSpec for LfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkstat::R`](R) reader structure"]
impl crate::Readable for LfclkstatSpec {}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LfclkstatSpec {}
