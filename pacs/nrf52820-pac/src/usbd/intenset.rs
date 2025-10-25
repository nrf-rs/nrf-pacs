#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event USBRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbreset {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Usbreset> for bool {
    #[inline(always)]
    fn from(variant: Usbreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` reader - Write '1' to enable interrupt for event USBRESET"]
pub type UsbresetR = crate::BitReader<Usbreset>;
impl UsbresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbreset {
        match self.bits {
            false => Usbreset::Disabled,
            true => Usbreset::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbreset::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbreset::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event USBRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbresetWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<UsbresetWO> for bool {
    #[inline(always)]
    fn from(variant: UsbresetWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` writer - Write '1' to enable interrupt for event USBRESET"]
pub type UsbresetW<'a, REG> = crate::BitWriter<'a, REG, UsbresetWO>;
impl<'a, REG> UsbresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UsbresetWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Started {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Started> for bool {
    #[inline(always)]
    fn from(variant: Started) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Write '1' to enable interrupt for event STARTED"]
pub type StartedR = crate::BitReader<Started>;
impl StartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Started {
        match self.bits {
            false => Started::Disabled,
            true => Started::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Started::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Started::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StartedWO> for bool {
    #[inline(always)]
    fn from(variant: StartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to enable interrupt for event STARTED"]
pub type StartedW<'a, REG> = crate::BitWriter<'a, REG, StartedWO>;
impl<'a, REG> StartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StartedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin0> for bool {
    #[inline(always)]
    fn from(variant: Endepin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN0` reader - Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
pub type Endepin0R = crate::BitReader<Endepin0>;
impl Endepin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin0 {
        match self.bits {
            false => Endepin0::Disabled,
            true => Endepin0::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin0::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin0WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin0WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN0` writer - Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
pub type Endepin0W<'a, REG> = crate::BitWriter<'a, REG, Endepin0WO>;
impl<'a, REG> Endepin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin0WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin1> for bool {
    #[inline(always)]
    fn from(variant: Endepin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN1` reader - Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
pub type Endepin1R = crate::BitReader<Endepin1>;
impl Endepin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin1 {
        match self.bits {
            false => Endepin1::Disabled,
            true => Endepin1::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin1::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin1WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin1WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN1` writer - Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
pub type Endepin1W<'a, REG> = crate::BitWriter<'a, REG, Endepin1WO>;
impl<'a, REG> Endepin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin1WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin2> for bool {
    #[inline(always)]
    fn from(variant: Endepin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN2` reader - Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
pub type Endepin2R = crate::BitReader<Endepin2>;
impl Endepin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin2 {
        match self.bits {
            false => Endepin2::Disabled,
            true => Endepin2::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin2::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin2WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin2WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN2` writer - Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
pub type Endepin2W<'a, REG> = crate::BitWriter<'a, REG, Endepin2WO>;
impl<'a, REG> Endepin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin2WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin3> for bool {
    #[inline(always)]
    fn from(variant: Endepin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN3` reader - Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
pub type Endepin3R = crate::BitReader<Endepin3>;
impl Endepin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin3 {
        match self.bits {
            false => Endepin3::Disabled,
            true => Endepin3::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin3::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin3WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin3WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN3` writer - Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
pub type Endepin3W<'a, REG> = crate::BitWriter<'a, REG, Endepin3WO>;
impl<'a, REG> Endepin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin3WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin4 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin4> for bool {
    #[inline(always)]
    fn from(variant: Endepin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN4` reader - Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
pub type Endepin4R = crate::BitReader<Endepin4>;
impl Endepin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin4 {
        match self.bits {
            false => Endepin4::Disabled,
            true => Endepin4::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin4::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin4::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin4WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin4WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN4` writer - Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
pub type Endepin4W<'a, REG> = crate::BitWriter<'a, REG, Endepin4WO>;
impl<'a, REG> Endepin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin4WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin5 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin5> for bool {
    #[inline(always)]
    fn from(variant: Endepin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN5` reader - Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
pub type Endepin5R = crate::BitReader<Endepin5>;
impl Endepin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin5 {
        match self.bits {
            false => Endepin5::Disabled,
            true => Endepin5::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin5::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin5::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin5WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin5WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN5` writer - Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
pub type Endepin5W<'a, REG> = crate::BitWriter<'a, REG, Endepin5WO>;
impl<'a, REG> Endepin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin5WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin6 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin6> for bool {
    #[inline(always)]
    fn from(variant: Endepin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN6` reader - Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
pub type Endepin6R = crate::BitReader<Endepin6>;
impl Endepin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin6 {
        match self.bits {
            false => Endepin6::Disabled,
            true => Endepin6::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin6::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin6::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin6WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin6WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN6` writer - Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
pub type Endepin6W<'a, REG> = crate::BitWriter<'a, REG, Endepin6WO>;
impl<'a, REG> Endepin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin6WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin7 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepin7> for bool {
    #[inline(always)]
    fn from(variant: Endepin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN7` reader - Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
pub type Endepin7R = crate::BitReader<Endepin7>;
impl Endepin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepin7 {
        match self.bits {
            false => Endepin7::Disabled,
            true => Endepin7::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepin7::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepin7::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPIN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepin7WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepin7WO> for bool {
    #[inline(always)]
    fn from(variant: Endepin7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN7` writer - Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
pub type Endepin7W<'a, REG> = crate::BitWriter<'a, REG, Endepin7WO>;
impl<'a, REG> Endepin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepin7WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event EP0DATADONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0datadone {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ep0datadone> for bool {
    #[inline(always)]
    fn from(variant: Ep0datadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE` reader - Write '1' to enable interrupt for event EP0DATADONE"]
pub type Ep0datadoneR = crate::BitReader<Ep0datadone>;
impl Ep0datadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0datadone {
        match self.bits {
            false => Ep0datadone::Disabled,
            true => Ep0datadone::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ep0datadone::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ep0datadone::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event EP0DATADONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0datadoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ep0datadoneWO> for bool {
    #[inline(always)]
    fn from(variant: Ep0datadoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE` writer - Write '1' to enable interrupt for event EP0DATADONE"]
pub type Ep0datadoneW<'a, REG> = crate::BitWriter<'a, REG, Ep0datadoneWO>;
impl<'a, REG> Ep0datadoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0datadoneWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDISOIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endisoin {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endisoin> for bool {
    #[inline(always)]
    fn from(variant: Endisoin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOIN` reader - Write '1' to enable interrupt for event ENDISOIN"]
pub type EndisoinR = crate::BitReader<Endisoin>;
impl EndisoinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endisoin {
        match self.bits {
            false => Endisoin::Disabled,
            true => Endisoin::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endisoin::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endisoin::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDISOIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndisoinWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndisoinWO> for bool {
    #[inline(always)]
    fn from(variant: EndisoinWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOIN` writer - Write '1' to enable interrupt for event ENDISOIN"]
pub type EndisoinW<'a, REG> = crate::BitWriter<'a, REG, EndisoinWO>;
impl<'a, REG> EndisoinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndisoinWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout0> for bool {
    #[inline(always)]
    fn from(variant: Endepout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0` reader - Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
pub type Endepout0R = crate::BitReader<Endepout0>;
impl Endepout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout0 {
        match self.bits {
            false => Endepout0::Disabled,
            true => Endepout0::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout0::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout0WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout0WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0` writer - Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
pub type Endepout0W<'a, REG> = crate::BitWriter<'a, REG, Endepout0WO>;
impl<'a, REG> Endepout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout0WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout1> for bool {
    #[inline(always)]
    fn from(variant: Endepout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT1` reader - Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
pub type Endepout1R = crate::BitReader<Endepout1>;
impl Endepout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout1 {
        match self.bits {
            false => Endepout1::Disabled,
            true => Endepout1::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout1::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout1WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout1WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT1` writer - Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
pub type Endepout1W<'a, REG> = crate::BitWriter<'a, REG, Endepout1WO>;
impl<'a, REG> Endepout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout1WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout2> for bool {
    #[inline(always)]
    fn from(variant: Endepout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT2` reader - Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
pub type Endepout2R = crate::BitReader<Endepout2>;
impl Endepout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout2 {
        match self.bits {
            false => Endepout2::Disabled,
            true => Endepout2::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout2::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout2WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout2WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT2` writer - Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
pub type Endepout2W<'a, REG> = crate::BitWriter<'a, REG, Endepout2WO>;
impl<'a, REG> Endepout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout2WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout3> for bool {
    #[inline(always)]
    fn from(variant: Endepout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT3` reader - Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
pub type Endepout3R = crate::BitReader<Endepout3>;
impl Endepout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout3 {
        match self.bits {
            false => Endepout3::Disabled,
            true => Endepout3::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout3::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout3WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout3WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT3` writer - Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
pub type Endepout3W<'a, REG> = crate::BitWriter<'a, REG, Endepout3WO>;
impl<'a, REG> Endepout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout3WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout4 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout4> for bool {
    #[inline(always)]
    fn from(variant: Endepout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT4` reader - Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
pub type Endepout4R = crate::BitReader<Endepout4>;
impl Endepout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout4 {
        match self.bits {
            false => Endepout4::Disabled,
            true => Endepout4::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout4::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout4::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout4WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout4WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT4` writer - Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
pub type Endepout4W<'a, REG> = crate::BitWriter<'a, REG, Endepout4WO>;
impl<'a, REG> Endepout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout4WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout5 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout5> for bool {
    #[inline(always)]
    fn from(variant: Endepout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT5` reader - Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
pub type Endepout5R = crate::BitReader<Endepout5>;
impl Endepout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout5 {
        match self.bits {
            false => Endepout5::Disabled,
            true => Endepout5::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout5::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout5::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout5WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout5WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT5` writer - Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
pub type Endepout5W<'a, REG> = crate::BitWriter<'a, REG, Endepout5WO>;
impl<'a, REG> Endepout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout5WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout6 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout6> for bool {
    #[inline(always)]
    fn from(variant: Endepout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT6` reader - Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
pub type Endepout6R = crate::BitReader<Endepout6>;
impl Endepout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout6 {
        match self.bits {
            false => Endepout6::Disabled,
            true => Endepout6::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout6::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout6::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout6WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout6WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT6` writer - Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
pub type Endepout6W<'a, REG> = crate::BitWriter<'a, REG, Endepout6WO>;
impl<'a, REG> Endepout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout6WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout7 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endepout7> for bool {
    #[inline(always)]
    fn from(variant: Endepout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT7` reader - Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
pub type Endepout7R = crate::BitReader<Endepout7>;
impl Endepout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endepout7 {
        match self.bits {
            false => Endepout7::Disabled,
            true => Endepout7::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endepout7::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endepout7::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDEPOUT\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endepout7WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Endepout7WO> for bool {
    #[inline(always)]
    fn from(variant: Endepout7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT7` writer - Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
pub type Endepout7W<'a, REG> = crate::BitWriter<'a, REG, Endepout7WO>;
impl<'a, REG> Endepout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Endepout7WO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ENDISOOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endisoout {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endisoout> for bool {
    #[inline(always)]
    fn from(variant: Endisoout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOOUT` reader - Write '1' to enable interrupt for event ENDISOOUT"]
pub type EndisooutR = crate::BitReader<Endisoout>;
impl EndisooutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endisoout {
        match self.bits {
            false => Endisoout::Disabled,
            true => Endisoout::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endisoout::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endisoout::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ENDISOOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndisooutWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndisooutWO> for bool {
    #[inline(always)]
    fn from(variant: EndisooutWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOOUT` writer - Write '1' to enable interrupt for event ENDISOOUT"]
pub type EndisooutW<'a, REG> = crate::BitWriter<'a, REG, EndisooutWO>;
impl<'a, REG> EndisooutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndisooutWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Write '1' to enable interrupt for event SOF"]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::Disabled,
            true => Sof::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sof::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sof::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SofWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<SofWO> for bool {
    #[inline(always)]
    fn from(variant: SofWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` writer - Write '1' to enable interrupt for event SOF"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, SofWO>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(SofWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event USBEVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbevent {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Usbevent> for bool {
    #[inline(always)]
    fn from(variant: Usbevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEVENT` reader - Write '1' to enable interrupt for event USBEVENT"]
pub type UsbeventR = crate::BitReader<Usbevent>;
impl UsbeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbevent {
        match self.bits {
            false => Usbevent::Disabled,
            true => Usbevent::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbevent::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbevent::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event USBEVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbeventWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<UsbeventWO> for bool {
    #[inline(always)]
    fn from(variant: UsbeventWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEVENT` writer - Write '1' to enable interrupt for event USBEVENT"]
pub type UsbeventW<'a, REG> = crate::BitWriter<'a, REG, UsbeventWO>;
impl<'a, REG> UsbeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UsbeventWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event EP0SETUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0setup {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ep0setup> for bool {
    #[inline(always)]
    fn from(variant: Ep0setup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0SETUP` reader - Write '1' to enable interrupt for event EP0SETUP"]
pub type Ep0setupR = crate::BitReader<Ep0setup>;
impl Ep0setupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0setup {
        match self.bits {
            false => Ep0setup::Disabled,
            true => Ep0setup::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ep0setup::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ep0setup::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event EP0SETUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0setupWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Ep0setupWO> for bool {
    #[inline(always)]
    fn from(variant: Ep0setupWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0SETUP` writer - Write '1' to enable interrupt for event EP0SETUP"]
pub type Ep0setupW<'a, REG> = crate::BitWriter<'a, REG, Ep0setupWO>;
impl<'a, REG> Ep0setupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0setupWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event EPDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epdata {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Epdata> for bool {
    #[inline(always)]
    fn from(variant: Epdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDATA` reader - Write '1' to enable interrupt for event EPDATA"]
pub type EpdataR = crate::BitReader<Epdata>;
impl EpdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epdata {
        match self.bits {
            false => Epdata::Disabled,
            true => Epdata::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Epdata::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Epdata::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event EPDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpdataWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EpdataWO> for bool {
    #[inline(always)]
    fn from(variant: EpdataWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDATA` writer - Write '1' to enable interrupt for event EPDATA"]
pub type EpdataW<'a, REG> = crate::BitWriter<'a, REG, EpdataWO>;
impl<'a, REG> EpdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EpdataWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&self) -> UsbresetR {
        UsbresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> StartedR {
        StartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&self) -> Endepin0R {
        Endepin0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&self) -> Endepin1R {
        Endepin1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&self) -> Endepin2R {
        Endepin2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&self) -> Endepin3R {
        Endepin3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&self) -> Endepin4R {
        Endepin4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&self) -> Endepin5R {
        Endepin5R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&self) -> Endepin6R {
        Endepin6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&self) -> Endepin7R {
        Endepin7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&self) -> Ep0datadoneR {
        Ep0datadoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&self) -> EndisoinR {
        EndisoinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&self) -> Endepout0R {
        Endepout0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&self) -> Endepout1R {
        Endepout1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&self) -> Endepout2R {
        Endepout2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&self) -> Endepout3R {
        Endepout3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&self) -> Endepout4R {
        Endepout4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&self) -> Endepout5R {
        Endepout5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&self) -> Endepout6R {
        Endepout6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&self) -> Endepout7R {
        Endepout7R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&self) -> EndisooutR {
        EndisooutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&self) -> UsbeventR {
        UsbeventR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&self) -> Ep0setupR {
        Ep0setupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&self) -> EpdataR {
        EpdataR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> UsbresetW<'_, IntensetSpec> {
        UsbresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> StartedW<'_, IntensetSpec> {
        StartedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&mut self) -> Endepin0W<'_, IntensetSpec> {
        Endepin0W::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&mut self) -> Endepin1W<'_, IntensetSpec> {
        Endepin1W::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&mut self) -> Endepin2W<'_, IntensetSpec> {
        Endepin2W::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&mut self) -> Endepin3W<'_, IntensetSpec> {
        Endepin3W::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&mut self) -> Endepin4W<'_, IntensetSpec> {
        Endepin4W::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&mut self) -> Endepin5W<'_, IntensetSpec> {
        Endepin5W::new(self, 7)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&mut self) -> Endepin6W<'_, IntensetSpec> {
        Endepin6W::new(self, 8)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&mut self) -> Endepin7W<'_, IntensetSpec> {
        Endepin7W::new(self, 9)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&mut self) -> Ep0datadoneW<'_, IntensetSpec> {
        Ep0datadoneW::new(self, 10)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&mut self) -> EndisoinW<'_, IntensetSpec> {
        EndisoinW::new(self, 11)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&mut self) -> Endepout0W<'_, IntensetSpec> {
        Endepout0W::new(self, 12)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&mut self) -> Endepout1W<'_, IntensetSpec> {
        Endepout1W::new(self, 13)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&mut self) -> Endepout2W<'_, IntensetSpec> {
        Endepout2W::new(self, 14)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&mut self) -> Endepout3W<'_, IntensetSpec> {
        Endepout3W::new(self, 15)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&mut self) -> Endepout4W<'_, IntensetSpec> {
        Endepout4W::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&mut self) -> Endepout5W<'_, IntensetSpec> {
        Endepout5W::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&mut self) -> Endepout6W<'_, IntensetSpec> {
        Endepout6W::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&mut self) -> Endepout7W<'_, IntensetSpec> {
        Endepout7W::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&mut self) -> EndisooutW<'_, IntensetSpec> {
        EndisooutW::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, IntensetSpec> {
        SofW::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&mut self) -> UsbeventW<'_, IntensetSpec> {
        UsbeventW::new(self, 22)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&mut self) -> Ep0setupW<'_, IntensetSpec> {
        Ep0setupW::new(self, 23)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&mut self) -> EpdataW<'_, IntensetSpec> {
        EpdataW::new(self, 24)
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
