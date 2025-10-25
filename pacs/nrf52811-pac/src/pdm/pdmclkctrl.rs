#[doc = "Register `PDMCLKCTRL` reader"]
pub type R = crate::R<PdmclkctrlSpec>;
#[doc = "Register `PDMCLKCTRL` writer"]
pub type W = crate::W<PdmclkctrlSpec>;
#[doc = "PDM_CLK frequency\n\nValue on reset: 138412032"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Freq {
    #[doc = "134217728: PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    _1000k = 134217728,
    #[doc = "138412032: PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
    Default = 138412032,
    #[doc = "142606336: PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    _1067k = 142606336,
}
impl From<Freq> for u32 {
    #[inline(always)]
    fn from(variant: Freq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freq {
    type Ux = u32;
}
impl crate::IsEnum for Freq {}
#[doc = "Field `FREQ` reader - PDM_CLK frequency"]
pub type FreqR = crate::FieldReader<Freq>;
impl FreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Freq> {
        match self.bits {
            134217728 => Some(Freq::_1000k),
            138412032 => Some(Freq::Default),
            142606336 => Some(Freq::_1067k),
            _ => None,
        }
    }
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    #[inline(always)]
    pub fn is_1000k(&self) -> bool {
        *self == Freq::_1000k
    }
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Freq::Default
    }
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    #[inline(always)]
    pub fn is_1067k(&self) -> bool {
        *self == Freq::_1067k
    }
}
#[doc = "Field `FREQ` writer - PDM_CLK frequency"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, Freq>;
impl<'a, REG> FreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    #[inline(always)]
    pub fn _1000k(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::_1000k)
    }
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::Default)
    }
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    #[inline(always)]
    pub fn _1067k(self) -> &'a mut crate::W<REG> {
        self.variant(Freq::_1067k)
    }
}
impl R {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, PdmclkctrlSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "PDM clock generator control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdmclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmclkctrlSpec;
impl crate::RegisterSpec for PdmclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmclkctrl::R`](R) reader structure"]
impl crate::Readable for PdmclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pdmclkctrl::W`](W) writer structure"]
impl crate::Writable for PdmclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDMCLKCTRL to value 0x0840_0000"]
impl crate::Resettable for PdmclkctrlSpec {
    const RESET_VALUE: u32 = 0x0840_0000;
}
