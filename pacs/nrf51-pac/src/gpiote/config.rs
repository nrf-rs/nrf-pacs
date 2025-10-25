#[doc = "Register `CONFIG[%s]` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG[%s]` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Channel configure in event mode."]
    Event = 1,
    #[doc = "3: Channel configure in task mode."]
    Task = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Disabled),
            1 => Some(Mode::Event),
            3 => Some(Mode::Task),
            _ => None,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode::Disabled
    }
    #[doc = "Channel configure in event mode."]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Mode::Event
    }
    #[doc = "Channel configure in task mode."]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == Mode::Task
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disabled)
    }
    #[doc = "Channel configure in event mode."]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Event)
    }
    #[doc = "Channel configure in task mode."]
    #[inline(always)]
    pub fn task(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Task)
    }
}
#[doc = "Field `PSEL` reader - Pin select."]
pub type PselR = crate::FieldReader;
#[doc = "Field `PSEL` writer - Pin select."]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Effects on output when in Task mode, or events on input that generates an event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    #[doc = "0: No task or event."]
    None = 0,
    #[doc = "1: Low to high."]
    LoToHi = 1,
    #[doc = "2: High to low."]
    HiToLo = 2,
    #[doc = "3: Toggle."]
    Toggle = 3,
}
impl From<Polarity> for u8 {
    #[inline(always)]
    fn from(variant: Polarity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity {
    type Ux = u8;
}
impl crate::IsEnum for Polarity {}
#[doc = "Field `POLARITY` reader - Effects on output when in Task mode, or events on input that generates an event."]
pub type PolarityR = crate::FieldReader<Polarity>;
impl PolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity {
        match self.bits {
            0 => Polarity::None,
            1 => Polarity::LoToHi,
            2 => Polarity::HiToLo,
            3 => Polarity::Toggle,
            _ => unreachable!(),
        }
    }
    #[doc = "No task or event."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Polarity::None
    }
    #[doc = "Low to high."]
    #[inline(always)]
    pub fn is_lo_to_hi(&self) -> bool {
        *self == Polarity::LoToHi
    }
    #[doc = "High to low."]
    #[inline(always)]
    pub fn is_hi_to_lo(&self) -> bool {
        *self == Polarity::HiToLo
    }
    #[doc = "Toggle."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Polarity::Toggle
    }
}
#[doc = "Field `POLARITY` writer - Effects on output when in Task mode, or events on input that generates an event."]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity, crate::Safe>;
impl<'a, REG> PolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No task or event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::None)
    }
    #[doc = "Low to high."]
    #[inline(always)]
    pub fn lo_to_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::LoToHi)
    }
    #[doc = "High to low."]
    #[inline(always)]
    pub fn hi_to_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::HiToLo)
    }
    #[doc = "Toggle."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Toggle)
    }
}
#[doc = "Initial value of the output when the GPIOTE channel is configured as a Task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outinit {
    #[doc = "0: Initial low output when in task mode."]
    Low = 0,
    #[doc = "1: Initial high output when in task mode."]
    High = 1,
}
impl From<Outinit> for bool {
    #[inline(always)]
    fn from(variant: Outinit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTINIT` reader - Initial value of the output when the GPIOTE channel is configured as a Task."]
pub type OutinitR = crate::BitReader<Outinit>;
impl OutinitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outinit {
        match self.bits {
            false => Outinit::Low,
            true => Outinit::High,
        }
    }
    #[doc = "Initial low output when in task mode."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Outinit::Low
    }
    #[doc = "Initial high output when in task mode."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Outinit::High
    }
}
#[doc = "Field `OUTINIT` writer - Initial value of the output when the GPIOTE channel is configured as a Task."]
pub type OutinitW<'a, REG> = crate::BitWriter<'a, REG, Outinit>;
impl<'a, REG> OutinitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial low output when in task mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Outinit::Low)
    }
    #[doc = "Initial high output when in task mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Outinit::High)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline(always)]
    pub fn outinit(&self) -> OutinitR {
        OutinitR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ConfigSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<'_, ConfigSpec> {
        PselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, ConfigSpec> {
        PolarityW::new(self, 16)
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline(always)]
    pub fn outinit(&mut self) -> OutinitW<'_, ConfigSpec> {
        OutinitW::new(self, 20)
    }
}
#[doc = "Channel configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG[%s] to value 0"]
impl crate::Resettable for ConfigSpec {}
