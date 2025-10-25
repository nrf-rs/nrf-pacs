#[doc = "Register `CONFIG2` reader"]
pub type R = crate::R<Config2Spec>;
#[doc = "Register `CONFIG2` writer"]
pub type W = crate::W<Config2Spec>;
#[doc = "Enable protection for region 64. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region64 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region64> for bool {
    #[inline(always)]
    fn from(variant: Region64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION64` reader - Enable protection for region 64. Write '0' has no effect."]
pub type Region64R = crate::BitReader<Region64>;
impl Region64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region64 {
        match self.bits {
            false => Region64::Disabled,
            true => Region64::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region64::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region64::Enabled
    }
}
#[doc = "Field `REGION64` writer - Enable protection for region 64. Write '0' has no effect."]
pub type Region64W<'a, REG> = crate::BitWriter<'a, REG, Region64>;
impl<'a, REG> Region64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region64::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region64::Enabled)
    }
}
#[doc = "Enable protection for region 65. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region65 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region65> for bool {
    #[inline(always)]
    fn from(variant: Region65) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION65` reader - Enable protection for region 65. Write '0' has no effect."]
pub type Region65R = crate::BitReader<Region65>;
impl Region65R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region65 {
        match self.bits {
            false => Region65::Disabled,
            true => Region65::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region65::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region65::Enabled
    }
}
#[doc = "Field `REGION65` writer - Enable protection for region 65. Write '0' has no effect."]
pub type Region65W<'a, REG> = crate::BitWriter<'a, REG, Region65>;
impl<'a, REG> Region65W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region65::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region65::Enabled)
    }
}
#[doc = "Enable protection for region 66. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region66 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region66> for bool {
    #[inline(always)]
    fn from(variant: Region66) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION66` reader - Enable protection for region 66. Write '0' has no effect."]
pub type Region66R = crate::BitReader<Region66>;
impl Region66R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region66 {
        match self.bits {
            false => Region66::Disabled,
            true => Region66::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region66::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region66::Enabled
    }
}
#[doc = "Field `REGION66` writer - Enable protection for region 66. Write '0' has no effect."]
pub type Region66W<'a, REG> = crate::BitWriter<'a, REG, Region66>;
impl<'a, REG> Region66W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region66::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region66::Enabled)
    }
}
#[doc = "Enable protection for region 67. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region67 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region67> for bool {
    #[inline(always)]
    fn from(variant: Region67) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION67` reader - Enable protection for region 67. Write '0' has no effect."]
pub type Region67R = crate::BitReader<Region67>;
impl Region67R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region67 {
        match self.bits {
            false => Region67::Disabled,
            true => Region67::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region67::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region67::Enabled
    }
}
#[doc = "Field `REGION67` writer - Enable protection for region 67. Write '0' has no effect."]
pub type Region67W<'a, REG> = crate::BitWriter<'a, REG, Region67>;
impl<'a, REG> Region67W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region67::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region67::Enabled)
    }
}
#[doc = "Enable protection for region 68. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region68 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region68> for bool {
    #[inline(always)]
    fn from(variant: Region68) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION68` reader - Enable protection for region 68. Write '0' has no effect."]
pub type Region68R = crate::BitReader<Region68>;
impl Region68R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region68 {
        match self.bits {
            false => Region68::Disabled,
            true => Region68::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region68::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region68::Enabled
    }
}
#[doc = "Field `REGION68` writer - Enable protection for region 68. Write '0' has no effect."]
pub type Region68W<'a, REG> = crate::BitWriter<'a, REG, Region68>;
impl<'a, REG> Region68W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region68::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region68::Enabled)
    }
}
#[doc = "Enable protection for region 69. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region69 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region69> for bool {
    #[inline(always)]
    fn from(variant: Region69) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION69` reader - Enable protection for region 69. Write '0' has no effect."]
