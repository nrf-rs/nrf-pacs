#[doc = "Register `SEND_CNF[%s]` reader"]
pub type R = crate::R<SendCnfSpec>;
#[doc = "Register `SEND_CNF[%s]` writer"]
pub type W = crate::W<SendCnfSpec>;
#[doc = "Enable broadcasting on IPC channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen0 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen0> for bool {
    #[inline(always)]
    fn from(variant: Chen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN0` reader - Enable broadcasting on IPC channel 0"]
pub type Chen0R = crate::BitReader<Chen0>;
impl Chen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen0 {
        match self.bits {
            false => Chen0::Disable,
            true => Chen0::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen0::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen0::Enable
    }
}
#[doc = "Field `CHEN0` writer - Enable broadcasting on IPC channel 0"]
pub type Chen0W<'a, REG> = crate::BitWriter<'a, REG, Chen0>;
impl<'a, REG> Chen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen0::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen0::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen1 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen1> for bool {
    #[inline(always)]
    fn from(variant: Chen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN1` reader - Enable broadcasting on IPC channel 1"]
pub type Chen1R = crate::BitReader<Chen1>;
impl Chen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen1 {
        match self.bits {
            false => Chen1::Disable,
            true => Chen1::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen1::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen1::Enable
    }
}
#[doc = "Field `CHEN1` writer - Enable broadcasting on IPC channel 1"]
pub type Chen1W<'a, REG> = crate::BitWriter<'a, REG, Chen1>;
impl<'a, REG> Chen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen1::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen1::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen2 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen2> for bool {
    #[inline(always)]
    fn from(variant: Chen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN2` reader - Enable broadcasting on IPC channel 2"]
pub type Chen2R = crate::BitReader<Chen2>;
impl Chen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen2 {
        match self.bits {
            false => Chen2::Disable,
            true => Chen2::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen2::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen2::Enable
    }
}
#[doc = "Field `CHEN2` writer - Enable broadcasting on IPC channel 2"]
pub type Chen2W<'a, REG> = crate::BitWriter<'a, REG, Chen2>;
impl<'a, REG> Chen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen2::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen2::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen3 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen3> for bool {
    #[inline(always)]
    fn from(variant: Chen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN3` reader - Enable broadcasting on IPC channel 3"]
pub type Chen3R = crate::BitReader<Chen3>;
impl Chen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen3 {
        match self.bits {
            false => Chen3::Disable,
            true => Chen3::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen3::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen3::Enable
    }
}
#[doc = "Field `CHEN3` writer - Enable broadcasting on IPC channel 3"]
pub type Chen3W<'a, REG> = crate::BitWriter<'a, REG, Chen3>;
impl<'a, REG> Chen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen3::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen3::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen4 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen4> for bool {
    #[inline(always)]
    fn from(variant: Chen4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN4` reader - Enable broadcasting on IPC channel 4"]
pub type Chen4R = crate::BitReader<Chen4>;
impl Chen4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen4 {
        match self.bits {
            false => Chen4::Disable,
            true => Chen4::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen4::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen4::Enable
    }
}
#[doc = "Field `CHEN4` writer - Enable broadcasting on IPC channel 4"]
pub type Chen4W<'a, REG> = crate::BitWriter<'a, REG, Chen4>;
impl<'a, REG> Chen4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen4::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen4::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen5 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen5> for bool {
    #[inline(always)]
    fn from(variant: Chen5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN5` reader - Enable broadcasting on IPC channel 5"]
pub type Chen5R = crate::BitReader<Chen5>;
impl Chen5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen5 {
        match self.bits {
            false => Chen5::Disable,
            true => Chen5::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen5::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen5::Enable
    }
}
#[doc = "Field `CHEN5` writer - Enable broadcasting on IPC channel 5"]
pub type Chen5W<'a, REG> = crate::BitWriter<'a, REG, Chen5>;
impl<'a, REG> Chen5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen5::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen5::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen6 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen6> for bool {
    #[inline(always)]
    fn from(variant: Chen6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN6` reader - Enable broadcasting on IPC channel 6"]
pub type Chen6R = crate::BitReader<Chen6>;
impl Chen6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen6 {
        match self.bits {
            false => Chen6::Disable,
            true => Chen6::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen6::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen6::Enable
    }
}
#[doc = "Field `CHEN6` writer - Enable broadcasting on IPC channel 6"]
pub type Chen6W<'a, REG> = crate::BitWriter<'a, REG, Chen6>;
impl<'a, REG> Chen6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen6::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen6::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen7 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen7> for bool {
    #[inline(always)]
    fn from(variant: Chen7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN7` reader - Enable broadcasting on IPC channel 7"]
pub type Chen7R = crate::BitReader<Chen7>;
impl Chen7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen7 {
        match self.bits {
            false => Chen7::Disable,
            true => Chen7::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen7::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen7::Enable
    }
}
#[doc = "Field `CHEN7` writer - Enable broadcasting on IPC channel 7"]
pub type Chen7W<'a, REG> = crate::BitWriter<'a, REG, Chen7>;
impl<'a, REG> Chen7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen7::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen7::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen8 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen8> for bool {
    #[inline(always)]
    fn from(variant: Chen8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN8` reader - Enable broadcasting on IPC channel 8"]
pub type Chen8R = crate::BitReader<Chen8>;
impl Chen8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen8 {
        match self.bits {
            false => Chen8::Disable,
            true => Chen8::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen8::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen8::Enable
    }
}
#[doc = "Field `CHEN8` writer - Enable broadcasting on IPC channel 8"]
pub type Chen8W<'a, REG> = crate::BitWriter<'a, REG, Chen8>;
impl<'a, REG> Chen8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen8::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen8::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen9 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen9> for bool {
    #[inline(always)]
    fn from(variant: Chen9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN9` reader - Enable broadcasting on IPC channel 9"]
pub type Chen9R = crate::BitReader<Chen9>;
impl Chen9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen9 {
        match self.bits {
            false => Chen9::Disable,
            true => Chen9::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen9::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen9::Enable
    }
}
#[doc = "Field `CHEN9` writer - Enable broadcasting on IPC channel 9"]
pub type Chen9W<'a, REG> = crate::BitWriter<'a, REG, Chen9>;
impl<'a, REG> Chen9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen9::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen9::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen10 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen10> for bool {
    #[inline(always)]
    fn from(variant: Chen10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN10` reader - Enable broadcasting on IPC channel 10"]
pub type Chen10R = crate::BitReader<Chen10>;
impl Chen10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen10 {
        match self.bits {
            false => Chen10::Disable,
            true => Chen10::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen10::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen10::Enable
    }
}
#[doc = "Field `CHEN10` writer - Enable broadcasting on IPC channel 10"]
pub type Chen10W<'a, REG> = crate::BitWriter<'a, REG, Chen10>;
impl<'a, REG> Chen10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen10::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen10::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen11 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen11> for bool {
    #[inline(always)]
    fn from(variant: Chen11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN11` reader - Enable broadcasting on IPC channel 11"]
pub type Chen11R = crate::BitReader<Chen11>;
impl Chen11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen11 {
        match self.bits {
            false => Chen11::Disable,
            true => Chen11::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen11::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen11::Enable
    }
}
#[doc = "Field `CHEN11` writer - Enable broadcasting on IPC channel 11"]
pub type Chen11W<'a, REG> = crate::BitWriter<'a, REG, Chen11>;
impl<'a, REG> Chen11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen11::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen11::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen12 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen12> for bool {
    #[inline(always)]
    fn from(variant: Chen12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN12` reader - Enable broadcasting on IPC channel 12"]
pub type Chen12R = crate::BitReader<Chen12>;
impl Chen12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen12 {
        match self.bits {
            false => Chen12::Disable,
            true => Chen12::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen12::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen12::Enable
    }
}
#[doc = "Field `CHEN12` writer - Enable broadcasting on IPC channel 12"]
pub type Chen12W<'a, REG> = crate::BitWriter<'a, REG, Chen12>;
impl<'a, REG> Chen12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen12::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen12::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen13 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen13> for bool {
    #[inline(always)]
    fn from(variant: Chen13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN13` reader - Enable broadcasting on IPC channel 13"]
pub type Chen13R = crate::BitReader<Chen13>;
impl Chen13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen13 {
        match self.bits {
            false => Chen13::Disable,
            true => Chen13::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen13::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen13::Enable
    }
}
#[doc = "Field `CHEN13` writer - Enable broadcasting on IPC channel 13"]
pub type Chen13W<'a, REG> = crate::BitWriter<'a, REG, Chen13>;
impl<'a, REG> Chen13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen13::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen13::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen14 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen14> for bool {
    #[inline(always)]
    fn from(variant: Chen14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN14` reader - Enable broadcasting on IPC channel 14"]
pub type Chen14R = crate::BitReader<Chen14>;
impl Chen14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen14 {
        match self.bits {
            false => Chen14::Disable,
            true => Chen14::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen14::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen14::Enable
    }
}
#[doc = "Field `CHEN14` writer - Enable broadcasting on IPC channel 14"]
pub type Chen14W<'a, REG> = crate::BitWriter<'a, REG, Chen14>;
impl<'a, REG> Chen14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen14::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen14::Enable)
    }
}
#[doc = "Enable broadcasting on IPC channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chen15 {
    #[doc = "0: Disable broadcast"]
    Disable = 0,
    #[doc = "1: Enable broadcast"]
    Enable = 1,
}
impl From<Chen15> for bool {
    #[inline(always)]
    fn from(variant: Chen15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN15` reader - Enable broadcasting on IPC channel 15"]
pub type Chen15R = crate::BitReader<Chen15>;
impl Chen15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chen15 {
        match self.bits {
            false => Chen15::Disable,
            true => Chen15::Enable,
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chen15::Disable
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chen15::Enable
    }
}
#[doc = "Field `CHEN15` writer - Enable broadcasting on IPC channel 15"]
pub type Chen15W<'a, REG> = crate::BitWriter<'a, REG, Chen15>;
impl<'a, REG> Chen15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen15::Disable)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chen15::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&self) -> Chen0R {
        Chen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&self) -> Chen1R {
        Chen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&self) -> Chen2R {
        Chen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&self) -> Chen3R {
        Chen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&self) -> Chen4R {
        Chen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&self) -> Chen5R {
        Chen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&self) -> Chen6R {
        Chen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&self) -> Chen7R {
        Chen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&self) -> Chen8R {
        Chen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&self) -> Chen9R {
        Chen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&self) -> Chen10R {
        Chen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&self) -> Chen11R {
        Chen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&self) -> Chen12R {
        Chen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&self) -> Chen13R {
        Chen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&self) -> Chen14R {
        Chen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&self) -> Chen15R {
        Chen15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&mut self) -> Chen0W<'_, SendCnfSpec> {
        Chen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&mut self) -> Chen1W<'_, SendCnfSpec> {
        Chen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&mut self) -> Chen2W<'_, SendCnfSpec> {
        Chen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&mut self) -> Chen3W<'_, SendCnfSpec> {
        Chen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&mut self) -> Chen4W<'_, SendCnfSpec> {
        Chen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&mut self) -> Chen5W<'_, SendCnfSpec> {
        Chen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&mut self) -> Chen6W<'_, SendCnfSpec> {
        Chen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&mut self) -> Chen7W<'_, SendCnfSpec> {
        Chen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&mut self) -> Chen8W<'_, SendCnfSpec> {
        Chen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&mut self) -> Chen9W<'_, SendCnfSpec> {
        Chen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&mut self) -> Chen10W<'_, SendCnfSpec> {
        Chen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&mut self) -> Chen11W<'_, SendCnfSpec> {
        Chen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&mut self) -> Chen12W<'_, SendCnfSpec> {
        Chen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&mut self) -> Chen13W<'_, SendCnfSpec> {
        Chen13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&mut self) -> Chen14W<'_, SendCnfSpec> {
        Chen14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&mut self) -> Chen15W<'_, SendCnfSpec> {
        Chen15W::new(self, 15)
    }
}
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`send_cnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`send_cnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SendCnfSpec;
impl crate::RegisterSpec for SendCnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`send_cnf::R`](R) reader structure"]
impl crate::Readable for SendCnfSpec {}
#[doc = "`write(|w| ..)` method takes [`send_cnf::W`](W) writer structure"]
impl crate::Writable for SendCnfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEND_CNF[%s] to value 0"]
impl crate::Resettable for SendCnfSpec {}
