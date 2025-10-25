#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FielddetectedActivate {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<FielddetectedActivate> for bool {
    #[inline(always)]
    fn from(variant: FielddetectedActivate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` reader - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FielddetectedActivateR = crate::BitReader<FielddetectedActivate>;
impl FielddetectedActivateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FielddetectedActivate {
        match self.bits {
            false => FielddetectedActivate::Disabled,
            true => FielddetectedActivate::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FielddetectedActivate::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FielddetectedActivate::Enabled
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` writer - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FielddetectedActivateW<'a, REG> = crate::BitWriter<'a, REG, FielddetectedActivate>;
impl<'a, REG> FielddetectedActivateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FielddetectedActivate::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FielddetectedActivate::Enabled)
    }
}
#[doc = "Shortcut between event FIELDLOST and task SENSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldlostSense {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<FieldlostSense> for bool {
    #[inline(always)]
    fn from(variant: FieldlostSense) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST_SENSE` reader - Shortcut between event FIELDLOST and task SENSE"]
pub type FieldlostSenseR = crate::BitReader<FieldlostSense>;
impl FieldlostSenseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FieldlostSense {
        match self.bits {
            false => FieldlostSense::Disabled,
            true => FieldlostSense::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FieldlostSense::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FieldlostSense::Enabled
    }
}
#[doc = "Field `FIELDLOST_SENSE` writer - Shortcut between event FIELDLOST and task SENSE"]
pub type FieldlostSenseW<'a, REG> = crate::BitWriter<'a, REG, FieldlostSense>;
impl<'a, REG> FieldlostSenseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FieldlostSense::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FieldlostSense::Enabled)
    }
}
#[doc = "Shortcut between event TXFRAMEEND and task ENABLERXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxframeendEnablerxdata {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<TxframeendEnablerxdata> for bool {
    #[inline(always)]
    fn from(variant: TxframeendEnablerxdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` reader - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub type TxframeendEnablerxdataR = crate::BitReader<TxframeendEnablerxdata>;
impl TxframeendEnablerxdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxframeendEnablerxdata {
        match self.bits {
            false => TxframeendEnablerxdata::Disabled,
            true => TxframeendEnablerxdata::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TxframeendEnablerxdata::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TxframeendEnablerxdata::Enabled
    }
}
#[doc = "Field `TXFRAMEEND_ENABLERXDATA` writer - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
pub type TxframeendEnablerxdataW<'a, REG> = crate::BitWriter<'a, REG, TxframeendEnablerxdata>;
impl<'a, REG> TxframeendEnablerxdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TxframeendEnablerxdata::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TxframeendEnablerxdata::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&self) -> FielddetectedActivateR {
        FielddetectedActivateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&self) -> FieldlostSenseR {
        FieldlostSenseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&self) -> TxframeendEnablerxdataR {
        TxframeendEnablerxdataR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&mut self) -> FielddetectedActivateW<'_, ShortsSpec> {
        FielddetectedActivateW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&mut self) -> FieldlostSenseW<'_, ShortsSpec> {
        FieldlostSenseW::new(self, 1)
    }
    #[doc = "Bit 5 - Shortcut between event TXFRAMEEND and task ENABLERXDATA"]
    #[inline(always)]
    pub fn txframeend_enablerxdata(&mut self) -> TxframeendEnablerxdataW<'_, ShortsSpec> {
        TxframeendEnablerxdataW::new(self, 5)
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