pub type Region69R = crate::BitReader<Region69>;
impl Region69R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region69 {
        match self.bits {
            false => Region69::Disabled,
            true => Region69::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region69::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region69::Enabled
    }
}
#[doc = "Field `REGION69` writer - Enable protection for region 69. Write '0' has no effect."]
pub type Region69W<'a, REG> = crate::BitWriter<'a, REG, Region69>;
impl<'a, REG> Region69W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region69::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region69::Enabled)
    }
}
#[doc = "Enable protection for region 70. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region70 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region70> for bool {
    #[inline(always)]
    fn from(variant: Region70) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION70` reader - Enable protection for region 70. Write '0' has no effect."]
pub type Region70R = crate::BitReader<Region70>;
impl Region70R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region70 {
        match self.bits {
            false => Region70::Disabled,
            true => Region70::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region70::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region70::Enabled
    }
}
#[doc = "Field `REGION70` writer - Enable protection for region 70. Write '0' has no effect."]
pub type Region70W<'a, REG> = crate::BitWriter<'a, REG, Region70>;
impl<'a, REG> Region70W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region70::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region70::Enabled)
    }
}
#[doc = "Enable protection for region 71. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region71 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region71> for bool {
    #[inline(always)]
    fn from(variant: Region71) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION71` reader - Enable protection for region 71. Write '0' has no effect."]
pub type Region71R = crate::BitReader<Region71>;
impl Region71R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region71 {
        match self.bits {
            false => Region71::Disabled,
            true => Region71::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region71::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region71::Enabled
    }
}
#[doc = "Field `REGION71` writer - Enable protection for region 71. Write '0' has no effect."]
pub type Region71W<'a, REG> = crate::BitWriter<'a, REG, Region71>;
impl<'a, REG> Region71W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region71::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region71::Enabled)
    }
}
#[doc = "Enable protection for region 72. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region72 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region72> for bool {
    #[inline(always)]
    fn from(variant: Region72) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION72` reader - Enable protection for region 72. Write '0' has no effect."]
pub type Region72R = crate::BitReader<Region72>;
impl Region72R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region72 {
        match self.bits {
            false => Region72::Disabled,
            true => Region72::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region72::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region72::Enabled
    }
}
#[doc = "Field `REGION72` writer - Enable protection for region 72. Write '0' has no effect."]
pub type Region72W<'a, REG> = crate::BitWriter<'a, REG, Region72>;
impl<'a, REG> Region72W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region72::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region72::Enabled)
    }
}
#[doc = "Enable protection for region 73. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region73 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region73> for bool {
    #[inline(always)]
    fn from(variant: Region73) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION73` reader - Enable protection for region 73. Write '0' has no effect."]
pub type Region73R = crate::BitReader<Region73>;
impl Region73R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region73 {
        match self.bits {
            false => Region73::Disabled,
            true => Region73::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region73::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region73::Enabled
    }
}
#[doc = "Field `REGION73` writer - Enable protection for region 73. Write '0' has no effect."]
pub type Region73W<'a, REG> = crate::BitWriter<'a, REG, Region73>;
impl<'a, REG> Region73W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region73::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region73::Enabled)
    }
}
#[doc = "Enable protection for region 74. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region74 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region74> for bool {
    #[inline(always)]
    fn from(variant: Region74) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION74` reader - Enable protection for region 74. Write '0' has no effect."]
pub type Region74R = crate::BitReader<Region74>;
impl Region74R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region74 {
        match self.bits {
            false => Region74::Disabled,
            true => Region74::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region74::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region74::Enabled
    }
}
#[doc = "Field `REGION74` writer - Enable protection for region 74. Write '0' has no effect."]
pub type Region74W<'a, REG> = crate::BitWriter<'a, REG, Region74>;
impl<'a, REG> Region74W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region74::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region74::Enabled)
    }
}
#[doc = "Enable protection for region 75. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region75 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region75> for bool {
    #[inline(always)]
    fn from(variant: Region75) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION75` reader - Enable protection for region 75. Write '0' has no effect."]
pub type Region75R = crate::BitReader<Region75>;
impl Region75R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region75 {
        match self.bits {
            false => Region75::Disabled,
            true => Region75::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region75::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region75::Enabled
    }
}
#[doc = "Field `REGION75` writer - Enable protection for region 75. Write '0' has no effect."]
pub type Region75W<'a, REG> = crate::BitWriter<'a, REG, Region75>;
impl<'a, REG> Region75W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region75::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region75::Enabled)
    }
}
#[doc = "Enable protection for region 76. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region76 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region76> for bool {
    #[inline(always)]
    fn from(variant: Region76) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION76` reader - Enable protection for region 76. Write '0' has no effect."]
pub type Region76R = crate::BitReader<Region76>;
impl Region76R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region76 {
        match self.bits {
            false => Region76::Disabled,
            true => Region76::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region76::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region76::Enabled
    }
}
#[doc = "Field `REGION76` writer - Enable protection for region 76. Write '0' has no effect."]
pub type Region76W<'a, REG> = crate::BitWriter<'a, REG, Region76>;
impl<'a, REG> Region76W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region76::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region76::Enabled)
    }
}
#[doc = "Enable protection for region 77. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region77 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region77> for bool {
    #[inline(always)]
    fn from(variant: Region77) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION77` reader - Enable protection for region 77. Write '0' has no effect."]
