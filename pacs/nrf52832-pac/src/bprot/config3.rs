#[doc = "Register `CONFIG3` reader"]
pub type R = crate::R<Config3Spec>;
#[doc = "Register `CONFIG3` writer"]
pub type W = crate::W<Config3Spec>;
#[doc = "Enable protection for region 96. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region96 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region96> for bool {
    #[inline(always)]
    fn from(variant: Region96) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION96` reader - Enable protection for region 96. Write '0' has no effect."]
pub type Region96R = crate::BitReader<Region96>;
impl Region96R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region96 {
        match self.bits {
            false => Region96::Disabled,
            true => Region96::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region96::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region96::Enabled
    }
}
#[doc = "Field `REGION96` writer - Enable protection for region 96. Write '0' has no effect."]
pub type Region96W<'a, REG> = crate::BitWriter<'a, REG, Region96>;
impl<'a, REG> Region96W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region96::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region96::Enabled)
    }
}
#[doc = "Enable protection for region 97. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region97 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region97> for bool {
    #[inline(always)]
    fn from(variant: Region97) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION97` reader - Enable protection for region 97. Write '0' has no effect."]
pub type Region97R = crate::BitReader<Region97>;
impl Region97R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region97 {
        match self.bits {
            false => Region97::Disabled,
            true => Region97::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region97::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region97::Enabled
    }
}
#[doc = "Field `REGION97` writer - Enable protection for region 97. Write '0' has no effect."]
pub type Region97W<'a, REG> = crate::BitWriter<'a, REG, Region97>;
impl<'a, REG> Region97W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region97::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region97::Enabled)
    }
}
#[doc = "Enable protection for region 98. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region98 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region98> for bool {
    #[inline(always)]
    fn from(variant: Region98) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION98` reader - Enable protection for region 98. Write '0' has no effect."]
pub type Region98R = crate::BitReader<Region98>;
impl Region98R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region98 {
        match self.bits {
            false => Region98::Disabled,
            true => Region98::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region98::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region98::Enabled
    }
}
#[doc = "Field `REGION98` writer - Enable protection for region 98. Write '0' has no effect."]
pub type Region98W<'a, REG> = crate::BitWriter<'a, REG, Region98>;
impl<'a, REG> Region98W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region98::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region98::Enabled)
    }
}
#[doc = "Enable protection for region 99. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region99 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region99> for bool {
    #[inline(always)]
    fn from(variant: Region99) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION99` reader - Enable protection for region 99. Write '0' has no effect."]
pub type Region99R = crate::BitReader<Region99>;
impl Region99R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region99 {
        match self.bits {
            false => Region99::Disabled,
            true => Region99::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region99::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region99::Enabled
    }
}
#[doc = "Field `REGION99` writer - Enable protection for region 99. Write '0' has no effect."]
pub type Region99W<'a, REG> = crate::BitWriter<'a, REG, Region99>;
impl<'a, REG> Region99W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region99::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region99::Enabled)
    }
}
#[doc = "Enable protection for region 100. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region100 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region100> for bool {
    #[inline(always)]
    fn from(variant: Region100) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION100` reader - Enable protection for region 100. Write '0' has no effect."]
pub type Region100R = crate::BitReader<Region100>;
impl Region100R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region100 {
        match self.bits {
            false => Region100::Disabled,
            true => Region100::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region100::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region100::Enabled
    }
}
#[doc = "Field `REGION100` writer - Enable protection for region 100. Write '0' has no effect."]
pub type Region100W<'a, REG> = crate::BitWriter<'a, REG, Region100>;
impl<'a, REG> Region100W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region100::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region100::Enabled)
    }
}
#[doc = "Enable protection for region 101. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region101 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region101> for bool {
    #[inline(always)]
    fn from(variant: Region101) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION101` reader - Enable protection for region 101. Write '0' has no effect."]
