#[doc = "Register `TRCPDCR` reader"]
pub type R = crate::R<TrcpdcrSpec>;
#[doc = "Register `TRCPDCR` writer"]
pub type W = crate::W<TrcpdcrSpec>;
#[doc = "Power up request, to request that power to ETM and access to the trace registers is maintained.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pu {
    #[doc = "0: Power not requested."]
    Disabled = 0,
    #[doc = "1: Power requested."]
    Enabled = 1,
}
impl From<Pu> for bool {
    #[inline(always)]
    fn from(variant: Pu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PU` reader - Power up request, to request that power to ETM and access to the trace registers is maintained."]
pub type PuR = crate::BitReader<Pu>;
impl PuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pu {
        match self.bits {
            false => Pu::Disabled,
            true => Pu::Enabled,
        }
    }
    #[doc = "Power not requested."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pu::Disabled
    }
    #[doc = "Power requested."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pu::Enabled
    }
}
#[doc = "Field `PU` writer - Power up request, to request that power to ETM and access to the trace registers is maintained."]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG, Pu>;
impl<'a, REG> PuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power not requested."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pu::Disabled)
    }
    #[doc = "Power requested."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pu::Enabled)
    }
}
impl R {
    #[doc = "Bit 24 - Power up request, to request that power to ETM and access to the trace registers is maintained."]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Power up request, to request that power to ETM and access to the trace registers is maintained."]
    #[inline(always)]
    pub fn pu(&mut self) -> PuW<'_, TrcpdcrSpec> {
        PuW::new(self, 24)
    }
}
#[doc = "Controls the single-shot comparator.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcpdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcpdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcpdcrSpec;
impl crate::RegisterSpec for TrcpdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcpdcr::R`](R) reader structure"]
impl crate::Readable for TrcpdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcpdcr::W`](W) writer structure"]
impl crate::Writable for TrcpdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCPDCR to value 0"]
impl crate::Resettable for TrcpdcrSpec {}
