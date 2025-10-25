#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event READY and task SAMPLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadySample {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReadySample> for bool {
    #[inline(always)]
    fn from(variant: ReadySample) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_SAMPLE` reader - Shortcut between event READY and task SAMPLE"]
pub type ReadySampleR = crate::BitReader<ReadySample>;
impl ReadySampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadySample {
        match self.bits {
            false => ReadySample::Disabled,
            true => ReadySample::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadySample::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadySample::Enabled
    }
}
#[doc = "Field `READY_SAMPLE` writer - Shortcut between event READY and task SAMPLE"]
pub type ReadySampleW<'a, REG> = crate::BitWriter<'a, REG, ReadySample>;
impl<'a, REG> ReadySampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadySample::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadySample::Enabled)
    }
}
#[doc = "Shortcut between event READY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReadyStop> for bool {
    #[inline(always)]
    fn from(variant: ReadyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_STOP` reader - Shortcut between event READY and task STOP"]
pub type ReadyStopR = crate::BitReader<ReadyStop>;
impl ReadyStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadyStop {
        match self.bits {
            false => ReadyStop::Disabled,
            true => ReadyStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadyStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadyStop::Enabled
    }
}
#[doc = "Field `READY_STOP` writer - Shortcut between event READY and task STOP"]
pub type ReadyStopW<'a, REG> = crate::BitWriter<'a, REG, ReadyStop>;
impl<'a, REG> ReadyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStop::Enabled)
    }
}
#[doc = "Shortcut between event DOWN and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DownStop> for bool {
    #[inline(always)]
    fn from(variant: DownStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN_STOP` reader - Shortcut between event DOWN and task STOP"]
pub type DownStopR = crate::BitReader<DownStop>;
impl DownStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DownStop {
        match self.bits {
            false => DownStop::Disabled,
            true => DownStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DownStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DownStop::Enabled
    }
}
#[doc = "Field `DOWN_STOP` writer - Shortcut between event DOWN and task STOP"]
pub type DownStopW<'a, REG> = crate::BitWriter<'a, REG, DownStop>;
impl<'a, REG> DownStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DownStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DownStop::Enabled)
    }
}
#[doc = "Shortcut between event UP and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<UpStop> for bool {
    #[inline(always)]
    fn from(variant: UpStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP_STOP` reader - Shortcut between event UP and task STOP"]
pub type UpStopR = crate::BitReader<UpStop>;
impl UpStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UpStop {
        match self.bits {
            false => UpStop::Disabled,
            true => UpStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UpStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UpStop::Enabled
    }
}
#[doc = "Field `UP_STOP` writer - Shortcut between event UP and task STOP"]
pub type UpStopW<'a, REG> = crate::BitWriter<'a, REG, UpStop>;
impl<'a, REG> UpStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UpStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UpStop::Enabled)
    }
}
#[doc = "Shortcut between event CROSS and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<CrossStop> for bool {
    #[inline(always)]
    fn from(variant: CrossStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS_STOP` reader - Shortcut between event CROSS and task STOP"]
pub type CrossStopR = crate::BitReader<CrossStop>;
impl CrossStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrossStop {
        match self.bits {
            false => CrossStop::Disabled,
            true => CrossStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CrossStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CrossStop::Enabled
    }
}
#[doc = "Field `CROSS_STOP` writer - Shortcut between event CROSS and task STOP"]
pub type CrossStopW<'a, REG> = crate::BitWriter<'a, REG, CrossStop>;
impl<'a, REG> CrossStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrossStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrossStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    pub fn ready_sample(&self) -> ReadySampleR {
        ReadySampleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event READY and task STOP"]
    #[inline(always)]
    pub fn ready_stop(&self) -> ReadyStopR {
        ReadyStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    pub fn down_stop(&self) -> DownStopR {
        DownStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event UP and task STOP"]
    #[inline(always)]
    pub fn up_stop(&self) -> UpStopR {
        UpStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    pub fn cross_stop(&self) -> CrossStopR {
        CrossStopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    pub fn ready_sample(&mut self) -> ReadySampleW<'_, ShortsSpec> {
        ReadySampleW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event READY and task STOP"]
    #[inline(always)]
    pub fn ready_stop(&mut self) -> ReadyStopW<'_, ShortsSpec> {
        ReadyStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    pub fn down_stop(&mut self) -> DownStopW<'_, ShortsSpec> {
        DownStopW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event UP and task STOP"]
    #[inline(always)]
    pub fn up_stop(&mut self) -> UpStopW<'_, ShortsSpec> {
        UpStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    pub fn cross_stop(&mut self) -> CrossStopW<'_, ShortsSpec> {
        CrossStopW::new(self, 4)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
