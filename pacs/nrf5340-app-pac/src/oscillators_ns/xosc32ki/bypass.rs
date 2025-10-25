#[doc = "Register `BYPASS` reader"]
pub type R = crate::R<BypassSpec>;
#[doc = "Register `BYPASS` writer"]
pub type W = crate::W<BypassSpec>;
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Disable (use with crystal or low-swing external source)"]
    Disabled = 0,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    Enabled = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Disabled,
            true => Bypass::Enabled,
        }
    }
    #[doc = "Disable (use with crystal or low-swing external source)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bypass::Disabled
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bypass::Enabled
    }
}
#[doc = "Field `BYPASS` writer - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (use with crystal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Disabled)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, BypassSpec> {
        BypassW::new(self, 0)
    }
}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nYou can [`read`](crate::Reg::read) this register and get [`bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BypassSpec;
impl crate::RegisterSpec for BypassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bypass::R`](R) reader structure"]
impl crate::Readable for BypassSpec {}
#[doc = "`write(|w| ..)` method takes [`bypass::W`](W) writer structure"]
impl crate::Writable for BypassSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BYPASS to value 0"]
impl crate::Resettable for BypassSpec {}
