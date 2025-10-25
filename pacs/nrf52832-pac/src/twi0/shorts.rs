#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between BB event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BbSuspend {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<BbSuspend> for bool {
    #[inline(always)]
    fn from(variant: BbSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB_SUSPEND` reader - Shortcut between BB event and SUSPEND task"]
pub type BbSuspendR = crate::BitReader<BbSuspend>;
impl BbSuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BbSuspend {
        match self.bits {
            false => BbSuspend::Disabled,
            true => BbSuspend::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BbSuspend::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BbSuspend::Enabled
    }
}
#[doc = "Field `BB_SUSPEND` writer - Shortcut between BB event and SUSPEND task"]
pub type BbSuspendW<'a, REG> = crate::BitWriter<'a, REG, BbSuspend>;
impl<'a, REG> BbSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BbSuspend::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BbSuspend::Enabled)
    }
}
#[doc = "Shortcut between BB event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BbStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<BbStop> for bool {
    #[inline(always)]
    fn from(variant: BbStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB_STOP` reader - Shortcut between BB event and STOP task"]
pub type BbStopR = crate::BitReader<BbStop>;
impl BbStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BbStop {
        match self.bits {
            false => BbStop::Disabled,
            true => BbStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BbStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BbStop::Enabled
    }
}
#[doc = "Field `BB_STOP` writer - Shortcut between BB event and STOP task"]
pub type BbStopW<'a, REG> = crate::BitWriter<'a, REG, BbStop>;
impl<'a, REG> BbStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BbStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BbStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub fn bb_suspend(&self) -> BbSuspendR {
        BbSuspendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub fn bb_stop(&self) -> BbStopR {
        BbStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub fn bb_suspend(&mut self) -> BbSuspendW<'_, ShortsSpec> {
        BbSuspendW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub fn bb_stop(&mut self) -> BbStopW<'_, ShortsSpec> {
        BbStopW::new(self, 1)
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
