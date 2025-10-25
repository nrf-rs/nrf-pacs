#[doc = "Register `PROFILINGENABLE` reader"]
pub type R = crate::R<ProfilingenableSpec>;
#[doc = "Register `PROFILINGENABLE` writer"]
pub type W = crate::W<ProfilingenableSpec>;
#[doc = "Enable the profiling counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable profiling"]
    Disable = 0,
    #[doc = "1: Enable profiling"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable the profiling counters"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disable,
            true => Enable::Enable,
        }
    }
    #[doc = "Disable profiling"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enable::Disable
    }
    #[doc = "Enable profiling"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Enable the profiling counters"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable profiling"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable profiling"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the profiling counters"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the profiling counters"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ProfilingenableSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Enable the profiling counters.\n\nYou can [`read`](crate::Reg::read) this register and get [`profilingenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`profilingenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProfilingenableSpec;
impl crate::RegisterSpec for ProfilingenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`profilingenable::R`](R) reader structure"]
impl crate::Readable for ProfilingenableSpec {}
#[doc = "`write(|w| ..)` method takes [`profilingenable::W`](W) writer structure"]
impl crate::Writable for ProfilingenableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PROFILINGENABLE to value 0"]
impl crate::Resettable for ProfilingenableSpec {}