pub type Region77R = crate::BitReader<Region77>;
impl Region77R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region77 {
        match self.bits {
            false => Region77::Disabled,
            true => Region77::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region77::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region77::Enabled
    }
}
#[doc = "Field `REGION77` writer - Enable protection for region 77. Write '0' has no effect."]
pub type Region77W<'a, REG> = crate::BitWriter<'a, REG, Region77>;
impl<'a, REG> Region77W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region77::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region77::Enabled)
    }
}
#[doc = "Enable protection for region 78. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region78 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region78> for bool {
    #[inline(always)]
    fn from(variant: Region78) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION78` reader - Enable protection for region 78. Write '0' has no effect."]
pub type Region78R = crate::BitReader<Region78>;
impl Region78R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region78 {
        match self.bits {
            false => Region78::Disabled,
            true => Region78::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region78::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region78::Enabled
    }
}
#[doc = "Field `REGION78` writer - Enable protection for region 78. Write '0' has no effect."]
pub type Region78W<'a, REG> = crate::BitWriter<'a, REG, Region78>;
impl<'a, REG> Region78W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region78::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region78::Enabled)
    }
}
#[doc = "Enable protection for region 79. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region79 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region79> for bool {
    #[inline(always)]
    fn from(variant: Region79) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION79` reader - Enable protection for region 79. Write '0' has no effect."]
pub type Region79R = crate::BitReader<Region79>;
impl Region79R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region79 {
        match self.bits {
            false => Region79::Disabled,
            true => Region79::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region79::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region79::Enabled
    }
}
#[doc = "Field `REGION79` writer - Enable protection for region 79. Write '0' has no effect."]
pub type Region79W<'a, REG> = crate::BitWriter<'a, REG, Region79>;
impl<'a, REG> Region79W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region79::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region79::Enabled)
    }
}
#[doc = "Enable protection for region 80. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region80 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region80> for bool {
    #[inline(always)]
    fn from(variant: Region80) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION80` reader - Enable protection for region 80. Write '0' has no effect."]
pub type Region80R = crate::BitReader<Region80>;
impl Region80R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region80 {
        match self.bits {
            false => Region80::Disabled,
            true => Region80::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region80::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region80::Enabled
    }
}
#[doc = "Field `REGION80` writer - Enable protection for region 80. Write '0' has no effect."]
pub type Region80W<'a, REG> = crate::BitWriter<'a, REG, Region80>;
impl<'a, REG> Region80W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region80::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region80::Enabled)
    }
}
#[doc = "Enable protection for region 81. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region81 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region81> for bool {
    #[inline(always)]
    fn from(variant: Region81) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION81` reader - Enable protection for region 81. Write '0' has no effect."]
pub type Region81R = crate::BitReader<Region81>;
impl Region81R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region81 {
        match self.bits {
            false => Region81::Disabled,
            true => Region81::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region81::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region81::Enabled
    }
}
#[doc = "Field `REGION81` writer - Enable protection for region 81. Write '0' has no effect."]
pub type Region81W<'a, REG> = crate::BitWriter<'a, REG, Region81>;
impl<'a, REG> Region81W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region81::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region81::Enabled)
    }
}
#[doc = "Enable protection for region 82. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region82 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region82> for bool {
    #[inline(always)]
    fn from(variant: Region82) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION82` reader - Enable protection for region 82. Write '0' has no effect."]
pub type Region82R = crate::BitReader<Region82>;
impl Region82R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region82 {
        match self.bits {
            false => Region82::Disabled,
            true => Region82::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region82::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region82::Enabled
    }
}
#[doc = "Field `REGION82` writer - Enable protection for region 82. Write '0' has no effect."]
pub type Region82W<'a, REG> = crate::BitWriter<'a, REG, Region82>;
impl<'a, REG> Region82W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region82::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region82::Enabled)
    }
}
#[doc = "Enable protection for region 83. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region83 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region83> for bool {
    #[inline(always)]
    fn from(variant: Region83) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION83` reader - Enable protection for region 83. Write '0' has no effect."]
