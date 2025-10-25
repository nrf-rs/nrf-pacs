#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive0> for bool {
    #[inline(always)]
    fn from(variant: Receive0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive0::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive0WO> for bool {
    #[inline(always)]
    fn from(variant: Receive0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` writer - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
pub type Receive0W<'a, REG> = crate::BitWriter<'a, REG, Receive0WO>;
impl<'a, REG> Receive0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive0WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive1> for bool {
    #[inline(always)]
    fn from(variant: Receive1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive1::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive1WO> for bool {
    #[inline(always)]
    fn from(variant: Receive1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` writer - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
pub type Receive1W<'a, REG> = crate::BitWriter<'a, REG, Receive1WO>;
impl<'a, REG> Receive1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive1WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive2> for bool {
    #[inline(always)]
    fn from(variant: Receive2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive2::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive2WO> for bool {
    #[inline(always)]
    fn from(variant: Receive2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` writer - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
pub type Receive2W<'a, REG> = crate::BitWriter<'a, REG, Receive2WO>;
impl<'a, REG> Receive2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive2WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive3> for bool {
    #[inline(always)]
    fn from(variant: Receive3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive3::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive3WO> for bool {
    #[inline(always)]
    fn from(variant: Receive3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` writer - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
pub type Receive3W<'a, REG> = crate::BitWriter<'a, REG, Receive3WO>;
impl<'a, REG> Receive3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive3WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive4> for bool {
    #[inline(always)]
    fn from(variant: Receive4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive4::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive4::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive4WO> for bool {
    #[inline(always)]
    fn from(variant: Receive4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` writer - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
pub type Receive4W<'a, REG> = crate::BitWriter<'a, REG, Receive4WO>;
impl<'a, REG> Receive4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive4WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive5> for bool {
    #[inline(always)]
    fn from(variant: Receive5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive5::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive5::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive5WO> for bool {
    #[inline(always)]
    fn from(variant: Receive5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` writer - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
pub type Receive5W<'a, REG> = crate::BitWriter<'a, REG, Receive5WO>;
impl<'a, REG> Receive5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive5WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive6> for bool {
    #[inline(always)]
    fn from(variant: Receive6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive6::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive6::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive6WO> for bool {
    #[inline(always)]
    fn from(variant: Receive6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` writer - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
pub type Receive6W<'a, REG> = crate::BitWriter<'a, REG, Receive6WO>;
impl<'a, REG> Receive6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive6WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive7> for bool {
    #[inline(always)]
    fn from(variant: Receive7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive7::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive7::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive7WO> for bool {
    #[inline(always)]
    fn from(variant: Receive7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` writer - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
pub type Receive7W<'a, REG> = crate::BitWriter<'a, REG, Receive7WO>;
impl<'a, REG> Receive7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive7WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive8 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive8> for bool {
    #[inline(always)]
    fn from(variant: Receive8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE8` reader - Write '1' to enable interrupt for event RECEIVE\\[8\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive8::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive8::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive8WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive8WO> for bool {
    #[inline(always)]
    fn from(variant: Receive8WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE8` writer - Write '1' to enable interrupt for event RECEIVE\\[8\\]"]
pub type Receive8W<'a, REG> = crate::BitWriter<'a, REG, Receive8WO>;
impl<'a, REG> Receive8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive8WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive9 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive9> for bool {
    #[inline(always)]
    fn from(variant: Receive9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE9` reader - Write '1' to enable interrupt for event RECEIVE\\[9\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive9::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive9::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive9WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive9WO> for bool {
    #[inline(always)]
    fn from(variant: Receive9WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE9` writer - Write '1' to enable interrupt for event RECEIVE\\[9\\]"]
pub type Receive9W<'a, REG> = crate::BitWriter<'a, REG, Receive9WO>;
impl<'a, REG> Receive9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive9WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive10 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive10> for bool {
    #[inline(always)]
    fn from(variant: Receive10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE10` reader - Write '1' to enable interrupt for event RECEIVE\\[10\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive10::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive10::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive10WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive10WO> for bool {
    #[inline(always)]
    fn from(variant: Receive10WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE10` writer - Write '1' to enable interrupt for event RECEIVE\\[10\\]"]
pub type Receive10W<'a, REG> = crate::BitWriter<'a, REG, Receive10WO>;
impl<'a, REG> Receive10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive10WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive11 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive11> for bool {
    #[inline(always)]
    fn from(variant: Receive11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE11` reader - Write '1' to enable interrupt for event RECEIVE\\[11\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive11::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive11::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive11WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive11WO> for bool {
    #[inline(always)]
    fn from(variant: Receive11WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE11` writer - Write '1' to enable interrupt for event RECEIVE\\[11\\]"]
pub type Receive11W<'a, REG> = crate::BitWriter<'a, REG, Receive11WO>;
impl<'a, REG> Receive11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive11WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive12 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive12> for bool {
    #[inline(always)]
    fn from(variant: Receive12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE12` reader - Write '1' to enable interrupt for event RECEIVE\\[12\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive12::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive12::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive12WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive12WO> for bool {
    #[inline(always)]
    fn from(variant: Receive12WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE12` writer - Write '1' to enable interrupt for event RECEIVE\\[12\\]"]
pub type Receive12W<'a, REG> = crate::BitWriter<'a, REG, Receive12WO>;
impl<'a, REG> Receive12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive12WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive13 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive13> for bool {
    #[inline(always)]
    fn from(variant: Receive13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE13` reader - Write '1' to enable interrupt for event RECEIVE\\[13\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive13::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive13::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive13WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive13WO> for bool {
    #[inline(always)]
    fn from(variant: Receive13WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE13` writer - Write '1' to enable interrupt for event RECEIVE\\[13\\]"]
pub type Receive13W<'a, REG> = crate::BitWriter<'a, REG, Receive13WO>;
impl<'a, REG> Receive13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive13WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive14 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive14> for bool {
    #[inline(always)]
    fn from(variant: Receive14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE14` reader - Write '1' to enable interrupt for event RECEIVE\\[14\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive14::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive14::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive14WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive14WO> for bool {
    #[inline(always)]
    fn from(variant: Receive14WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE14` writer - Write '1' to enable interrupt for event RECEIVE\\[14\\]"]
pub type Receive14W<'a, REG> = crate::BitWriter<'a, REG, Receive14WO>;
impl<'a, REG> Receive14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive14WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive15 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Receive15> for bool {
    #[inline(always)]
    fn from(variant: Receive15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE15` reader - Write '1' to enable interrupt for event RECEIVE\\[15\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive15::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive15::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive15WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Receive15WO> for bool {
    #[inline(always)]
    fn from(variant: Receive15WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE15` writer - Write '1' to enable interrupt for event RECEIVE\\[15\\]"]
pub type Receive15W<'a, REG> = crate::BitWriter<'a, REG, Receive15WO>;
impl<'a, REG> Receive15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Receive15WO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> Receive0R {
        Receive0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> Receive1R {
        Receive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> Receive2R {
        Receive2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> Receive3R {
        Receive3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> Receive4R {
        Receive4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> Receive5R {
        Receive5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> Receive6R {
        Receive6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> Receive7R {
        Receive7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> Receive8R {
        Receive8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> Receive9R {
        Receive9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> Receive10R {
        Receive10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> Receive11R {
        Receive11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> Receive12R {
        Receive12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> Receive13R {
        Receive13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> Receive14R {
        Receive14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> Receive15R {
        Receive15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&mut self) -> Receive0W<'_, IntensetSpec> {
        Receive0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&mut self) -> Receive1W<'_, IntensetSpec> {
        Receive1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&mut self) -> Receive2W<'_, IntensetSpec> {
        Receive2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&mut self) -> Receive3W<'_, IntensetSpec> {
        Receive3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&mut self) -> Receive4W<'_, IntensetSpec> {
        Receive4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&mut self) -> Receive5W<'_, IntensetSpec> {
        Receive5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&mut self) -> Receive6W<'_, IntensetSpec> {
        Receive6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&mut self) -> Receive7W<'_, IntensetSpec> {
        Receive7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&mut self) -> Receive8W<'_, IntensetSpec> {
        Receive8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&mut self) -> Receive9W<'_, IntensetSpec> {
        Receive9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&mut self) -> Receive10W<'_, IntensetSpec> {
        Receive10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&mut self) -> Receive11W<'_, IntensetSpec> {
        Receive11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&mut self) -> Receive12W<'_, IntensetSpec> {
        Receive12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&mut self) -> Receive13W<'_, IntensetSpec> {
        Receive13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&mut self) -> Receive14W<'_, IntensetSpec> {
        Receive14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&mut self) -> Receive15W<'_, IntensetSpec> {
        Receive15W::new(self, 15)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
