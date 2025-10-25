#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<Config1Spec>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<Config1Spec>;
#[doc = "Enable protection for region 32. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region32 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region32> for bool {
    #[inline(always)]
    fn from(variant: Region32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION32` reader - Enable protection for region 32. Write '0' has no effect."]
pub type Region32R = crate::BitReader<Region32>;
impl Region32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region32 {
        match self.bits {
            false => Region32::Disabled,
            true => Region32::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region32::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region32::Enabled
    }
}
#[doc = "Field `REGION32` writer - Enable protection for region 32. Write '0' has no effect."]
pub type Region32W<'a, REG> = crate::BitWriter<'a, REG, Region32>;
impl<'a, REG> Region32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region32::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region32::Enabled)
    }
}
#[doc = "Enable protection for region 33. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region33 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region33> for bool {
    #[inline(always)]
    fn from(variant: Region33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION33` reader - Enable protection for region 33. Write '0' has no effect."]
pub type Region33R = crate::BitReader<Region33>;
impl Region33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region33 {
        match self.bits {
            false => Region33::Disabled,
            true => Region33::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region33::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region33::Enabled
    }
}
#[doc = "Field `REGION33` writer - Enable protection for region 33. Write '0' has no effect."]
pub type Region33W<'a, REG> = crate::BitWriter<'a, REG, Region33>;
impl<'a, REG> Region33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region33::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region33::Enabled)
    }
}
#[doc = "Enable protection for region 34. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region34 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region34> for bool {
    #[inline(always)]
    fn from(variant: Region34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION34` reader - Enable protection for region 34. Write '0' has no effect."]
pub type Region34R = crate::BitReader<Region34>;
impl Region34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region34 {
        match self.bits {
            false => Region34::Disabled,
            true => Region34::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region34::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region34::Enabled
    }
}
#[doc = "Field `REGION34` writer - Enable protection for region 34. Write '0' has no effect."]
pub type Region34W<'a, REG> = crate::BitWriter<'a, REG, Region34>;
impl<'a, REG> Region34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region34::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region34::Enabled)
    }
}
#[doc = "Enable protection for region 35. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region35 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region35> for bool {
    #[inline(always)]
    fn from(variant: Region35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION35` reader - Enable protection for region 35. Write '0' has no effect."]
pub type Region35R = crate::BitReader<Region35>;
impl Region35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region35 {
        match self.bits {
            false => Region35::Disabled,
            true => Region35::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region35::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region35::Enabled
    }
}
#[doc = "Field `REGION35` writer - Enable protection for region 35. Write '0' has no effect."]
pub type Region35W<'a, REG> = crate::BitWriter<'a, REG, Region35>;
impl<'a, REG> Region35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region35::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region35::Enabled)
    }
}
#[doc = "Enable protection for region 36. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region36 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region36> for bool {
    #[inline(always)]
    fn from(variant: Region36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION36` reader - Enable protection for region 36. Write '0' has no effect."]
pub type Region36R = crate::BitReader<Region36>;
impl Region36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region36 {
        match self.bits {
            false => Region36::Disabled,
            true => Region36::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region36::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region36::Enabled
    }
}
#[doc = "Field `REGION36` writer - Enable protection for region 36. Write '0' has no effect."]
pub type Region36W<'a, REG> = crate::BitWriter<'a, REG, Region36>;
impl<'a, REG> Region36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region36::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region36::Enabled)
    }
}
#[doc = "Enable protection for region 37. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region37 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region37> for bool {
    #[inline(always)]
    fn from(variant: Region37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION37` reader - Enable protection for region 37. Write '0' has no effect."]
pub type Region37R = crate::BitReader<Region37>;
impl Region37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region37 {
        match self.bits {
            false => Region37::Disabled,
            true => Region37::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region37::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region37::Enabled
    }
}
#[doc = "Field `REGION37` writer - Enable protection for region 37. Write '0' has no effect."]
pub type Region37W<'a, REG> = crate::BitWriter<'a, REG, Region37>;
impl<'a, REG> Region37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region37::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region37::Enabled)
    }
}
#[doc = "Enable protection for region 38. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region38 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region38> for bool {
    #[inline(always)]
    fn from(variant: Region38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION38` reader - Enable protection for region 38. Write '0' has no effect."]
pub type Region38R = crate::BitReader<Region38>;
impl Region38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region38 {
        match self.bits {
            false => Region38::Disabled,
            true => Region38::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region38::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region38::Enabled
    }
}
#[doc = "Field `REGION38` writer - Enable protection for region 38. Write '0' has no effect."]
pub type Region38W<'a, REG> = crate::BitWriter<'a, REG, Region38>;
impl<'a, REG> Region38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region38::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region38::Enabled)
    }
}
#[doc = "Enable protection for region 39. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region39 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region39> for bool {
    #[inline(always)]
    fn from(variant: Region39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION39` reader - Enable protection for region 39. Write '0' has no effect."]
pub type Region39R = crate::BitReader<Region39>;
impl Region39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region39 {
        match self.bits {
            false => Region39::Disabled,
            true => Region39::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region39::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region39::Enabled
    }
}
#[doc = "Field `REGION39` writer - Enable protection for region 39. Write '0' has no effect."]
pub type Region39W<'a, REG> = crate::BitWriter<'a, REG, Region39>;
impl<'a, REG> Region39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region39::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region39::Enabled)
    }
}
#[doc = "Enable protection for region 40. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region40 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region40> for bool {
    #[inline(always)]
    fn from(variant: Region40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION40` reader - Enable protection for region 40. Write '0' has no effect."]
pub type Region40R = crate::BitReader<Region40>;
impl Region40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region40 {
        match self.bits {
            false => Region40::Disabled,
            true => Region40::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region40::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region40::Enabled
    }
}
#[doc = "Field `REGION40` writer - Enable protection for region 40. Write '0' has no effect."]
pub type Region40W<'a, REG> = crate::BitWriter<'a, REG, Region40>;
impl<'a, REG> Region40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region40::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region40::Enabled)
    }
}
#[doc = "Enable protection for region 41. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region41 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region41> for bool {
    #[inline(always)]
    fn from(variant: Region41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION41` reader - Enable protection for region 41. Write '0' has no effect."]
pub type Region41R = crate::BitReader<Region41>;
impl Region41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region41 {
        match self.bits {
            false => Region41::Disabled,
            true => Region41::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region41::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region41::Enabled
    }
}
#[doc = "Field `REGION41` writer - Enable protection for region 41. Write '0' has no effect."]
pub type Region41W<'a, REG> = crate::BitWriter<'a, REG, Region41>;
impl<'a, REG> Region41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region41::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region41::Enabled)
    }
}
#[doc = "Enable protection for region 42. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region42 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region42> for bool {
    #[inline(always)]
    fn from(variant: Region42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION42` reader - Enable protection for region 42. Write '0' has no effect."]
pub type Region42R = crate::BitReader<Region42>;
impl Region42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region42 {
        match self.bits {
            false => Region42::Disabled,
            true => Region42::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region42::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region42::Enabled
    }
}
#[doc = "Field `REGION42` writer - Enable protection for region 42. Write '0' has no effect."]
pub type Region42W<'a, REG> = crate::BitWriter<'a, REG, Region42>;
impl<'a, REG> Region42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region42::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region42::Enabled)
    }
}
#[doc = "Enable protection for region 43. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region43 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region43> for bool {
    #[inline(always)]
    fn from(variant: Region43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION43` reader - Enable protection for region 43. Write '0' has no effect."]
pub type Region43R = crate::BitReader<Region43>;
impl Region43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region43 {
        match self.bits {
            false => Region43::Disabled,
            true => Region43::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region43::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region43::Enabled
    }
}
#[doc = "Field `REGION43` writer - Enable protection for region 43. Write '0' has no effect."]
pub type Region43W<'a, REG> = crate::BitWriter<'a, REG, Region43>;
impl<'a, REG> Region43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region43::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region43::Enabled)
    }
}
#[doc = "Enable protection for region 44. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region44 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region44> for bool {
    #[inline(always)]
    fn from(variant: Region44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION44` reader - Enable protection for region 44. Write '0' has no effect."]
pub type Region44R = crate::BitReader<Region44>;
impl Region44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region44 {
        match self.bits {
            false => Region44::Disabled,
            true => Region44::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region44::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region44::Enabled
    }
}
#[doc = "Field `REGION44` writer - Enable protection for region 44. Write '0' has no effect."]
pub type Region44W<'a, REG> = crate::BitWriter<'a, REG, Region44>;
impl<'a, REG> Region44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region44::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region44::Enabled)
    }
}
#[doc = "Enable protection for region 45. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region45 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region45> for bool {
    #[inline(always)]
    fn from(variant: Region45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION45` reader - Enable protection for region 45. Write '0' has no effect."]
pub type Region45R = crate::BitReader<Region45>;
impl Region45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region45 {
        match self.bits {
            false => Region45::Disabled,
            true => Region45::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region45::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region45::Enabled
    }
}
#[doc = "Field `REGION45` writer - Enable protection for region 45. Write '0' has no effect."]
pub type Region45W<'a, REG> = crate::BitWriter<'a, REG, Region45>;
impl<'a, REG> Region45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region45::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region45::Enabled)
    }
}
#[doc = "Enable protection for region 46. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region46 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region46> for bool {
    #[inline(always)]
    fn from(variant: Region46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION46` reader - Enable protection for region 46. Write '0' has no effect."]
pub type Region46R = crate::BitReader<Region46>;
impl Region46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region46 {
        match self.bits {
            false => Region46::Disabled,
            true => Region46::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region46::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region46::Enabled
    }
}
#[doc = "Field `REGION46` writer - Enable protection for region 46. Write '0' has no effect."]
pub type Region46W<'a, REG> = crate::BitWriter<'a, REG, Region46>;
impl<'a, REG> Region46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region46::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region46::Enabled)
    }
}
#[doc = "Enable protection for region 47. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region47 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region47> for bool {
    #[inline(always)]
    fn from(variant: Region47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION47` reader - Enable protection for region 47. Write '0' has no effect."]
pub type Region47R = crate::BitReader<Region47>;
impl Region47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region47 {
        match self.bits {
            false => Region47::Disabled,
            true => Region47::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region47::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region47::Enabled
    }
}
#[doc = "Field `REGION47` writer - Enable protection for region 47. Write '0' has no effect."]
pub type Region47W<'a, REG> = crate::BitWriter<'a, REG, Region47>;
impl<'a, REG> Region47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region47::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region47::Enabled)
    }
}
#[doc = "Enable protection for region 48. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region48 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region48> for bool {
    #[inline(always)]
    fn from(variant: Region48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION48` reader - Enable protection for region 48. Write '0' has no effect."]
pub type Region48R = crate::BitReader<Region48>;
impl Region48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region48 {
        match self.bits {
            false => Region48::Disabled,
            true => Region48::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region48::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region48::Enabled
    }
}
#[doc = "Field `REGION48` writer - Enable protection for region 48. Write '0' has no effect."]
pub type Region48W<'a, REG> = crate::BitWriter<'a, REG, Region48>;
impl<'a, REG> Region48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region48::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region48::Enabled)
    }
}
#[doc = "Enable protection for region 49. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region49 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region49> for bool {
    #[inline(always)]
    fn from(variant: Region49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION49` reader - Enable protection for region 49. Write '0' has no effect."]
pub type Region49R = crate::BitReader<Region49>;
impl Region49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region49 {
        match self.bits {
            false => Region49::Disabled,
            true => Region49::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region49::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region49::Enabled
    }
}
#[doc = "Field `REGION49` writer - Enable protection for region 49. Write '0' has no effect."]
pub type Region49W<'a, REG> = crate::BitWriter<'a, REG, Region49>;
impl<'a, REG> Region49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region49::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region49::Enabled)
    }
}
#[doc = "Enable protection for region 50. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region50 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region50> for bool {
    #[inline(always)]
    fn from(variant: Region50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION50` reader - Enable protection for region 50. Write '0' has no effect."]
pub type Region50R = crate::BitReader<Region50>;
impl Region50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region50 {
        match self.bits {
            false => Region50::Disabled,
            true => Region50::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region50::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region50::Enabled
    }
}
#[doc = "Field `REGION50` writer - Enable protection for region 50. Write '0' has no effect."]
pub type Region50W<'a, REG> = crate::BitWriter<'a, REG, Region50>;
impl<'a, REG> Region50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region50::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region50::Enabled)
    }
}
#[doc = "Enable protection for region 51. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region51 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region51> for bool {
    #[inline(always)]
    fn from(variant: Region51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION51` reader - Enable protection for region 51. Write '0' has no effect."]
pub type Region51R = crate::BitReader<Region51>;
impl Region51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region51 {
        match self.bits {
            false => Region51::Disabled,
            true => Region51::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region51::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region51::Enabled
    }
}
#[doc = "Field `REGION51` writer - Enable protection for region 51. Write '0' has no effect."]
pub type Region51W<'a, REG> = crate::BitWriter<'a, REG, Region51>;
impl<'a, REG> Region51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region51::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region51::Enabled)
    }
}
#[doc = "Enable protection for region 52. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region52 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region52> for bool {
    #[inline(always)]
    fn from(variant: Region52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION52` reader - Enable protection for region 52. Write '0' has no effect."]
pub type Region52R = crate::BitReader<Region52>;
impl Region52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region52 {
        match self.bits {
            false => Region52::Disabled,
            true => Region52::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region52::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region52::Enabled
    }
}
#[doc = "Field `REGION52` writer - Enable protection for region 52. Write '0' has no effect."]
pub type Region52W<'a, REG> = crate::BitWriter<'a, REG, Region52>;
impl<'a, REG> Region52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region52::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region52::Enabled)
    }
}
#[doc = "Enable protection for region 53. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region53 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region53> for bool {
    #[inline(always)]
    fn from(variant: Region53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION53` reader - Enable protection for region 53. Write '0' has no effect."]
pub type Region53R = crate::BitReader<Region53>;
impl Region53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region53 {
        match self.bits {
            false => Region53::Disabled,
            true => Region53::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region53::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region53::Enabled
    }
}
#[doc = "Field `REGION53` writer - Enable protection for region 53. Write '0' has no effect."]
pub type Region53W<'a, REG> = crate::BitWriter<'a, REG, Region53>;
impl<'a, REG> Region53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region53::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region53::Enabled)
    }
}
#[doc = "Enable protection for region 54. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region54 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region54> for bool {
    #[inline(always)]
    fn from(variant: Region54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION54` reader - Enable protection for region 54. Write '0' has no effect."]
pub type Region54R = crate::BitReader<Region54>;
impl Region54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region54 {
        match self.bits {
            false => Region54::Disabled,
            true => Region54::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region54::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region54::Enabled
    }
}
#[doc = "Field `REGION54` writer - Enable protection for region 54. Write '0' has no effect."]
pub type Region54W<'a, REG> = crate::BitWriter<'a, REG, Region54>;
impl<'a, REG> Region54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region54::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region54::Enabled)
    }
}
#[doc = "Enable protection for region 55. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region55 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region55> for bool {
    #[inline(always)]
    fn from(variant: Region55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION55` reader - Enable protection for region 55. Write '0' has no effect."]
pub type Region55R = crate::BitReader<Region55>;
impl Region55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region55 {
        match self.bits {
            false => Region55::Disabled,
            true => Region55::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region55::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region55::Enabled
    }
}
#[doc = "Field `REGION55` writer - Enable protection for region 55. Write '0' has no effect."]
pub type Region55W<'a, REG> = crate::BitWriter<'a, REG, Region55>;
impl<'a, REG> Region55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region55::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region55::Enabled)
    }
}
#[doc = "Enable protection for region 56. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region56 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region56> for bool {
    #[inline(always)]
    fn from(variant: Region56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION56` reader - Enable protection for region 56. Write '0' has no effect."]
pub type Region56R = crate::BitReader<Region56>;
impl Region56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region56 {
        match self.bits {
            false => Region56::Disabled,
            true => Region56::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region56::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region56::Enabled
    }
}
#[doc = "Field `REGION56` writer - Enable protection for region 56. Write '0' has no effect."]
pub type Region56W<'a, REG> = crate::BitWriter<'a, REG, Region56>;
impl<'a, REG> Region56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region56::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region56::Enabled)
    }
}
#[doc = "Enable protection for region 57. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region57 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region57> for bool {
    #[inline(always)]
    fn from(variant: Region57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION57` reader - Enable protection for region 57. Write '0' has no effect."]
pub type Region57R = crate::BitReader<Region57>;
impl Region57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region57 {
        match self.bits {
            false => Region57::Disabled,
            true => Region57::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region57::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region57::Enabled
    }
}
#[doc = "Field `REGION57` writer - Enable protection for region 57. Write '0' has no effect."]
pub type Region57W<'a, REG> = crate::BitWriter<'a, REG, Region57>;
impl<'a, REG> Region57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region57::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region57::Enabled)
    }
}
#[doc = "Enable protection for region 58. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region58 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region58> for bool {
    #[inline(always)]
    fn from(variant: Region58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION58` reader - Enable protection for region 58. Write '0' has no effect."]
pub type Region58R = crate::BitReader<Region58>;
impl Region58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region58 {
        match self.bits {
            false => Region58::Disabled,
            true => Region58::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region58::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region58::Enabled
    }
}
#[doc = "Field `REGION58` writer - Enable protection for region 58. Write '0' has no effect."]
pub type Region58W<'a, REG> = crate::BitWriter<'a, REG, Region58>;
impl<'a, REG> Region58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region58::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region58::Enabled)
    }
}
#[doc = "Enable protection for region 59. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region59 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region59> for bool {
    #[inline(always)]
    fn from(variant: Region59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION59` reader - Enable protection for region 59. Write '0' has no effect."]
pub type Region59R = crate::BitReader<Region59>;
impl Region59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region59 {
        match self.bits {
            false => Region59::Disabled,
            true => Region59::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region59::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region59::Enabled
    }
}
#[doc = "Field `REGION59` writer - Enable protection for region 59. Write '0' has no effect."]
pub type Region59W<'a, REG> = crate::BitWriter<'a, REG, Region59>;
impl<'a, REG> Region59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region59::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region59::Enabled)
    }
}
#[doc = "Enable protection for region 60. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region60 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region60> for bool {
    #[inline(always)]
    fn from(variant: Region60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION60` reader - Enable protection for region 60. Write '0' has no effect."]
pub type Region60R = crate::BitReader<Region60>;
impl Region60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region60 {
        match self.bits {
            false => Region60::Disabled,
            true => Region60::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region60::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region60::Enabled
    }
}
#[doc = "Field `REGION60` writer - Enable protection for region 60. Write '0' has no effect."]
pub type Region60W<'a, REG> = crate::BitWriter<'a, REG, Region60>;
impl<'a, REG> Region60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region60::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region60::Enabled)
    }
}
#[doc = "Enable protection for region 61. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region61 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region61> for bool {
    #[inline(always)]
    fn from(variant: Region61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION61` reader - Enable protection for region 61. Write '0' has no effect."]
pub type Region61R = crate::BitReader<Region61>;
impl Region61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region61 {
        match self.bits {
            false => Region61::Disabled,
            true => Region61::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region61::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region61::Enabled
    }
}
#[doc = "Field `REGION61` writer - Enable protection for region 61. Write '0' has no effect."]
pub type Region61W<'a, REG> = crate::BitWriter<'a, REG, Region61>;
impl<'a, REG> Region61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region61::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region61::Enabled)
    }
}
#[doc = "Enable protection for region 62. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region62 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region62> for bool {
    #[inline(always)]
    fn from(variant: Region62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION62` reader - Enable protection for region 62. Write '0' has no effect."]
pub type Region62R = crate::BitReader<Region62>;
impl Region62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region62 {
        match self.bits {
            false => Region62::Disabled,
            true => Region62::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region62::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region62::Enabled
    }
}
#[doc = "Field `REGION62` writer - Enable protection for region 62. Write '0' has no effect."]
pub type Region62W<'a, REG> = crate::BitWriter<'a, REG, Region62>;
impl<'a, REG> Region62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region62::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region62::Enabled)
    }
}
#[doc = "Enable protection for region 63. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region63 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region63> for bool {
    #[inline(always)]
    fn from(variant: Region63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION63` reader - Enable protection for region 63. Write '0' has no effect."]
pub type Region63R = crate::BitReader<Region63>;
impl Region63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region63 {
        match self.bits {
            false => Region63::Disabled,
            true => Region63::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region63::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region63::Enabled
    }
}
#[doc = "Field `REGION63` writer - Enable protection for region 63. Write '0' has no effect."]
pub type Region63W<'a, REG> = crate::BitWriter<'a, REG, Region63>;
impl<'a, REG> Region63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region63::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region63::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    pub fn region32(&self) -> Region32R {
        Region32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    pub fn region33(&self) -> Region33R {
        Region33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    pub fn region34(&self) -> Region34R {
        Region34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    pub fn region35(&self) -> Region35R {
        Region35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    pub fn region36(&self) -> Region36R {
        Region36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    pub fn region37(&self) -> Region37R {
        Region37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    pub fn region38(&self) -> Region38R {
        Region38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    pub fn region39(&self) -> Region39R {
        Region39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    pub fn region40(&self) -> Region40R {
        Region40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    pub fn region41(&self) -> Region41R {
        Region41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    pub fn region42(&self) -> Region42R {
        Region42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    pub fn region43(&self) -> Region43R {
        Region43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    pub fn region44(&self) -> Region44R {
        Region44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    pub fn region45(&self) -> Region45R {
        Region45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    pub fn region46(&self) -> Region46R {
        Region46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    pub fn region47(&self) -> Region47R {
        Region47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    pub fn region48(&self) -> Region48R {
        Region48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    pub fn region49(&self) -> Region49R {
        Region49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    pub fn region50(&self) -> Region50R {
        Region50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    pub fn region51(&self) -> Region51R {
        Region51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    pub fn region52(&self) -> Region52R {
        Region52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    pub fn region53(&self) -> Region53R {
        Region53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    pub fn region54(&self) -> Region54R {
        Region54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    pub fn region55(&self) -> Region55R {
        Region55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    pub fn region56(&self) -> Region56R {
        Region56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    pub fn region57(&self) -> Region57R {
        Region57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    pub fn region58(&self) -> Region58R {
        Region58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    pub fn region59(&self) -> Region59R {
        Region59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    pub fn region60(&self) -> Region60R {
        Region60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    pub fn region61(&self) -> Region61R {
        Region61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    pub fn region62(&self) -> Region62R {
        Region62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    pub fn region63(&self) -> Region63R {
        Region63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    pub fn region32(&mut self) -> Region32W<'_, Config1Spec> {
        Region32W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    pub fn region33(&mut self) -> Region33W<'_, Config1Spec> {
        Region33W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    pub fn region34(&mut self) -> Region34W<'_, Config1Spec> {
        Region34W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    pub fn region35(&mut self) -> Region35W<'_, Config1Spec> {
        Region35W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    pub fn region36(&mut self) -> Region36W<'_, Config1Spec> {
        Region36W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    pub fn region37(&mut self) -> Region37W<'_, Config1Spec> {
        Region37W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    pub fn region38(&mut self) -> Region38W<'_, Config1Spec> {
        Region38W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    pub fn region39(&mut self) -> Region39W<'_, Config1Spec> {
        Region39W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    pub fn region40(&mut self) -> Region40W<'_, Config1Spec> {
        Region40W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    pub fn region41(&mut self) -> Region41W<'_, Config1Spec> {
        Region41W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    pub fn region42(&mut self) -> Region42W<'_, Config1Spec> {
        Region42W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    pub fn region43(&mut self) -> Region43W<'_, Config1Spec> {
        Region43W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    pub fn region44(&mut self) -> Region44W<'_, Config1Spec> {
        Region44W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    pub fn region45(&mut self) -> Region45W<'_, Config1Spec> {
        Region45W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    pub fn region46(&mut self) -> Region46W<'_, Config1Spec> {
        Region46W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    pub fn region47(&mut self) -> Region47W<'_, Config1Spec> {
        Region47W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    pub fn region48(&mut self) -> Region48W<'_, Config1Spec> {
        Region48W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    pub fn region49(&mut self) -> Region49W<'_, Config1Spec> {
        Region49W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    pub fn region50(&mut self) -> Region50W<'_, Config1Spec> {
        Region50W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    pub fn region51(&mut self) -> Region51W<'_, Config1Spec> {
        Region51W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    pub fn region52(&mut self) -> Region52W<'_, Config1Spec> {
        Region52W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    pub fn region53(&mut self) -> Region53W<'_, Config1Spec> {
        Region53W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    pub fn region54(&mut self) -> Region54W<'_, Config1Spec> {
        Region54W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    pub fn region55(&mut self) -> Region55W<'_, Config1Spec> {
        Region55W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    pub fn region56(&mut self) -> Region56W<'_, Config1Spec> {
        Region56W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    pub fn region57(&mut self) -> Region57W<'_, Config1Spec> {
        Region57W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    pub fn region58(&mut self) -> Region58W<'_, Config1Spec> {
        Region58W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    pub fn region59(&mut self) -> Region59W<'_, Config1Spec> {
        Region59W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    pub fn region60(&mut self) -> Region60W<'_, Config1Spec> {
        Region60W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    pub fn region61(&mut self) -> Region61W<'_, Config1Spec> {
        Region61W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    pub fn region62(&mut self) -> Region62W<'_, Config1Spec> {
        Region62W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    pub fn region63(&mut self) -> Region63W<'_, Config1Spec> {
        Region63W::new(self, 31)
    }
}
#[doc = "Block protect configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config1Spec;
impl crate::RegisterSpec for Config1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for Config1Spec {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for Config1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for Config1Spec {}