pub type Region101R = crate::BitReader<Region101>;
impl Region101R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region101 {
        match self.bits {
            false => Region101::Disabled,
            true => Region101::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region101::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region101::Enabled
    }
}
#[doc = "Field `REGION101` writer - Enable protection for region 101. Write '0' has no effect."]
pub type Region101W<'a, REG> = crate::BitWriter<'a, REG, Region101>;
impl<'a, REG> Region101W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region101::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region101::Enabled)
    }
}
#[doc = "Enable protection for region 102. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region102 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region102> for bool {
    #[inline(always)]
    fn from(variant: Region102) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION102` reader - Enable protection for region 102. Write '0' has no effect."]
pub type Region102R = crate::BitReader<Region102>;
impl Region102R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region102 {
        match self.bits {
            false => Region102::Disabled,
            true => Region102::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region102::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region102::Enabled
    }
}
#[doc = "Field `REGION102` writer - Enable protection for region 102. Write '0' has no effect."]
pub type Region102W<'a, REG> = crate::BitWriter<'a, REG, Region102>;
impl<'a, REG> Region102W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region102::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region102::Enabled)
    }
}
#[doc = "Enable protection for region 103. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region103 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region103> for bool {
    #[inline(always)]
    fn from(variant: Region103) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION103` reader - Enable protection for region 103. Write '0' has no effect."]
pub type Region103R = crate::BitReader<Region103>;
impl Region103R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region103 {
        match self.bits {
            false => Region103::Disabled,
            true => Region103::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region103::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region103::Enabled
    }
}
#[doc = "Field `REGION103` writer - Enable protection for region 103. Write '0' has no effect."]
pub type Region103W<'a, REG> = crate::BitWriter<'a, REG, Region103>;
impl<'a, REG> Region103W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region103::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region103::Enabled)
    }
}
#[doc = "Enable protection for region 104. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region104 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region104> for bool {
    #[inline(always)]
    fn from(variant: Region104) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION104` reader - Enable protection for region 104. Write '0' has no effect."]
pub type Region104R = crate::BitReader<Region104>;
impl Region104R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region104 {
        match self.bits {
            false => Region104::Disabled,
            true => Region104::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region104::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region104::Enabled
    }
}
#[doc = "Field `REGION104` writer - Enable protection for region 104. Write '0' has no effect."]
pub type Region104W<'a, REG> = crate::BitWriter<'a, REG, Region104>;
impl<'a, REG> Region104W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region104::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region104::Enabled)
    }
}
#[doc = "Enable protection for region 105. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region105 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region105> for bool {
    #[inline(always)]
    fn from(variant: Region105) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION105` reader - Enable protection for region 105. Write '0' has no effect."]
pub type Region105R = crate::BitReader<Region105>;
impl Region105R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region105 {
        match self.bits {
            false => Region105::Disabled,
            true => Region105::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region105::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region105::Enabled
    }
}
#[doc = "Field `REGION105` writer - Enable protection for region 105. Write '0' has no effect."]
pub type Region105W<'a, REG> = crate::BitWriter<'a, REG, Region105>;
impl<'a, REG> Region105W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region105::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region105::Enabled)
    }
}
#[doc = "Enable protection for region 106. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region106 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region106> for bool {
    #[inline(always)]
    fn from(variant: Region106) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION106` reader - Enable protection for region 106. Write '0' has no effect."]
pub type Region106R = crate::BitReader<Region106>;
impl Region106R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region106 {
        match self.bits {
            false => Region106::Disabled,
            true => Region106::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region106::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region106::Enabled
    }
}
#[doc = "Field `REGION106` writer - Enable protection for region 106. Write '0' has no effect."]
pub type Region106W<'a, REG> = crate::BitWriter<'a, REG, Region106>;
impl<'a, REG> Region106W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region106::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region106::Enabled)
    }
}
#[doc = "Enable protection for region 107. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region107 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region107> for bool {
    #[inline(always)]
    fn from(variant: Region107) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION107` reader - Enable protection for region 107. Write '0' has no effect."]
pub type Region107R = crate::BitReader<Region107>;
impl Region107R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region107 {
        match self.bits {
            false => Region107::Disabled,
            true => Region107::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region107::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region107::Enabled
    }
}
#[doc = "Field `REGION107` writer - Enable protection for region 107. Write '0' has no effect."]
pub type Region107W<'a, REG> = crate::BitWriter<'a, REG, Region107>;
impl<'a, REG> Region107W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region107::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region107::Enabled)
    }
}
#[doc = "Enable protection for region 108. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region108 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region108> for bool {
    #[inline(always)]
    fn from(variant: Region108) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION108` reader - Enable protection for region 108. Write '0' has no effect."]
pub type Region108R = crate::BitReader<Region108>;
impl Region108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region108 {
        match self.bits {
            false => Region108::Disabled,
            true => Region108::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region108::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region108::Enabled
    }
}
#[doc = "Field `REGION108` writer - Enable protection for region 108. Write '0' has no effect."]
pub type Region108W<'a, REG> = crate::BitWriter<'a, REG, Region108>;
impl<'a, REG> Region108W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region108::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region108::Enabled)
    }
}
#[doc = "Enable protection for region 109. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region109 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region109> for bool {
    #[inline(always)]
    fn from(variant: Region109) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION109` reader - Enable protection for region 109. Write '0' has no effect."]
