#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Configure WDT to either be paused, or kept running, while the CPU is sleeping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleep {
    #[doc = "0: Pause WDT while the CPU is sleeping"]
    Pause = 0,
    #[doc = "1: Keep WDT running while the CPU is sleeping"]
    Run = 1,
}
impl From<Sleep> for bool {
    #[inline(always)]
    fn from(variant: Sleep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
pub type SleepR = crate::BitReader<Sleep>;
impl SleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleep {
        match self.bits {
            false => Sleep::Pause,
            true => Sleep::Run,
        }
    }
    #[doc = "Pause WDT while the CPU is sleeping"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == Sleep::Pause
    }
    #[doc = "Keep WDT running while the CPU is sleeping"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Sleep::Run
    }
}
#[doc = "Field `SLEEP` writer - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG, Sleep>;
impl<'a, REG> SleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pause WDT while the CPU is sleeping"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Pause)
    }
    #[doc = "Keep WDT running while the CPU is sleeping"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Sleep::Run)
    }
}
#[doc = "Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: Pause WDT while the CPU is halted by the debugger"]
    Pause = 0,
    #[doc = "1: Keep WDT running while the CPU is halted by the debugger"]
    Run = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::Pause,
            true => Halt::Run,
        }
    }
    #[doc = "Pause WDT while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == Halt::Pause
    }
    #[doc = "Keep WDT running while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Halt::Run
    }
}
#[doc = "Field `HALT` writer - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pause WDT while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Pause)
    }
    #[doc = "Keep WDT running while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Run)
    }
}
#[doc = "Allow stopping WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopen {
    #[doc = "0: Do not allow stopping WDT"]
    Disable = 0,
    #[doc = "1: Allow stopping WDT"]
    Enable = 1,
}
impl From<Stopen> for bool {
    #[inline(always)]
    fn from(variant: Stopen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPEN` reader - Allow stopping WDT"]
pub type StopenR = crate::BitReader<Stopen>;
impl StopenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopen {
        match self.bits {
            false => Stopen::Disable,
            true => Stopen::Enable,
        }
    }
    #[doc = "Do not allow stopping WDT"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopen::Disable
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopen::Enable
    }
}
#[doc = "Field `STOPEN` writer - Allow stopping WDT"]
pub type StopenW<'a, REG> = crate::BitWriter<'a, REG, Stopen>;
impl<'a, REG> StopenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not allow stopping WDT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopen::Disable)
    }
    #[doc = "Allow stopping WDT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Allow stopping WDT"]
    #[inline(always)]
    pub fn stopen(&self) -> StopenR {
        StopenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure WDT to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<'_, ConfigSpec> {
        SleepW::new(self, 0)
    }
    #[doc = "Bit 3 - Configure WDT to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, ConfigSpec> {
        HaltW::new(self, 3)
    }
    #[doc = "Bit 6 - Allow stopping WDT"]
    #[inline(always)]
    pub fn stopen(&mut self) -> StopenW<'_, ConfigSpec> {
        StopenW::new(self, 6)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x01;
}
