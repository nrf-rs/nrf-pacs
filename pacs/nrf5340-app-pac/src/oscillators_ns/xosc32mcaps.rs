#[doc = "Register `XOSC32MCAPS` reader"]
pub type R = crate::R<Xosc32mcapsSpec>;
#[doc = "Register `XOSC32MCAPS` writer"]
pub type W = crate::W<Xosc32mcapsSpec>;
#[doc = "Field `CAPVALUE` reader - Value representing capacitance, calculated using provided equation"]
pub type CapvalueR = crate::FieldReader;
#[doc = "Field `CAPVALUE` writer - Value representing capacitance, calculated using provided equation"]
pub type CapvalueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Enable on-chip capacitors on XC1 and XC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Capacitor disabled (use external caps)"]
    Disabled = 0,
    #[doc = "1: Capacitor enabled"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable on-chip capacitors on XC1 and XC2"]
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
    #[doc = "Capacitor disabled (use external caps)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Capacitor enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - Enable on-chip capacitors on XC1 and XC2"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capacitor disabled (use external caps)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Capacitor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&self) -> CapvalueR {
        CapvalueR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&mut self) -> CapvalueW<'_, Xosc32mcapsSpec> {
        CapvalueW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Xosc32mcapsSpec> {
        EnableW::new(self, 8)
    }
}
#[doc = "Programmable capacitance of XC1 and XC2\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32mcaps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32mcaps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32mcapsSpec;
impl crate::RegisterSpec for Xosc32mcapsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32mcaps::R`](R) reader structure"]
impl crate::Readable for Xosc32mcapsSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc32mcaps::W`](W) writer structure"]
impl crate::Writable for Xosc32mcapsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XOSC32MCAPS to value 0"]
impl crate::Resettable for Xosc32mcapsSpec {}
