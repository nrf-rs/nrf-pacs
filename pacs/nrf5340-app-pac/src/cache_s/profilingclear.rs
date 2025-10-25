#[doc = "Register `PROFILINGCLEAR` writer"]
pub type W = crate::W<ProfilingclearSpec>;
#[doc = "Clearing the profiling counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clear {
    #[doc = "1: Clear the profiling counters"]
    Clear = 1,
}
impl From<Clear> for bool {
    #[inline(always)]
    fn from(variant: Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` writer - Clearing the profiling counters"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG, Clear>;
impl<'a, REG> ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the profiling counters"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clearing the profiling counters"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, ProfilingclearSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "Clear the profiling counters.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`profilingclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProfilingclearSpec;
impl crate::RegisterSpec for ProfilingclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`profilingclear::W`](W) writer structure"]
impl crate::Writable for ProfilingclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROFILINGCLEAR to value 0"]
impl crate::Resettable for ProfilingclearSpec {}