pub type Region83R = crate::BitReader<Region83>;
impl Region83R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region83 {
        match self.bits {
            false => Region83::Disabled,
            true => Region83::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region83::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region83::Enabled
    }
}
#[doc = "Field `REGION83` writer - Enable protection for region 83. Write '0' has no effect."]
pub type Region83W<'a, REG> = crate::BitWriter<'a, REG, Region83>;
impl<'a, REG> Region83W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region83::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region83::Enabled)
    }
}
#[doc = "Enable protection for region 84. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region84 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region84> for bool {
    #[inline(always)]
    fn from(variant: Region84) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION84` reader - Enable protection for region 84. Write '0' has no effect."]
pub type Region84R = crate::BitReader<Region84>;
impl Region84R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region84 {
        match self.bits {
            false => Region84::Disabled,
            true => Region84::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region84::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region84::Enabled
    }
}
#[doc = "Field `REGION84` writer - Enable protection for region 84. Write '0' has no effect."]
pub type Region84W<'a, REG> = crate::BitWriter<'a, REG, Region84>;
impl<'a, REG> Region84W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region84::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region84::Enabled)
    }
}
#[doc = "Enable protection for region 85. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region85 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region85> for bool {
    #[inline(always)]
    fn from(variant: Region85) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION85` reader - Enable protection for region 85. Write '0' has no effect."]
pub type Region85R = crate::BitReader<Region85>;
impl Region85R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region85 {
        match self.bits {
            false => Region85::Disabled,
            true => Region85::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region85::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region85::Enabled
    }
}
#[doc = "Field `REGION85` writer - Enable protection for region 85. Write '0' has no effect."]
pub type Region85W<'a, REG> = crate::BitWriter<'a, REG, Region85>;
impl<'a, REG> Region85W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region85::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region85::Enabled)
    }
}
#[doc = "Enable protection for region 86. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region86 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region86> for bool {
    #[inline(always)]
    fn from(variant: Region86) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION86` reader - Enable protection for region 86. Write '0' has no effect."]
pub type Region86R = crate::BitReader<Region86>;
impl Region86R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region86 {
        match self.bits {
            false => Region86::Disabled,
            true => Region86::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region86::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region86::Enabled
    }
}
#[doc = "Field `REGION86` writer - Enable protection for region 86. Write '0' has no effect."]
pub type Region86W<'a, REG> = crate::BitWriter<'a, REG, Region86>;
impl<'a, REG> Region86W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region86::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region86::Enabled)
    }
}
#[doc = "Enable protection for region 87. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region87 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region87> for bool {
    #[inline(always)]
    fn from(variant: Region87) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION87` reader - Enable protection for region 87. Write '0' has no effect."]
pub type Region87R = crate::BitReader<Region87>;
impl Region87R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region87 {
        match self.bits {
            false => Region87::Disabled,
            true => Region87::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region87::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region87::Enabled
    }
}
#[doc = "Field `REGION87` writer - Enable protection for region 87. Write '0' has no effect."]
pub type Region87W<'a, REG> = crate::BitWriter<'a, REG, Region87>;
impl<'a, REG> Region87W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region87::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region87::Enabled)
    }
}
#[doc = "Enable protection for region 88. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region88 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region88> for bool {
    #[inline(always)]
    fn from(variant: Region88) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION88` reader - Enable protection for region 88. Write '0' has no effect."]
pub type Region88R = crate::BitReader<Region88>;
impl Region88R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region88 {
        match self.bits {
            false => Region88::Disabled,
            true => Region88::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region88::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region88::Enabled
    }
}
#[doc = "Field `REGION88` writer - Enable protection for region 88. Write '0' has no effect."]
pub type Region88W<'a, REG> = crate::BitWriter<'a, REG, Region88>;
impl<'a, REG> Region88W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region88::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region88::Enabled)
    }
}
#[doc = "Enable protection for region 89. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region89 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region89> for bool {
    #[inline(always)]
    fn from(variant: Region89) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION89` reader - Enable protection for region 89. Write '0' has no effect."]
