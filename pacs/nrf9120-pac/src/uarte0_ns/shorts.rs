#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event ENDRX and task STARTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndrxStartrx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndrxStartrx> for bool {
    #[inline(always)]
    fn from(variant: EndrxStartrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX_STARTRX` reader - Shortcut between event ENDRX and task STARTRX"]
pub type EndrxStartrxR = crate::BitReader<EndrxStartrx>;
impl EndrxStartrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndrxStartrx {
        match self.bits {
            false => EndrxStartrx::Disabled,
            true => EndrxStartrx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndrxStartrx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndrxStartrx::Enabled
    }
}
#[doc = "Field `ENDRX_STARTRX` writer - Shortcut between event ENDRX and task STARTRX"]
pub type EndrxStartrxW<'a, REG> = crate::BitWriter<'a, REG, EndrxStartrx>;
impl<'a, REG> EndrxStartrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxStartrx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxStartrx::Enabled)
    }
}
#[doc = "Shortcut between event ENDRX and task STOPRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndrxStoprx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndrxStoprx> for bool {
    #[inline(always)]
    fn from(variant: EndrxStoprx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX_STOPRX` reader - Shortcut between event ENDRX and task STOPRX"]
pub type EndrxStoprxR = crate::BitReader<EndrxStoprx>;
impl EndrxStoprxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndrxStoprx {
        match self.bits {
            false => EndrxStoprx::Disabled,
            true => EndrxStoprx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndrxStoprx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndrxStoprx::Enabled
    }
}
#[doc = "Field `ENDRX_STOPRX` writer - Shortcut between event ENDRX and task STOPRX"]
pub type EndrxStoprxW<'a, REG> = crate::BitWriter<'a, REG, EndrxStoprx>;
impl<'a, REG> EndrxStoprxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxStoprx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxStoprx::Enabled)
    }
}
impl R {
    #[doc = "Bit 5 - Shortcut between event ENDRX and task STARTRX"]
    #[inline(always)]
    pub fn endrx_startrx(&self) -> EndrxStartrxR {
        EndrxStartrxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event ENDRX and task STOPRX"]
    #[inline(always)]
    pub fn endrx_stoprx(&self) -> EndrxStoprxR {
        EndrxStoprxR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Shortcut between event ENDRX and task STARTRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx_startrx(&mut self) -> EndrxStartrxW<ShortsSpec> {
        EndrxStartrxW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event ENDRX and task STOPRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx_stoprx(&mut self) -> EndrxStoprxW<ShortsSpec> {
        EndrxStoprxW::new(self, 6)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shorts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
