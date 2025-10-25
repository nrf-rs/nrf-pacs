#[doc = "Register `MODULATIONCTRL` reader"]
pub type R = crate::R<ModulationctrlSpec>;
#[doc = "Register `MODULATIONCTRL` writer"]
pub type W = crate::W<ModulationctrlSpec>;
#[doc = "Configuration of modulation control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modulationctrl {
    #[doc = "0: Invalid, defaults to same behaviour as for Internal"]
    Invalid = 0,
    #[doc = "1: Use internal modulator only"]
    Internal = 1,
    #[doc = "2: Output digital modulation signal to a GPIO pin."]
    ModToGpio = 2,
    #[doc = "3: Use internal modulator and output digital modulation signal to a GPIO pin."]
    InternalAndModToGpio = 3,
}
impl From<Modulationctrl> for u8 {
    #[inline(always)]
    fn from(variant: Modulationctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modulationctrl {
    type Ux = u8;
}
impl crate::IsEnum for Modulationctrl {}
#[doc = "Field `MODULATIONCTRL` reader - Configuration of modulation control."]
pub type ModulationctrlR = crate::FieldReader<Modulationctrl>;
impl ModulationctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modulationctrl {
        match self.bits {
            0 => Modulationctrl::Invalid,
            1 => Modulationctrl::Internal,
            2 => Modulationctrl::ModToGpio,
            3 => Modulationctrl::InternalAndModToGpio,
            _ => unreachable!(),
        }
    }
    #[doc = "Invalid, defaults to same behaviour as for Internal"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Modulationctrl::Invalid
    }
    #[doc = "Use internal modulator only"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Modulationctrl::Internal
    }
    #[doc = "Output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn is_mod_to_gpio(&self) -> bool {
        *self == Modulationctrl::ModToGpio
    }
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn is_internal_and_mod_to_gpio(&self) -> bool {
        *self == Modulationctrl::InternalAndModToGpio
    }
}
#[doc = "Field `MODULATIONCTRL` writer - Configuration of modulation control."]
pub type ModulationctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modulationctrl, crate::Safe>;
impl<'a, REG> ModulationctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Invalid, defaults to same behaviour as for Internal"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(Modulationctrl::Invalid)
    }
    #[doc = "Use internal modulator only"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Modulationctrl::Internal)
    }
    #[doc = "Output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn mod_to_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Modulationctrl::ModToGpio)
    }
    #[doc = "Use internal modulator and output digital modulation signal to a GPIO pin."]
    #[inline(always)]
    pub fn internal_and_mod_to_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Modulationctrl::InternalAndModToGpio)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&self) -> ModulationctrlR {
        ModulationctrlR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration of modulation control."]
    #[inline(always)]
    pub fn modulationctrl(&mut self) -> ModulationctrlW<'_, ModulationctrlSpec> {
        ModulationctrlW::new(self, 0)
    }
}
#[doc = "Enables the modulation output to a GPIO pin which can be connected to a second external antenna.\n\nYou can [`read`](crate::Reg::read) this register and get [`modulationctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modulationctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModulationctrlSpec;
impl crate::RegisterSpec for ModulationctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulationctrl::R`](R) reader structure"]
impl crate::Readable for ModulationctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`modulationctrl::W`](W) writer structure"]
impl crate::Writable for ModulationctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODULATIONCTRL to value 0x01"]
impl crate::Resettable for ModulationctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
