#[doc = "Register `CLKCONFIG` reader"]
pub type R = crate::R<ClkconfigSpec>;
#[doc = "Register `CLKCONFIG` writer"]
pub type W = crate::W<ClkconfigSpec>;
#[doc = "Clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: 32MHz peripheral clock"]
    Pclk32m = 0,
    #[doc = "1: Audio PLL clock"]
    Aclk = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Clock source selection"]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::Pclk32m,
            true => Clksrc::Aclk,
        }
    }
    #[doc = "32MHz peripheral clock"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        *self == Clksrc::Pclk32m
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Clksrc::Aclk
    }
}
#[doc = "Field `CLKSRC` writer - Clock source selection"]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::Pclk32m)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::Aclk)
    }
}
#[doc = "Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Disable bypass"]
    Disable = 0,
    #[doc = "1: Enable bypass"]
    Enable = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Disable,
            true => Bypass::Enable,
        }
    }
    #[doc = "Disable bypass"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bypass::Disable
    }
    #[doc = "Enable bypass"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bypass::Enable
    }
}
#[doc = "Field `BYPASS` writer - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable bypass"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Disable)
    }
    #[doc = "Enable bypass"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, ClkconfigSpec> {
        ClksrcW::new(self, 0)
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, ClkconfigSpec> {
        BypassW::new(self, 8)
    }
}
#[doc = "Clock source selection for the I2S module\n\nYou can [`read`](crate::Reg::read) this register and get [`clkconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkconfigSpec;
impl crate::RegisterSpec for ClkconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkconfig::R`](R) reader structure"]
impl crate::Readable for ClkconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clkconfig::W`](W) writer structure"]
impl crate::Writable for ClkconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCONFIG to value 0"]
impl crate::Resettable for ClkconfigSpec {}