pub type Region109R = crate::BitReader<Region109>;
impl Region109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region109 {
        match self.bits {
            false => Region109::Disabled,
            true => Region109::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region109::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region109::Enabled
    }
}
#[doc = "Field `REGION109` writer - Enable protection for region 109. Write '0' has no effect."]
pub type Region109W<'a, REG> = crate::BitWriter<'a, REG, Region109>;
impl<'a, REG> Region109W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region109::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region109::Enabled)
    }
}
#[doc = "Enable protection for region 110. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region110 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region110> for bool {
    #[inline(always)]
    fn from(variant: Region110) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION110` reader - Enable protection for region 110. Write '0' has no effect."]
pub type Region110R = crate::BitReader<Region110>;
impl Region110R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region110 {
        match self.bits {
            false => Region110::Disabled,
            true => Region110::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region110::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region110::Enabled
    }
}
#[doc = "Field `REGION110` writer - Enable protection for region 110. Write '0' has no effect."]
pub type Region110W<'a, REG> = crate::BitWriter<'a, REG, Region110>;
impl<'a, REG> Region110W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region110::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region110::Enabled)
    }
}
#[doc = "Enable protection for region 111. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region111 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region111> for bool {
    #[inline(always)]
    fn from(variant: Region111) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION111` reader - Enable protection for region 111. Write '0' has no effect."]
pub type Region111R = crate::BitReader<Region111>;
impl Region111R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region111 {
        match self.bits {
            false => Region111::Disabled,
            true => Region111::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region111::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region111::Enabled
    }
}
#[doc = "Field `REGION111` writer - Enable protection for region 111. Write '0' has no effect."]
pub type Region111W<'a, REG> = crate::BitWriter<'a, REG, Region111>;
impl<'a, REG> Region111W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region111::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region111::Enabled)
    }
}
#[doc = "Enable protection for region 112. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region112 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region112> for bool {
    #[inline(always)]
    fn from(variant: Region112) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION112` reader - Enable protection for region 112. Write '0' has no effect."]
pub type Region112R = crate::BitReader<Region112>;
impl Region112R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region112 {
        match self.bits {
            false => Region112::Disabled,
            true => Region112::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region112::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region112::Enabled
    }
}
#[doc = "Field `REGION112` writer - Enable protection for region 112. Write '0' has no effect."]
pub type Region112W<'a, REG> = crate::BitWriter<'a, REG, Region112>;
impl<'a, REG> Region112W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region112::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region112::Enabled)
    }
}
#[doc = "Enable protection for region 113. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region113 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region113> for bool {
    #[inline(always)]
    fn from(variant: Region113) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION113` reader - Enable protection for region 113. Write '0' has no effect."]
pub type Region113R = crate::BitReader<Region113>;
impl Region113R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region113 {
        match self.bits {
            false => Region113::Disabled,
            true => Region113::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region113::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region113::Enabled
    }
}
#[doc = "Field `REGION113` writer - Enable protection for region 113. Write '0' has no effect."]
pub type Region113W<'a, REG> = crate::BitWriter<'a, REG, Region113>;
impl<'a, REG> Region113W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region113::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region113::Enabled)
    }
}
#[doc = "Enable protection for region 114. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region114 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region114> for bool {
    #[inline(always)]
    fn from(variant: Region114) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION114` reader - Enable protection for region 114. Write '0' has no effect."]
pub type Region114R = crate::BitReader<Region114>;
impl Region114R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region114 {
        match self.bits {
            false => Region114::Disabled,
            true => Region114::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region114::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region114::Enabled
    }
}
#[doc = "Field `REGION114` writer - Enable protection for region 114. Write '0' has no effect."]
pub type Region114W<'a, REG> = crate::BitWriter<'a, REG, Region114>;
impl<'a, REG> Region114W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region114::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region114::Enabled)
    }
}
#[doc = "Enable protection for region 115. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region115 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region115> for bool {
    #[inline(always)]
    fn from(variant: Region115) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION115` reader - Enable protection for region 115. Write '0' has no effect."]
