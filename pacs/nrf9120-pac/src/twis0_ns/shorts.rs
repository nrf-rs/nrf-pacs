#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event WRITE and task SUSPEND\n\nValue on reset: 0"]
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
#[doc = "Field `WRITE_SUSPEND` reader - Shortcut between event WRITE and task SUSPEND"]
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
#[doc = "Field `WRITE_SUSPEND` writer - Shortcut between event WRITE and task SUSPEND"]
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
#[doc = "Shortcut between event READ and task SUSPEND\n\nValue on reset: 0"]
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
#[doc = "Field `READ_SUSPEND` reader - Shortcut between event READ and task SUSPEND"]
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
#[doc = "Field `READ_SUSPEND` writer - Shortcut between event READ and task SUSPEND"]
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
    #[doc = "Bit 13 - Shortcut between event WRITE and task SUSPEND"]
    #[inline(always)]
    pub fn write_suspend(&self) -> WriteSuspendR {
        WriteSuspendR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shortcut between event READ and task SUSPEND"]
    #[inline(always)]
    pub fn read_suspend(&self) -> ReadSuspendR {
        ReadSuspendR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Shortcut between event WRITE and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn write_suspend(&mut self) -> WriteSuspendW<ShortsSpec> {
        WriteSuspendW::new(self, 13)
    }
    #[doc = "Bit 14 - Shortcut between event READ and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn read_suspend(&mut self) -> ReadSuspendW<ShortsSpec> {
        ReadSuspendW::new(self, 14)
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
