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
}
impl W {
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ChgSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ChgSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ChgSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ChgSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<ChgSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<ChgSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<ChgSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<ChgSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<ChgSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<ChgSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<ChgSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<ChgSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<ChgSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<ChgSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<ChgSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<ChgSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Description collection: Channel group n Note: Writes to this register are ignored if either SUBSCRIBE_CHG\\[n\\].EN or SUBSCRIBE_CHG\\[n\\].DIS is enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChgSpec;
impl crate::RegisterSpec for ChgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chg::R`](R) reader structure"]
impl crate::Readable for ChgSpec {}
#[doc = "`write(|w| ..)` method takes [`chg::W`](W) writer structure"]
impl crate::Writable for ChgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHG[%s]
to value 0"]
impl crate::Resettable for ChgSpec {
    const RESET_VALUE: u32 = 0;
}
