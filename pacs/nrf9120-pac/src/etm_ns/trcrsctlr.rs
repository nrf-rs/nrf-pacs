#[doc = "Register `TRCRSCTLR[%s]` reader"]
pub type R = crate::R<TrcrsctlrSpec>;
#[doc = "Register `TRCRSCTLR[%s]` writer"]
pub type W = crate::W<TrcrsctlrSpec>;
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
    pub fn en(&mut self) -> EnW<'_, TrcrsctlrSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Description collection: Controls the selection of the resources in the trace unit. Might ignore writes when the trace unit is enabled or not idle. If software selects a non-implemented resource then CONSTRAINED UNPREDICTABLE behavior of the resource selector occurs, so the resource selector might fire unexpectedly or might not fire. Reads of the TRCRSCTLRn might return UNKNOWN.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcrsctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcrsctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcrsctlrSpec;
impl crate::RegisterSpec for TrcrsctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcrsctlr::R`](R) reader structure"]
impl crate::Readable for TrcrsctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcrsctlr::W`](W) writer structure"]
impl crate::Writable for TrcrsctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCRSCTLR[%s] to value 0"]
impl crate::Resettable for TrcrsctlrSpec {}
