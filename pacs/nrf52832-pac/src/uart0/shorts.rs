#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between CTS event and STARTRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtsStartrx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<CtsStartrx> for bool {
    #[inline(always)]
    fn from(variant: CtsStartrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS_STARTRX` reader - Shortcut between CTS event and STARTRX task"]
pub type CtsStartrxR = crate::BitReader<CtsStartrx>;
impl CtsStartrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtsStartrx {
        match self.bits {
            false => CtsStartrx::Disabled,
            true => CtsStartrx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CtsStartrx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CtsStartrx::Enabled
    }
}
#[doc = "Field `CTS_STARTRX` writer - Shortcut between CTS event and STARTRX task"]
pub type CtsStartrxW<'a, REG> = crate::BitWriter<'a, REG, CtsStartrx>;
impl<'a, REG> CtsStartrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtsStartrx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtsStartrx::Enabled)
    }
}
#[doc = "Shortcut between NCTS event and STOPRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NctsStoprx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<NctsStoprx> for bool {
    #[inline(always)]
    fn from(variant: NctsStoprx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS_STOPRX` reader - Shortcut between NCTS event and STOPRX task"]
pub type NctsStoprxR = crate::BitReader<NctsStoprx>;
impl NctsStoprxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NctsStoprx {
        match self.bits {
            false => NctsStoprx::Disabled,
            true => NctsStoprx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NctsStoprx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NctsStoprx::Enabled
    }
}
#[doc = "Field `NCTS_STOPRX` writer - Shortcut between NCTS event and STOPRX task"]
pub type NctsStoprxW<'a, REG> = crate::BitWriter<'a, REG, NctsStoprx>;
impl<'a, REG> NctsStoprxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NctsStoprx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NctsStoprx::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub fn cts_startrx(&self) -> CtsStartrxR {
        CtsStartrxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub fn ncts_stoprx(&self) -> NctsStoprxR {
        NctsStoprxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub fn cts_startrx(&mut self) -> CtsStartrxW<'_, ShortsSpec> {
        CtsStartrxW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub fn ncts_stoprx(&mut self) -> NctsStoprxW<'_, ShortsSpec> {
        NctsStoprxW::new(self, 4)
    }
}
#[doc = "Shortcut register\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
