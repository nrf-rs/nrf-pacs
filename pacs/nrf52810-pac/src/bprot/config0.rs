#[doc = "Register `CONFIG0` reader"]
pub type R = crate::R<Config0Spec>;
#[doc = "Register `CONFIG0` writer"]
pub type W = crate::W<Config0Spec>;
#[doc = "Enable protection for region 0. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region0> for bool {
    #[inline(always)]
    fn from(variant: Region0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0` reader - Enable protection for region 0. Write '0' has no effect."]
pub type Region0R = crate::BitReader<Region0>;
impl Region0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region0 {
        match self.bits {
            false => Region0::Disabled,
            true => Region0::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0::Enabled
    }
}
#[doc = "Field `REGION0` writer - Enable protection for region 0. Write '0' has no effect."]
pub type Region0W<'a, REG> = crate::BitWriter<'a, REG, Region0>;
impl<'a, REG> Region0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0::Enabled)
    }
}
#[doc = "Enable protection for region 1. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region1> for bool {
    #[inline(always)]
    fn from(variant: Region1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1` reader - Enable protection for region 1. Write '0' has no effect."]
pub type Region1R = crate::BitReader<Region1>;
impl Region1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region1 {
        match self.bits {
            false => Region1::Disabled,
            true => Region1::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1::Enabled
    }
}
#[doc = "Field `REGION1` writer - Enable protection for region 1. Write '0' has no effect."]
pub type Region1W<'a, REG> = crate::BitWriter<'a, REG, Region1>;
impl<'a, REG> Region1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1::Enabled)
    }
}
#[doc = "Enable protection for region 2. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region2> for bool {
    #[inline(always)]
    fn from(variant: Region2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2` reader - Enable protection for region 2. Write '0' has no effect."]
pub type Region2R = crate::BitReader<Region2>;
impl Region2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region2 {
        match self.bits {
            false => Region2::Disabled,
            true => Region2::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2::Enabled
    }
}
#[doc = "Field `REGION2` writer - Enable protection for region 2. Write '0' has no effect."]
pub type Region2W<'a, REG> = crate::BitWriter<'a, REG, Region2>;
impl<'a, REG> Region2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2::Enabled)
    }
}
#[doc = "Enable protection for region 3. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region3> for bool {
    #[inline(always)]
    fn from(variant: Region3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3` reader - Enable protection for region 3. Write '0' has no effect."]
pub type Region3R = crate::BitReader<Region3>;
impl Region3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region3 {
        match self.bits {
            false => Region3::Disabled,
            true => Region3::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3::Enabled
    }
}
#[doc = "Field `REGION3` writer - Enable protection for region 3. Write '0' has no effect."]
pub type Region3W<'a, REG> = crate::BitWriter<'a, REG, Region3>;
impl<'a, REG> Region3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3::Enabled)
    }
}
#[doc = "Enable protection for region 4. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region4 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region4> for bool {
    #[inline(always)]
    fn from(variant: Region4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION4` reader - Enable protection for region 4. Write '0' has no effect."]
pub type Region4R = crate::BitReader<Region4>;
impl Region4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region4 {
        match self.bits {
            false => Region4::Disabled,
            true => Region4::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region4::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region4::Enabled
    }
}
#[doc = "Field `REGION4` writer - Enable protection for region 4. Write '0' has no effect."]
pub type Region4W<'a, REG> = crate::BitWriter<'a, REG, Region4>;
impl<'a, REG> Region4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region4::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region4::Enabled)
    }
}
#[doc = "Enable protection for region 5. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region5 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region5> for bool {
    #[inline(always)]
    fn from(variant: Region5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION5` reader - Enable protection for region 5. Write '0' has no effect."]
pub type Region5R = crate::BitReader<Region5>;
impl Region5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region5 {
        match self.bits {
            false => Region5::Disabled,
            true => Region5::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region5::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region5::Enabled
    }
}
#[doc = "Field `REGION5` writer - Enable protection for region 5. Write '0' has no effect."]
pub type Region5W<'a, REG> = crate::BitWriter<'a, REG, Region5>;
impl<'a, REG> Region5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region5::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region5::Enabled)
    }
}
#[doc = "Enable protection for region 6. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region6 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region6> for bool {
    #[inline(always)]
    fn from(variant: Region6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION6` reader - Enable protection for region 6. Write '0' has no effect."]
pub type Region6R = crate::BitReader<Region6>;
impl Region6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region6 {
        match self.bits {
            false => Region6::Disabled,
            true => Region6::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region6::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region6::Enabled
    }
}
#[doc = "Field `REGION6` writer - Enable protection for region 6. Write '0' has no effect."]
pub type Region6W<'a, REG> = crate::BitWriter<'a, REG, Region6>;
impl<'a, REG> Region6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region6::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region6::Enabled)
    }
}
#[doc = "Enable protection for region 7. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region7 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region7> for bool {
    #[inline(always)]
    fn from(variant: Region7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION7` reader - Enable protection for region 7. Write '0' has no effect."]
pub type Region7R = crate::BitReader<Region7>;
impl Region7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region7 {
        match self.bits {
            false => Region7::Disabled,
            true => Region7::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region7::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region7::Enabled
    }
}
#[doc = "Field `REGION7` writer - Enable protection for region 7. Write '0' has no effect."]
pub type Region7W<'a, REG> = crate::BitWriter<'a, REG, Region7>;
impl<'a, REG> Region7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region7::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region7::Enabled)
    }
}
#[doc = "Enable protection for region 8. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region8 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region8> for bool {
    #[inline(always)]
    fn from(variant: Region8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION8` reader - Enable protection for region 8. Write '0' has no effect."]
pub type Region8R = crate::BitReader<Region8>;
impl Region8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region8 {
        match self.bits {
            false => Region8::Disabled,
            true => Region8::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region8::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region8::Enabled
    }
}
#[doc = "Field `REGION8` writer - Enable protection for region 8. Write '0' has no effect."]
pub type Region8W<'a, REG> = crate::BitWriter<'a, REG, Region8>;
impl<'a, REG> Region8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region8::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region8::Enabled)
    }
}
#[doc = "Enable protection for region 9. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region9 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region9> for bool {
    #[inline(always)]
    fn from(variant: Region9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION9` reader - Enable protection for region 9. Write '0' has no effect."]
pub type Region9R = crate::BitReader<Region9>;
impl Region9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region9 {
        match self.bits {
            false => Region9::Disabled,
            true => Region9::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region9::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region9::Enabled
    }
}
#[doc = "Field `REGION9` writer - Enable protection for region 9. Write '0' has no effect."]
pub type Region9W<'a, REG> = crate::BitWriter<'a, REG, Region9>;
impl<'a, REG> Region9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region9::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region9::Enabled)
    }
}
#[doc = "Enable protection for region 10. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region10 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region10> for bool {
    #[inline(always)]
    fn from(variant: Region10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION10` reader - Enable protection for region 10. Write '0' has no effect."]
pub type Region10R = crate::BitReader<Region10>;
impl Region10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region10 {
        match self.bits {
            false => Region10::Disabled,
            true => Region10::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region10::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region10::Enabled
    }
}
#[doc = "Field `REGION10` writer - Enable protection for region 10. Write '0' has no effect."]
pub type Region10W<'a, REG> = crate::BitWriter<'a, REG, Region10>;
impl<'a, REG> Region10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region10::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region10::Enabled)
    }
}
#[doc = "Enable protection for region 11. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region11 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region11> for bool {
    #[inline(always)]
    fn from(variant: Region11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION11` reader - Enable protection for region 11. Write '0' has no effect."]
pub type Region11R = crate::BitReader<Region11>;
impl Region11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region11 {
        match self.bits {
            false => Region11::Disabled,
            true => Region11::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region11::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region11::Enabled
    }
}
#[doc = "Field `REGION11` writer - Enable protection for region 11. Write '0' has no effect."]
pub type Region11W<'a, REG> = crate::BitWriter<'a, REG, Region11>;
impl<'a, REG> Region11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region11::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region11::Enabled)
    }
}
#[doc = "Enable protection for region 12. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region12 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region12> for bool {
    #[inline(always)]
    fn from(variant: Region12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION12` reader - Enable protection for region 12. Write '0' has no effect."]
pub type Region12R = crate::BitReader<Region12>;
impl Region12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region12 {
        match self.bits {
            false => Region12::Disabled,
            true => Region12::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region12::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region12::Enabled
    }
}
#[doc = "Field `REGION12` writer - Enable protection for region 12. Write '0' has no effect."]
pub type Region12W<'a, REG> = crate::BitWriter<'a, REG, Region12>;
impl<'a, REG> Region12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region12::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region12::Enabled)
    }
}
#[doc = "Enable protection for region 13. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region13 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region13> for bool {
    #[inline(always)]
    fn from(variant: Region13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION13` reader - Enable protection for region 13. Write '0' has no effect."]
pub type Region13R = crate::BitReader<Region13>;
impl Region13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region13 {
        match self.bits {
            false => Region13::Disabled,
            true => Region13::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region13::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region13::Enabled
    }
}
#[doc = "Field `REGION13` writer - Enable protection for region 13. Write '0' has no effect."]
pub type Region13W<'a, REG> = crate::BitWriter<'a, REG, Region13>;
impl<'a, REG> Region13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region13::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region13::Enabled)
    }
}
#[doc = "Enable protection for region 14. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region14 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region14> for bool {
    #[inline(always)]
    fn from(variant: Region14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION14` reader - Enable protection for region 14. Write '0' has no effect."]
pub type Region14R = crate::BitReader<Region14>;
impl Region14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region14 {
        match self.bits {
            false => Region14::Disabled,
            true => Region14::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region14::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region14::Enabled
    }
}
#[doc = "Field `REGION14` writer - Enable protection for region 14. Write '0' has no effect."]
pub type Region14W<'a, REG> = crate::BitWriter<'a, REG, Region14>;
impl<'a, REG> Region14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region14::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region14::Enabled)
    }
}
#[doc = "Enable protection for region 15. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region15 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region15> for bool {
    #[inline(always)]
    fn from(variant: Region15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION15` reader - Enable protection for region 15. Write '0' has no effect."]
pub type Region15R = crate::BitReader<Region15>;
impl Region15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region15 {
        match self.bits {
            false => Region15::Disabled,
            true => Region15::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region15::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region15::Enabled
    }
}
#[doc = "Field `REGION15` writer - Enable protection for region 15. Write '0' has no effect."]
pub type Region15W<'a, REG> = crate::BitWriter<'a, REG, Region15>;
impl<'a, REG> Region15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region15::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region15::Enabled)
    }
}
#[doc = "Enable protection for region 16. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region16 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region16> for bool {
    #[inline(always)]
    fn from(variant: Region16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION16` reader - Enable protection for region 16. Write '0' has no effect."]
pub type Region16R = crate::BitReader<Region16>;
impl Region16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region16 {
        match self.bits {
            false => Region16::Disabled,
            true => Region16::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region16::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region16::Enabled
    }
}
#[doc = "Field `REGION16` writer - Enable protection for region 16. Write '0' has no effect."]
pub type Region16W<'a, REG> = crate::BitWriter<'a, REG, Region16>;
impl<'a, REG> Region16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region16::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region16::Enabled)
    }
}
#[doc = "Enable protection for region 17. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region17 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region17> for bool {
    #[inline(always)]
    fn from(variant: Region17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION17` reader - Enable protection for region 17. Write '0' has no effect."]
pub type Region17R = crate::BitReader<Region17>;
impl Region17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region17 {
        match self.bits {
            false => Region17::Disabled,
            true => Region17::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region17::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region17::Enabled
    }
}
#[doc = "Field `REGION17` writer - Enable protection for region 17. Write '0' has no effect."]
pub type Region17W<'a, REG> = crate::BitWriter<'a, REG, Region17>;
impl<'a, REG> Region17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region17::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region17::Enabled)
    }
}
#[doc = "Enable protection for region 18. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region18 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region18> for bool {
    #[inline(always)]
    fn from(variant: Region18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION18` reader - Enable protection for region 18. Write '0' has no effect."]
pub type Region18R = crate::BitReader<Region18>;
impl Region18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region18 {
        match self.bits {
            false => Region18::Disabled,
            true => Region18::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region18::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region18::Enabled
    }
}
#[doc = "Field `REGION18` writer - Enable protection for region 18. Write '0' has no effect."]
pub type Region18W<'a, REG> = crate::BitWriter<'a, REG, Region18>;
impl<'a, REG> Region18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region18::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region18::Enabled)
    }
}
#[doc = "Enable protection for region 19. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region19 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region19> for bool {
    #[inline(always)]
    fn from(variant: Region19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION19` reader - Enable protection for region 19. Write '0' has no effect."]
pub type Region19R = crate::BitReader<Region19>;
impl Region19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region19 {
        match self.bits {
            false => Region19::Disabled,
            true => Region19::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region19::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region19::Enabled
    }
}
#[doc = "Field `REGION19` writer - Enable protection for region 19. Write '0' has no effect."]
pub type Region19W<'a, REG> = crate::BitWriter<'a, REG, Region19>;
impl<'a, REG> Region19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region19::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region19::Enabled)
    }
}
#[doc = "Enable protection for region 20. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region20 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region20> for bool {
    #[inline(always)]
    fn from(variant: Region20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION20` reader - Enable protection for region 20. Write '0' has no effect."]
pub type Region20R = crate::BitReader<Region20>;
impl Region20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region20 {
        match self.bits {
            false => Region20::Disabled,
            true => Region20::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region20::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region20::Enabled
    }
}
#[doc = "Field `REGION20` writer - Enable protection for region 20. Write '0' has no effect."]
pub type Region20W<'a, REG> = crate::BitWriter<'a, REG, Region20>;
impl<'a, REG> Region20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region20::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region20::Enabled)
    }
}
#[doc = "Enable protection for region 21. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region21 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region21> for bool {
    #[inline(always)]
    fn from(variant: Region21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION21` reader - Enable protection for region 21. Write '0' has no effect."]
pub type Region21R = crate::BitReader<Region21>;
impl Region21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region21 {
        match self.bits {
            false => Region21::Disabled,
            true => Region21::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region21::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region21::Enabled
    }
}
#[doc = "Field `REGION21` writer - Enable protection for region 21. Write '0' has no effect."]
pub type Region21W<'a, REG> = crate::BitWriter<'a, REG, Region21>;
impl<'a, REG> Region21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region21::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region21::Enabled)
    }
}
#[doc = "Enable protection for region 22. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region22 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region22> for bool {
    #[inline(always)]
    fn from(variant: Region22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION22` reader - Enable protection for region 22. Write '0' has no effect."]
pub type Region22R = crate::BitReader<Region22>;
impl Region22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region22 {
        match self.bits {
            false => Region22::Disabled,
            true => Region22::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region22::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region22::Enabled
    }
}
#[doc = "Field `REGION22` writer - Enable protection for region 22. Write '0' has no effect."]
pub type Region22W<'a, REG> = crate::BitWriter<'a, REG, Region22>;
impl<'a, REG> Region22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region22::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region22::Enabled)
    }
}
#[doc = "Enable protection for region 23. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region23 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region23> for bool {
    #[inline(always)]
    fn from(variant: Region23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION23` reader - Enable protection for region 23. Write '0' has no effect."]
pub type Region23R = crate::BitReader<Region23>;
impl Region23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region23 {
        match self.bits {
            false => Region23::Disabled,
            true => Region23::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region23::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region23::Enabled
    }
}
#[doc = "Field `REGION23` writer - Enable protection for region 23. Write '0' has no effect."]
pub type Region23W<'a, REG> = crate::BitWriter<'a, REG, Region23>;
impl<'a, REG> Region23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region23::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region23::Enabled)
    }
}
#[doc = "Enable protection for region 24. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region24 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region24> for bool {
    #[inline(always)]
    fn from(variant: Region24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION24` reader - Enable protection for region 24. Write '0' has no effect."]
pub type Region24R = crate::BitReader<Region24>;
impl Region24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region24 {
        match self.bits {
            false => Region24::Disabled,
            true => Region24::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region24::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region24::Enabled
    }
}
#[doc = "Field `REGION24` writer - Enable protection for region 24. Write '0' has no effect."]
pub type Region24W<'a, REG> = crate::BitWriter<'a, REG, Region24>;
impl<'a, REG> Region24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region24::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region24::Enabled)
    }
}
#[doc = "Enable protection for region 25. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region25 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region25> for bool {
    #[inline(always)]
    fn from(variant: Region25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION25` reader - Enable protection for region 25. Write '0' has no effect."]
pub type Region25R = crate::BitReader<Region25>;
impl Region25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region25 {
        match self.bits {
            false => Region25::Disabled,
            true => Region25::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region25::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region25::Enabled
    }
}
#[doc = "Field `REGION25` writer - Enable protection for region 25. Write '0' has no effect."]
pub type Region25W<'a, REG> = crate::BitWriter<'a, REG, Region25>;
impl<'a, REG> Region25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region25::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region25::Enabled)
    }
}
#[doc = "Enable protection for region 26. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region26 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region26> for bool {
    #[inline(always)]
    fn from(variant: Region26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION26` reader - Enable protection for region 26. Write '0' has no effect."]
pub type Region26R = crate::BitReader<Region26>;
impl Region26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region26 {
        match self.bits {
            false => Region26::Disabled,
            true => Region26::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region26::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region26::Enabled
    }
}
#[doc = "Field `REGION26` writer - Enable protection for region 26. Write '0' has no effect."]
pub type Region26W<'a, REG> = crate::BitWriter<'a, REG, Region26>;
impl<'a, REG> Region26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region26::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region26::Enabled)
    }
}
#[doc = "Enable protection for region 27. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region27 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region27> for bool {
    #[inline(always)]
    fn from(variant: Region27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION27` reader - Enable protection for region 27. Write '0' has no effect."]
pub type Region27R = crate::BitReader<Region27>;
impl Region27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region27 {
        match self.bits {
            false => Region27::Disabled,
            true => Region27::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region27::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region27::Enabled
    }
}
#[doc = "Field `REGION27` writer - Enable protection for region 27. Write '0' has no effect."]
pub type Region27W<'a, REG> = crate::BitWriter<'a, REG, Region27>;
impl<'a, REG> Region27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region27::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region27::Enabled)
    }
}
#[doc = "Enable protection for region 28. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region28 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region28> for bool {
    #[inline(always)]
    fn from(variant: Region28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION28` reader - Enable protection for region 28. Write '0' has no effect."]
pub type Region28R = crate::BitReader<Region28>;
impl Region28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region28 {
        match self.bits {
            false => Region28::Disabled,
            true => Region28::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region28::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region28::Enabled
    }
}
#[doc = "Field `REGION28` writer - Enable protection for region 28. Write '0' has no effect."]
pub type Region28W<'a, REG> = crate::BitWriter<'a, REG, Region28>;
impl<'a, REG> Region28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region28::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region28::Enabled)
    }
}
#[doc = "Enable protection for region 29. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region29 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region29> for bool {
    #[inline(always)]
    fn from(variant: Region29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION29` reader - Enable protection for region 29. Write '0' has no effect."]
pub type Region29R = crate::BitReader<Region29>;
impl Region29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region29 {
        match self.bits {
            false => Region29::Disabled,
            true => Region29::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region29::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region29::Enabled
    }
}
#[doc = "Field `REGION29` writer - Enable protection for region 29. Write '0' has no effect."]
pub type Region29W<'a, REG> = crate::BitWriter<'a, REG, Region29>;
impl<'a, REG> Region29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region29::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region29::Enabled)
    }
}
#[doc = "Enable protection for region 30. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region30 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region30> for bool {
    #[inline(always)]
    fn from(variant: Region30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION30` reader - Enable protection for region 30. Write '0' has no effect."]
pub type Region30R = crate::BitReader<Region30>;
impl Region30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region30 {
        match self.bits {
            false => Region30::Disabled,
            true => Region30::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region30::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region30::Enabled
    }
}
#[doc = "Field `REGION30` writer - Enable protection for region 30. Write '0' has no effect."]
pub type Region30W<'a, REG> = crate::BitWriter<'a, REG, Region30>;
impl<'a, REG> Region30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region30::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region30::Enabled)
    }
}
#[doc = "Enable protection for region 31. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region31 {
    #[doc = "0: Protection disabled"]
    Disabled = 0,
    #[doc = "1: Protection enabled"]
    Enabled = 1,
}
impl From<Region31> for bool {
    #[inline(always)]
    fn from(variant: Region31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION31` reader - Enable protection for region 31. Write '0' has no effect."]
pub type Region31R = crate::BitReader<Region31>;
impl Region31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region31 {
        match self.bits {
            false => Region31::Disabled,
            true => Region31::Enabled,
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region31::Disabled
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region31::Enabled
    }
}
#[doc = "Field `REGION31` writer - Enable protection for region 31. Write '0' has no effect."]
pub type Region31W<'a, REG> = crate::BitWriter<'a, REG, Region31>;
impl<'a, REG> Region31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region31::Disabled)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region31::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&self) -> Region0R {
        Region0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&self) -> Region1R {
        Region1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&self) -> Region2R {
        Region2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&self) -> Region3R {
        Region3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&self) -> Region4R {
        Region4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&self) -> Region5R {
        Region5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&self) -> Region6R {
        Region6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&self) -> Region7R {
        Region7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&self) -> Region8R {
        Region8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&self) -> Region9R {
        Region9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&self) -> Region10R {
        Region10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&self) -> Region11R {
        Region11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&self) -> Region12R {
        Region12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&self) -> Region13R {
        Region13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&self) -> Region14R {
        Region14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&self) -> Region15R {
        Region15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&self) -> Region16R {
        Region16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&self) -> Region17R {
        Region17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&self) -> Region18R {
        Region18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&self) -> Region19R {
        Region19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&self) -> Region20R {
        Region20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&self) -> Region21R {
        Region21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&self) -> Region22R {
        Region22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&self) -> Region23R {
        Region23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&self) -> Region24R {
        Region24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&self) -> Region25R {
        Region25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&self) -> Region26R {
        Region26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&self) -> Region27R {
        Region27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&self) -> Region28R {
        Region28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&self) -> Region29R {
        Region29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&self) -> Region30R {
        Region30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&self) -> Region31R {
        Region31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&mut self) -> Region0W<'_, Config0Spec> {
        Region0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&mut self) -> Region1W<'_, Config0Spec> {
        Region1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&mut self) -> Region2W<'_, Config0Spec> {
        Region2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&mut self) -> Region3W<'_, Config0Spec> {
        Region3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&mut self) -> Region4W<'_, Config0Spec> {
        Region4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&mut self) -> Region5W<'_, Config0Spec> {
        Region5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&mut self) -> Region6W<'_, Config0Spec> {
        Region6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&mut self) -> Region7W<'_, Config0Spec> {
        Region7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&mut self) -> Region8W<'_, Config0Spec> {
        Region8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&mut self) -> Region9W<'_, Config0Spec> {
        Region9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&mut self) -> Region10W<'_, Config0Spec> {
        Region10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&mut self) -> Region11W<'_, Config0Spec> {
        Region11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&mut self) -> Region12W<'_, Config0Spec> {
        Region12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&mut self) -> Region13W<'_, Config0Spec> {
        Region13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&mut self) -> Region14W<'_, Config0Spec> {
        Region14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&mut self) -> Region15W<'_, Config0Spec> {
        Region15W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&mut self) -> Region16W<'_, Config0Spec> {
        Region16W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&mut self) -> Region17W<'_, Config0Spec> {
        Region17W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&mut self) -> Region18W<'_, Config0Spec> {
        Region18W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&mut self) -> Region19W<'_, Config0Spec> {
        Region19W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&mut self) -> Region20W<'_, Config0Spec> {
        Region20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&mut self) -> Region21W<'_, Config0Spec> {
        Region21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&mut self) -> Region22W<'_, Config0Spec> {
        Region22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&mut self) -> Region23W<'_, Config0Spec> {
        Region23W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&mut self) -> Region24W<'_, Config0Spec> {
        Region24W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&mut self) -> Region25W<'_, Config0Spec> {
        Region25W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&mut self) -> Region26W<'_, Config0Spec> {
        Region26W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&mut self) -> Region27W<'_, Config0Spec> {
        Region27W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&mut self) -> Region28W<'_, Config0Spec> {
        Region28W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&mut self) -> Region29W<'_, Config0Spec> {
        Region29W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&mut self) -> Region30W<'_, Config0Spec> {
        Region30W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&mut self) -> Region31W<'_, Config0Spec> {
        Region31W::new(self, 31)
    }
}
#[doc = "Block protect configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config0Spec;
impl crate::RegisterSpec for Config0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config0::R`](R) reader structure"]
impl crate::Readable for Config0Spec {}
#[doc = "`write(|w| ..)` method takes [`config0::W`](W) writer structure"]
impl crate::Writable for Config0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG0 to value 0"]
impl crate::Resettable for Config0Spec {}
