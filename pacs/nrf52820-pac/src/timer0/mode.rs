#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Timer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Select Timer mode"]
    Timer = 0,
    #[doc = "1: Deprecated enumerator - Select Counter mode"]
    Counter = 1,
    #[doc = "2: Select Low Power Counter mode"]
    LowPowerCounter = 2,
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
#[doc = "Field `MODE` reader - Timer mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Timer),
            1 => Some(Mode::Counter),
            2 => Some(Mode::LowPowerCounter),
            _ => None,
        }
    }
    #[doc = "Select Timer mode"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Mode::Timer
    }
    #[doc = "Deprecated enumerator - Select Counter mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Mode::Counter
    }
    #[doc = "Select Low Power Counter mode"]
    #[inline(always)]
    pub fn is_low_power_counter(&self) -> bool {
        *self == Mode::LowPowerCounter
    }
}
#[doc = "Field `MODE` writer - Timer mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Timer)
    }
    #[doc = "Deprecated enumerator - Select Counter mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Counter)
    }
    #[doc = "Select Low Power Counter mode"]
    #[inline(always)]
    pub fn low_power_counter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::LowPowerCounter)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Timer mode selection\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