pub type Region115R = crate::BitReader<Region115>;
impl Region115R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region115 {
        match self.bits {
            false => Region115::Disabled,
            true => Region115::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region115::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region115::Enabled
    }
}
#[doc = "Field `REGION115` writer - Enable protection for region 115. Write '0' has no effect."]
pub type Region115W<'a, REG> = crate::BitWriter<'a, REG, Region115>;
impl<'a, REG> Region115W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region115::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region115::Enabled)
    }
}
#[doc = "Enable protection for region 116. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region116 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region116> for bool {
    #[inline(always)]
    fn from(variant: Region116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION116` reader - Enable protection for region 116. Write '0' has no effect."]
pub type Region116R = crate::BitReader<Region116>;
impl Region116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region116 {
        match self.bits {
            false => Region116::Disabled,
            true => Region116::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region116::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region116::Enabled
    }
}
#[doc = "Field `REGION116` writer - Enable protection for region 116. Write '0' has no effect."]
pub type Region116W<'a, REG> = crate::BitWriter<'a, REG, Region116>;
impl<'a, REG> Region116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region116::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region116::Enabled)
    }
}
#[doc = "Enable protection for region 117. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region117 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region117> for bool {
    #[inline(always)]
    fn from(variant: Region117) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION117` reader - Enable protection for region 117. Write '0' has no effect."]
pub type Region117R = crate::BitReader<Region117>;
impl Region117R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region117 {
        match self.bits {
            false => Region117::Disabled,
            true => Region117::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region117::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region117::Enabled
    }
}
#[doc = "Field `REGION117` writer - Enable protection for region 117. Write '0' has no effect."]
pub type Region117W<'a, REG> = crate::BitWriter<'a, REG, Region117>;
impl<'a, REG> Region117W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region117::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region117::Enabled)
    }
}
#[doc = "Enable protection for region 118. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region118 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region118> for bool {
    #[inline(always)]
    fn from(variant: Region118) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION118` reader - Enable protection for region 118. Write '0' has no effect."]
pub type Region118R = crate::BitReader<Region118>;
impl Region118R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region118 {
        match self.bits {
            false => Region118::Disabled,
            true => Region118::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region118::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region118::Enabled
    }
}
#[doc = "Field `REGION118` writer - Enable protection for region 118. Write '0' has no effect."]
pub type Region118W<'a, REG> = crate::BitWriter<'a, REG, Region118>;
impl<'a, REG> Region118W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region118::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region118::Enabled)
    }
}
#[doc = "Enable protection for region 119. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region119 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region119> for bool {
    #[inline(always)]
    fn from(variant: Region119) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION119` reader - Enable protection for region 119. Write '0' has no effect."]
pub type Region119R = crate::BitReader<Region119>;
impl Region119R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region119 {
        match self.bits {
            false => Region119::Disabled,
            true => Region119::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region119::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region119::Enabled
    }
}
#[doc = "Field `REGION119` writer - Enable protection for region 119. Write '0' has no effect."]
pub type Region119W<'a, REG> = crate::BitWriter<'a, REG, Region119>;
impl<'a, REG> Region119W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region119::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region119::Enabled)
    }
}
#[doc = "Enable protection for region 120. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region120 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region120> for bool {
    #[inline(always)]
    fn from(variant: Region120) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION120` reader - Enable protection for region 120. Write '0' has no effect."]
pub type Region120R = crate::BitReader<Region120>;
impl Region120R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region120 {
        match self.bits {
            false => Region120::Disabled,
            true => Region120::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region120::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region120::Enabled
    }
}
#[doc = "Field `REGION120` writer - Enable protection for region 120. Write '0' has no effect."]
pub type Region120W<'a, REG> = crate::BitWriter<'a, REG, Region120>;
impl<'a, REG> Region120W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region120::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region120::Enabled)
    }
}
#[doc = "Enable protection for region 121. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region121 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region121> for bool {
    #[inline(always)]
    fn from(variant: Region121) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION121` reader - Enable protection for region 121. Write '0' has no effect."]
