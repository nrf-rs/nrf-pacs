#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between REPORTRDY event and READCLRACC task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyReadclracc {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<ReportrdyReadclracc> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyReadclracc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` reader - Shortcut between REPORTRDY event and READCLRACC task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ReportrdyReadclracc::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ReportrdyReadclracc::Enabled
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` writer - Shortcut between REPORTRDY event and READCLRACC task."]
pub type ReportrdyReadclraccW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyReadclracc>;
impl<'a, REG> ReportrdyReadclraccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyReadclracc::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyReadclracc::Enabled)
    }
}
#[doc = "Shortcut between SAMPLERDY event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplerdyStop {
    #[doc = "0: Shortcut disabled."]
    Disabled = 0,
    #[doc = "1: Shortcut enabled."]
    Enabled = 1,
}
impl From<SamplerdyStop> for bool {
    #[inline(always)]
    fn from(variant: SamplerdyStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY_STOP` reader - Shortcut between SAMPLERDY event and STOP task."]
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
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SamplerdyStop::Disabled
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SamplerdyStop::Enabled
    }
}
#[doc = "Field `SAMPLERDY_STOP` writer - Shortcut between SAMPLERDY event and STOP task."]
pub type SamplerdyStopW<'a, REG> = crate::BitWriter<'a, REG, SamplerdyStop>;
impl<'a, REG> SamplerdyStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyStop::Disabled)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> ReportrdyReadclraccR {
        ReportrdyReadclraccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SamplerdyStopR {
        SamplerdyStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&mut self) -> ReportrdyReadclraccW<'_, ShortsSpec> {
        ReportrdyReadclraccW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&mut self) -> SamplerdyStopW<'_, ShortsSpec> {
        SamplerdyStopW::new(self, 1)
    }
}
#[doc = "Shortcuts for the QDEC.\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
