#[doc = "Register `CHG[%s]` reader"]
pub type R = crate::R<ChgSpec>;
#[doc = "Register `CHG[%s]` writer"]
pub type W = crate::W<ChgSpec>;
#[doc = "Include or exclude channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Include or exclude channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Excluded,
            true => Ch0::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch0::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch0::Included
    }
}
#[doc = "Field `CH0` writer - Include or exclude channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Included)
    }
}
#[doc = "Include or exclude channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Include or exclude channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Excluded,
            true => Ch1::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch1::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch1::Included
    }
}
#[doc = "Field `CH1` writer - Include or exclude channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Included)
    }
}
#[doc = "Include or exclude channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Include or exclude channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Excluded,
            true => Ch2::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch2::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch2::Included
    }
}
#[doc = "Field `CH2` writer - Include or exclude channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Included)
    }
}
#[doc = "Include or exclude channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Include or exclude channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Excluded,
            true => Ch3::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch3::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch3::Included
    }
}
#[doc = "Field `CH3` writer - Include or exclude channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Included)
    }
}
#[doc = "Include or exclude channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Include or exclude channel 4"]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            false => Ch4::Excluded,
            true => Ch4::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch4::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch4::Included
    }
}
#[doc = "Field `CH4` writer - Include or exclude channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Included)
    }
}
#[doc = "Include or exclude channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Include or exclude channel 5"]
pub type Ch5R = crate::BitReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            false => Ch5::Excluded,
            true => Ch5::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch5::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch5::Included
    }
}
#[doc = "Field `CH5` writer - Include or exclude channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG, Ch5>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Included)
    }
}
#[doc = "Include or exclude channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Include or exclude channel 6"]
pub type Ch6R = crate::BitReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            false => Ch6::Excluded,
            true => Ch6::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch6::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch6::Included
    }
}
#[doc = "Field `CH6` writer - Include or exclude channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG, Ch6>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Included)
    }
}
#[doc = "Include or exclude channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Include or exclude channel 7"]
pub type Ch7R = crate::BitReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            false => Ch7::Excluded,
            true => Ch7::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch7::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch7::Included
    }
}
#[doc = "Field `CH7` writer - Include or exclude channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG, Ch7>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Included)
    }
}
#[doc = "Include or exclude channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch8 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch8> for bool {
    #[inline(always)]
    fn from(variant: Ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Include or exclude channel 8"]
pub type Ch8R = crate::BitReader<Ch8>;
impl Ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch8 {
        match self.bits {
            false => Ch8::Excluded,
            true => Ch8::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch8::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch8::Included
    }
}
#[doc = "Field `CH8` writer - Include or exclude channel 8"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG, Ch8>;
impl<'a, REG> Ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Included)
    }
}
#[doc = "Include or exclude channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch9 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch9> for bool {
    #[inline(always)]
    fn from(variant: Ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Include or exclude channel 9"]
pub type Ch9R = crate::BitReader<Ch9>;
impl Ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch9 {
        match self.bits {
            false => Ch9::Excluded,
            true => Ch9::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch9::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch9::Included
    }
}
#[doc = "Field `CH9` writer - Include or exclude channel 9"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG, Ch9>;
impl<'a, REG> Ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Included)
    }
}
#[doc = "Include or exclude channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch10 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch10> for bool {
    #[inline(always)]
    fn from(variant: Ch10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` reader - Include or exclude channel 10"]
pub type Ch10R = crate::BitReader<Ch10>;
impl Ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch10 {
        match self.bits {
            false => Ch10::Excluded,
            true => Ch10::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch10::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch10::Included
    }
}
#[doc = "Field `CH10` writer - Include or exclude channel 10"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG, Ch10>;
impl<'a, REG> Ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Included)
    }
}
#[doc = "Include or exclude channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch11 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch11> for bool {
    #[inline(always)]
    fn from(variant: Ch11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` reader - Include or exclude channel 11"]
pub type Ch11R = crate::BitReader<Ch11>;
impl Ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch11 {
        match self.bits {
            false => Ch11::Excluded,
            true => Ch11::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch11::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch11::Included
    }
}
#[doc = "Field `CH11` writer - Include or exclude channel 11"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG, Ch11>;
impl<'a, REG> Ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Included)
    }
}
#[doc = "Include or exclude channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch12 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch12> for bool {
    #[inline(always)]
    fn from(variant: Ch12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` reader - Include or exclude channel 12"]
pub type Ch12R = crate::BitReader<Ch12>;
impl Ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch12 {
        match self.bits {
            false => Ch12::Excluded,
            true => Ch12::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch12::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch12::Included
    }
}
#[doc = "Field `CH12` writer - Include or exclude channel 12"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG, Ch12>;
impl<'a, REG> Ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Included)
    }
}
#[doc = "Include or exclude channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch13 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch13> for bool {
    #[inline(always)]
    fn from(variant: Ch13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` reader - Include or exclude channel 13"]
pub type Ch13R = crate::BitReader<Ch13>;
impl Ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch13 {
        match self.bits {
            false => Ch13::Excluded,
            true => Ch13::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch13::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch13::Included
    }
}
#[doc = "Field `CH13` writer - Include or exclude channel 13"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG, Ch13>;
impl<'a, REG> Ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Included)
    }
}
#[doc = "Include or exclude channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch14 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch14> for bool {
    #[inline(always)]
    fn from(variant: Ch14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` reader - Include or exclude channel 14"]
pub type Ch14R = crate::BitReader<Ch14>;
impl Ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch14 {
        match self.bits {
            false => Ch14::Excluded,
            true => Ch14::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch14::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch14::Included
    }
}
#[doc = "Field `CH14` writer - Include or exclude channel 14"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG, Ch14>;
impl<'a, REG> Ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Included)
    }
}
#[doc = "Include or exclude channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch15 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch15> for bool {
    #[inline(always)]
    fn from(variant: Ch15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` reader - Include or exclude channel 15"]
pub type Ch15R = crate::BitReader<Ch15>;
impl Ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch15 {
        match self.bits {
            false => Ch15::Excluded,
            true => Ch15::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch15::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch15::Included
    }
}
#[doc = "Field `CH15` writer - Include or exclude channel 15"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG, Ch15>;
impl<'a, REG> Ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Included)
    }
}
#[doc = "Include or exclude channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch16 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch16> for bool {
    #[inline(always)]
    fn from(variant: Ch16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH16` reader - Include or exclude channel 16"]
pub type Ch16R = crate::BitReader<Ch16>;
impl Ch16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch16 {
        match self.bits {
            false => Ch16::Excluded,
            true => Ch16::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch16::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch16::Included
    }
}
#[doc = "Field `CH16` writer - Include or exclude channel 16"]
pub type Ch16W<'a, REG> = crate::BitWriter<'a, REG, Ch16>;
impl<'a, REG> Ch16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16::Included)
    }
}
#[doc = "Include or exclude channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch17 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch17> for bool {
    #[inline(always)]
    fn from(variant: Ch17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH17` reader - Include or exclude channel 17"]
pub type Ch17R = crate::BitReader<Ch17>;
impl Ch17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch17 {
        match self.bits {
            false => Ch17::Excluded,
            true => Ch17::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch17::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch17::Included
    }
}
#[doc = "Field `CH17` writer - Include or exclude channel 17"]
pub type Ch17W<'a, REG> = crate::BitWriter<'a, REG, Ch17>;
impl<'a, REG> Ch17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17::Included)
    }
}
#[doc = "Include or exclude channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch18 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch18> for bool {
    #[inline(always)]
    fn from(variant: Ch18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH18` reader - Include or exclude channel 18"]
pub type Ch18R = crate::BitReader<Ch18>;
impl Ch18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch18 {
        match self.bits {
            false => Ch18::Excluded,
            true => Ch18::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch18::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch18::Included
    }
}
#[doc = "Field `CH18` writer - Include or exclude channel 18"]
pub type Ch18W<'a, REG> = crate::BitWriter<'a, REG, Ch18>;
impl<'a, REG> Ch18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18::Included)
    }
}
#[doc = "Include or exclude channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch19 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch19> for bool {
    #[inline(always)]
    fn from(variant: Ch19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH19` reader - Include or exclude channel 19"]
pub type Ch19R = crate::BitReader<Ch19>;
impl Ch19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch19 {
        match self.bits {
            false => Ch19::Excluded,
            true => Ch19::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch19::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch19::Included
    }
}
#[doc = "Field `CH19` writer - Include or exclude channel 19"]
pub type Ch19W<'a, REG> = crate::BitWriter<'a, REG, Ch19>;
impl<'a, REG> Ch19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19::Included)
    }
}
#[doc = "Include or exclude channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch20 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch20> for bool {
    #[inline(always)]
    fn from(variant: Ch20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` reader - Include or exclude channel 20"]
pub type Ch20R = crate::BitReader<Ch20>;
impl Ch20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch20 {
        match self.bits {
            false => Ch20::Excluded,
            true => Ch20::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch20::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch20::Included
    }
}
#[doc = "Field `CH20` writer - Include or exclude channel 20"]
pub type Ch20W<'a, REG> = crate::BitWriter<'a, REG, Ch20>;
impl<'a, REG> Ch20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20::Included)
    }
}
#[doc = "Include or exclude channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch21 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch21> for bool {
    #[inline(always)]
    fn from(variant: Ch21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` reader - Include or exclude channel 21"]
pub type Ch21R = crate::BitReader<Ch21>;
impl Ch21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch21 {
        match self.bits {
            false => Ch21::Excluded,
            true => Ch21::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch21::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch21::Included
    }
}
#[doc = "Field `CH21` writer - Include or exclude channel 21"]
pub type Ch21W<'a, REG> = crate::BitWriter<'a, REG, Ch21>;
impl<'a, REG> Ch21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21::Included)
    }
}
#[doc = "Include or exclude channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch22 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch22> for bool {
    #[inline(always)]
    fn from(variant: Ch22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` reader - Include or exclude channel 22"]
pub type Ch22R = crate::BitReader<Ch22>;
impl Ch22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch22 {
        match self.bits {
            false => Ch22::Excluded,
            true => Ch22::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch22::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch22::Included
    }
}
#[doc = "Field `CH22` writer - Include or exclude channel 22"]
pub type Ch22W<'a, REG> = crate::BitWriter<'a, REG, Ch22>;
impl<'a, REG> Ch22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22::Included)
    }
}
#[doc = "Include or exclude channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch23 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch23> for bool {
    #[inline(always)]
    fn from(variant: Ch23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` reader - Include or exclude channel 23"]
pub type Ch23R = crate::BitReader<Ch23>;
impl Ch23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch23 {
        match self.bits {
            false => Ch23::Excluded,
            true => Ch23::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch23::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch23::Included
    }
}
#[doc = "Field `CH23` writer - Include or exclude channel 23"]
pub type Ch23W<'a, REG> = crate::BitWriter<'a, REG, Ch23>;
impl<'a, REG> Ch23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23::Included)
    }
}
#[doc = "Include or exclude channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch24 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch24> for bool {
    #[inline(always)]
    fn from(variant: Ch24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` reader - Include or exclude channel 24"]
pub type Ch24R = crate::BitReader<Ch24>;
impl Ch24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch24 {
        match self.bits {
            false => Ch24::Excluded,
            true => Ch24::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch24::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch24::Included
    }
}
#[doc = "Field `CH24` writer - Include or exclude channel 24"]
pub type Ch24W<'a, REG> = crate::BitWriter<'a, REG, Ch24>;
impl<'a, REG> Ch24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch24::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch24::Included)
    }
}
#[doc = "Include or exclude channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch25 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch25> for bool {
    #[inline(always)]
    fn from(variant: Ch25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` reader - Include or exclude channel 25"]
pub type Ch25R = crate::BitReader<Ch25>;
impl Ch25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch25 {
        match self.bits {
            false => Ch25::Excluded,
            true => Ch25::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch25::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch25::Included
    }
}
#[doc = "Field `CH25` writer - Include or exclude channel 25"]
pub type Ch25W<'a, REG> = crate::BitWriter<'a, REG, Ch25>;
impl<'a, REG> Ch25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch25::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch25::Included)
    }
}
#[doc = "Include or exclude channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch26 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch26> for bool {
    #[inline(always)]
    fn from(variant: Ch26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` reader - Include or exclude channel 26"]
pub type Ch26R = crate::BitReader<Ch26>;
impl Ch26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch26 {
        match self.bits {
            false => Ch26::Excluded,
            true => Ch26::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch26::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch26::Included
    }
}
#[doc = "Field `CH26` writer - Include or exclude channel 26"]
pub type Ch26W<'a, REG> = crate::BitWriter<'a, REG, Ch26>;
impl<'a, REG> Ch26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch26::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch26::Included)
    }
}
#[doc = "Include or exclude channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch27 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch27> for bool {
    #[inline(always)]
    fn from(variant: Ch27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` reader - Include or exclude channel 27"]
pub type Ch27R = crate::BitReader<Ch27>;
impl Ch27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch27 {
        match self.bits {
            false => Ch27::Excluded,
            true => Ch27::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch27::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch27::Included
    }
}
#[doc = "Field `CH27` writer - Include or exclude channel 27"]
pub type Ch27W<'a, REG> = crate::BitWriter<'a, REG, Ch27>;
impl<'a, REG> Ch27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch27::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch27::Included)
    }
}
#[doc = "Include or exclude channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch28 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch28> for bool {
    #[inline(always)]
    fn from(variant: Ch28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` reader - Include or exclude channel 28"]
pub type Ch28R = crate::BitReader<Ch28>;
impl Ch28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch28 {
        match self.bits {
            false => Ch28::Excluded,
            true => Ch28::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch28::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch28::Included
    }
}
#[doc = "Field `CH28` writer - Include or exclude channel 28"]
pub type Ch28W<'a, REG> = crate::BitWriter<'a, REG, Ch28>;
impl<'a, REG> Ch28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch28::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch28::Included)
    }
}
#[doc = "Include or exclude channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch29 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch29> for bool {
    #[inline(always)]
    fn from(variant: Ch29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` reader - Include or exclude channel 29"]
pub type Ch29R = crate::BitReader<Ch29>;
impl Ch29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch29 {
        match self.bits {
            false => Ch29::Excluded,
            true => Ch29::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch29::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch29::Included
    }
}
#[doc = "Field `CH29` writer - Include or exclude channel 29"]
pub type Ch29W<'a, REG> = crate::BitWriter<'a, REG, Ch29>;
impl<'a, REG> Ch29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch29::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch29::Included)
    }
}
#[doc = "Include or exclude channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch30 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch30> for bool {
    #[inline(always)]
    fn from(variant: Ch30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` reader - Include or exclude channel 30"]
pub type Ch30R = crate::BitReader<Ch30>;
impl Ch30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch30 {
        match self.bits {
            false => Ch30::Excluded,
            true => Ch30::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch30::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch30::Included
    }
}
#[doc = "Field `CH30` writer - Include or exclude channel 30"]
pub type Ch30W<'a, REG> = crate::BitWriter<'a, REG, Ch30>;
impl<'a, REG> Ch30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch30::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch30::Included)
    }
}
#[doc = "Include or exclude channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch31 {
    #[doc = "0: Exclude"]
    Excluded = 0,
    #[doc = "1: Include"]
    Included = 1,
}
impl From<Ch31> for bool {
    #[inline(always)]
    fn from(variant: Ch31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` reader - Include or exclude channel 31"]
pub type Ch31R = crate::BitReader<Ch31>;
impl Ch31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch31 {
        match self.bits {
            false => Ch31::Excluded,
            true => Ch31::Included,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Ch31::Excluded
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Ch31::Included
    }
}
#[doc = "Field `CH31` writer - Include or exclude channel 31"]
pub type Ch31W<'a, REG> = crate::BitWriter<'a, REG, Ch31>;
impl<'a, REG> Ch31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Ch31::Excluded)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Ch31::Included)
    }
}
impl R {
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Include or exclude channel 16"]
    #[inline(always)]
    pub fn ch16(&self) -> Ch16R {
        Ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Include or exclude channel 17"]
    #[inline(always)]
    pub fn ch17(&self) -> Ch17R {
        Ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Include or exclude channel 18"]
    #[inline(always)]
    pub fn ch18(&self) -> Ch18R {
        Ch18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Include or exclude channel 19"]
    #[inline(always)]
    pub fn ch19(&self) -> Ch19R {
        Ch19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline(always)]
    pub fn ch20(&self) -> Ch20R {
        Ch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline(always)]
    pub fn ch21(&self) -> Ch21R {
        Ch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline(always)]
    pub fn ch22(&self) -> Ch22R {
        Ch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline(always)]
    pub fn ch23(&self) -> Ch23R {
        Ch23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline(always)]
    pub fn ch24(&self) -> Ch24R {
        Ch24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline(always)]
    pub fn ch25(&self) -> Ch25R {
        Ch25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline(always)]
    pub fn ch26(&self) -> Ch26R {
        Ch26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline(always)]
    pub fn ch27(&self) -> Ch27R {
        Ch27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline(always)]
    pub fn ch28(&self) -> Ch28R {
        Ch28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline(always)]
    pub fn ch29(&self) -> Ch29R {
        Ch29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline(always)]
    pub fn ch30(&self) -> Ch30R {
        Ch30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
    #[inline(always)]
    pub fn ch31(&self) -> Ch31R {
        Ch31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChgSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChgSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChgSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChgSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, ChgSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, ChgSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, ChgSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, ChgSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, ChgSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, ChgSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, ChgSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, ChgSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, ChgSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, ChgSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, ChgSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, ChgSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Include or exclude channel 16"]
    #[inline(always)]
    pub fn ch16(&mut self) -> Ch16W<'_, ChgSpec> {
        Ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Include or exclude channel 17"]
    #[inline(always)]
    pub fn ch17(&mut self) -> Ch17W<'_, ChgSpec> {
        Ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Include or exclude channel 18"]
    #[inline(always)]
    pub fn ch18(&mut self) -> Ch18W<'_, ChgSpec> {
        Ch18W::new(self, 18)
    }
    #[doc = "Bit 19 - Include or exclude channel 19"]
    #[inline(always)]
    pub fn ch19(&mut self) -> Ch19W<'_, ChgSpec> {
        Ch19W::new(self, 19)
    }
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> Ch20W<'_, ChgSpec> {
        Ch20W::new(self, 20)
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> Ch21W<'_, ChgSpec> {
        Ch21W::new(self, 21)
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> Ch22W<'_, ChgSpec> {
        Ch22W::new(self, 22)
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> Ch23W<'_, ChgSpec> {
        Ch23W::new(self, 23)
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> Ch24W<'_, ChgSpec> {
        Ch24W::new(self, 24)
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> Ch25W<'_, ChgSpec> {
        Ch25W::new(self, 25)
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> Ch26W<'_, ChgSpec> {
        Ch26W::new(self, 26)
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> Ch27W<'_, ChgSpec> {
        Ch27W::new(self, 27)
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> Ch28W<'_, ChgSpec> {
        Ch28W::new(self, 28)
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> Ch29W<'_, ChgSpec> {
        Ch29W::new(self, 29)
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> Ch30W<'_, ChgSpec> {
        Ch30W::new(self, 30)
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> Ch31W<'_, ChgSpec> {
        Ch31W::new(self, 31)
    }
}
#[doc = "Description collection: Channel group n\n\nYou can [`read`](crate::Reg::read) this register and get [`chg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChgSpec;
impl crate::RegisterSpec for ChgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chg::R`](R) reader structure"]
impl crate::Readable for ChgSpec {}
#[doc = "`write(|w| ..)` method takes [`chg::W`](W) writer structure"]
impl crate::Writable for ChgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHG[%s] to value 0"]
impl crate::Resettable for ChgSpec {}
