#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive0> for bool {
    #[inline(always)]
    fn from(variant: Receive0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type Receive0R = crate::BitReader<Receive0>;
impl Receive0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive0 {
        match self.bits {
            false => Receive0::Disabled,
            true => Receive0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive0::Enabled
    }
}
#[doc = "Field `RECEIVE0` writer - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type Receive0W<'a, REG> = crate::BitWriter<'a, REG, Receive0>;
impl<'a, REG> Receive0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive1> for bool {
    #[inline(always)]
    fn from(variant: Receive1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type Receive1R = crate::BitReader<Receive1>;
impl Receive1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive1 {
        match self.bits {
            false => Receive1::Disabled,
            true => Receive1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive1::Enabled
    }
}
#[doc = "Field `RECEIVE1` writer - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type Receive1W<'a, REG> = crate::BitWriter<'a, REG, Receive1>;
impl<'a, REG> Receive1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive2> for bool {
    #[inline(always)]
    fn from(variant: Receive2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type Receive2R = crate::BitReader<Receive2>;
impl Receive2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive2 {
        match self.bits {
            false => Receive2::Disabled,
            true => Receive2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive2::Enabled
    }
}
#[doc = "Field `RECEIVE2` writer - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type Receive2W<'a, REG> = crate::BitWriter<'a, REG, Receive2>;
impl<'a, REG> Receive2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive3> for bool {
    #[inline(always)]
    fn from(variant: Receive3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type Receive3R = crate::BitReader<Receive3>;
impl Receive3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive3 {
        match self.bits {
            false => Receive3::Disabled,
            true => Receive3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive3::Enabled
    }
}
#[doc = "Field `RECEIVE3` writer - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type Receive3W<'a, REG> = crate::BitWriter<'a, REG, Receive3>;
impl<'a, REG> Receive3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive3::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive4> for bool {
    #[inline(always)]
    fn from(variant: Receive4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type Receive4R = crate::BitReader<Receive4>;
impl Receive4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive4 {
        match self.bits {
            false => Receive4::Disabled,
            true => Receive4::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive4::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive4::Enabled
    }
}
#[doc = "Field `RECEIVE4` writer - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type Receive4W<'a, REG> = crate::BitWriter<'a, REG, Receive4>;
impl<'a, REG> Receive4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive4::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive4::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive5> for bool {
    #[inline(always)]
    fn from(variant: Receive5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type Receive5R = crate::BitReader<Receive5>;
impl Receive5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive5 {
        match self.bits {
            false => Receive5::Disabled,
            true => Receive5::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive5::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive5::Enabled
    }
}
#[doc = "Field `RECEIVE5` writer - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type Receive5W<'a, REG> = crate::BitWriter<'a, REG, Receive5>;
impl<'a, REG> Receive5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive5::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive5::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive6> for bool {
    #[inline(always)]
    fn from(variant: Receive6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type Receive6R = crate::BitReader<Receive6>;
impl Receive6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive6 {
        match self.bits {
            false => Receive6::Disabled,
            true => Receive6::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive6::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive6::Enabled
    }
}
#[doc = "Field `RECEIVE6` writer - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type Receive6W<'a, REG> = crate::BitWriter<'a, REG, Receive6>;
impl<'a, REG> Receive6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive6::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive6::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive7> for bool {
    #[inline(always)]
    fn from(variant: Receive7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type Receive7R = crate::BitReader<Receive7>;
impl Receive7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive7 {
        match self.bits {
            false => Receive7::Disabled,
            true => Receive7::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive7::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive7::Enabled
    }
}
#[doc = "Field `RECEIVE7` writer - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type Receive7W<'a, REG> = crate::BitWriter<'a, REG, Receive7>;
impl<'a, REG> Receive7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive7::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive7::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive8 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive8> for bool {
    #[inline(always)]
    fn from(variant: Receive8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE8` reader - Enable or disable interrupt for event RECEIVE\\[8\\]"]
pub type Receive8R = crate::BitReader<Receive8>;
impl Receive8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive8 {
        match self.bits {
            false => Receive8::Disabled,
            true => Receive8::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive8::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive8::Enabled
    }
}
#[doc = "Field `RECEIVE8` writer - Enable or disable interrupt for event RECEIVE\\[8\\]"]
pub type Receive8W<'a, REG> = crate::BitWriter<'a, REG, Receive8>;
impl<'a, REG> Receive8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive8::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive8::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive9 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive9> for bool {
    #[inline(always)]
    fn from(variant: Receive9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE9` reader - Enable or disable interrupt for event RECEIVE\\[9\\]"]
pub type Receive9R = crate::BitReader<Receive9>;
impl Receive9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive9 {
        match self.bits {
            false => Receive9::Disabled,
            true => Receive9::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive9::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive9::Enabled
    }
}
#[doc = "Field `RECEIVE9` writer - Enable or disable interrupt for event RECEIVE\\[9\\]"]
pub type Receive9W<'a, REG> = crate::BitWriter<'a, REG, Receive9>;
impl<'a, REG> Receive9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive9::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive9::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive10 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive10> for bool {
    #[inline(always)]
    fn from(variant: Receive10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE10` reader - Enable or disable interrupt for event RECEIVE\\[10\\]"]
pub type Receive10R = crate::BitReader<Receive10>;
impl Receive10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive10 {
        match self.bits {
            false => Receive10::Disabled,
            true => Receive10::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive10::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive10::Enabled
    }
}
#[doc = "Field `RECEIVE10` writer - Enable or disable interrupt for event RECEIVE\\[10\\]"]
pub type Receive10W<'a, REG> = crate::BitWriter<'a, REG, Receive10>;
impl<'a, REG> Receive10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive10::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive10::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive11 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive11> for bool {
    #[inline(always)]
    fn from(variant: Receive11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE11` reader - Enable or disable interrupt for event RECEIVE\\[11\\]"]
pub type Receive11R = crate::BitReader<Receive11>;
impl Receive11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive11 {
        match self.bits {
            false => Receive11::Disabled,
            true => Receive11::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive11::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive11::Enabled
    }
}
#[doc = "Field `RECEIVE11` writer - Enable or disable interrupt for event RECEIVE\\[11\\]"]
pub type Receive11W<'a, REG> = crate::BitWriter<'a, REG, Receive11>;
impl<'a, REG> Receive11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive11::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive11::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive12 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive12> for bool {
    #[inline(always)]
    fn from(variant: Receive12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE12` reader - Enable or disable interrupt for event RECEIVE\\[12\\]"]
pub type Receive12R = crate::BitReader<Receive12>;
impl Receive12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive12 {
        match self.bits {
            false => Receive12::Disabled,
            true => Receive12::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive12::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive12::Enabled
    }
}
#[doc = "Field `RECEIVE12` writer - Enable or disable interrupt for event RECEIVE\\[12\\]"]
pub type Receive12W<'a, REG> = crate::BitWriter<'a, REG, Receive12>;
impl<'a, REG> Receive12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive12::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive12::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive13 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive13> for bool {
    #[inline(always)]
    fn from(variant: Receive13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE13` reader - Enable or disable interrupt for event RECEIVE\\[13\\]"]
pub type Receive13R = crate::BitReader<Receive13>;
impl Receive13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive13 {
        match self.bits {
            false => Receive13::Disabled,
            true => Receive13::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive13::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive13::Enabled
    }
}
#[doc = "Field `RECEIVE13` writer - Enable or disable interrupt for event RECEIVE\\[13\\]"]
pub type Receive13W<'a, REG> = crate::BitWriter<'a, REG, Receive13>;
impl<'a, REG> Receive13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive13::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive13::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive14 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive14> for bool {
    #[inline(always)]
    fn from(variant: Receive14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE14` reader - Enable or disable interrupt for event RECEIVE\\[14\\]"]
pub type Receive14R = crate::BitReader<Receive14>;
impl Receive14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive14 {
        match self.bits {
            false => Receive14::Disabled,
            true => Receive14::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive14::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive14::Enabled
    }
}
#[doc = "Field `RECEIVE14` writer - Enable or disable interrupt for event RECEIVE\\[14\\]"]
pub type Receive14W<'a, REG> = crate::BitWriter<'a, REG, Receive14>;
impl<'a, REG> Receive14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive14::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive14::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive15 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive15> for bool {
    #[inline(always)]
    fn from(variant: Receive15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE15` reader - Enable or disable interrupt for event RECEIVE\\[15\\]"]
pub type Receive15R = crate::BitReader<Receive15>;
impl Receive15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive15 {
        match self.bits {
            false => Receive15::Disabled,
            true => Receive15::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive15::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive15::Enabled
    }
}
#[doc = "Field `RECEIVE15` writer - Enable or disable interrupt for event RECEIVE\\[15\\]"]
pub type Receive15W<'a, REG> = crate::BitWriter<'a, REG, Receive15>;
impl<'a, REG> Receive15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive15::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive15::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> Receive0R {
        Receive0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> Receive1R {
        Receive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> Receive2R {
        Receive2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> Receive3R {
        Receive3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> Receive4R {
        Receive4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> Receive5R {
        Receive5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> Receive6R {
        Receive6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> Receive7R {
        Receive7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> Receive8R {
        Receive8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> Receive9R {
        Receive9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> Receive10R {
        Receive10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> Receive11R {
        Receive11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> Receive12R {
        Receive12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> Receive13R {
        Receive13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> Receive14R {
        Receive14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> Receive15R {
        Receive15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&mut self) -> Receive0W<'_, IntenSpec> {
        Receive0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&mut self) -> Receive1W<'_, IntenSpec> {
        Receive1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&mut self) -> Receive2W<'_, IntenSpec> {
        Receive2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&mut self) -> Receive3W<'_, IntenSpec> {
        Receive3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&mut self) -> Receive4W<'_, IntenSpec> {
        Receive4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&mut self) -> Receive5W<'_, IntenSpec> {
        Receive5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&mut self) -> Receive6W<'_, IntenSpec> {
        Receive6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&mut self) -> Receive7W<'_, IntenSpec> {
        Receive7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&mut self) -> Receive8W<'_, IntenSpec> {
        Receive8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&mut self) -> Receive9W<'_, IntenSpec> {
        Receive9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&mut self) -> Receive10W<'_, IntenSpec> {
        Receive10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&mut self) -> Receive11W<'_, IntenSpec> {
        Receive11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&mut self) -> Receive12W<'_, IntenSpec> {
        Receive12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&mut self) -> Receive13W<'_, IntenSpec> {
        Receive13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&mut self) -> Receive14W<'_, IntenSpec> {
        Receive14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&mut self) -> Receive15W<'_, IntenSpec> {
        Receive15W::new(self, 15)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
