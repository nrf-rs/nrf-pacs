#[doc = "Register `HFCLK192MSTAT` reader"]
pub type R = crate::R<Hfclk192mstatSpec>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: Clock source: HFINT - on-chip oscillator"]
    Hfint = 0,
    #[doc = "1: Clock source: HFXO - derived from external 32 MHz crystal oscillator"]
    Hfxo = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Active clock source"]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            false => Src::Hfint,
            true => Src::Hfxo,
        }
    }
    #[doc = "Clock source: HFINT - on-chip oscillator"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == Src::Hfint
    }
    #[doc = "Clock source: HFXO - derived from external 32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Src::Hfxo
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
#[doc = "HFCLK192M state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: HFCLK192M not running"]
    NotRunning = 0,
    #[doc = "1: HFCLK192M running"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - HFCLK192M state"]
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
    #[doc = "HFCLK192M not running"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "HFCLK192M running"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bit 0 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ALWAYSRUN activated"]
    #[inline(always)]
    pub fn alwaysrunning(&self) -> AlwaysrunningR {
        AlwaysrunningR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLK192M state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status indicating which HFCLK192M source is running\n\nYou can [`read`](crate::Reg::read) this register and get [`hfclk192mstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfclk192mstatSpec;
impl crate::RegisterSpec for Hfclk192mstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclk192mstat::R`](R) reader structure"]
impl crate::Readable for Hfclk192mstatSpec {}
#[doc = "`reset()` method sets HFCLK192MSTAT to value 0"]
impl crate::Resettable for Hfclk192mstatSpec {}
