#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between WRITE event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteSuspend {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<WriteSuspend> for bool {
    #[inline(always)]
    fn from(variant: WriteSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_SUSPEND` reader - Shortcut between WRITE event and SUSPEND task"]
pub type WriteSuspendR = crate::BitReader<WriteSuspend>;
impl WriteSuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteSuspend {
        match self.bits {
            false => WriteSuspend::Disabled,
            true => WriteSuspend::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WriteSuspend::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WriteSuspend::Enabled
    }
}
#[doc = "Field `WRITE_SUSPEND` writer - Shortcut between WRITE event and SUSPEND task"]
pub type WriteSuspendW<'a, REG> = crate::BitWriter<'a, REG, WriteSuspend>;
impl<'a, REG> WriteSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WriteSuspend::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WriteSuspend::Enabled)
    }
}
#[doc = "Shortcut between READ event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadSuspend {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReadSuspend> for bool {
    #[inline(always)]
    fn from(variant: ReadSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_SUSPEND` reader - Shortcut between READ event and SUSPEND task"]
pub type ReadSuspendR = crate::BitReader<ReadSuspend>;
impl ReadSuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadSuspend {
        match self.bits {
            false => ReadSuspend::Disabled,
            true => ReadSuspend::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadSuspend::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadSuspend::Enabled
    }
}
#[doc = "Field `READ_SUSPEND` writer - Shortcut between READ event and SUSPEND task"]
pub type ReadSuspendW<'a, REG> = crate::BitWriter<'a, REG, ReadSuspend>;
impl<'a, REG> ReadSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadSuspend::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadSuspend::Enabled)
    }
}
impl R {
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub fn write_suspend(&self) -> WriteSuspendR {
        WriteSuspendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub fn read_suspend(&self) -> ReadSuspendR {
        ReadSuspendR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub fn write_suspend(&mut self) -> WriteSuspendW<'_, ShortsSpec> {
        WriteSuspendW::new(self, 13)
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub fn read_suspend(&mut self) -> ReadSuspendW<'_, ShortsSpec> {
        ReadSuspendW::new(self, 14)
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
