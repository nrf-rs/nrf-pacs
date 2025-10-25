#[doc = "Register `CHEN` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `CHEN` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Enable or disable channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Enable or disable channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Disabled,
            true => Ch0::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0::Enabled
    }
}
#[doc = "Field `CH0` writer - Enable or disable channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Enabled)
    }
}
#[doc = "Enable or disable channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Enable or disable channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Disabled,
            true => Ch1::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1::Enabled
    }
}
#[doc = "Field `CH1` writer - Enable or disable channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Enabled)
    }
}
#[doc = "Enable or disable channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Enable or disable channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Disabled,
            true => Ch2::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2::Enabled
    }
}
#[doc = "Field `CH2` writer - Enable or disable channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Enabled)
    }
}
#[doc = "Enable or disable channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Enable or disable channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Disabled,
            true => Ch3::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3::Enabled
    }
}
#[doc = "Field `CH3` writer - Enable or disable channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Enabled)
    }
}
#[doc = "Enable or disable channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Enable or disable channel 4"]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            false => Ch4::Disabled,
            true => Ch4::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4::Enabled
    }
}
#[doc = "Field `CH4` writer - Enable or disable channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Enabled)
    }
}
#[doc = "Enable or disable channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Enable or disable channel 5"]
pub type Ch5R = crate::BitReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            false => Ch5::Disabled,
            true => Ch5::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5::Enabled
    }
}
#[doc = "Field `CH5` writer - Enable or disable channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG, Ch5>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Enabled)
    }
}
#[doc = "Enable or disable channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Enable or disable channel 6"]
pub type Ch6R = crate::BitReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            false => Ch6::Disabled,
            true => Ch6::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6::Enabled
    }
}
#[doc = "Field `CH6` writer - Enable or disable channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG, Ch6>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Enabled)
    }
}
#[doc = "Enable or disable channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Enable or disable channel 7"]
pub type Ch7R = crate::BitReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            false => Ch7::Disabled,
            true => Ch7::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7::Enabled
    }
}
#[doc = "Field `CH7` writer - Enable or disable channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG, Ch7>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Enabled)
    }
}
#[doc = "Enable or disable channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch8 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch8> for bool {
    #[inline(always)]
    fn from(variant: Ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Enable or disable channel 8"]
pub type Ch8R = crate::BitReader<Ch8>;
impl Ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch8 {
        match self.bits {
            false => Ch8::Disabled,
            true => Ch8::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch8::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch8::Enabled
    }
}
#[doc = "Field `CH8` writer - Enable or disable channel 8"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG, Ch8>;
impl<'a, REG> Ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Enabled)
    }
}
#[doc = "Enable or disable channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch9 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch9> for bool {
    #[inline(always)]
    fn from(variant: Ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Enable or disable channel 9"]
pub type Ch9R = crate::BitReader<Ch9>;
impl Ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch9 {
        match self.bits {
            false => Ch9::Disabled,
            true => Ch9::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch9::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch9::Enabled
    }
}
#[doc = "Field `CH9` writer - Enable or disable channel 9"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG, Ch9>;
impl<'a, REG> Ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Enabled)
    }
}
#[doc = "Enable or disable channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch20 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch20> for bool {
    #[inline(always)]
    fn from(variant: Ch20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` reader - Enable or disable channel 20"]
pub type Ch20R = crate::BitReader<Ch20>;
impl Ch20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch20 {
        match self.bits {
            false => Ch20::Disabled,
            true => Ch20::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch20::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch20::Enabled
    }
}
#[doc = "Field `CH20` writer - Enable or disable channel 20"]
pub type Ch20W<'a, REG> = crate::BitWriter<'a, REG, Ch20>;
impl<'a, REG> Ch20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20::Enabled)
    }
}
#[doc = "Enable or disable channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch21 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch21> for bool {
    #[inline(always)]
    fn from(variant: Ch21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` reader - Enable or disable channel 21"]
pub type Ch21R = crate::BitReader<Ch21>;
impl Ch21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch21 {
        match self.bits {
            false => Ch21::Disabled,
            true => Ch21::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch21::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch21::Enabled
    }
}
#[doc = "Field `CH21` writer - Enable or disable channel 21"]
pub type Ch21W<'a, REG> = crate::BitWriter<'a, REG, Ch21>;
impl<'a, REG> Ch21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21::Enabled)
    }
}
#[doc = "Enable or disable channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch22 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch22> for bool {
    #[inline(always)]
    fn from(variant: Ch22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` reader - Enable or disable channel 22"]
pub type Ch22R = crate::BitReader<Ch22>;
impl Ch22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch22 {
        match self.bits {
            false => Ch22::Disabled,
            true => Ch22::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch22::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch22::Enabled
    }
}
#[doc = "Field `CH22` writer - Enable or disable channel 22"]
pub type Ch22W<'a, REG> = crate::BitWriter<'a, REG, Ch22>;
impl<'a, REG> Ch22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22::Enabled)
    }
}
#[doc = "Enable or disable channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch23 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch23> for bool {
    #[inline(always)]
    fn from(variant: Ch23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` reader - Enable or disable channel 23"]
pub type Ch23R = crate::BitReader<Ch23>;
impl Ch23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch23 {
        match self.bits {
            false => Ch23::Disabled,
            true => Ch23::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch23::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch23::Enabled
    }
}
#[doc = "Field `CH23` writer - Enable or disable channel 23"]
pub type Ch23W<'a, REG> = crate::BitWriter<'a, REG, Ch23>;
impl<'a, REG> Ch23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23::Enabled)
    }
}
#[doc = "Enable or disable channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch24 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch24> for bool {
    #[inline(always)]
    fn from(variant: Ch24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` reader - Enable or disable channel 24"]
pub type Ch24R = crate::BitReader<Ch24>;
impl Ch24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch24 {
        match self.bits {
            false => Ch24::Disabled,
            true => Ch24::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch24::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch24::Enabled
    }
}
#[doc = "Field `CH24` writer - Enable or disable channel 24"]
pub type Ch24W<'a, REG> = crate::BitWriter<'a, REG, Ch24>;
impl<'a, REG> Ch24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch24::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch24::Enabled)
    }
}
#[doc = "Enable or disable channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch25 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch25> for bool {
    #[inline(always)]
    fn from(variant: Ch25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` reader - Enable or disable channel 25"]
pub type Ch25R = crate::BitReader<Ch25>;
impl Ch25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch25 {
        match self.bits {
            false => Ch25::Disabled,
            true => Ch25::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch25::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch25::Enabled
    }
}
#[doc = "Field `CH25` writer - Enable or disable channel 25"]
pub type Ch25W<'a, REG> = crate::BitWriter<'a, REG, Ch25>;
impl<'a, REG> Ch25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch25::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch25::Enabled)
    }
}
#[doc = "Enable or disable channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch26 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch26> for bool {
    #[inline(always)]
    fn from(variant: Ch26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` reader - Enable or disable channel 26"]
pub type Ch26R = crate::BitReader<Ch26>;
impl Ch26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch26 {
        match self.bits {
            false => Ch26::Disabled,
            true => Ch26::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch26::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch26::Enabled
    }
}
#[doc = "Field `CH26` writer - Enable or disable channel 26"]
pub type Ch26W<'a, REG> = crate::BitWriter<'a, REG, Ch26>;
impl<'a, REG> Ch26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch26::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch26::Enabled)
    }
}
#[doc = "Enable or disable channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch27 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch27> for bool {
    #[inline(always)]
    fn from(variant: Ch27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` reader - Enable or disable channel 27"]
pub type Ch27R = crate::BitReader<Ch27>;
impl Ch27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch27 {
        match self.bits {
            false => Ch27::Disabled,
            true => Ch27::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch27::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch27::Enabled
    }
}
#[doc = "Field `CH27` writer - Enable or disable channel 27"]
pub type Ch27W<'a, REG> = crate::BitWriter<'a, REG, Ch27>;
impl<'a, REG> Ch27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch27::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch27::Enabled)
    }
}
#[doc = "Enable or disable channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch28 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch28> for bool {
    #[inline(always)]
    fn from(variant: Ch28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` reader - Enable or disable channel 28"]
pub type Ch28R = crate::BitReader<Ch28>;
impl Ch28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch28 {
        match self.bits {
            false => Ch28::Disabled,
            true => Ch28::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch28::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch28::Enabled
    }
}
#[doc = "Field `CH28` writer - Enable or disable channel 28"]
pub type Ch28W<'a, REG> = crate::BitWriter<'a, REG, Ch28>;
impl<'a, REG> Ch28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch28::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch28::Enabled)
    }
}
#[doc = "Enable or disable channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch29 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch29> for bool {
    #[inline(always)]
    fn from(variant: Ch29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` reader - Enable or disable channel 29"]
pub type Ch29R = crate::BitReader<Ch29>;
impl Ch29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch29 {
        match self.bits {
            false => Ch29::Disabled,
            true => Ch29::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch29::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch29::Enabled
    }
}
#[doc = "Field `CH29` writer - Enable or disable channel 29"]
pub type Ch29W<'a, REG> = crate::BitWriter<'a, REG, Ch29>;
impl<'a, REG> Ch29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch29::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch29::Enabled)
    }
}
#[doc = "Enable or disable channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch30 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch30> for bool {
    #[inline(always)]
    fn from(variant: Ch30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` reader - Enable or disable channel 30"]
pub type Ch30R = crate::BitReader<Ch30>;
impl Ch30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch30 {
        match self.bits {
            false => Ch30::Disabled,
            true => Ch30::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch30::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch30::Enabled
    }
}
#[doc = "Field `CH30` writer - Enable or disable channel 30"]
pub type Ch30W<'a, REG> = crate::BitWriter<'a, REG, Ch30>;
impl<'a, REG> Ch30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch30::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch30::Enabled)
    }
}
#[doc = "Enable or disable channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch31 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch31> for bool {
    #[inline(always)]
    fn from(variant: Ch31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` reader - Enable or disable channel 31"]
pub type Ch31R = crate::BitReader<Ch31>;
impl Ch31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch31 {
        match self.bits {
            false => Ch31::Disabled,
            true => Ch31::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch31::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch31::Enabled
    }
}
#[doc = "Field `CH31` writer - Enable or disable channel 31"]
pub type Ch31W<'a, REG> = crate::BitWriter<'a, REG, Ch31>;
impl<'a, REG> Ch31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch31::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch31::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable channel 8"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable channel 9"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable channel 20"]
    #[inline(always)]
    pub fn ch20(&self) -> Ch20R {
        Ch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable or disable channel 21"]
    #[inline(always)]
    pub fn ch21(&self) -> Ch21R {
        Ch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable or disable channel 22"]
    #[inline(always)]
    pub fn ch22(&self) -> Ch22R {
        Ch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable or disable channel 23"]
    #[inline(always)]
    pub fn ch23(&self) -> Ch23R {
        Ch23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable or disable channel 24"]
    #[inline(always)]
    pub fn ch24(&self) -> Ch24R {
        Ch24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable channel 25"]
    #[inline(always)]
    pub fn ch25(&self) -> Ch25R {
        Ch25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable channel 26"]
    #[inline(always)]
    pub fn ch26(&self) -> Ch26R {
        Ch26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable or disable channel 27"]
    #[inline(always)]
    pub fn ch27(&self) -> Ch27R {
        Ch27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable or disable channel 28"]
    #[inline(always)]
    pub fn ch28(&self) -> Ch28R {
        Ch28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable or disable channel 29"]
    #[inline(always)]
    pub fn ch29(&self) -> Ch29R {
        Ch29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable or disable channel 30"]
    #[inline(always)]
    pub fn ch30(&self) -> Ch30R {
        Ch30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable or disable channel 31"]
    #[inline(always)]
    pub fn ch31(&self) -> Ch31R {
        Ch31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChenSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, ChenSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, ChenSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, ChenSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, ChenSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable channel 8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, ChenSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable channel 9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, ChenSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 20 - Enable or disable channel 20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> Ch20W<'_, ChenSpec> {
        Ch20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable or disable channel 21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> Ch21W<'_, ChenSpec> {
        Ch21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable or disable channel 22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> Ch22W<'_, ChenSpec> {
        Ch22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable or disable channel 23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> Ch23W<'_, ChenSpec> {
        Ch23W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable or disable channel 24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> Ch24W<'_, ChenSpec> {
        Ch24W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable or disable channel 25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> Ch25W<'_, ChenSpec> {
        Ch25W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable or disable channel 26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> Ch26W<'_, ChenSpec> {
        Ch26W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable or disable channel 27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> Ch27W<'_, ChenSpec> {
        Ch27W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable or disable channel 28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> Ch28W<'_, ChenSpec> {
        Ch28W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable or disable channel 29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> Ch29W<'_, ChenSpec> {
        Ch29W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable or disable channel 30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> Ch30W<'_, ChenSpec> {
        Ch30W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable or disable channel 31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> Ch31W<'_, ChenSpec> {
        Ch31W::new(self, 31)
    }
}
#[doc = "Channel enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for ChenSpec {}
