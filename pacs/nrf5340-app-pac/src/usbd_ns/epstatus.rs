#[doc = "Register `EPSTATUS` reader"]
pub type R = crate::R<EpstatusSpec>;
#[doc = "Register `EPSTATUS` writer"]
pub type W = crate::W<EpstatusSpec>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin0 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin0> for bool {
    #[inline(always)]
    fn from(variant: Epin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN0` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin0R = crate::BitReader<Epin0>;
impl Epin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin0 {
        match self.bits {
            false => Epin0::NoData,
            true => Epin0::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin0::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin0::DataDone
    }
}
#[doc = "Field `EPIN0` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin0W<'a, REG> = crate::BitWriter1C<'a, REG, Epin0>;
impl<'a, REG> Epin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin0::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin0::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin1 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin1> for bool {
    #[inline(always)]
    fn from(variant: Epin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN1` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin1R = crate::BitReader<Epin1>;
impl Epin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin1 {
        match self.bits {
            false => Epin1::NoData,
            true => Epin1::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin1::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin1::DataDone
    }
}
#[doc = "Field `EPIN1` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin1W<'a, REG> = crate::BitWriter1C<'a, REG, Epin1>;
impl<'a, REG> Epin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin1::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin1::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin2 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin2> for bool {
    #[inline(always)]
    fn from(variant: Epin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN2` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin2R = crate::BitReader<Epin2>;
impl Epin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin2 {
        match self.bits {
            false => Epin2::NoData,
            true => Epin2::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin2::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin2::DataDone
    }
}
#[doc = "Field `EPIN2` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin2W<'a, REG> = crate::BitWriter1C<'a, REG, Epin2>;
impl<'a, REG> Epin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin2::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin2::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin3 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin3> for bool {
    #[inline(always)]
    fn from(variant: Epin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN3` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin3R = crate::BitReader<Epin3>;
impl Epin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin3 {
        match self.bits {
            false => Epin3::NoData,
            true => Epin3::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin3::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin3::DataDone
    }
}
#[doc = "Field `EPIN3` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin3W<'a, REG> = crate::BitWriter1C<'a, REG, Epin3>;
impl<'a, REG> Epin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin3::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin3::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin4 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin4> for bool {
    #[inline(always)]
    fn from(variant: Epin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN4` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin4R = crate::BitReader<Epin4>;
impl Epin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin4 {
        match self.bits {
            false => Epin4::NoData,
            true => Epin4::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin4::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin4::DataDone
    }
}
#[doc = "Field `EPIN4` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin4W<'a, REG> = crate::BitWriter1C<'a, REG, Epin4>;
impl<'a, REG> Epin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin4::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin4::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin5 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin5> for bool {
    #[inline(always)]
    fn from(variant: Epin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN5` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin5R = crate::BitReader<Epin5>;
impl Epin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin5 {
        match self.bits {
            false => Epin5::NoData,
            true => Epin5::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin5::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin5::DataDone
    }
}
#[doc = "Field `EPIN5` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin5W<'a, REG> = crate::BitWriter1C<'a, REG, Epin5>;
impl<'a, REG> Epin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin5::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin5::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin6 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin6> for bool {
    #[inline(always)]
    fn from(variant: Epin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN6` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin6R = crate::BitReader<Epin6>;
impl Epin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin6 {
        match self.bits {
            false => Epin6::NoData,
            true => Epin6::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin6::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin6::DataDone
    }
}
#[doc = "Field `EPIN6` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin6W<'a, REG> = crate::BitWriter1C<'a, REG, Epin6>;
impl<'a, REG> Epin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin6::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin6::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin7 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin7> for bool {
    #[inline(always)]
    fn from(variant: Epin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN7` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin7R = crate::BitReader<Epin7>;
impl Epin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin7 {
        match self.bits {
            false => Epin7::NoData,
            true => Epin7::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin7::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin7::DataDone
    }
}
#[doc = "Field `EPIN7` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin7W<'a, REG> = crate::BitWriter1C<'a, REG, Epin7>;
impl<'a, REG> Epin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin7::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin7::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epin8 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epin8> for bool {
    #[inline(always)]
    fn from(variant: Epin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN8` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin8R = crate::BitReader<Epin8>;
impl Epin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epin8 {
        match self.bits {
            false => Epin8::NoData,
            true => Epin8::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epin8::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epin8::DataDone
    }
}
#[doc = "Field `EPIN8` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epin8W<'a, REG> = crate::BitWriter1C<'a, REG, Epin8>;
impl<'a, REG> Epin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epin8::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epin8::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout0 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout0> for bool {
    #[inline(always)]
    fn from(variant: Epout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT0` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout0R = crate::BitReader<Epout0>;
impl Epout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout0 {
        match self.bits {
            false => Epout0::NoData,
            true => Epout0::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout0::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout0::DataDone
    }
}
#[doc = "Field `EPOUT0` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout0W<'a, REG> = crate::BitWriter1C<'a, REG, Epout0>;
impl<'a, REG> Epout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout0::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout0::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout1 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout1> for bool {
    #[inline(always)]
    fn from(variant: Epout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT1` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout1R = crate::BitReader<Epout1>;
impl Epout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout1 {
        match self.bits {
            false => Epout1::NoData,
            true => Epout1::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout1::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout1::DataDone
    }
}
#[doc = "Field `EPOUT1` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout1W<'a, REG> = crate::BitWriter1C<'a, REG, Epout1>;
impl<'a, REG> Epout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout1::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout1::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout2 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout2> for bool {
    #[inline(always)]
    fn from(variant: Epout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT2` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout2R = crate::BitReader<Epout2>;
impl Epout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout2 {
        match self.bits {
            false => Epout2::NoData,
            true => Epout2::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout2::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout2::DataDone
    }
}
#[doc = "Field `EPOUT2` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout2W<'a, REG> = crate::BitWriter1C<'a, REG, Epout2>;
impl<'a, REG> Epout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout2::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout2::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout3 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout3> for bool {
    #[inline(always)]
    fn from(variant: Epout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT3` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout3R = crate::BitReader<Epout3>;
impl Epout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout3 {
        match self.bits {
            false => Epout3::NoData,
            true => Epout3::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout3::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout3::DataDone
    }
}
#[doc = "Field `EPOUT3` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout3W<'a, REG> = crate::BitWriter1C<'a, REG, Epout3>;
impl<'a, REG> Epout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout3::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout3::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout4 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout4> for bool {
    #[inline(always)]
    fn from(variant: Epout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT4` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout4R = crate::BitReader<Epout4>;
impl Epout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout4 {
        match self.bits {
            false => Epout4::NoData,
            true => Epout4::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout4::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout4::DataDone
    }
}
#[doc = "Field `EPOUT4` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout4W<'a, REG> = crate::BitWriter1C<'a, REG, Epout4>;
impl<'a, REG> Epout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout4::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout4::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout5 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout5> for bool {
    #[inline(always)]
    fn from(variant: Epout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT5` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout5R = crate::BitReader<Epout5>;
impl Epout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout5 {
        match self.bits {
            false => Epout5::NoData,
            true => Epout5::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout5::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout5::DataDone
    }
}
#[doc = "Field `EPOUT5` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout5W<'a, REG> = crate::BitWriter1C<'a, REG, Epout5>;
impl<'a, REG> Epout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout5::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout5::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout6 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout6> for bool {
    #[inline(always)]
    fn from(variant: Epout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT6` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout6R = crate::BitReader<Epout6>;
impl Epout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout6 {
        match self.bits {
            false => Epout6::NoData,
            true => Epout6::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout6::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout6::DataDone
    }
}
#[doc = "Field `EPOUT6` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout6W<'a, REG> = crate::BitWriter1C<'a, REG, Epout6>;
impl<'a, REG> Epout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout6::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout6::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout7 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout7> for bool {
    #[inline(always)]
    fn from(variant: Epout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT7` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout7R = crate::BitReader<Epout7>;
impl Epout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout7 {
        match self.bits {
            false => Epout7::NoData,
            true => Epout7::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout7::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout7::DataDone
    }
}
#[doc = "Field `EPOUT7` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout7W<'a, REG> = crate::BitWriter1C<'a, REG, Epout7>;
impl<'a, REG> Epout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout7::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout7::DataDone)
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epout8 {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NoData = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DataDone = 1,
}
impl From<Epout8> for bool {
    #[inline(always)]
    fn from(variant: Epout8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT8` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout8R = crate::BitReader<Epout8>;
impl Epout8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epout8 {
        match self.bits {
            false => Epout8::NoData,
            true => Epout8::DataDone,
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == Epout8::NoData
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == Epout8::DataDone
    }
}
#[doc = "Field `EPOUT8` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type Epout8W<'a, REG> = crate::BitWriter1C<'a, REG, Epout8>;
impl<'a, REG> Epout8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(Epout8::NoData)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut crate::W<REG> {
        self.variant(Epout8::DataDone)
    }
}
impl R {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&self) -> Epin0R {
        Epin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> Epin1R {
        Epin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> Epin2R {
        Epin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> Epin3R {
        Epin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> Epin4R {
        Epin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> Epin5R {
        Epin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> Epin6R {
        Epin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> Epin7R {
        Epin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&self) -> Epin8R {
        Epin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&self) -> Epout0R {
        Epout0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> Epout1R {
        Epout1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> Epout2R {
        Epout2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> Epout3R {
        Epout3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> Epout4R {
        Epout4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> Epout5R {
        Epout5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> Epout6R {
        Epout6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> Epout7R {
        Epout7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&self) -> Epout8R {
        Epout8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&mut self) -> Epin0W<'_, EpstatusSpec> {
        Epin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> Epin1W<'_, EpstatusSpec> {
        Epin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> Epin2W<'_, EpstatusSpec> {
        Epin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> Epin3W<'_, EpstatusSpec> {
        Epin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> Epin4W<'_, EpstatusSpec> {
        Epin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> Epin5W<'_, EpstatusSpec> {
        Epin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> Epin6W<'_, EpstatusSpec> {
        Epin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> Epin7W<'_, EpstatusSpec> {
        Epin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&mut self) -> Epin8W<'_, EpstatusSpec> {
        Epin8W::new(self, 8)
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&mut self) -> Epout0W<'_, EpstatusSpec> {
        Epout0W::new(self, 16)
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> Epout1W<'_, EpstatusSpec> {
        Epout1W::new(self, 17)
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> Epout2W<'_, EpstatusSpec> {
        Epout2W::new(self, 18)
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> Epout3W<'_, EpstatusSpec> {
        Epout3W::new(self, 19)
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> Epout4W<'_, EpstatusSpec> {
        Epout4W::new(self, 20)
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> Epout5W<'_, EpstatusSpec> {
        Epout5W::new(self, 21)
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> Epout6W<'_, EpstatusSpec> {
        Epout6W::new(self, 22)
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> Epout7W<'_, EpstatusSpec> {
        Epout7W::new(self, 23)
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&mut self) -> Epout8W<'_, EpstatusSpec> {
        Epout8W::new(self, 24)
    }
}
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured\n\nYou can [`read`](crate::Reg::read) this register and get [`epstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpstatusSpec;
impl crate::RegisterSpec for EpstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epstatus::R`](R) reader structure"]
impl crate::Readable for EpstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`epstatus::W`](W) writer structure"]
impl crate::Writable for EpstatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01ff_01ff;
}
#[doc = "`reset()` method sets EPSTATUS to value 0"]
impl crate::Resettable for EpstatusSpec {}
