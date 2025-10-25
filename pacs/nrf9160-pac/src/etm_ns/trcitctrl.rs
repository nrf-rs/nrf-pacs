#[doc = "Register `TRCITCTRL` reader"]
pub type R = crate::R<TrcitctrlSpec>;
#[doc = "Register `TRCITCTRL` writer"]
pub type W = crate::W<TrcitctrlSpec>;
#[doc = "Integration mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ime {
    #[doc = "0: ETM is not in integration mode."]
    Disabled = 0,
    #[doc = "1: ETM is in integration mode."]
    Enabled = 1,
}
impl From<Ime> for bool {
    #[inline(always)]
    fn from(variant: Ime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IME` reader - Integration mode enable"]
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
    #[doc = "ETM is not in integration mode."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ime::Disabled
    }
    #[doc = "ETM is in integration mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ime::Enabled
    }
}
#[doc = "Field `IME` writer - Integration mode enable"]
pub type ImeW<'a, REG> = crate::BitWriter<'a, REG, Ime>;
impl<'a, REG> ImeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETM is not in integration mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ime::Disabled)
    }
    #[doc = "ETM is in integration mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ime::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Integration mode enable"]
    #[inline(always)]
    pub fn ime(&self) -> ImeR {
        ImeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration mode enable"]
    #[inline(always)]
    pub fn ime(&mut self) -> ImeW<'_, TrcitctrlSpec> {
        ImeW::new(self, 0)
    }
}
#[doc = "Enables topology detection or integration testing, by putting ETM-M33 into integration mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcitctrlSpec;
impl crate::RegisterSpec for TrcitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitctrl::R`](R) reader structure"]
impl crate::Readable for TrcitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`trcitctrl::W`](W) writer structure"]
impl crate::Writable for TrcitctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCITCTRL to value 0"]
impl crate::Resettable for TrcitctrlSpec {}
