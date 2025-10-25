#[doc = "Register `SAMPLERATE` reader"]
pub type R = crate::R<SamplerateSpec>;
#[doc = "Register `SAMPLERATE` writer"]
pub type W = crate::W<SamplerateSpec>;
#[doc = "Field `CC` reader - Capture and compare value. Sample rate is 16 MHz/CC"]
pub type CcR = crate::FieldReader<u16>;
#[doc = "Field `CC` writer - Capture and compare value. Sample rate is 16 MHz/CC"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Select mode for sample rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Rate is controlled from SAMPLE task"]
    Task = 0,
    #[doc = "1: Rate is controlled from local timer (use CC to control the rate)"]
    Timers = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Select mode for sample rate control"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Task,
            true => Mode::Timers,
        }
    }
    #[doc = "Rate is controlled from SAMPLE task"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == Mode::Task
    }
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    #[inline(always)]
    pub fn is_timers(&self) -> bool {
        *self == Mode::Timers
    }
}
#[doc = "Field `MODE` writer - Select mode for sample rate control"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rate is controlled from SAMPLE task"]
    #[inline(always)]
    pub fn task(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Task)
    }
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    #[inline(always)]
    pub fn timers(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Timers)
    }
}
impl R {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, SamplerateSpec> {
        CcW::new(self, 0)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, SamplerateSpec> {
        ModeW::new(self, 12)
    }
}
#[doc = "Controls normal or continuous sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`samplerate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samplerate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SamplerateSpec;
impl crate::RegisterSpec for SamplerateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samplerate::R`](R) reader structure"]
impl crate::Readable for SamplerateSpec {}
#[doc = "`write(|w| ..)` method takes [`samplerate::W`](W) writer structure"]
impl crate::Writable for SamplerateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLERATE to value 0"]
impl crate::Resettable for SamplerateSpec {}