pub type Region89R = crate::BitReader<Region89>;
impl Region89R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region89 {
        match self.bits {
            false => Region89::Disabled,
            true => Region89::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region89::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region89::Enabled
    }
}
#[doc = "Field `REGION89` writer - Enable protection for region 89. Write '0' has no effect."]
pub type Region89W<'a, REG> = crate::BitWriter<'a, REG, Region89>;
impl<'a, REG> Region89W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region89::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region89::Enabled)
    }
}
#[doc = "Enable protection for region 90. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region90 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region90> for bool {
    #[inline(always)]
    fn from(variant: Region90) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION90` reader - Enable protection for region 90. Write '0' has no effect."]
pub type Region90R = crate::BitReader<Region90>;
impl Region90R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region90 {
        match self.bits {
            false => Region90::Disabled,
            true => Region90::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region90::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region90::Enabled
    }
}
#[doc = "Field `REGION90` writer - Enable protection for region 90. Write '0' has no effect."]
pub type Region90W<'a, REG> = crate::BitWriter<'a, REG, Region90>;
impl<'a, REG> Region90W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region90::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region90::Enabled)
    }
}
#[doc = "Enable protection for region 91. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region91 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region91> for bool {
    #[inline(always)]
    fn from(variant: Region91) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION91` reader - Enable protection for region 91. Write '0' has no effect."]
pub type Region91R = crate::BitReader<Region91>;
impl Region91R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region91 {
        match self.bits {
            false => Region91::Disabled,
            true => Region91::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region91::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region91::Enabled
    }
}
#[doc = "Field `REGION91` writer - Enable protection for region 91. Write '0' has no effect."]
pub type Region91W<'a, REG> = crate::BitWriter<'a, REG, Region91>;
impl<'a, REG> Region91W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region91::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region91::Enabled)
    }
}
#[doc = "Enable protection for region 92. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region92 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region92> for bool {
    #[inline(always)]
    fn from(variant: Region92) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION92` reader - Enable protection for region 92. Write '0' has no effect."]
pub type Region92R = crate::BitReader<Region92>;
impl Region92R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region92 {
        match self.bits {
            false => Region92::Disabled,
            true => Region92::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region92::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region92::Enabled
    }
}
#[doc = "Field `REGION92` writer - Enable protection for region 92. Write '0' has no effect."]
pub type Region92W<'a, REG> = crate::BitWriter<'a, REG, Region92>;
impl<'a, REG> Region92W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region92::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region92::Enabled)
    }
}
#[doc = "Enable protection for region 93. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region93 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region93> for bool {
    #[inline(always)]
    fn from(variant: Region93) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION93` reader - Enable protection for region 93. Write '0' has no effect."]
pub type Region93R = crate::BitReader<Region93>;
impl Region93R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region93 {
        match self.bits {
            false => Region93::Disabled,
            true => Region93::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region93::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region93::Enabled
    }
}
#[doc = "Field `REGION93` writer - Enable protection for region 93. Write '0' has no effect."]
pub type Region93W<'a, REG> = crate::BitWriter<'a, REG, Region93>;
impl<'a, REG> Region93W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region93::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region93::Enabled)
    }
}
#[doc = "Enable protection for region 94. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region94 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region94> for bool {
    #[inline(always)]
    fn from(variant: Region94) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION94` reader - Enable protection for region 94. Write '0' has no effect."]
pub type Region94R = crate::BitReader<Region94>;
impl Region94R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region94 {
        match self.bits {
            false => Region94::Disabled,
            true => Region94::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region94::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region94::Enabled
    }
}
#[doc = "Field `REGION94` writer - Enable protection for region 94. Write '0' has no effect."]
pub type Region94W<'a, REG> = crate::BitWriter<'a, REG, Region94>;
impl<'a, REG> Region94W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region94::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region94::Enabled)
    }
}
#[doc = "Enable protection for region 95. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region95 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region95> for bool {
    #[inline(always)]
    fn from(variant: Region95) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION95` reader - Enable protection for region 95. Write '0' has no effect."]
