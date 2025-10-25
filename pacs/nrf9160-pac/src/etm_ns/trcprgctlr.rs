#[doc = "Register `TRCPRGCTLR` reader"]
pub type R = crate::R<TrcprgctlrSpec>;
#[doc = "Register `TRCPRGCTLR` writer"]
pub type W = crate::W<TrcprgctlrSpec>;
#[doc = "Trace unit enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: The trace unit is disabled. All trace resources are inactive and no trace is generated."]
    Disabled = 0,
    #[doc = "1: The trace unit is enabled."]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Trace unit enable bit"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "The trace unit is disabled. All trace resources are inactive and no trace is generated."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "The trace unit is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - Trace unit enable bit"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit is disabled. All trace resources are inactive and no trace is generated."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "The trace unit is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Trace unit enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace unit enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, TrcprgctlrSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Enables the trace unit.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcprgctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcprgctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcprgctlrSpec;
impl crate::RegisterSpec for TrcprgctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcprgctlr::R`](R) reader structure"]
impl crate::Readable for TrcprgctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcprgctlr::W`](W) writer structure"]
impl crate::Writable for TrcprgctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCPRGCTLR to value 0"]
impl crate::Resettable for TrcprgctlrSpec {}
