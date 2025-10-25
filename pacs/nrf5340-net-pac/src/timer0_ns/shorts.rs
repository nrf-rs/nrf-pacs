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
#[doc = "Shortcut between event COMPARE\\[4\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare4Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare4Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4_CLEAR` reader - Shortcut between event COMPARE\\[4\\] and task CLEAR"]
pub type Compare4ClearR = crate::BitReader<Compare4Clear>;
impl Compare4ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare4Clear {
        match self.bits {
            false => Compare4Clear::Disabled,
            true => Compare4Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare4Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare4Clear::Enabled
    }
}
#[doc = "Field `COMPARE4_CLEAR` writer - Shortcut between event COMPARE\\[4\\] and task CLEAR"]
pub type Compare4ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare4Clear>;
impl<'a, REG> Compare4ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[5\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare5Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare5Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5_CLEAR` reader - Shortcut between event COMPARE\\[5\\] and task CLEAR"]
pub type Compare5ClearR = crate::BitReader<Compare5Clear>;
impl Compare5ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare5Clear {
        match self.bits {
            false => Compare5Clear::Disabled,
            true => Compare5Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare5Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare5Clear::Enabled
    }
}
#[doc = "Field `COMPARE5_CLEAR` writer - Shortcut between event COMPARE\\[5\\] and task CLEAR"]
pub type Compare5ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare5Clear>;
impl<'a, REG> Compare5ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[6\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare6Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare6Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6_CLEAR` reader - Shortcut between event COMPARE\\[6\\] and task CLEAR"]
pub type Compare6ClearR = crate::BitReader<Compare6Clear>;
impl Compare6ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare6Clear {
        match self.bits {
            false => Compare6Clear::Disabled,
            true => Compare6Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare6Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare6Clear::Enabled
    }
}
#[doc = "Field `COMPARE6_CLEAR` writer - Shortcut between event COMPARE\\[6\\] and task CLEAR"]
pub type Compare6ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare6Clear>;
impl<'a, REG> Compare6ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[7\\] and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7Clear {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare7Clear> for bool {
    #[inline(always)]
    fn from(variant: Compare7Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7_CLEAR` reader - Shortcut between event COMPARE\\[7\\] and task CLEAR"]
pub type Compare7ClearR = crate::BitReader<Compare7Clear>;
impl Compare7ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare7Clear {
        match self.bits {
            false => Compare7Clear::Disabled,
            true => Compare7Clear::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare7Clear::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare7Clear::Enabled
    }
}
#[doc = "Field `COMPARE7_CLEAR` writer - Shortcut between event COMPARE\\[7\\] and task CLEAR"]
pub type Compare7ClearW<'a, REG> = crate::BitWriter<'a, REG, Compare7Clear>;
impl<'a, REG> Compare7ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7Clear::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7Clear::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[0\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare0Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare0Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0_STOP` reader - Shortcut between event COMPARE\\[0\\] and task STOP"]
pub type Compare0StopR = crate::BitReader<Compare0Stop>;
impl Compare0StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0Stop {
        match self.bits {
            false => Compare0Stop::Disabled,
            true => Compare0Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0Stop::Enabled
    }
}
#[doc = "Field `COMPARE0_STOP` writer - Shortcut between event COMPARE\\[0\\] and task STOP"]
pub type Compare0StopW<'a, REG> = crate::BitWriter<'a, REG, Compare0Stop>;
impl<'a, REG> Compare0StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[1\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare1Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1_STOP` reader - Shortcut between event COMPARE\\[1\\] and task STOP"]
pub type Compare1StopR = crate::BitReader<Compare1Stop>;
impl Compare1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1Stop {
        match self.bits {
            false => Compare1Stop::Disabled,
            true => Compare1Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1Stop::Enabled
    }
}
#[doc = "Field `COMPARE1_STOP` writer - Shortcut between event COMPARE\\[1\\] and task STOP"]
pub type Compare1StopW<'a, REG> = crate::BitWriter<'a, REG, Compare1Stop>;
impl<'a, REG> Compare1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[2\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare2Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare2Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2_STOP` reader - Shortcut between event COMPARE\\[2\\] and task STOP"]
pub type Compare2StopR = crate::BitReader<Compare2Stop>;
impl Compare2StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2Stop {
        match self.bits {
            false => Compare2Stop::Disabled,
            true => Compare2Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2Stop::Enabled
    }
}
#[doc = "Field `COMPARE2_STOP` writer - Shortcut between event COMPARE\\[2\\] and task STOP"]
pub type Compare2StopW<'a, REG> = crate::BitWriter<'a, REG, Compare2Stop>;
impl<'a, REG> Compare2StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[3\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare3Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare3Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3_STOP` reader - Shortcut between event COMPARE\\[3\\] and task STOP"]
pub type Compare3StopR = crate::BitReader<Compare3Stop>;
impl Compare3StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3Stop {
        match self.bits {
            false => Compare3Stop::Disabled,
            true => Compare3Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3Stop::Enabled
    }
}
#[doc = "Field `COMPARE3_STOP` writer - Shortcut between event COMPARE\\[3\\] and task STOP"]
pub type Compare3StopW<'a, REG> = crate::BitWriter<'a, REG, Compare3Stop>;
impl<'a, REG> Compare3StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[4\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare4Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare4Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare4Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4_STOP` reader - Shortcut between event COMPARE\\[4\\] and task STOP"]
pub type Compare4StopR = crate::BitReader<Compare4Stop>;
impl Compare4StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare4Stop {
        match self.bits {
            false => Compare4Stop::Disabled,
            true => Compare4Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare4Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare4Stop::Enabled
    }
}
#[doc = "Field `COMPARE4_STOP` writer - Shortcut between event COMPARE\\[4\\] and task STOP"]
pub type Compare4StopW<'a, REG> = crate::BitWriter<'a, REG, Compare4Stop>;
impl<'a, REG> Compare4StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare4Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[5\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare5Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare5Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare5Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5_STOP` reader - Shortcut between event COMPARE\\[5\\] and task STOP"]
pub type Compare5StopR = crate::BitReader<Compare5Stop>;
impl Compare5StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare5Stop {
        match self.bits {
            false => Compare5Stop::Disabled,
            true => Compare5Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare5Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare5Stop::Enabled
    }
}
#[doc = "Field `COMPARE5_STOP` writer - Shortcut between event COMPARE\\[5\\] and task STOP"]
pub type Compare5StopW<'a, REG> = crate::BitWriter<'a, REG, Compare5Stop>;
impl<'a, REG> Compare5StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare5Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[6\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare6Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare6Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare6Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE6_STOP` reader - Shortcut between event COMPARE\\[6\\] and task STOP"]
pub type Compare6StopR = crate::BitReader<Compare6Stop>;
impl Compare6StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare6Stop {
        match self.bits {
            false => Compare6Stop::Disabled,
            true => Compare6Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare6Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare6Stop::Enabled
    }
}
#[doc = "Field `COMPARE6_STOP` writer - Shortcut between event COMPARE\\[6\\] and task STOP"]
pub type Compare6StopW<'a, REG> = crate::BitWriter<'a, REG, Compare6Stop>;
impl<'a, REG> Compare6StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare6Stop::Enabled)
    }
}
#[doc = "Shortcut between event COMPARE\\[7\\] and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare7Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Compare7Stop> for bool {
    #[inline(always)]
    fn from(variant: Compare7Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE7_STOP` reader - Shortcut between event COMPARE\\[7\\] and task STOP"]
pub type Compare7StopR = crate::BitReader<Compare7Stop>;
impl Compare7StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare7Stop {
        match self.bits {
            false => Compare7Stop::Disabled,
            true => Compare7Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare7Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare7Stop::Enabled
    }
}
#[doc = "Field `COMPARE7_STOP` writer - Shortcut between event COMPARE\\[7\\] and task STOP"]
pub type Compare7StopW<'a, REG> = crate::BitWriter<'a, REG, Compare7Stop>;
impl<'a, REG> Compare7StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Compare7Stop::Enabled)
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
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare4_clear(&self) -> Compare4ClearR {
        Compare4ClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare5_clear(&self) -> Compare5ClearR {
        Compare5ClearR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event COMPARE\\[6\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare6_clear(&self) -> Compare6ClearR {
        Compare6ClearR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shortcut between event COMPARE\\[7\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare7_clear(&self) -> Compare7ClearR {
        Compare7ClearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Shortcut between event COMPARE\\[0\\] and task STOP"]
    #[inline(always)]
    pub fn compare0_stop(&self) -> Compare0StopR {
        Compare0StopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shortcut between event COMPARE\\[1\\] and task STOP"]
    #[inline(always)]
    pub fn compare1_stop(&self) -> Compare1StopR {
        Compare1StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shortcut between event COMPARE\\[2\\] and task STOP"]
    #[inline(always)]
    pub fn compare2_stop(&self) -> Compare2StopR {
        Compare2StopR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shortcut between event COMPARE\\[3\\] and task STOP"]
    #[inline(always)]
    pub fn compare3_stop(&self) -> Compare3StopR {
        Compare3StopR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shortcut between event COMPARE\\[4\\] and task STOP"]
    #[inline(always)]
    pub fn compare4_stop(&self) -> Compare4StopR {
        Compare4StopR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shortcut between event COMPARE\\[5\\] and task STOP"]
    #[inline(always)]
    pub fn compare5_stop(&self) -> Compare5StopR {
        Compare5StopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shortcut between event COMPARE\\[6\\] and task STOP"]
    #[inline(always)]
    pub fn compare6_stop(&self) -> Compare6StopR {
        Compare6StopR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shortcut between event COMPARE\\[7\\] and task STOP"]
    #[inline(always)]
    pub fn compare7_stop(&self) -> Compare7StopR {
        Compare7StopR::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare4_clear(&mut self) -> Compare4ClearW<'_, ShortsSpec> {
        Compare4ClearW::new(self, 4)
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare5_clear(&mut self) -> Compare5ClearW<'_, ShortsSpec> {
        Compare5ClearW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event COMPARE\\[6\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare6_clear(&mut self) -> Compare6ClearW<'_, ShortsSpec> {
        Compare6ClearW::new(self, 6)
    }
    #[doc = "Bit 7 - Shortcut between event COMPARE\\[7\\] and task CLEAR"]
    #[inline(always)]
    pub fn compare7_clear(&mut self) -> Compare7ClearW<'_, ShortsSpec> {
        Compare7ClearW::new(self, 7)
    }
    #[doc = "Bit 16 - Shortcut between event COMPARE\\[0\\] and task STOP"]
    #[inline(always)]
    pub fn compare0_stop(&mut self) -> Compare0StopW<'_, ShortsSpec> {
        Compare0StopW::new(self, 16)
    }
    #[doc = "Bit 17 - Shortcut between event COMPARE\\[1\\] and task STOP"]
    #[inline(always)]
    pub fn compare1_stop(&mut self) -> Compare1StopW<'_, ShortsSpec> {
        Compare1StopW::new(self, 17)
    }
    #[doc = "Bit 18 - Shortcut between event COMPARE\\[2\\] and task STOP"]
    #[inline(always)]
    pub fn compare2_stop(&mut self) -> Compare2StopW<'_, ShortsSpec> {
        Compare2StopW::new(self, 18)
    }
    #[doc = "Bit 19 - Shortcut between event COMPARE\\[3\\] and task STOP"]
    #[inline(always)]
    pub fn compare3_stop(&mut self) -> Compare3StopW<'_, ShortsSpec> {
        Compare3StopW::new(self, 19)
    }
    #[doc = "Bit 20 - Shortcut between event COMPARE\\[4\\] and task STOP"]
    #[inline(always)]
    pub fn compare4_stop(&mut self) -> Compare4StopW<'_, ShortsSpec> {
        Compare4StopW::new(self, 20)
    }
    #[doc = "Bit 21 - Shortcut between event COMPARE\\[5\\] and task STOP"]
    #[inline(always)]
    pub fn compare5_stop(&mut self) -> Compare5StopW<'_, ShortsSpec> {
        Compare5StopW::new(self, 21)
    }
    #[doc = "Bit 22 - Shortcut between event COMPARE\\[6\\] and task STOP"]
    #[inline(always)]
    pub fn compare6_stop(&mut self) -> Compare6StopW<'_, ShortsSpec> {
        Compare6StopW::new(self, 22)
    }
    #[doc = "Bit 23 - Shortcut between event COMPARE\\[7\\] and task STOP"]
    #[inline(always)]
    pub fn compare7_stop(&mut self) -> Compare7StopW<'_, ShortsSpec> {
        Compare7StopW::new(self, 23)
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
