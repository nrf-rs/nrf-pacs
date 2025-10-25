#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ItctrlSpec>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ItctrlSpec>;
#[doc = "Integration Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ime {
    #[doc = "0: Integration mode disabled."]
    Disabled = 0,
    #[doc = "1: Integration mode enabled."]
    Enabled = 1,
}
impl From<Ime> for bool {
    #[inline(always)]
    fn from(variant: Ime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IME` reader - Integration Mode Enable."]
pub type ImeR = crate::BitReader<Ime>;
impl ImeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ime {
        match self.bits {
            false => Ime::Disabled,
            true => Ime::Enabled,
        }
    }
    #[doc = "Integration mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ime::Disabled
    }
    #[doc = "Integration mode enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ime::Enabled
    }
}
#[doc = "Field `IME` writer - Integration Mode Enable."]
pub type ImeW<'a, REG> = crate::BitWriter<'a, REG, Ime>;
impl<'a, REG> ImeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Integration mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ime::Disabled)
    }
    #[doc = "Integration mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ime::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Integration Mode Enable."]
    #[inline(always)]
    pub fn ime(&self) -> ImeR {
        ImeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable."]
    #[inline(always)]
    pub fn ime(&mut self) -> ImeW<'_, ItctrlSpec> {
        ImeW::new(self, 0)
    }
}
#[doc = "The ITCTRL register enables the component to switch from a functional mode, which is the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purposes of integration testing and topology detection.\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItctrlSpec;
impl crate::RegisterSpec for ItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itctrl::R`](R) reader structure"]
impl crate::Readable for ItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`itctrl::W`](W) writer structure"]
impl crate::Writable for ItctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ItctrlSpec {}
