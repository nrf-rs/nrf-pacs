#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between READY event and SAMPLE task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadySample {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<ReadySample> for bool {
    #[inline(always)]
    fn from(variant: ReadySample) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_SAMPLE` reader - Shortcut between READY event and SAMPLE task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadySample::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadySample::Enabled
    }
}
#[doc = "Field `READY_SAMPLE` writer - Shortcut between READY event and SAMPLE task."]
pub type ReadySampleW<'a, REG> = crate::BitWriter<'a, REG, ReadySample>;
impl<'a, REG> ReadySampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadySample::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadySample::Enabled)
    }
}
#[doc = "Shortcut between RADY event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<ReadyStop> for bool {
    #[inline(always)]
    fn from(variant: ReadyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY_STOP` reader - Shortcut between RADY event and STOP task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReadyStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReadyStop::Enabled
    }
}
#[doc = "Field `READY_STOP` writer - Shortcut between RADY event and STOP task."]
pub type ReadyStopW<'a, REG> = crate::BitWriter<'a, REG, ReadyStop>;
impl<'a, REG> ReadyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReadyStop::Enabled)
    }
}
#[doc = "Shortcut between DOWN event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<DownStop> for bool {
    #[inline(always)]
    fn from(variant: DownStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWN_STOP` reader - Shortcut between DOWN event and STOP task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DownStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DownStop::Enabled
    }
}
#[doc = "Field `DOWN_STOP` writer - Shortcut between DOWN event and STOP task."]
pub type DownStopW<'a, REG> = crate::BitWriter<'a, REG, DownStop>;
impl<'a, REG> DownStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DownStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DownStop::Enabled)
    }
}
#[doc = "Shortcut between UP event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<UpStop> for bool {
    #[inline(always)]
    fn from(variant: UpStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UP_STOP` reader - Shortcut between UP event and STOP task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UpStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UpStop::Enabled
    }
}
#[doc = "Field `UP_STOP` writer - Shortcut between UP event and STOP task."]
pub type UpStopW<'a, REG> = crate::BitWriter<'a, REG, UpStop>;
impl<'a, REG> UpStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UpStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UpStop::Enabled)
    }
}
#[doc = "Shortcut between CROSS event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<CrossStop> for bool {
    #[inline(always)]
    fn from(variant: CrossStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROSS_STOP` reader - Shortcut between CROSS event and STOP task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CrossStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CrossStop::Enabled
    }
}
#[doc = "Field `CROSS_STOP` writer - Shortcut between CROSS event and STOP task."]
pub type CrossStopW<'a, REG> = crate::BitWriter<'a, REG, CrossStop>;
impl<'a, REG> CrossStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrossStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CrossStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task."]
    #[inline(always)]
    pub fn ready_sample(&self) -> ReadySampleR {
        ReadySampleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between RADY event and STOP task."]
    #[inline(always)]
    pub fn ready_stop(&self) -> ReadyStopR {
        ReadyStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task."]
    #[inline(always)]
    pub fn down_stop(&self) -> DownStopR {
        DownStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task."]
    #[inline(always)]
    pub fn up_stop(&self) -> UpStopR {
        UpStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task."]
    #[inline(always)]
    pub fn cross_stop(&self) -> CrossStopR {
        CrossStopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task."]
    #[inline(always)]
    pub fn ready_sample(&mut self) -> ReadySampleW<'_, ShortsSpec> {
        ReadySampleW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between RADY event and STOP task."]
    #[inline(always)]
    pub fn ready_stop(&mut self) -> ReadyStopW<'_, ShortsSpec> {
        ReadyStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task."]
    #[inline(always)]
    pub fn down_stop(&mut self) -> DownStopW<'_, ShortsSpec> {
        DownStopW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task."]
    #[inline(always)]
    pub fn up_stop(&mut self) -> UpStopW<'_, ShortsSpec> {
        UpStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task."]
    #[inline(always)]
    pub fn cross_stop(&mut self) -> CrossStopW<'_, ShortsSpec> {
        CrossStopW::new(self, 4)
    }
}
#[doc = "Shortcuts for the LPCOMP.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