pub type Region121R = crate::BitReader<Region121>;
impl Region121R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region121 {
        match self.bits {
            false => Region121::Disabled,
            true => Region121::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region121::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region121::Enabled
    }
}
#[doc = "Field `REGION121` writer - Enable protection for region 121. Write '0' has no effect."]
pub type Region121W<'a, REG> = crate::BitWriter<'a, REG, Region121>;
impl<'a, REG> Region121W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region121::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region121::Enabled)
    }
}
#[doc = "Enable protection for region 122. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region122 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region122> for bool {
    #[inline(always)]
    fn from(variant: Region122) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION122` reader - Enable protection for region 122. Write '0' has no effect."]
pub type Region122R = crate::BitReader<Region122>;
impl Region122R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region122 {
        match self.bits {
            false => Region122::Disabled,
            true => Region122::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region122::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region122::Enabled
    }
}
#[doc = "Field `REGION122` writer - Enable protection for region 122. Write '0' has no effect."]
pub type Region122W<'a, REG> = crate::BitWriter<'a, REG, Region122>;
impl<'a, REG> Region122W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region122::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region122::Enabled)
    }
}
#[doc = "Enable protection for region 123. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region123 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region123> for bool {
    #[inline(always)]
    fn from(variant: Region123) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION123` reader - Enable protection for region 123. Write '0' has no effect."]
pub type Region123R = crate::BitReader<Region123>;
impl Region123R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region123 {
        match self.bits {
            false => Region123::Disabled,
            true => Region123::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region123::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region123::Enabled
    }
}
#[doc = "Field `REGION123` writer - Enable protection for region 123. Write '0' has no effect."]
pub type Region123W<'a, REG> = crate::BitWriter<'a, REG, Region123>;
impl<'a, REG> Region123W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region123::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region123::Enabled)
    }
}
#[doc = "Enable protection for region 124. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region124 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region124> for bool {
    #[inline(always)]
    fn from(variant: Region124) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION124` reader - Enable protection for region 124. Write '0' has no effect."]
pub type Region124R = crate::BitReader<Region124>;
impl Region124R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region124 {
        match self.bits {
            false => Region124::Disabled,
            true => Region124::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region124::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region124::Enabled
    }
}
#[doc = "Field `REGION124` writer - Enable protection for region 124. Write '0' has no effect."]
pub type Region124W<'a, REG> = crate::BitWriter<'a, REG, Region124>;
impl<'a, REG> Region124W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region124::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region124::Enabled)
    }
}
#[doc = "Enable protection for region 125. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region125 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region125> for bool {
    #[inline(always)]
    fn from(variant: Region125) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION125` reader - Enable protection for region 125. Write '0' has no effect."]
pub type Region125R = crate::BitReader<Region125>;
impl Region125R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region125 {
        match self.bits {
            false => Region125::Disabled,
            true => Region125::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region125::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region125::Enabled
    }
}
#[doc = "Field `REGION125` writer - Enable protection for region 125. Write '0' has no effect."]
pub type Region125W<'a, REG> = crate::BitWriter<'a, REG, Region125>;
impl<'a, REG> Region125W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region125::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region125::Enabled)
    }
}
#[doc = "Enable protection for region 126. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region126 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region126> for bool {
    #[inline(always)]
    fn from(variant: Region126) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION126` reader - Enable protection for region 126. Write '0' has no effect."]
pub type Region126R = crate::BitReader<Region126>;
impl Region126R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region126 {
        match self.bits {
            false => Region126::Disabled,
            true => Region126::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region126::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region126::Enabled
    }
}
#[doc = "Field `REGION126` writer - Enable protection for region 126. Write '0' has no effect."]
pub type Region126W<'a, REG> = crate::BitWriter<'a, REG, Region126>;
impl<'a, REG> Region126W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region126::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region126::Enabled)
    }
}
#[doc = "Enable protection for region 127. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region127 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region127> for bool {
    #[inline(always)]
    fn from(variant: Region127) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION127` reader - Enable protection for region 127. Write '0' has no effect."]
