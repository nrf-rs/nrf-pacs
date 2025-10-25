#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event REPORTRDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyReadclracc {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReportrdyReadclracc> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyReadclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` reader - Shortcut between event REPORTRDY and task READCLRACC"]
pub type ReportrdyReadclraccR = crate::BitReader<ReportrdyReadclracc>;
impl ReportrdyReadclraccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReportrdyReadclracc {
        match self.bits {
            false => ReportrdyReadclracc::Disabled,
            true => ReportrdyReadclracc::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReportrdyReadclracc::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReportrdyReadclracc::Enabled
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` writer - Shortcut between event REPORTRDY and task READCLRACC"]
pub type ReportrdyReadclraccW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyReadclracc>;
impl<'a, REG> ReportrdyReadclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyReadclracc::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyReadclracc::Enabled)
    }
}
#[doc = "Shortcut between event SAMPLERDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplerdyStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<SamplerdyStop> for bool {
    #[inline(always)]
    fn from(variant: SamplerdyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY_STOP` reader - Shortcut between event SAMPLERDY and task STOP"]
pub type SamplerdyStopR = crate::BitReader<SamplerdyStop>;
impl SamplerdyStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SamplerdyStop {
        match self.bits {
            false => SamplerdyStop::Disabled,
            true => SamplerdyStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SamplerdyStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SamplerdyStop::Enabled
    }
}
#[doc = "Field `SAMPLERDY_STOP` writer - Shortcut between event SAMPLERDY and task STOP"]
pub type SamplerdyStopW<'a, REG> = crate::BitWriter<'a, REG, SamplerdyStop>;
impl<'a, REG> SamplerdyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyStop::Enabled)
    }
}
#[doc = "Shortcut between event REPORTRDY and task RDCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyRdclracc {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReportrdyRdclracc> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyRdclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` reader - Shortcut between event REPORTRDY and task RDCLRACC"]
pub type ReportrdyRdclraccR = crate::BitReader<ReportrdyRdclracc>;
impl ReportrdyRdclraccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReportrdyRdclracc {
        match self.bits {
            false => ReportrdyRdclracc::Disabled,
            true => ReportrdyRdclracc::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReportrdyRdclracc::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReportrdyRdclracc::Enabled
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` writer - Shortcut between event REPORTRDY and task RDCLRACC"]
pub type ReportrdyRdclraccW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyRdclracc>;
impl<'a, REG> ReportrdyRdclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyRdclracc::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyRdclracc::Enabled)
    }
}
#[doc = "Shortcut between event REPORTRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<ReportrdyStop> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_STOP` reader - Shortcut between event REPORTRDY and task STOP"]
pub type ReportrdyStopR = crate::BitReader<ReportrdyStop>;
impl ReportrdyStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReportrdyStop {
        match self.bits {
            false => ReportrdyStop::Disabled,
            true => ReportrdyStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReportrdyStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReportrdyStop::Enabled
    }
}
#[doc = "Field `REPORTRDY_STOP` writer - Shortcut between event REPORTRDY and task STOP"]
pub type ReportrdyStopW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyStop>;
impl<'a, REG> ReportrdyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyStop::Enabled)
    }
}
#[doc = "Shortcut between event DBLRDY and task RDCLRDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DblrdyRdclrdbl {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DblrdyRdclrdbl> for bool {
    #[inline(always)]
    fn from(variant: DblrdyRdclrdbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` reader - Shortcut between event DBLRDY and task RDCLRDBL"]
pub type DblrdyRdclrdblR = crate::BitReader<DblrdyRdclrdbl>;
impl DblrdyRdclrdblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DblrdyRdclrdbl {
        match self.bits {
            false => DblrdyRdclrdbl::Disabled,
            true => DblrdyRdclrdbl::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DblrdyRdclrdbl::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DblrdyRdclrdbl::Enabled
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` writer - Shortcut between event DBLRDY and task RDCLRDBL"]
pub type DblrdyRdclrdblW<'a, REG> = crate::BitWriter<'a, REG, DblrdyRdclrdbl>;
impl<'a, REG> DblrdyRdclrdblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DblrdyRdclrdbl::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DblrdyRdclrdbl::Enabled)
    }
}
#[doc = "Shortcut between event DBLRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DblrdyStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DblrdyStop> for bool {
    #[inline(always)]
    fn from(variant: DblrdyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY_STOP` reader - Shortcut between event DBLRDY and task STOP"]
pub type DblrdyStopR = crate::BitReader<DblrdyStop>;
impl DblrdyStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DblrdyStop {
        match self.bits {
            false => DblrdyStop::Disabled,
            true => DblrdyStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DblrdyStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DblrdyStop::Enabled
    }
}
#[doc = "Field `DBLRDY_STOP` writer - Shortcut between event DBLRDY and task STOP"]
pub type DblrdyStopW<'a, REG> = crate::BitWriter<'a, REG, DblrdyStop>;
impl<'a, REG> DblrdyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DblrdyStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DblrdyStop::Enabled)
    }
}
#[doc = "Shortcut between event SAMPLERDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplerdyReadclracc {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<SamplerdyReadclracc> for bool {
    #[inline(always)]
    fn from(variant: SamplerdyReadclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` reader - Shortcut between event SAMPLERDY and task READCLRACC"]
pub type SamplerdyReadclraccR = crate::BitReader<SamplerdyReadclracc>;
impl SamplerdyReadclraccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SamplerdyReadclracc {
        match self.bits {
            false => SamplerdyReadclracc::Disabled,
            true => SamplerdyReadclracc::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SamplerdyReadclracc::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SamplerdyReadclracc::Enabled
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` writer - Shortcut between event SAMPLERDY and task READCLRACC"]
pub type SamplerdyReadclraccW<'a, REG> = crate::BitWriter<'a, REG, SamplerdyReadclracc>;
impl<'a, REG> SamplerdyReadclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyReadclracc::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyReadclracc::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> ReportrdyReadclraccR {
        ReportrdyReadclraccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SamplerdyStopR {
        SamplerdyStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn reportrdy_rdclracc(&self) -> ReportrdyRdclraccR {
        ReportrdyRdclraccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn reportrdy_stop(&self) -> ReportrdyStopR {
        ReportrdyStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn dblrdy_rdclrdbl(&self) -> DblrdyRdclrdblR {
        DblrdyRdclrdblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn dblrdy_stop(&self) -> DblrdyStopR {
        DblrdyStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn samplerdy_readclracc(&self) -> SamplerdyReadclraccR {
        SamplerdyReadclraccR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn reportrdy_readclracc(&mut self) -> ReportrdyReadclraccW<'_, ShortsSpec> {
        ReportrdyReadclraccW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn samplerdy_stop(&mut self) -> SamplerdyStopW<'_, ShortsSpec> {
        SamplerdyStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn reportrdy_rdclracc(&mut self) -> ReportrdyRdclraccW<'_, ShortsSpec> {
        ReportrdyRdclraccW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn reportrdy_stop(&mut self) -> ReportrdyStopW<'_, ShortsSpec> {
        ReportrdyStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn dblrdy_rdclrdbl(&mut self) -> DblrdyRdclrdblW<'_, ShortsSpec> {
        DblrdyRdclrdblW::new(self, 4)
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn dblrdy_stop(&mut self) -> DblrdyStopW<'_, ShortsSpec> {
        DblrdyStopW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn samplerdy_readclracc(&mut self) -> SamplerdyReadclraccW<'_, ShortsSpec> {
        SamplerdyReadclraccW::new(self, 6)
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
