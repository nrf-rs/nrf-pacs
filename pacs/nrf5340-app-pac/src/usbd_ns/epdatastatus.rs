#[doc = "Register `EPDATASTATUS` reader"]
pub type R = crate::R<EpdatastatusSpec>;
#[doc = "Register `EPDATASTATUS` writer"]
pub type W = crate::W<EpdatastatusSpec>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin1 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin1> for bool {
    #[inline(always)]
    fn from(variant: Epin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN1` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin1R = crate::BitReader<Epin1>;
impl Epin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin1 {
        match self.bits {
            false => Epin1::NotDone,
            true => Epin1::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin1::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin1::DataDone
    }
}
#[doc = "Field `EPIN1` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin1W<'a, REG> = crate::BitWriter1C<'a, REG, Epin1>;
impl<'a, REG> Epin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin1::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin1::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin2 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin2> for bool {
    #[inline(always)]
    fn from(variant: Epin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN2` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin2R = crate::BitReader<Epin2>;
impl Epin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin2 {
        match self.bits {
            false => Epin2::NotDone,
            true => Epin2::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin2::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin2::DataDone
    }
}
#[doc = "Field `EPIN2` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin2W<'a, REG> = crate::BitWriter1C<'a, REG, Epin2>;
impl<'a, REG> Epin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin2::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin2::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin3 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin3> for bool {
    #[inline(always)]
    fn from(variant: Epin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN3` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin3R = crate::BitReader<Epin3>;
impl Epin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin3 {
        match self.bits {
            false => Epin3::NotDone,
            true => Epin3::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin3::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin3::DataDone
    }
}
#[doc = "Field `EPIN3` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin3W<'a, REG> = crate::BitWriter1C<'a, REG, Epin3>;
impl<'a, REG> Epin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin3::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin3::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin4 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin4> for bool {
    #[inline(always)]
    fn from(variant: Epin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN4` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin4R = crate::BitReader<Epin4>;
impl Epin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin4 {
        match self.bits {
            false => Epin4::NotDone,
            true => Epin4::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin4::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin4::DataDone
    }
}
#[doc = "Field `EPIN4` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin4W<'a, REG> = crate::BitWriter1C<'a, REG, Epin4>;
impl<'a, REG> Epin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin4::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin4::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin5 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin5> for bool {
    #[inline(always)]
    fn from(variant: Epin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN5` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin5R = crate::BitReader<Epin5>;
impl Epin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin5 {
        match self.bits {
            false => Epin5::NotDone,
            true => Epin5::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin5::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin5::DataDone
    }
}
#[doc = "Field `EPIN5` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin5W<'a, REG> = crate::BitWriter1C<'a, REG, Epin5>;
impl<'a, REG> Epin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin5::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin5::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin6 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin6> for bool {
    #[inline(always)]
    fn from(variant: Epin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN6` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin6R = crate::BitReader<Epin6>;
impl Epin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin6 {
        match self.bits {
            false => Epin6::NotDone,
            true => Epin6::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin6::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin6::DataDone
    }
}
#[doc = "Field `EPIN6` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin6W<'a, REG> = crate::BitWriter1C<'a, REG, Epin6>;
impl<'a, REG> Epin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin6::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin6::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin7 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotDone = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DataDone = 1,
}
impl From<Epin7> for bool {
    #[inline(always)]
    fn from(variant: Epin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN7` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin7R = crate::BitReader<Epin7>;
impl Epin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin7 {
        match self.bits {
            false => Epin7::NotDone,
            true => Epin7::DataDone,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == Epin7::NotDone
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin7::DataDone
    }
}
#[doc = "Field `EPIN7` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type Epin7W<'a, REG> = crate::BitWriter1C<'a, REG, Epin7>;
impl<'a, REG> Epin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin7::NotDone)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin7::DataDone)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout1 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout1> for bool {
    #[inline(always)]
    fn from(variant: Epout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT1` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout1R = crate::BitReader<Epout1>;
impl Epout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout1 {
        match self.bits {
            false => Epout1::NotStarted,
            true => Epout1::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout1::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout1::Started
    }
}
#[doc = "Field `EPOUT1` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout1W<'a, REG> = crate::BitWriter1C<'a, REG, Epout1>;
impl<'a, REG> Epout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout1::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout1::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout2 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout2> for bool {
    #[inline(always)]
    fn from(variant: Epout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT2` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout2R = crate::BitReader<Epout2>;
impl Epout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout2 {
        match self.bits {
            false => Epout2::NotStarted,
            true => Epout2::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout2::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout2::Started
    }
}
#[doc = "Field `EPOUT2` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout2W<'a, REG> = crate::BitWriter1C<'a, REG, Epout2>;
impl<'a, REG> Epout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout2::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout2::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout3 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout3> for bool {
    #[inline(always)]
    fn from(variant: Epout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT3` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout3R = crate::BitReader<Epout3>;
impl Epout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout3 {
        match self.bits {
            false => Epout3::NotStarted,
            true => Epout3::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout3::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout3::Started
    }
}
#[doc = "Field `EPOUT3` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout3W<'a, REG> = crate::BitWriter1C<'a, REG, Epout3>;
impl<'a, REG> Epout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout3::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout3::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout4 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout4> for bool {
    #[inline(always)]
    fn from(variant: Epout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT4` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout4R = crate::BitReader<Epout4>;
impl Epout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout4 {
        match self.bits {
            false => Epout4::NotStarted,
            true => Epout4::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout4::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout4::Started
    }
}
#[doc = "Field `EPOUT4` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout4W<'a, REG> = crate::BitWriter1C<'a, REG, Epout4>;
impl<'a, REG> Epout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout4::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout4::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout5 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout5> for bool {
    #[inline(always)]
    fn from(variant: Epout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT5` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout5R = crate::BitReader<Epout5>;
impl Epout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout5 {
        match self.bits {
            false => Epout5::NotStarted,
            true => Epout5::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout5::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout5::Started
    }
}
#[doc = "Field `EPOUT5` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout5W<'a, REG> = crate::BitWriter1C<'a, REG, Epout5>;
impl<'a, REG> Epout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout5::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout5::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout6 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout6> for bool {
    #[inline(always)]
    fn from(variant: Epout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT6` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout6R = crate::BitReader<Epout6>;
impl Epout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout6 {
        match self.bits {
            false => Epout6::NotStarted,
            true => Epout6::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout6::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout6::Started
    }
}
#[doc = "Field `EPOUT6` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout6W<'a, REG> = crate::BitWriter1C<'a, REG, Epout6>;
impl<'a, REG> Epout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout6::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout6::Started)
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout7 {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NotStarted = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    Started = 1,
}
impl From<Epout7> for bool {
    #[inline(always)]
    fn from(variant: Epout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT7` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout7R = crate::BitReader<Epout7>;
impl Epout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout7 {
        match self.bits {
            false => Epout7::NotStarted,
            true => Epout7::Started,
        }
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Epout7::NotStarted
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Epout7::Started
    }
}
#[doc = "Field `EPOUT7` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type Epout7W<'a, REG> = crate::BitWriter1C<'a, REG, Epout7>;
impl<'a, REG> Epout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout7::NotStarted)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Epout7::Started)
    }
}
impl R {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> Epin1R {
        Epin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> Epin2R {
        Epin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> Epin3R {
        Epin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> Epin4R {
        Epin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> Epin5R {
        Epin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> Epin6R {
        Epin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> Epin7R {
        Epin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> Epout1R {
        Epout1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> Epout2R {
        Epout2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> Epout3R {
        Epout3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> Epout4R {
        Epout4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> Epout5R {
        Epout5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> Epout6R {
        Epout6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> Epout7R {
        Epout7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> Epin1W<'_, EpdatastatusSpec> {
        Epin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> Epin2W<'_, EpdatastatusSpec> {
        Epin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> Epin3W<'_, EpdatastatusSpec> {
        Epin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> Epin4W<'_, EpdatastatusSpec> {
        Epin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> Epin5W<'_, EpdatastatusSpec> {
        Epin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> Epin6W<'_, EpdatastatusSpec> {
        Epin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> Epin7W<'_, EpdatastatusSpec> {
        Epin7W::new(self, 7)
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> Epout1W<'_, EpdatastatusSpec> {
        Epout1W::new(self, 17)
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> Epout2W<'_, EpdatastatusSpec> {
        Epout2W::new(self, 18)
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> Epout3W<'_, EpdatastatusSpec> {
        Epout3W::new(self, 19)
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> Epout4W<'_, EpdatastatusSpec> {
        Epout4W::new(self, 20)
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> Epout5W<'_, EpdatastatusSpec> {
        Epout5W::new(self, 21)
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> Epout6W<'_, EpdatastatusSpec> {
        Epout6W::new(self, 22)
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> Epout7W<'_, EpdatastatusSpec> {
        Epout7W::new(self, 23)
    }
}
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)\n\nYou can [`read`](crate::Reg::read) this register and get [`epdatastatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epdatastatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpdatastatusSpec;
impl crate::RegisterSpec for EpdatastatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epdatastatus::R`](R) reader structure"]
impl crate::Readable for EpdatastatusSpec {}
#[doc = "`write(|w| ..)` method takes [`epdatastatus::W`](W) writer structure"]
impl crate::Writable for EpdatastatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00fe_00fe;
}
#[doc = "`reset()` method sets EPDATASTATUS to value 0"]
impl crate::Resettable for EpdatastatusSpec {}
