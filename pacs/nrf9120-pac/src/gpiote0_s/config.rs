#[doc = "Register `CONFIG[%s]` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG[%s]` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    Disabled = 0,
    #[doc = "1: Event mode"]
    Event = 1,
    #[doc = "3: Task mode"]
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
    #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode::Disabled
    }
    #[doc = "Event mode"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Mode::Event
    }
    #[doc = "Task mode"]
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
    #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disabled)
    }
    #[doc = "Event mode"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Event)
    }
    #[doc = "Task mode"]
    #[inline(always)]
    pub fn task(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Task)
    }
}
#[doc = "Field `PSEL` reader - GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
pub type PselR = crate::FieldReader;
#[doc = "Field `PSEL` writer - GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    #[doc = "0: Task mode: No effect on pin from OUT\\[n\\]
task. Event mode: no IN\\[n\\]
event generated on pin activity."]
    None = 0,
    #[doc = "1: Task mode: Set pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when rising edge on pin."]
    LoToHi = 1,
    #[doc = "2: Task mode: Clear pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when falling edge on pin."]
    HiToLo = 2,
    #[doc = "3: Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\]
when any change on pin."]
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
#[doc = "Field `POLARITY` reader - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
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
    #[doc = "Task mode: No effect on pin from OUT\\[n\\]
task. Event mode: no IN\\[n\\]
event generated on pin activity."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Polarity::None
    }
    #[doc = "Task mode: Set pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when rising edge on pin."]
    #[inline(always)]
    pub fn is_lo_to_hi(&self) -> bool {
        *self == Polarity::LoToHi
    }
    #[doc = "Task mode: Clear pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when falling edge on pin."]
    #[inline(always)]
    pub fn is_hi_to_lo(&self) -> bool {
        *self == Polarity::HiToLo
    }
    #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\]
when any change on pin."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Polarity::Toggle
    }
}
#[doc = "Field `POLARITY` writer - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity, crate::Safe>;
impl<'a, REG> PolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Task mode: No effect on pin from OUT\\[n\\]
task. Event mode: no IN\\[n\\]
event generated on pin activity."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::None)
    }
    #[doc = "Task mode: Set pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when rising edge on pin."]
    #[inline(always)]
    pub fn lo_to_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::LoToHi)
    }
    #[doc = "Task mode: Clear pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when falling edge on pin."]
    #[inline(always)]
    pub fn hi_to_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::HiToLo)
    }
    #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\]
when any change on pin."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Toggle)
    }
}
#[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outinit {
    #[doc = "0: Task mode: Initial value of pin before task triggering is low"]
    Low = 0,
    #[doc = "1: Task mode: Initial value of pin before task triggering is high"]
    High = 1,
}
impl From<Outinit> for bool {
    #[inline(always)]
    fn from(variant: Outinit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTINIT` reader - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
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
    #[doc = "Task mode: Initial value of pin before task triggering is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Outinit::Low
    }
    #[doc = "Task mode: Initial value of pin before task triggering is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Outinit::High
    }
}
#[doc = "Field `OUTINIT` writer - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
pub type OutinitW<'a, REG> = crate::BitWriter<'a, REG, Outinit>;
impl<'a, REG> OutinitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Task mode: Initial value of pin before task triggering is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Outinit::Low)
    }
    #[doc = "Task mode: Initial value of pin before task triggering is high"]
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
    #[doc = "Bits 8:12 - GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    pub fn outinit(&self) -> OutinitR {
        OutinitR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ConfigSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 8:12 - GPIO number associated with SET\\[n\\], CLR\\[n\\], and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<ConfigSpec> {
        PselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<ConfigSpec> {
        PolarityW::new(self, 16)
    }
    #[doc = "Bit 20 - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    #[must_use]
    pub fn outinit(&mut self) -> OutinitW<ConfigSpec> {
        OutinitW::new(self, 20)
    }
}
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
