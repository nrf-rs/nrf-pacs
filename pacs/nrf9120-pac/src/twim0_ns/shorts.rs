#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event LASTTX and task STARTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxStartrx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxStartrx> for bool {
    #[inline(always)]
    fn from(variant: LasttxStartrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_STARTRX` reader - Shortcut between event LASTTX and task STARTRX"]
pub type LasttxStartrxR = crate::BitReader<LasttxStartrx>;
impl LasttxStartrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxStartrx {
        match self.bits {
            false => LasttxStartrx::Disabled,
            true => LasttxStartrx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxStartrx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxStartrx::Enabled
    }
}
#[doc = "Field `LASTTX_STARTRX` writer - Shortcut between event LASTTX and task STARTRX"]
pub type LasttxStartrxW<'a, REG> = crate::BitWriter<'a, REG, LasttxStartrx>;
impl<'a, REG> LasttxStartrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStartrx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStartrx::Enabled)
    }
}
#[doc = "Shortcut between event LASTTX and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxSuspend {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxSuspend> for bool {
    #[inline(always)]
    fn from(variant: LasttxSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_SUSPEND` reader - Shortcut between event LASTTX and task SUSPEND"]
pub type LasttxSuspendR = crate::BitReader<LasttxSuspend>;
impl LasttxSuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxSuspend {
        match self.bits {
            false => LasttxSuspend::Disabled,
            true => LasttxSuspend::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxSuspend::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxSuspend::Enabled
    }
}
#[doc = "Field `LASTTX_SUSPEND` writer - Shortcut between event LASTTX and task SUSPEND"]
pub type LasttxSuspendW<'a, REG> = crate::BitWriter<'a, REG, LasttxSuspend>;
impl<'a, REG> LasttxSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxSuspend::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxSuspend::Enabled)
    }
}
#[doc = "Shortcut between event LASTTX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxStop> for bool {
    #[inline(always)]
    fn from(variant: LasttxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_STOP` reader - Shortcut between event LASTTX and task STOP"]
pub type LasttxStopR = crate::BitReader<LasttxStop>;
impl LasttxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxStop {
        match self.bits {
            false => LasttxStop::Disabled,
            true => LasttxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxStop::Enabled
    }
}
#[doc = "Field `LASTTX_STOP` writer - Shortcut between event LASTTX and task STOP"]
pub type LasttxStopW<'a, REG> = crate::BitWriter<'a, REG, LasttxStop>;
impl<'a, REG> LasttxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStop::Enabled)
    }
}
#[doc = "Shortcut between event LASTRX and task STARTTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastrxStarttx {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LastrxStarttx> for bool {
    #[inline(always)]
    fn from(variant: LastrxStarttx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_STARTTX` reader - Shortcut between event LASTRX and task STARTTX"]
pub type LastrxStarttxR = crate::BitReader<LastrxStarttx>;
impl LastrxStarttxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastrxStarttx {
        match self.bits {
            false => LastrxStarttx::Disabled,
            true => LastrxStarttx::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LastrxStarttx::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LastrxStarttx::Enabled
    }
}
#[doc = "Field `LASTRX_STARTTX` writer - Shortcut between event LASTRX and task STARTTX"]
pub type LastrxStarttxW<'a, REG> = crate::BitWriter<'a, REG, LastrxStarttx>;
impl<'a, REG> LastrxStarttxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStarttx::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStarttx::Enabled)
    }
}
#[doc = "Shortcut between event LASTRX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastrxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LastrxStop> for bool {
    #[inline(always)]
    fn from(variant: LastrxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_STOP` reader - Shortcut between event LASTRX and task STOP"]
pub type LastrxStopR = crate::BitReader<LastrxStop>;
impl LastrxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastrxStop {
        match self.bits {
            false => LastrxStop::Disabled,
            true => LastrxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LastrxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LastrxStop::Enabled
    }
}
#[doc = "Field `LASTRX_STOP` writer - Shortcut between event LASTRX and task STOP"]
pub type LastrxStopW<'a, REG> = crate::BitWriter<'a, REG, LastrxStop>;
impl<'a, REG> LastrxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task STARTRX"]
    #[inline(always)]
    pub fn lasttx_startrx(&self) -> LasttxStartrxR {
        LasttxStartrxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    pub fn lasttx_suspend(&self) -> LasttxSuspendR {
        LasttxSuspendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    pub fn lasttx_stop(&self) -> LasttxStopR {
        LasttxStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task STARTTX"]
    #[inline(always)]
    pub fn lastrx_starttx(&self) -> LastrxStarttxR {
        LastrxStarttxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    pub fn lastrx_stop(&self) -> LastrxStopR {
        LastrxStopR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task STARTRX"]
    #[inline(always)]
    #[must_use]
    pub fn lasttx_startrx(&mut self) -> LasttxStartrxW<ShortsSpec> {
        LasttxStartrxW::new(self, 7)
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn lasttx_suspend(&mut self) -> LasttxSuspendW<ShortsSpec> {
        LasttxSuspendW::new(self, 8)
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn lasttx_stop(&mut self) -> LasttxStopW<ShortsSpec> {
        LasttxStopW::new(self, 9)
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task STARTTX"]
    #[inline(always)]
    #[must_use]
    pub fn lastrx_starttx(&mut self) -> LastrxStarttxW<ShortsSpec> {
        LastrxStarttxW::new(self, 10)
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn lastrx_stop(&mut self) -> LastrxStopW<ShortsSpec> {
        LastrxStopW::new(self, 12)
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
