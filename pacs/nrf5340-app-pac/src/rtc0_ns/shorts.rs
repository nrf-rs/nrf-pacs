#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event COMPARE\\[0\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare0Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare0Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0_CLEAR` reader - Shortcut between event COMPARE\\[0\\] and task CLEAR"]
pub type Compare0ClearR = crate::BitReader<Compare0Clear>;
impl Compare0ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0Clear {
        match self.bits {
            false => Compare0Clear::Disabled,
            true => Compare0Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0Clear::Enabled
    }
}
#[doc = "Field `COMPARE0_CLEAR` writer - Shortcut between event COMPARE\\[0\\] and task CLEAR"]
pub type Compare0ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare0Clear>;
impl<'a, REG> Compare0ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[1\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare1Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare1Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1_CLEAR` reader - Shortcut between event COMPARE\\[1\\] and task CLEAR"]
pub type Compare1ClearR = crate::BitReader<Compare1Clear>;
impl Compare1ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1Clear {
        match self.bits {
            false => Compare1Clear::Disabled,
            true => Compare1Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1Clear::Enabled
    }
}
#[doc = "Field `COMPARE1_CLEAR` writer - Shortcut between event COMPARE\\[1\\] and task CLEAR"]
pub type Compare1ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare1Clear>;
impl<'a, REG> Compare1ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[2\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare2Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare2Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2_CLEAR` reader - Shortcut between event COMPARE\\[2\\] and task CLEAR"]
pub type Compare2ClearR = crate::BitReader<Compare2Clear>;
impl Compare2ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2Clear {
        match self.bits {
            false => Compare2Clear::Disabled,
            true => Compare2Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2Clear::Enabled
    }
}
#[doc = "Field `COMPARE2_CLEAR` writer - Shortcut between event COMPARE\\[2\\] and task CLEAR"]
pub type Compare2ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare2Clear>;
impl<'a, REG> Compare2ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[3\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare3Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare3Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3_CLEAR` reader - Shortcut between event COMPARE\\[3\\] and task CLEAR"]
pub type Compare3ClearR = crate::BitReader<Compare3Clear>;
impl Compare3ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3Clear {
        match self.bits {
            false => Compare3Clear::Disabled,
            true => Compare3Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3Clear::Enabled
    }
}
#[doc = "Field `COMPARE3_CLEAR` writer - Shortcut between event COMPARE\\[3\\] and task CLEAR"]
pub type Compare3ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare3Clear>;
impl<'a, REG> Compare3ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3Clear::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare0_clear(&self) -> Compare0ClearR {
        Compare0ClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare1_clear(&self) -> Compare1ClearR {
        Compare1ClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare2_clear(&self) -> Compare2ClearR {
        Compare2ClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare3_clear(&self) -> Compare3ClearR {
        Compare3ClearR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare0_clear(&mut self) -> Compare0ClearW<'_, ShortsSpec> {
        Compare0ClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare1_clear(&mut self) -> Compare1ClearW<'_, ShortsSpec> {
        Compare1ClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare2_clear(&mut self) -> Compare2ClearW<'_, ShortsSpec> {
        Compare2ClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare3_clear(&mut self) -> Compare3ClearW<'_, ShortsSpec> {
        Compare3ClearW::new(self, 3)
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
