#[doc = "Register `CLOCKSTOP` writer"]
pub type W = crate::W<ClockstopSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "1: Stop all trace and debug clocks."]
    Stop = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - "]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop all trace and debug clocks."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Stop)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, ClockstopSpec> {
        StopW::new(self, 0)
    }
}
#[doc = "Stop all trace and debug clocks.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockstopSpec;
impl crate::RegisterSpec for ClockstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clockstop::W`](W) writer structure"]
impl crate::Writable for ClockstopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKSTOP to value 0"]
impl crate::Resettable for ClockstopSpec {}
