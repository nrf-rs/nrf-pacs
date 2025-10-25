#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel0 {
    #[doc = "1: Channel 0 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 0 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel0> for bool {
    #[inline(always)]
    fn from(variant: Channel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL0` reader - Select secure attribute"]
pub type Channel0R = crate::BitReader<Channel0>;
impl Channel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel0 {
        match self.bits {
            true => Channel0::Secure,
            false => Channel0::NonSecure,
        }
    }
    #[doc = "Channel 0 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel0::Secure
    }
    #[doc = "Channel 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel0::NonSecure
    }
}
#[doc = "Field `CHANNEL0` writer - Select secure attribute"]
pub type Channel0W<'a, REG> = crate::BitWriter<'a, REG, Channel0>;
impl<'a, REG> Channel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel0::Secure)
    }
    #[doc = "Channel 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel0::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel1 {
    #[doc = "1: Channel 1 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 1 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel1> for bool {
    #[inline(always)]
    fn from(variant: Channel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL1` reader - Select secure attribute"]
pub type Channel1R = crate::BitReader<Channel1>;
impl Channel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel1 {
        match self.bits {
            true => Channel1::Secure,
            false => Channel1::NonSecure,
        }
    }
    #[doc = "Channel 1 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel1::Secure
    }
    #[doc = "Channel 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel1::NonSecure
    }
}
#[doc = "Field `CHANNEL1` writer - Select secure attribute"]
pub type Channel1W<'a, REG> = crate::BitWriter<'a, REG, Channel1>;
impl<'a, REG> Channel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel1::Secure)
    }
    #[doc = "Channel 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel1::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel2 {
    #[doc = "1: Channel 2 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 2 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel2> for bool {
    #[inline(always)]
    fn from(variant: Channel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL2` reader - Select secure attribute"]
pub type Channel2R = crate::BitReader<Channel2>;
impl Channel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel2 {
        match self.bits {
            true => Channel2::Secure,
            false => Channel2::NonSecure,
        }
    }
    #[doc = "Channel 2 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel2::Secure
    }
    #[doc = "Channel 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel2::NonSecure
    }
}
#[doc = "Field `CHANNEL2` writer - Select secure attribute"]
pub type Channel2W<'a, REG> = crate::BitWriter<'a, REG, Channel2>;
impl<'a, REG> Channel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel2::Secure)
    }
    #[doc = "Channel 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel2::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel3 {
    #[doc = "1: Channel 3 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 3 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel3> for bool {
    #[inline(always)]
    fn from(variant: Channel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL3` reader - Select secure attribute"]
pub type Channel3R = crate::BitReader<Channel3>;
impl Channel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel3 {
        match self.bits {
            true => Channel3::Secure,
            false => Channel3::NonSecure,
        }
    }
    #[doc = "Channel 3 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel3::Secure
    }
    #[doc = "Channel 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel3::NonSecure
    }
}
#[doc = "Field `CHANNEL3` writer - Select secure attribute"]
pub type Channel3W<'a, REG> = crate::BitWriter<'a, REG, Channel3>;
impl<'a, REG> Channel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel3::Secure)
    }
    #[doc = "Channel 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel3::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel4 {
    #[doc = "1: Channel 4 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 4 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel4> for bool {
    #[inline(always)]
    fn from(variant: Channel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL4` reader - Select secure attribute"]
pub type Channel4R = crate::BitReader<Channel4>;
impl Channel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel4 {
        match self.bits {
            true => Channel4::Secure,
            false => Channel4::NonSecure,
        }
    }
    #[doc = "Channel 4 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel4::Secure
    }
    #[doc = "Channel 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel4::NonSecure
    }
}
#[doc = "Field `CHANNEL4` writer - Select secure attribute"]
pub type Channel4W<'a, REG> = crate::BitWriter<'a, REG, Channel4>;
impl<'a, REG> Channel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel4::Secure)
    }
    #[doc = "Channel 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel4::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel5 {
    #[doc = "1: Channel 5 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 5 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel5> for bool {
    #[inline(always)]
    fn from(variant: Channel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL5` reader - Select secure attribute"]
pub type Channel5R = crate::BitReader<Channel5>;
impl Channel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel5 {
        match self.bits {
            true => Channel5::Secure,
            false => Channel5::NonSecure,
        }
    }
    #[doc = "Channel 5 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel5::Secure
    }
    #[doc = "Channel 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel5::NonSecure
    }
}
#[doc = "Field `CHANNEL5` writer - Select secure attribute"]
pub type Channel5W<'a, REG> = crate::BitWriter<'a, REG, Channel5>;
impl<'a, REG> Channel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel5::Secure)
    }
    #[doc = "Channel 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel5::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel6 {
    #[doc = "1: Channel 6 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 6 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel6> for bool {
    #[inline(always)]
    fn from(variant: Channel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL6` reader - Select secure attribute"]
pub type Channel6R = crate::BitReader<Channel6>;
impl Channel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel6 {
        match self.bits {
            true => Channel6::Secure,
            false => Channel6::NonSecure,
        }
    }
    #[doc = "Channel 6 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel6::Secure
    }
    #[doc = "Channel 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel6::NonSecure
    }
}
#[doc = "Field `CHANNEL6` writer - Select secure attribute"]
pub type Channel6W<'a, REG> = crate::BitWriter<'a, REG, Channel6>;
impl<'a, REG> Channel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel6::Secure)
    }
    #[doc = "Channel 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel6::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel7 {
    #[doc = "1: Channel 7 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 7 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel7> for bool {
    #[inline(always)]
    fn from(variant: Channel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL7` reader - Select secure attribute"]
pub type Channel7R = crate::BitReader<Channel7>;
impl Channel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel7 {
        match self.bits {
            true => Channel7::Secure,
            false => Channel7::NonSecure,
        }
    }
    #[doc = "Channel 7 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel7::Secure
    }
    #[doc = "Channel 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel7::NonSecure
    }
}
#[doc = "Field `CHANNEL7` writer - Select secure attribute"]
pub type Channel7W<'a, REG> = crate::BitWriter<'a, REG, Channel7>;
impl<'a, REG> Channel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel7::Secure)
    }
    #[doc = "Channel 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel7::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel8 {
    #[doc = "1: Channel 8 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 8 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel8> for bool {
    #[inline(always)]
    fn from(variant: Channel8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL8` reader - Select secure attribute"]
pub type Channel8R = crate::BitReader<Channel8>;
impl Channel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel8 {
        match self.bits {
            true => Channel8::Secure,
            false => Channel8::NonSecure,
        }
    }
    #[doc = "Channel 8 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel8::Secure
    }
    #[doc = "Channel 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel8::NonSecure
    }
}
#[doc = "Field `CHANNEL8` writer - Select secure attribute"]
pub type Channel8W<'a, REG> = crate::BitWriter<'a, REG, Channel8>;
impl<'a, REG> Channel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel8::Secure)
    }
    #[doc = "Channel 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel8::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel9 {
    #[doc = "1: Channel 9 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 9 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel9> for bool {
    #[inline(always)]
    fn from(variant: Channel9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL9` reader - Select secure attribute"]
pub type Channel9R = crate::BitReader<Channel9>;
impl Channel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel9 {
        match self.bits {
            true => Channel9::Secure,
            false => Channel9::NonSecure,
        }
    }
    #[doc = "Channel 9 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel9::Secure
    }
    #[doc = "Channel 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel9::NonSecure
    }
}
#[doc = "Field `CHANNEL9` writer - Select secure attribute"]
pub type Channel9W<'a, REG> = crate::BitWriter<'a, REG, Channel9>;
impl<'a, REG> Channel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel9::Secure)
    }
    #[doc = "Channel 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel9::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel10 {
    #[doc = "1: Channel 10 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 10 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel10> for bool {
    #[inline(always)]
    fn from(variant: Channel10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL10` reader - Select secure attribute"]
pub type Channel10R = crate::BitReader<Channel10>;
impl Channel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel10 {
        match self.bits {
            true => Channel10::Secure,
            false => Channel10::NonSecure,
        }
    }
    #[doc = "Channel 10 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel10::Secure
    }
    #[doc = "Channel 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel10::NonSecure
    }
}
#[doc = "Field `CHANNEL10` writer - Select secure attribute"]
pub type Channel10W<'a, REG> = crate::BitWriter<'a, REG, Channel10>;
impl<'a, REG> Channel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel10::Secure)
    }
    #[doc = "Channel 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel10::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel11 {
    #[doc = "1: Channel 11 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 11 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel11> for bool {
    #[inline(always)]
    fn from(variant: Channel11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL11` reader - Select secure attribute"]
pub type Channel11R = crate::BitReader<Channel11>;
impl Channel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel11 {
        match self.bits {
            true => Channel11::Secure,
            false => Channel11::NonSecure,
        }
    }
    #[doc = "Channel 11 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel11::Secure
    }
    #[doc = "Channel 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel11::NonSecure
    }
}
#[doc = "Field `CHANNEL11` writer - Select secure attribute"]
pub type Channel11W<'a, REG> = crate::BitWriter<'a, REG, Channel11>;
impl<'a, REG> Channel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel11::Secure)
    }
    #[doc = "Channel 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel11::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel12 {
    #[doc = "1: Channel 12 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 12 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel12> for bool {
    #[inline(always)]
    fn from(variant: Channel12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL12` reader - Select secure attribute"]
pub type Channel12R = crate::BitReader<Channel12>;
impl Channel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel12 {
        match self.bits {
            true => Channel12::Secure,
            false => Channel12::NonSecure,
        }
    }
    #[doc = "Channel 12 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel12::Secure
    }
    #[doc = "Channel 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel12::NonSecure
    }
}
#[doc = "Field `CHANNEL12` writer - Select secure attribute"]
pub type Channel12W<'a, REG> = crate::BitWriter<'a, REG, Channel12>;
impl<'a, REG> Channel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel12::Secure)
    }
    #[doc = "Channel 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel12::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel13 {
    #[doc = "1: Channel 13 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 13 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel13> for bool {
    #[inline(always)]
    fn from(variant: Channel13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL13` reader - Select secure attribute"]
pub type Channel13R = crate::BitReader<Channel13>;
impl Channel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel13 {
        match self.bits {
            true => Channel13::Secure,
            false => Channel13::NonSecure,
        }
    }
    #[doc = "Channel 13 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel13::Secure
    }
    #[doc = "Channel 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel13::NonSecure
    }
}
#[doc = "Field `CHANNEL13` writer - Select secure attribute"]
pub type Channel13W<'a, REG> = crate::BitWriter<'a, REG, Channel13>;
impl<'a, REG> Channel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel13::Secure)
    }
    #[doc = "Channel 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel13::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel14 {
    #[doc = "1: Channel 14 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 14 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel14> for bool {
    #[inline(always)]
    fn from(variant: Channel14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL14` reader - Select secure attribute"]
pub type Channel14R = crate::BitReader<Channel14>;
impl Channel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel14 {
        match self.bits {
            true => Channel14::Secure,
            false => Channel14::NonSecure,
        }
    }
    #[doc = "Channel 14 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel14::Secure
    }
    #[doc = "Channel 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel14::NonSecure
    }
}
#[doc = "Field `CHANNEL14` writer - Select secure attribute"]
pub type Channel14W<'a, REG> = crate::BitWriter<'a, REG, Channel14>;
impl<'a, REG> Channel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel14::Secure)
    }
    #[doc = "Channel 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel14::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel15 {
    #[doc = "1: Channel 15 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 15 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel15> for bool {
    #[inline(always)]
    fn from(variant: Channel15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL15` reader - Select secure attribute"]
pub type Channel15R = crate::BitReader<Channel15>;
impl Channel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel15 {
        match self.bits {
            true => Channel15::Secure,
            false => Channel15::NonSecure,
        }
    }
    #[doc = "Channel 15 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel15::Secure
    }
    #[doc = "Channel 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel15::NonSecure
    }
}
#[doc = "Field `CHANNEL15` writer - Select secure attribute"]
pub type Channel15W<'a, REG> = crate::BitWriter<'a, REG, Channel15>;
impl<'a, REG> Channel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15::Secure)
    }
    #[doc = "Channel 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel16 {
    #[doc = "1: Channel 16 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 16 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel16> for bool {
    #[inline(always)]
    fn from(variant: Channel16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL16` reader - Select secure attribute"]
pub type Channel16R = crate::BitReader<Channel16>;
impl Channel16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel16 {
        match self.bits {
            true => Channel16::Secure,
            false => Channel16::NonSecure,
        }
    }
    #[doc = "Channel 16 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel16::Secure
    }
    #[doc = "Channel 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel16::NonSecure
    }
}
#[doc = "Field `CHANNEL16` writer - Select secure attribute"]
pub type Channel16W<'a, REG> = crate::BitWriter<'a, REG, Channel16>;
impl<'a, REG> Channel16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 16 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel16::Secure)
    }
    #[doc = "Channel 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel16::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel17 {
    #[doc = "1: Channel 17 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 17 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel17> for bool {
    #[inline(always)]
    fn from(variant: Channel17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL17` reader - Select secure attribute"]
pub type Channel17R = crate::BitReader<Channel17>;
impl Channel17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel17 {
        match self.bits {
            true => Channel17::Secure,
            false => Channel17::NonSecure,
        }
    }
    #[doc = "Channel 17 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel17::Secure
    }
    #[doc = "Channel 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel17::NonSecure
    }
}
#[doc = "Field `CHANNEL17` writer - Select secure attribute"]
pub type Channel17W<'a, REG> = crate::BitWriter<'a, REG, Channel17>;
impl<'a, REG> Channel17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 17 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel17::Secure)
    }
    #[doc = "Channel 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel17::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel18 {
    #[doc = "1: Channel 18 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 18 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel18> for bool {
    #[inline(always)]
    fn from(variant: Channel18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL18` reader - Select secure attribute"]
pub type Channel18R = crate::BitReader<Channel18>;
impl Channel18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel18 {
        match self.bits {
            true => Channel18::Secure,
            false => Channel18::NonSecure,
        }
    }
    #[doc = "Channel 18 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel18::Secure
    }
    #[doc = "Channel 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel18::NonSecure
    }
}
#[doc = "Field `CHANNEL18` writer - Select secure attribute"]
pub type Channel18W<'a, REG> = crate::BitWriter<'a, REG, Channel18>;
impl<'a, REG> Channel18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 18 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel18::Secure)
    }
    #[doc = "Channel 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel18::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel19 {
    #[doc = "1: Channel 19 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 19 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel19> for bool {
    #[inline(always)]
    fn from(variant: Channel19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL19` reader - Select secure attribute"]
pub type Channel19R = crate::BitReader<Channel19>;
impl Channel19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel19 {
        match self.bits {
            true => Channel19::Secure,
            false => Channel19::NonSecure,
        }
    }
    #[doc = "Channel 19 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel19::Secure
    }
    #[doc = "Channel 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel19::NonSecure
    }
}
#[doc = "Field `CHANNEL19` writer - Select secure attribute"]
pub type Channel19W<'a, REG> = crate::BitWriter<'a, REG, Channel19>;
impl<'a, REG> Channel19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 19 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel19::Secure)
    }
    #[doc = "Channel 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel19::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel20 {
    #[doc = "1: Channel 20 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 20 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel20> for bool {
    #[inline(always)]
    fn from(variant: Channel20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL20` reader - Select secure attribute"]
pub type Channel20R = crate::BitReader<Channel20>;
impl Channel20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel20 {
        match self.bits {
            true => Channel20::Secure,
            false => Channel20::NonSecure,
        }
    }
    #[doc = "Channel 20 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel20::Secure
    }
    #[doc = "Channel 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel20::NonSecure
    }
}
#[doc = "Field `CHANNEL20` writer - Select secure attribute"]
pub type Channel20W<'a, REG> = crate::BitWriter<'a, REG, Channel20>;
impl<'a, REG> Channel20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 20 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel20::Secure)
    }
    #[doc = "Channel 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel20::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel21 {
    #[doc = "1: Channel 21 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 21 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel21> for bool {
    #[inline(always)]
    fn from(variant: Channel21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL21` reader - Select secure attribute"]
pub type Channel21R = crate::BitReader<Channel21>;
impl Channel21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel21 {
        match self.bits {
            true => Channel21::Secure,
            false => Channel21::NonSecure,
        }
    }
    #[doc = "Channel 21 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel21::Secure
    }
    #[doc = "Channel 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel21::NonSecure
    }
}
#[doc = "Field `CHANNEL21` writer - Select secure attribute"]
pub type Channel21W<'a, REG> = crate::BitWriter<'a, REG, Channel21>;
impl<'a, REG> Channel21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 21 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel21::Secure)
    }
    #[doc = "Channel 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel21::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel22 {
    #[doc = "1: Channel 22 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 22 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel22> for bool {
    #[inline(always)]
    fn from(variant: Channel22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL22` reader - Select secure attribute"]
pub type Channel22R = crate::BitReader<Channel22>;
impl Channel22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel22 {
        match self.bits {
            true => Channel22::Secure,
            false => Channel22::NonSecure,
        }
    }
    #[doc = "Channel 22 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel22::Secure
    }
    #[doc = "Channel 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel22::NonSecure
    }
}
#[doc = "Field `CHANNEL22` writer - Select secure attribute"]
pub type Channel22W<'a, REG> = crate::BitWriter<'a, REG, Channel22>;
impl<'a, REG> Channel22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 22 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel22::Secure)
    }
    #[doc = "Channel 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel22::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel23 {
    #[doc = "1: Channel 23 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 23 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel23> for bool {
    #[inline(always)]
    fn from(variant: Channel23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL23` reader - Select secure attribute"]
pub type Channel23R = crate::BitReader<Channel23>;
impl Channel23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel23 {
        match self.bits {
            true => Channel23::Secure,
            false => Channel23::NonSecure,
        }
    }
    #[doc = "Channel 23 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel23::Secure
    }
    #[doc = "Channel 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel23::NonSecure
    }
}
#[doc = "Field `CHANNEL23` writer - Select secure attribute"]
pub type Channel23W<'a, REG> = crate::BitWriter<'a, REG, Channel23>;
impl<'a, REG> Channel23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 23 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel23::Secure)
    }
    #[doc = "Channel 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel23::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel24 {
    #[doc = "1: Channel 24 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 24 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel24> for bool {
    #[inline(always)]
    fn from(variant: Channel24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL24` reader - Select secure attribute"]
pub type Channel24R = crate::BitReader<Channel24>;
impl Channel24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel24 {
        match self.bits {
            true => Channel24::Secure,
            false => Channel24::NonSecure,
        }
    }
    #[doc = "Channel 24 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel24::Secure
    }
    #[doc = "Channel 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel24::NonSecure
    }
}
#[doc = "Field `CHANNEL24` writer - Select secure attribute"]
pub type Channel24W<'a, REG> = crate::BitWriter<'a, REG, Channel24>;
impl<'a, REG> Channel24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 24 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel24::Secure)
    }
    #[doc = "Channel 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel24::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel25 {
    #[doc = "1: Channel 25 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 25 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel25> for bool {
    #[inline(always)]
    fn from(variant: Channel25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL25` reader - Select secure attribute"]
pub type Channel25R = crate::BitReader<Channel25>;
impl Channel25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel25 {
        match self.bits {
            true => Channel25::Secure,
            false => Channel25::NonSecure,
        }
    }
    #[doc = "Channel 25 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel25::Secure
    }
    #[doc = "Channel 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel25::NonSecure
    }
}
#[doc = "Field `CHANNEL25` writer - Select secure attribute"]
pub type Channel25W<'a, REG> = crate::BitWriter<'a, REG, Channel25>;
impl<'a, REG> Channel25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 25 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel25::Secure)
    }
    #[doc = "Channel 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel25::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel26 {
    #[doc = "1: Channel 26 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 26 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel26> for bool {
    #[inline(always)]
    fn from(variant: Channel26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL26` reader - Select secure attribute"]
pub type Channel26R = crate::BitReader<Channel26>;
impl Channel26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel26 {
        match self.bits {
            true => Channel26::Secure,
            false => Channel26::NonSecure,
        }
    }
    #[doc = "Channel 26 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel26::Secure
    }
    #[doc = "Channel 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel26::NonSecure
    }
}
#[doc = "Field `CHANNEL26` writer - Select secure attribute"]
pub type Channel26W<'a, REG> = crate::BitWriter<'a, REG, Channel26>;
impl<'a, REG> Channel26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 26 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel26::Secure)
    }
    #[doc = "Channel 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel26::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel27 {
    #[doc = "1: Channel 27 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 27 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel27> for bool {
    #[inline(always)]
    fn from(variant: Channel27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL27` reader - Select secure attribute"]
pub type Channel27R = crate::BitReader<Channel27>;
impl Channel27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel27 {
        match self.bits {
            true => Channel27::Secure,
            false => Channel27::NonSecure,
        }
    }
    #[doc = "Channel 27 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel27::Secure
    }
    #[doc = "Channel 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel27::NonSecure
    }
}
#[doc = "Field `CHANNEL27` writer - Select secure attribute"]
pub type Channel27W<'a, REG> = crate::BitWriter<'a, REG, Channel27>;
impl<'a, REG> Channel27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 27 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel27::Secure)
    }
    #[doc = "Channel 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel27::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel28 {
    #[doc = "1: Channel 28 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 28 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel28> for bool {
    #[inline(always)]
    fn from(variant: Channel28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL28` reader - Select secure attribute"]
pub type Channel28R = crate::BitReader<Channel28>;
impl Channel28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel28 {
        match self.bits {
            true => Channel28::Secure,
            false => Channel28::NonSecure,
        }
    }
    #[doc = "Channel 28 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel28::Secure
    }
    #[doc = "Channel 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel28::NonSecure
    }
}
#[doc = "Field `CHANNEL28` writer - Select secure attribute"]
pub type Channel28W<'a, REG> = crate::BitWriter<'a, REG, Channel28>;
impl<'a, REG> Channel28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 28 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel28::Secure)
    }
    #[doc = "Channel 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel28::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel29 {
    #[doc = "1: Channel 29 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 29 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel29> for bool {
    #[inline(always)]
    fn from(variant: Channel29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL29` reader - Select secure attribute"]
pub type Channel29R = crate::BitReader<Channel29>;
impl Channel29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel29 {
        match self.bits {
            true => Channel29::Secure,
            false => Channel29::NonSecure,
        }
    }
    #[doc = "Channel 29 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel29::Secure
    }
    #[doc = "Channel 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel29::NonSecure
    }
}
#[doc = "Field `CHANNEL29` writer - Select secure attribute"]
pub type Channel29W<'a, REG> = crate::BitWriter<'a, REG, Channel29>;
impl<'a, REG> Channel29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 29 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel29::Secure)
    }
    #[doc = "Channel 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel29::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel30 {
    #[doc = "1: Channel 30 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 30 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel30> for bool {
    #[inline(always)]
    fn from(variant: Channel30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL30` reader - Select secure attribute"]
pub type Channel30R = crate::BitReader<Channel30>;
impl Channel30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel30 {
        match self.bits {
            true => Channel30::Secure,
            false => Channel30::NonSecure,
        }
    }
    #[doc = "Channel 30 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel30::Secure
    }
    #[doc = "Channel 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel30::NonSecure
    }
}
#[doc = "Field `CHANNEL30` writer - Select secure attribute"]
pub type Channel30W<'a, REG> = crate::BitWriter<'a, REG, Channel30>;
impl<'a, REG> Channel30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 30 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel30::Secure)
    }
    #[doc = "Channel 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel30::NonSecure)
    }
}
#[doc = "Select secure attribute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel31 {
    #[doc = "1: Channel 31 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel 31 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel31> for bool {
    #[inline(always)]
    fn from(variant: Channel31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL31` reader - Select secure attribute"]
pub type Channel31R = crate::BitReader<Channel31>;
impl Channel31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel31 {
        match self.bits {
            true => Channel31::Secure,
            false => Channel31::NonSecure,
        }
    }
    #[doc = "Channel 31 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel31::Secure
    }
    #[doc = "Channel 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel31::NonSecure
    }
}
#[doc = "Field `CHANNEL31` writer - Select secure attribute"]
pub type Channel31W<'a, REG> = crate::BitWriter<'a, REG, Channel31>;
impl<'a, REG> Channel31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 31 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel31::Secure)
    }
    #[doc = "Channel 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel31::NonSecure)
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute"]
    #[inline(always)]
    pub fn channel0(&self) -> Channel0R {
        Channel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute"]
    #[inline(always)]
    pub fn channel1(&self) -> Channel1R {
        Channel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute"]
    #[inline(always)]
    pub fn channel2(&self) -> Channel2R {
        Channel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute"]
    #[inline(always)]
    pub fn channel3(&self) -> Channel3R {
        Channel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute"]
    #[inline(always)]
    pub fn channel4(&self) -> Channel4R {
        Channel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute"]
    #[inline(always)]
    pub fn channel5(&self) -> Channel5R {
        Channel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute"]
    #[inline(always)]
    pub fn channel6(&self) -> Channel6R {
        Channel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute"]
    #[inline(always)]
    pub fn channel7(&self) -> Channel7R {
        Channel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute"]
    #[inline(always)]
    pub fn channel8(&self) -> Channel8R {
        Channel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute"]
    #[inline(always)]
    pub fn channel9(&self) -> Channel9R {
        Channel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute"]
    #[inline(always)]
    pub fn channel10(&self) -> Channel10R {
        Channel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute"]
    #[inline(always)]
    pub fn channel11(&self) -> Channel11R {
        Channel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute"]
    #[inline(always)]
    pub fn channel12(&self) -> Channel12R {
        Channel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute"]
    #[inline(always)]
    pub fn channel13(&self) -> Channel13R {
        Channel13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute"]
    #[inline(always)]
    pub fn channel14(&self) -> Channel14R {
        Channel14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute"]
    #[inline(always)]
    pub fn channel15(&self) -> Channel15R {
        Channel15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select secure attribute"]
    #[inline(always)]
    pub fn channel16(&self) -> Channel16R {
        Channel16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select secure attribute"]
    #[inline(always)]
    pub fn channel17(&self) -> Channel17R {
        Channel17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select secure attribute"]
    #[inline(always)]
    pub fn channel18(&self) -> Channel18R {
        Channel18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select secure attribute"]
    #[inline(always)]
    pub fn channel19(&self) -> Channel19R {
        Channel19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select secure attribute"]
    #[inline(always)]
    pub fn channel20(&self) -> Channel20R {
        Channel20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select secure attribute"]
    #[inline(always)]
    pub fn channel21(&self) -> Channel21R {
        Channel21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Select secure attribute"]
    #[inline(always)]
    pub fn channel22(&self) -> Channel22R {
        Channel22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Select secure attribute"]
    #[inline(always)]
    pub fn channel23(&self) -> Channel23R {
        Channel23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select secure attribute"]
    #[inline(always)]
    pub fn channel24(&self) -> Channel24R {
        Channel24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select secure attribute"]
    #[inline(always)]
    pub fn channel25(&self) -> Channel25R {
        Channel25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select secure attribute"]
    #[inline(always)]
    pub fn channel26(&self) -> Channel26R {
        Channel26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select secure attribute"]
    #[inline(always)]
    pub fn channel27(&self) -> Channel27R {
        Channel27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select secure attribute"]
    #[inline(always)]
    pub fn channel28(&self) -> Channel28R {
        Channel28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select secure attribute"]
    #[inline(always)]
    pub fn channel29(&self) -> Channel29R {
        Channel29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select secure attribute"]
    #[inline(always)]
    pub fn channel30(&self) -> Channel30R {
        Channel30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select secure attribute"]
    #[inline(always)]
    pub fn channel31(&self) -> Channel31R {
        Channel31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute"]
    #[inline(always)]
    pub fn channel0(&mut self) -> Channel0W<'_, PermSpec> {
        Channel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Select secure attribute"]
    #[inline(always)]
    pub fn channel1(&mut self) -> Channel1W<'_, PermSpec> {
        Channel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Select secure attribute"]
    #[inline(always)]
    pub fn channel2(&mut self) -> Channel2W<'_, PermSpec> {
        Channel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Select secure attribute"]
    #[inline(always)]
    pub fn channel3(&mut self) -> Channel3W<'_, PermSpec> {
        Channel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Select secure attribute"]
    #[inline(always)]
    pub fn channel4(&mut self) -> Channel4W<'_, PermSpec> {
        Channel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Select secure attribute"]
    #[inline(always)]
    pub fn channel5(&mut self) -> Channel5W<'_, PermSpec> {
        Channel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Select secure attribute"]
    #[inline(always)]
    pub fn channel6(&mut self) -> Channel6W<'_, PermSpec> {
        Channel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Select secure attribute"]
    #[inline(always)]
    pub fn channel7(&mut self) -> Channel7W<'_, PermSpec> {
        Channel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Select secure attribute"]
    #[inline(always)]
    pub fn channel8(&mut self) -> Channel8W<'_, PermSpec> {
        Channel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Select secure attribute"]
    #[inline(always)]
    pub fn channel9(&mut self) -> Channel9W<'_, PermSpec> {
        Channel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Select secure attribute"]
    #[inline(always)]
    pub fn channel10(&mut self) -> Channel10W<'_, PermSpec> {
        Channel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Select secure attribute"]
    #[inline(always)]
    pub fn channel11(&mut self) -> Channel11W<'_, PermSpec> {
        Channel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Select secure attribute"]
    #[inline(always)]
    pub fn channel12(&mut self) -> Channel12W<'_, PermSpec> {
        Channel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Select secure attribute"]
    #[inline(always)]
    pub fn channel13(&mut self) -> Channel13W<'_, PermSpec> {
        Channel13W::new(self, 13)
    }
    #[doc = "Bit 14 - Select secure attribute"]
    #[inline(always)]
    pub fn channel14(&mut self) -> Channel14W<'_, PermSpec> {
        Channel14W::new(self, 14)
    }
    #[doc = "Bit 15 - Select secure attribute"]
    #[inline(always)]
    pub fn channel15(&mut self) -> Channel15W<'_, PermSpec> {
        Channel15W::new(self, 15)
    }
    #[doc = "Bit 16 - Select secure attribute"]
    #[inline(always)]
    pub fn channel16(&mut self) -> Channel16W<'_, PermSpec> {
        Channel16W::new(self, 16)
    }
    #[doc = "Bit 17 - Select secure attribute"]
    #[inline(always)]
    pub fn channel17(&mut self) -> Channel17W<'_, PermSpec> {
        Channel17W::new(self, 17)
    }
    #[doc = "Bit 18 - Select secure attribute"]
    #[inline(always)]
    pub fn channel18(&mut self) -> Channel18W<'_, PermSpec> {
        Channel18W::new(self, 18)
    }
    #[doc = "Bit 19 - Select secure attribute"]
    #[inline(always)]
    pub fn channel19(&mut self) -> Channel19W<'_, PermSpec> {
        Channel19W::new(self, 19)
    }
    #[doc = "Bit 20 - Select secure attribute"]
    #[inline(always)]
    pub fn channel20(&mut self) -> Channel20W<'_, PermSpec> {
        Channel20W::new(self, 20)
    }
    #[doc = "Bit 21 - Select secure attribute"]
    #[inline(always)]
    pub fn channel21(&mut self) -> Channel21W<'_, PermSpec> {
        Channel21W::new(self, 21)
    }
    #[doc = "Bit 22 - Select secure attribute"]
    #[inline(always)]
    pub fn channel22(&mut self) -> Channel22W<'_, PermSpec> {
        Channel22W::new(self, 22)
    }
    #[doc = "Bit 23 - Select secure attribute"]
    #[inline(always)]
    pub fn channel23(&mut self) -> Channel23W<'_, PermSpec> {
        Channel23W::new(self, 23)
    }
    #[doc = "Bit 24 - Select secure attribute"]
    #[inline(always)]
    pub fn channel24(&mut self) -> Channel24W<'_, PermSpec> {
        Channel24W::new(self, 24)
    }
    #[doc = "Bit 25 - Select secure attribute"]
    #[inline(always)]
    pub fn channel25(&mut self) -> Channel25W<'_, PermSpec> {
        Channel25W::new(self, 25)
    }
    #[doc = "Bit 26 - Select secure attribute"]
    #[inline(always)]
    pub fn channel26(&mut self) -> Channel26W<'_, PermSpec> {
        Channel26W::new(self, 26)
    }
    #[doc = "Bit 27 - Select secure attribute"]
    #[inline(always)]
    pub fn channel27(&mut self) -> Channel27W<'_, PermSpec> {
        Channel27W::new(self, 27)
    }
    #[doc = "Bit 28 - Select secure attribute"]
    #[inline(always)]
    pub fn channel28(&mut self) -> Channel28W<'_, PermSpec> {
        Channel28W::new(self, 28)
    }
    #[doc = "Bit 29 - Select secure attribute"]
    #[inline(always)]
    pub fn channel29(&mut self) -> Channel29W<'_, PermSpec> {
        Channel29W::new(self, 29)
    }
    #[doc = "Bit 30 - Select secure attribute"]
    #[inline(always)]
    pub fn channel30(&mut self) -> Channel30W<'_, PermSpec> {
        Channel30W::new(self, 30)
    }
    #[doc = "Bit 31 - Select secure attribute"]
    #[inline(always)]
    pub fn channel31(&mut self) -> Channel31W<'_, PermSpec> {
        Channel31W::new(self, 31)
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERM to value 0xffff_ffff"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