pub type Region127R = crate::BitReader<Region127>;
impl Region127R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region127 {
        match self.bits {
            false => Region127::Disabled,
            true => Region127::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region127::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region127::Enabled
    }
}
#[doc = "Field `REGION127` writer - Enable protection for region 127. Write '0' has no effect."]
pub type Region127W<'a, REG> = crate::BitWriter<'a, REG, Region127>;
impl<'a, REG> Region127W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region127::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region127::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&self) -> Region96R {
        Region96R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&self) -> Region97R {
        Region97R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&self) -> Region98R {
        Region98R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&self) -> Region99R {
        Region99R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&self) -> Region100R {
        Region100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&self) -> Region101R {
        Region101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&self) -> Region102R {
        Region102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&self) -> Region103R {
        Region103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&self) -> Region104R {
        Region104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&self) -> Region105R {
        Region105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&self) -> Region106R {
        Region106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&self) -> Region107R {
        Region107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&self) -> Region108R {
        Region108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&self) -> Region109R {
        Region109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&self) -> Region110R {
        Region110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&self) -> Region111R {
        Region111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&self) -> Region112R {
        Region112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&self) -> Region113R {
        Region113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&self) -> Region114R {
        Region114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&self) -> Region115R {
        Region115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&self) -> Region116R {
        Region116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&self) -> Region117R {
        Region117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&self) -> Region118R {
        Region118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&self) -> Region119R {
        Region119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&self) -> Region120R {
        Region120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&self) -> Region121R {
        Region121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&self) -> Region122R {
        Region122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&self) -> Region123R {
        Region123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&self) -> Region124R {
        Region124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&self) -> Region125R {
        Region125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&self) -> Region126R {
        Region126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&self) -> Region127R {
        Region127R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&mut self) -> Region96W<'_, Config3Spec> {
        Region96W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&mut self) -> Region97W<'_, Config3Spec> {
        Region97W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&mut self) -> Region98W<'_, Config3Spec> {
        Region98W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&mut self) -> Region99W<'_, Config3Spec> {
        Region99W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&mut self) -> Region100W<'_, Config3Spec> {
        Region100W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&mut self) -> Region101W<'_, Config3Spec> {
        Region101W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&mut self) -> Region102W<'_, Config3Spec> {
        Region102W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&mut self) -> Region103W<'_, Config3Spec> {
        Region103W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&mut self) -> Region104W<'_, Config3Spec> {
        Region104W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&mut self) -> Region105W<'_, Config3Spec> {
        Region105W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&mut self) -> Region106W<'_, Config3Spec> {
        Region106W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&mut self) -> Region107W<'_, Config3Spec> {
        Region107W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&mut self) -> Region108W<'_, Config3Spec> {
        Region108W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&mut self) -> Region109W<'_, Config3Spec> {
        Region109W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&mut self) -> Region110W<'_, Config3Spec> {
        Region110W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&mut self) -> Region111W<'_, Config3Spec> {
        Region111W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&mut self) -> Region112W<'_, Config3Spec> {
        Region112W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&mut self) -> Region113W<'_, Config3Spec> {
        Region113W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&mut self) -> Region114W<'_, Config3Spec> {
        Region114W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&mut self) -> Region115W<'_, Config3Spec> {
        Region115W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&mut self) -> Region116W<'_, Config3Spec> {
        Region116W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&mut self) -> Region117W<'_, Config3Spec> {
        Region117W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&mut self) -> Region118W<'_, Config3Spec> {
        Region118W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&mut self) -> Region119W<'_, Config3Spec> {
        Region119W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&mut self) -> Region120W<'_, Config3Spec> {
        Region120W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&mut self) -> Region121W<'_, Config3Spec> {
        Region121W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&mut self) -> Region122W<'_, Config3Spec> {
        Region122W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&mut self) -> Region123W<'_, Config3Spec> {
        Region123W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&mut self) -> Region124W<'_, Config3Spec> {
        Region124W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&mut self) -> Region125W<'_, Config3Spec> {
        Region125W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&mut self) -> Region126W<'_, Config3Spec> {
        Region126W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&mut self) -> Region127W<'_, Config3Spec> {
        Region127W::new(self, 31)
    }
}
#[doc = "Block protect configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`config3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config3Spec;
impl crate::RegisterSpec for Config3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config3::R`](R) reader structure"]
impl crate::Readable for Config3Spec {}
#[doc = "`write(|w| ..)` method takes [`config3::W`](W) writer structure"]
impl crate::Writable for Config3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG3 to value 0"]
impl crate::Resettable for Config3Spec {}
