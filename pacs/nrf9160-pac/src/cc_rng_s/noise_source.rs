#[doc = "Register `NOISE_SOURCE` reader"]
pub type R = crate::R<NoiseSourceSpec>;
#[doc = "Register `NOISE_SOURCE` writer"]
pub type W = crate::W<NoiseSourceSpec>;
#[doc = "Enable or disable the noise source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Noise source is disabled"]
    Disabled = 0,
    #[doc = "1: Noise source is enabled"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable or disable the noise source."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Noise source is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Noise source is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable or disable the noise source."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise source is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Noise source is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable the noise source."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the noise source."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, NoiseSourceSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "This register controls the ring oscillator circuit used as a noise source.\n\nYou can [`read`](crate::Reg::read) this register and get [`noise_source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noise_source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NoiseSourceSpec;
impl crate::RegisterSpec for NoiseSourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`noise_source::R`](R) reader structure"]
impl crate::Readable for NoiseSourceSpec {}
#[doc = "`write(|w| ..)` method takes [`noise_source::W`](W) writer structure"]
impl crate::Writable for NoiseSourceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NOISE_SOURCE to value 0"]
impl crate::Resettable for NoiseSourceSpec {}
