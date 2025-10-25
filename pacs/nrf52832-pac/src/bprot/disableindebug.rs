#[doc = "Register `DISABLEINDEBUG` reader"]
pub type R = crate::R<DisableindebugSpec>;
#[doc = "Register `DISABLEINDEBUG` writer"]
pub type W = crate::W<DisableindebugSpec>;
#[doc = "Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disableindebug {
    #[doc = "1: Disable in debug"]
    Disabled = 1,
    #[doc = "0: Enable in debug"]
    Enabled = 0,
}
impl From<Disableindebug> for bool {
    #[inline(always)]
    fn from(variant: Disableindebug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLEINDEBUG` reader - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
pub type DisableindebugR = crate::BitReader<Disableindebug>;
impl DisableindebugR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disableindebug {
        match self.bits {
            true => Disableindebug::Disabled,
            false => Disableindebug::Enabled,
        }
    }
    #[doc = "Disable in debug"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disableindebug::Disabled
    }
    #[doc = "Enable in debug"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disableindebug::Enabled
    }
}
#[doc = "Field `DISABLEINDEBUG` writer - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
pub type DisableindebugW<'a, REG> = crate::BitWriter<'a, REG, Disableindebug>;
impl<'a, REG> DisableindebugW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable in debug"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disableindebug::Disabled)
    }
    #[doc = "Enable in debug"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disableindebug::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    pub fn disableindebug(&self) -> DisableindebugR {
        DisableindebugR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    pub fn disableindebug(&mut self) -> DisableindebugW<'_, DisableindebugSpec> {
        DisableindebugW::new(self, 0)
    }
}
#[doc = "Disable protection mechanism in debug interface mode\n\nYou can [`read`](crate::Reg::read) this register and get [`disableindebug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disableindebug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableindebugSpec;
impl crate::RegisterSpec for DisableindebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disableindebug::R`](R) reader structure"]
impl crate::Readable for DisableindebugSpec {}
#[doc = "`write(|w| ..)` method takes [`disableindebug::W`](W) writer structure"]
impl crate::Writable for DisableindebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISABLEINDEBUG to value 0x01"]
impl crate::Resettable for DisableindebugSpec {
    const RESET_VALUE: u32 = 0x01;
}