pub type Region95R = crate::BitReader<Region95>;
impl Region95R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region95 {
        match self.bits {
            false => Region95::Disabled,
            true => Region95::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region95::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region95::Enabled
    }
}
#[doc = "Field `REGION95` writer - Enable protection for region 95. Write '0' has no effect."]
pub type Region95W<'a, REG> = crate::BitWriter<'a, REG, Region95>;
impl<'a, REG> Region95W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region95::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region95::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&self) -> Region64R {
        Region64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&self) -> Region65R {
        Region65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&self) -> Region66R {
        Region66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&self) -> Region67R {
        Region67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&self) -> Region68R {
        Region68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&self) -> Region69R {
        Region69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&self) -> Region70R {
        Region70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&self) -> Region71R {
        Region71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&self) -> Region72R {
        Region72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&self) -> Region73R {
        Region73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&self) -> Region74R {
        Region74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&self) -> Region75R {
        Region75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&self) -> Region76R {
        Region76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&self) -> Region77R {
        Region77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&self) -> Region78R {
        Region78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&self) -> Region79R {
        Region79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&self) -> Region80R {
        Region80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&self) -> Region81R {
        Region81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&self) -> Region82R {
        Region82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&self) -> Region83R {
        Region83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&self) -> Region84R {
        Region84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&self) -> Region85R {
        Region85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&self) -> Region86R {
        Region86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&self) -> Region87R {
        Region87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&self) -> Region88R {
        Region88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&self) -> Region89R {
        Region89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&self) -> Region90R {
        Region90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&self) -> Region91R {
        Region91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&self) -> Region92R {
        Region92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&self) -> Region93R {
        Region93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&self) -> Region94R {
        Region94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&self) -> Region95R {
        Region95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&mut self) -> Region64W<'_, Config2Spec> {
        Region64W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&mut self) -> Region65W<'_, Config2Spec> {
        Region65W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&mut self) -> Region66W<'_, Config2Spec> {
        Region66W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&mut self) -> Region67W<'_, Config2Spec> {
        Region67W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&mut self) -> Region68W<'_, Config2Spec> {
        Region68W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&mut self) -> Region69W<'_, Config2Spec> {
        Region69W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&mut self) -> Region70W<'_, Config2Spec> {
        Region70W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&mut self) -> Region71W<'_, Config2Spec> {
        Region71W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&mut self) -> Region72W<'_, Config2Spec> {
        Region72W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&mut self) -> Region73W<'_, Config2Spec> {
        Region73W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&mut self) -> Region74W<'_, Config2Spec> {
        Region74W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&mut self) -> Region75W<'_, Config2Spec> {
        Region75W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&mut self) -> Region76W<'_, Config2Spec> {
        Region76W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&mut self) -> Region77W<'_, Config2Spec> {
        Region77W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&mut self) -> Region78W<'_, Config2Spec> {
        Region78W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&mut self) -> Region79W<'_, Config2Spec> {
        Region79W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&mut self) -> Region80W<'_, Config2Spec> {
        Region80W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&mut self) -> Region81W<'_, Config2Spec> {
        Region81W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&mut self) -> Region82W<'_, Config2Spec> {
        Region82W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&mut self) -> Region83W<'_, Config2Spec> {
        Region83W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&mut self) -> Region84W<'_, Config2Spec> {
        Region84W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&mut self) -> Region85W<'_, Config2Spec> {
        Region85W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&mut self) -> Region86W<'_, Config2Spec> {
        Region86W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&mut self) -> Region87W<'_, Config2Spec> {
        Region87W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&mut self) -> Region88W<'_, Config2Spec> {
        Region88W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&mut self) -> Region89W<'_, Config2Spec> {
        Region89W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&mut self) -> Region90W<'_, Config2Spec> {
        Region90W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&mut self) -> Region91W<'_, Config2Spec> {
        Region91W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&mut self) -> Region92W<'_, Config2Spec> {
        Region92W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&mut self) -> Region93W<'_, Config2Spec> {
        Region93W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&mut self) -> Region94W<'_, Config2Spec> {
        Region94W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&mut self) -> Region95W<'_, Config2Spec> {
        Region95W::new(self, 31)
    }
}
#[doc = "Block protect configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config2Spec;
impl crate::RegisterSpec for Config2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config2::R`](R) reader structure"]
impl crate::Readable for Config2Spec {}
#[doc = "`write(|w| ..)` method takes [`config2::W`](W) writer structure"]
impl crate::Writable for Config2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for Config2Spec {}
