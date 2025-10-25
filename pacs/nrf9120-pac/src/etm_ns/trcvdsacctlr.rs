#[doc = "Register `TRCVDSACCTLR` reader"]
pub type R = crate::R<TrcvdsacctlrSpec>;
#[doc = "Register `TRCVDSACCTLR` writer"]
pub type W = crate::W<TrcvdsacctlrSpec>;
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include0 {
    #[doc = "0: The single address comparator 0, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 0, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include0> for bool {
    #[inline(always)]
    fn from(variant: Include0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_0` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include0R = crate::BitReader<Include0>;
impl Include0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include0 {
        match self.bits {
            false => Include0::Disabled,
            true => Include0::Enabled,
        }
    }
    #[doc = "The single address comparator 0, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include0::Disabled
    }
    #[doc = "The single address comparator 0, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include0::Enabled
    }
}
#[doc = "Field `INCLUDE_0` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include0W<'a, REG> = crate::BitWriter<'a, REG, Include0>;
impl<'a, REG> Include0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 0, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include0::Disabled)
    }
    #[doc = "The single address comparator 0, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include0::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include1 {
    #[doc = "0: The single address comparator 1, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 1, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include1> for bool {
    #[inline(always)]
    fn from(variant: Include1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_1` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include1R = crate::BitReader<Include1>;
impl Include1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include1 {
        match self.bits {
            false => Include1::Disabled,
            true => Include1::Enabled,
        }
    }
    #[doc = "The single address comparator 1, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include1::Disabled
    }
    #[doc = "The single address comparator 1, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include1::Enabled
    }
}
#[doc = "Field `INCLUDE_1` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include1W<'a, REG> = crate::BitWriter<'a, REG, Include1>;
impl<'a, REG> Include1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 1, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include1::Disabled)
    }
    #[doc = "The single address comparator 1, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include1::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include2 {
    #[doc = "0: The single address comparator 2, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 2, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include2> for bool {
    #[inline(always)]
    fn from(variant: Include2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_2` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include2R = crate::BitReader<Include2>;
impl Include2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include2 {
        match self.bits {
            false => Include2::Disabled,
            true => Include2::Enabled,
        }
    }
    #[doc = "The single address comparator 2, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include2::Disabled
    }
    #[doc = "The single address comparator 2, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include2::Enabled
    }
}
#[doc = "Field `INCLUDE_2` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include2W<'a, REG> = crate::BitWriter<'a, REG, Include2>;
impl<'a, REG> Include2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 2, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include2::Disabled)
    }
    #[doc = "The single address comparator 2, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include2::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include3 {
    #[doc = "0: The single address comparator 3, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 3, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include3> for bool {
    #[inline(always)]
    fn from(variant: Include3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_3` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include3R = crate::BitReader<Include3>;
impl Include3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include3 {
        match self.bits {
            false => Include3::Disabled,
            true => Include3::Enabled,
        }
    }
    #[doc = "The single address comparator 3, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include3::Disabled
    }
    #[doc = "The single address comparator 3, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include3::Enabled
    }
}
#[doc = "Field `INCLUDE_3` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include3W<'a, REG> = crate::BitWriter<'a, REG, Include3>;
impl<'a, REG> Include3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 3, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include3::Disabled)
    }
    #[doc = "The single address comparator 3, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include3::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include4 {
    #[doc = "0: The single address comparator 4, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 4, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include4> for bool {
    #[inline(always)]
    fn from(variant: Include4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_4` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include4R = crate::BitReader<Include4>;
impl Include4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include4 {
        match self.bits {
            false => Include4::Disabled,
            true => Include4::Enabled,
        }
    }
    #[doc = "The single address comparator 4, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include4::Disabled
    }
    #[doc = "The single address comparator 4, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include4::Enabled
    }
}
#[doc = "Field `INCLUDE_4` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include4W<'a, REG> = crate::BitWriter<'a, REG, Include4>;
impl<'a, REG> Include4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 4, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include4::Disabled)
    }
    #[doc = "The single address comparator 4, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include4::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include5 {
    #[doc = "0: The single address comparator 5, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 5, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include5> for bool {
    #[inline(always)]
    fn from(variant: Include5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_5` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include5R = crate::BitReader<Include5>;
impl Include5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include5 {
        match self.bits {
            false => Include5::Disabled,
            true => Include5::Enabled,
        }
    }
    #[doc = "The single address comparator 5, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include5::Disabled
    }
    #[doc = "The single address comparator 5, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include5::Enabled
    }
}
#[doc = "Field `INCLUDE_5` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include5W<'a, REG> = crate::BitWriter<'a, REG, Include5>;
impl<'a, REG> Include5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 5, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include5::Disabled)
    }
    #[doc = "The single address comparator 5, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include5::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include6 {
    #[doc = "0: The single address comparator 6, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 6, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include6> for bool {
    #[inline(always)]
    fn from(variant: Include6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_6` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include6R = crate::BitReader<Include6>;
impl Include6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include6 {
        match self.bits {
            false => Include6::Disabled,
            true => Include6::Enabled,
        }
    }
    #[doc = "The single address comparator 6, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include6::Disabled
    }
    #[doc = "The single address comparator 6, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include6::Enabled
    }
}
#[doc = "Field `INCLUDE_6` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include6W<'a, REG> = crate::BitWriter<'a, REG, Include6>;
impl<'a, REG> Include6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 6, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include6::Disabled)
    }
    #[doc = "The single address comparator 6, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include6::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData include control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Include7 {
    #[doc = "0: The single address comparator 7, is not selected for ViewData include control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 7, is selected for ViewData include control."]
    Enabled = 1,
}
impl From<Include7> for bool {
    #[inline(always)]
    fn from(variant: Include7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INCLUDE_7` reader - Selects which single address comparators are in use with ViewData include control."]
pub type Include7R = crate::BitReader<Include7>;
impl Include7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Include7 {
        match self.bits {
            false => Include7::Disabled,
            true => Include7::Enabled,
        }
    }
    #[doc = "The single address comparator 7, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Include7::Disabled
    }
    #[doc = "The single address comparator 7, is selected for ViewData include control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Include7::Enabled
    }
}
#[doc = "Field `INCLUDE_7` writer - Selects which single address comparators are in use with ViewData include control."]
pub type Include7W<'a, REG> = crate::BitWriter<'a, REG, Include7>;
impl<'a, REG> Include7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 7, is not selected for ViewData include control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include7::Disabled)
    }
    #[doc = "The single address comparator 7, is selected for ViewData include control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Include7::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude0 {
    #[doc = "0: The single address comparator 0, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 0, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude0> for bool {
    #[inline(always)]
    fn from(variant: Exclude0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_0` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude0R = crate::BitReader<Exclude0>;
impl Exclude0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude0 {
        match self.bits {
            false => Exclude0::Disabled,
            true => Exclude0::Enabled,
        }
    }
    #[doc = "The single address comparator 0, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude0::Disabled
    }
    #[doc = "The single address comparator 0, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude0::Enabled
    }
}
#[doc = "Field `EXCLUDE_0` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude0W<'a, REG> = crate::BitWriter<'a, REG, Exclude0>;
impl<'a, REG> Exclude0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 0, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude0::Disabled)
    }
    #[doc = "The single address comparator 0, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude0::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude1 {
    #[doc = "0: The single address comparator 1, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 1, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude1> for bool {
    #[inline(always)]
    fn from(variant: Exclude1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_1` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude1R = crate::BitReader<Exclude1>;
impl Exclude1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude1 {
        match self.bits {
            false => Exclude1::Disabled,
            true => Exclude1::Enabled,
        }
    }
    #[doc = "The single address comparator 1, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude1::Disabled
    }
    #[doc = "The single address comparator 1, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude1::Enabled
    }
}
#[doc = "Field `EXCLUDE_1` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude1W<'a, REG> = crate::BitWriter<'a, REG, Exclude1>;
impl<'a, REG> Exclude1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 1, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude1::Disabled)
    }
    #[doc = "The single address comparator 1, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude1::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude2 {
    #[doc = "0: The single address comparator 2, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 2, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude2> for bool {
    #[inline(always)]
    fn from(variant: Exclude2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_2` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude2R = crate::BitReader<Exclude2>;
impl Exclude2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude2 {
        match self.bits {
            false => Exclude2::Disabled,
            true => Exclude2::Enabled,
        }
    }
    #[doc = "The single address comparator 2, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude2::Disabled
    }
    #[doc = "The single address comparator 2, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude2::Enabled
    }
}
#[doc = "Field `EXCLUDE_2` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude2W<'a, REG> = crate::BitWriter<'a, REG, Exclude2>;
impl<'a, REG> Exclude2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 2, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude2::Disabled)
    }
    #[doc = "The single address comparator 2, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude2::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude3 {
    #[doc = "0: The single address comparator 3, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 3, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude3> for bool {
    #[inline(always)]
    fn from(variant: Exclude3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_3` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude3R = crate::BitReader<Exclude3>;
impl Exclude3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude3 {
        match self.bits {
            false => Exclude3::Disabled,
            true => Exclude3::Enabled,
        }
    }
    #[doc = "The single address comparator 3, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude3::Disabled
    }
    #[doc = "The single address comparator 3, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude3::Enabled
    }
}
#[doc = "Field `EXCLUDE_3` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude3W<'a, REG> = crate::BitWriter<'a, REG, Exclude3>;
impl<'a, REG> Exclude3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 3, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude3::Disabled)
    }
    #[doc = "The single address comparator 3, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude3::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude4 {
    #[doc = "0: The single address comparator 4, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 4, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude4> for bool {
    #[inline(always)]
    fn from(variant: Exclude4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_4` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude4R = crate::BitReader<Exclude4>;
impl Exclude4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude4 {
        match self.bits {
            false => Exclude4::Disabled,
            true => Exclude4::Enabled,
        }
    }
    #[doc = "The single address comparator 4, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude4::Disabled
    }
    #[doc = "The single address comparator 4, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude4::Enabled
    }
}
#[doc = "Field `EXCLUDE_4` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude4W<'a, REG> = crate::BitWriter<'a, REG, Exclude4>;
impl<'a, REG> Exclude4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 4, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude4::Disabled)
    }
    #[doc = "The single address comparator 4, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude4::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude5 {
    #[doc = "0: The single address comparator 5, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 5, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude5> for bool {
    #[inline(always)]
    fn from(variant: Exclude5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_5` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude5R = crate::BitReader<Exclude5>;
impl Exclude5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude5 {
        match self.bits {
            false => Exclude5::Disabled,
            true => Exclude5::Enabled,
        }
    }
    #[doc = "The single address comparator 5, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude5::Disabled
    }
    #[doc = "The single address comparator 5, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude5::Enabled
    }
}
#[doc = "Field `EXCLUDE_5` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude5W<'a, REG> = crate::BitWriter<'a, REG, Exclude5>;
impl<'a, REG> Exclude5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 5, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude5::Disabled)
    }
    #[doc = "The single address comparator 5, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude5::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude6 {
    #[doc = "0: The single address comparator 6, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 6, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude6> for bool {
    #[inline(always)]
    fn from(variant: Exclude6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_6` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude6R = crate::BitReader<Exclude6>;
impl Exclude6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude6 {
        match self.bits {
            false => Exclude6::Disabled,
            true => Exclude6::Enabled,
        }
    }
    #[doc = "The single address comparator 6, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude6::Disabled
    }
    #[doc = "The single address comparator 6, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude6::Enabled
    }
}
#[doc = "Field `EXCLUDE_6` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude6W<'a, REG> = crate::BitWriter<'a, REG, Exclude6>;
impl<'a, REG> Exclude6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 6, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude6::Disabled)
    }
    #[doc = "The single address comparator 6, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude6::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewData exclude control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exclude7 {
    #[doc = "0: The single address comparator 7, is not selected for ViewData exclude control."]
    Disabled = 0,
    #[doc = "1: The single address comparator 7, s selected for ViewData exclude control."]
    Enabled = 1,
}
impl From<Exclude7> for bool {
    #[inline(always)]
    fn from(variant: Exclude7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXCLUDE_7` reader - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude7R = crate::BitReader<Exclude7>;
impl Exclude7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exclude7 {
        match self.bits {
            false => Exclude7::Disabled,
            true => Exclude7::Enabled,
        }
    }
    #[doc = "The single address comparator 7, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Exclude7::Disabled
    }
    #[doc = "The single address comparator 7, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Exclude7::Enabled
    }
}
#[doc = "Field `EXCLUDE_7` writer - Selects which single address comparators are in use with ViewData exclude control."]
pub type Exclude7W<'a, REG> = crate::BitWriter<'a, REG, Exclude7>;
impl<'a, REG> Exclude7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 7, is not selected for ViewData exclude control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude7::Disabled)
    }
    #[doc = "The single address comparator 7, s selected for ViewData exclude control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Exclude7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_0(&self) -> Include0R {
        Include0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_1(&self) -> Include1R {
        Include1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_2(&self) -> Include2R {
        Include2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_3(&self) -> Include3R {
        Include3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_4(&self) -> Include4R {
        Include4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_5(&self) -> Include5R {
        Include5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_6(&self) -> Include6R {
        Include6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_7(&self) -> Include7R {
        Include7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_0(&self) -> Exclude0R {
        Exclude0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_1(&self) -> Exclude1R {
        Exclude1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_2(&self) -> Exclude2R {
        Exclude2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_3(&self) -> Exclude3R {
        Exclude3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_4(&self) -> Exclude4R {
        Exclude4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_5(&self) -> Exclude5R {
        Exclude5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_6(&self) -> Exclude6R {
        Exclude6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_7(&self) -> Exclude7R {
        Exclude7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_0(&mut self) -> Include0W<'_, TrcvdsacctlrSpec> {
        Include0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_1(&mut self) -> Include1W<'_, TrcvdsacctlrSpec> {
        Include1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_2(&mut self) -> Include2W<'_, TrcvdsacctlrSpec> {
        Include2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_3(&mut self) -> Include3W<'_, TrcvdsacctlrSpec> {
        Include3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_4(&mut self) -> Include4W<'_, TrcvdsacctlrSpec> {
        Include4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_5(&mut self) -> Include5W<'_, TrcvdsacctlrSpec> {
        Include5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_6(&mut self) -> Include6W<'_, TrcvdsacctlrSpec> {
        Include6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects which single address comparators are in use with ViewData include control."]
    #[inline(always)]
    pub fn include_7(&mut self) -> Include7W<'_, TrcvdsacctlrSpec> {
        Include7W::new(self, 7)
    }
    #[doc = "Bit 16 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_0(&mut self) -> Exclude0W<'_, TrcvdsacctlrSpec> {
        Exclude0W::new(self, 16)
    }
    #[doc = "Bit 17 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_1(&mut self) -> Exclude1W<'_, TrcvdsacctlrSpec> {
        Exclude1W::new(self, 17)
    }
    #[doc = "Bit 18 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_2(&mut self) -> Exclude2W<'_, TrcvdsacctlrSpec> {
        Exclude2W::new(self, 18)
    }
    #[doc = "Bit 19 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_3(&mut self) -> Exclude3W<'_, TrcvdsacctlrSpec> {
        Exclude3W::new(self, 19)
    }
    #[doc = "Bit 20 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_4(&mut self) -> Exclude4W<'_, TrcvdsacctlrSpec> {
        Exclude4W::new(self, 20)
    }
    #[doc = "Bit 21 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_5(&mut self) -> Exclude5W<'_, TrcvdsacctlrSpec> {
        Exclude5W::new(self, 21)
    }
    #[doc = "Bit 22 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_6(&mut self) -> Exclude6W<'_, TrcvdsacctlrSpec> {
        Exclude6W::new(self, 22)
    }
    #[doc = "Bit 23 - Selects which single address comparators are in use with ViewData exclude control."]
    #[inline(always)]
    pub fn exclude_7(&mut self) -> Exclude7W<'_, TrcvdsacctlrSpec> {
        Exclude7W::new(self, 23)
    }
}
#[doc = "ViewData include / exclude control. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed when one or more address comparators are implemented.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvdsacctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvdsacctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcvdsacctlrSpec;
impl crate::RegisterSpec for TrcvdsacctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcvdsacctlr::R`](R) reader structure"]
impl crate::Readable for TrcvdsacctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcvdsacctlr::W`](W) writer structure"]
impl crate::Writable for TrcvdsacctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCVDSACCTLR to value 0"]
impl crate::Resettable for TrcvdsacctlrSpec {}
