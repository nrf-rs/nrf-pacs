#[doc = "Register `CLOCKSTART` writer"]
pub type W = crate::W<ClockstartSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: Start all trace and debug clocks."]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - "]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start all trace and debug clocks."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, ClockstartSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Start all trace and debug clocks.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockstartSpec;
impl crate::RegisterSpec for ClockstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clockstart::W`](W) writer structure"]
impl crate::Writable for ClockstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKSTART to value 0"]
impl crate::Resettable for ClockstartSpec {}
