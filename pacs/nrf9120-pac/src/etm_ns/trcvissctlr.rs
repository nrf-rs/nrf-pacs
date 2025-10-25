#[doc = "Register `TRCVISSCTLR` reader"]
pub type R = crate::R<TrcvissctlrSpec>;
#[doc = "Register `TRCVISSCTLR` writer"]
pub type W = crate::W<TrcvissctlrSpec>;
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start0 {
    #[doc = "0: The single address comparator 0, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 0, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start0> for bool {
    #[inline(always)]
    fn from(variant: Start0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_0` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start0R = crate::BitReader<Start0>;
impl Start0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start0 {
        match self.bits {
            false => Start0::Disabled,
            true => Start0::Enabled,
        }
    }
    #[doc = "The single address comparator 0, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start0::Disabled
    }
    #[doc = "The single address comparator 0, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start0::Enabled
    }
}
#[doc = "Field `START_0` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start0W<'a, REG> = crate::BitWriter<'a, REG, Start0>;
impl<'a, REG> Start0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 0, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start0::Disabled)
    }
    #[doc = "The single address comparator 0, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start0::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start1 {
    #[doc = "0: The single address comparator 1, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 1, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start1> for bool {
    #[inline(always)]
    fn from(variant: Start1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_1` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start1R = crate::BitReader<Start1>;
impl Start1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start1 {
        match self.bits {
            false => Start1::Disabled,
            true => Start1::Enabled,
        }
    }
    #[doc = "The single address comparator 1, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start1::Disabled
    }
    #[doc = "The single address comparator 1, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start1::Enabled
    }
}
#[doc = "Field `START_1` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start1W<'a, REG> = crate::BitWriter<'a, REG, Start1>;
impl<'a, REG> Start1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 1, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start1::Disabled)
    }
    #[doc = "The single address comparator 1, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start1::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start2 {
    #[doc = "0: The single address comparator 2, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 2, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start2> for bool {
    #[inline(always)]
    fn from(variant: Start2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_2` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start2R = crate::BitReader<Start2>;
impl Start2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start2 {
        match self.bits {
            false => Start2::Disabled,
            true => Start2::Enabled,
        }
    }
    #[doc = "The single address comparator 2, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start2::Disabled
    }
    #[doc = "The single address comparator 2, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start2::Enabled
    }
}
#[doc = "Field `START_2` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start2W<'a, REG> = crate::BitWriter<'a, REG, Start2>;
impl<'a, REG> Start2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 2, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start2::Disabled)
    }
    #[doc = "The single address comparator 2, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start2::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start3 {
    #[doc = "0: The single address comparator 3, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 3, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start3> for bool {
    #[inline(always)]
    fn from(variant: Start3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_3` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start3R = crate::BitReader<Start3>;
impl Start3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start3 {
        match self.bits {
            false => Start3::Disabled,
            true => Start3::Enabled,
        }
    }
    #[doc = "The single address comparator 3, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start3::Disabled
    }
    #[doc = "The single address comparator 3, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start3::Enabled
    }
}
#[doc = "Field `START_3` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start3W<'a, REG> = crate::BitWriter<'a, REG, Start3>;
impl<'a, REG> Start3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 3, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start3::Disabled)
    }
    #[doc = "The single address comparator 3, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start3::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start4 {
    #[doc = "0: The single address comparator 4, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 4, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start4> for bool {
    #[inline(always)]
    fn from(variant: Start4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_4` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start4R = crate::BitReader<Start4>;
impl Start4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start4 {
        match self.bits {
            false => Start4::Disabled,
            true => Start4::Enabled,
        }
    }
    #[doc = "The single address comparator 4, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start4::Disabled
    }
    #[doc = "The single address comparator 4, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start4::Enabled
    }
}
#[doc = "Field `START_4` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start4W<'a, REG> = crate::BitWriter<'a, REG, Start4>;
impl<'a, REG> Start4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 4, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start4::Disabled)
    }
    #[doc = "The single address comparator 4, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start4::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start5 {
    #[doc = "0: The single address comparator 5, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 5, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start5> for bool {
    #[inline(always)]
    fn from(variant: Start5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_5` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start5R = crate::BitReader<Start5>;
impl Start5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start5 {
        match self.bits {
            false => Start5::Disabled,
            true => Start5::Enabled,
        }
    }
    #[doc = "The single address comparator 5, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start5::Disabled
    }
    #[doc = "The single address comparator 5, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start5::Enabled
    }
}
#[doc = "Field `START_5` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start5W<'a, REG> = crate::BitWriter<'a, REG, Start5>;
impl<'a, REG> Start5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 5, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start5::Disabled)
    }
    #[doc = "The single address comparator 5, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start5::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start6 {
    #[doc = "0: The single address comparator 6, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 6, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start6> for bool {
    #[inline(always)]
    fn from(variant: Start6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_6` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start6R = crate::BitReader<Start6>;
impl Start6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start6 {
        match self.bits {
            false => Start6::Disabled,
            true => Start6::Enabled,
        }
    }
    #[doc = "The single address comparator 6, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start6::Disabled
    }
    #[doc = "The single address comparator 6, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start6::Enabled
    }
}
#[doc = "Field `START_6` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start6W<'a, REG> = crate::BitWriter<'a, REG, Start6>;
impl<'a, REG> Start6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 6, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start6::Disabled)
    }
    #[doc = "The single address comparator 6, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start6::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start7 {
    #[doc = "0: The single address comparator 7, is not selected as a start resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 7, is selected as a start resource."]
    Enabled = 1,
}
impl From<Start7> for bool {
    #[inline(always)]
    fn from(variant: Start7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_7` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start7R = crate::BitReader<Start7>;
impl Start7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start7 {
        match self.bits {
            false => Start7::Disabled,
            true => Start7::Enabled,
        }
    }
    #[doc = "The single address comparator 7, is not selected as a start resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Start7::Disabled
    }
    #[doc = "The single address comparator 7, is selected as a start resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Start7::Enabled
    }
}
#[doc = "Field `START_7` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
pub type Start7W<'a, REG> = crate::BitWriter<'a, REG, Start7>;
impl<'a, REG> Start7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 7, is not selected as a start resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start7::Disabled)
    }
    #[doc = "The single address comparator 7, is selected as a start resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Start7::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop0 {
    #[doc = "0: The single address comparator 0, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 0, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop0> for bool {
    #[inline(always)]
    fn from(variant: Stop0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_0` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop0R = crate::BitReader<Stop0>;
impl Stop0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop0 {
        match self.bits {
            false => Stop0::Disabled,
            true => Stop0::Enabled,
        }
    }
    #[doc = "The single address comparator 0, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop0::Disabled
    }
    #[doc = "The single address comparator 0, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop0::Enabled
    }
}
#[doc = "Field `STOP_0` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop0W<'a, REG> = crate::BitWriter<'a, REG, Stop0>;
impl<'a, REG> Stop0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 0, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop0::Disabled)
    }
    #[doc = "The single address comparator 0, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop0::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop1 {
    #[doc = "0: The single address comparator 1, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 1, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop1> for bool {
    #[inline(always)]
    fn from(variant: Stop1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_1` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop1R = crate::BitReader<Stop1>;
impl Stop1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop1 {
        match self.bits {
            false => Stop1::Disabled,
            true => Stop1::Enabled,
        }
    }
    #[doc = "The single address comparator 1, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop1::Disabled
    }
    #[doc = "The single address comparator 1, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop1::Enabled
    }
}
#[doc = "Field `STOP_1` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop1W<'a, REG> = crate::BitWriter<'a, REG, Stop1>;
impl<'a, REG> Stop1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 1, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop1::Disabled)
    }
    #[doc = "The single address comparator 1, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop1::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop2 {
    #[doc = "0: The single address comparator 2, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 2, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop2> for bool {
    #[inline(always)]
    fn from(variant: Stop2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_2` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop2R = crate::BitReader<Stop2>;
impl Stop2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop2 {
        match self.bits {
            false => Stop2::Disabled,
            true => Stop2::Enabled,
        }
    }
    #[doc = "The single address comparator 2, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop2::Disabled
    }
    #[doc = "The single address comparator 2, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop2::Enabled
    }
}
#[doc = "Field `STOP_2` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop2W<'a, REG> = crate::BitWriter<'a, REG, Stop2>;
impl<'a, REG> Stop2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 2, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop2::Disabled)
    }
    #[doc = "The single address comparator 2, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop2::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop3 {
    #[doc = "0: The single address comparator 3, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 3, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop3> for bool {
    #[inline(always)]
    fn from(variant: Stop3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_3` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop3R = crate::BitReader<Stop3>;
impl Stop3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop3 {
        match self.bits {
            false => Stop3::Disabled,
            true => Stop3::Enabled,
        }
    }
    #[doc = "The single address comparator 3, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop3::Disabled
    }
    #[doc = "The single address comparator 3, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop3::Enabled
    }
}
#[doc = "Field `STOP_3` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop3W<'a, REG> = crate::BitWriter<'a, REG, Stop3>;
impl<'a, REG> Stop3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 3, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3::Disabled)
    }
    #[doc = "The single address comparator 3, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop4 {
    #[doc = "0: The single address comparator 4, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 4, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop4> for bool {
    #[inline(always)]
    fn from(variant: Stop4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_4` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop4R = crate::BitReader<Stop4>;
impl Stop4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop4 {
        match self.bits {
            false => Stop4::Disabled,
            true => Stop4::Enabled,
        }
    }
    #[doc = "The single address comparator 4, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop4::Disabled
    }
    #[doc = "The single address comparator 4, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop4::Enabled
    }
}
#[doc = "Field `STOP_4` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop4W<'a, REG> = crate::BitWriter<'a, REG, Stop4>;
impl<'a, REG> Stop4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 4, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop4::Disabled)
    }
    #[doc = "The single address comparator 4, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop4::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop5 {
    #[doc = "0: The single address comparator 5, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 5, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop5> for bool {
    #[inline(always)]
    fn from(variant: Stop5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_5` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop5R = crate::BitReader<Stop5>;
impl Stop5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop5 {
        match self.bits {
            false => Stop5::Disabled,
            true => Stop5::Enabled,
        }
    }
    #[doc = "The single address comparator 5, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop5::Disabled
    }
    #[doc = "The single address comparator 5, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop5::Enabled
    }
}
#[doc = "Field `STOP_5` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop5W<'a, REG> = crate::BitWriter<'a, REG, Stop5>;
impl<'a, REG> Stop5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 5, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop5::Disabled)
    }
    #[doc = "The single address comparator 5, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop5::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop6 {
    #[doc = "0: The single address comparator 6, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 6, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop6> for bool {
    #[inline(always)]
    fn from(variant: Stop6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_6` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop6R = crate::BitReader<Stop6>;
impl Stop6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop6 {
        match self.bits {
            false => Stop6::Disabled,
            true => Stop6::Enabled,
        }
    }
    #[doc = "The single address comparator 6, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop6::Disabled
    }
    #[doc = "The single address comparator 6, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop6::Enabled
    }
}
#[doc = "Field `STOP_6` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop6W<'a, REG> = crate::BitWriter<'a, REG, Stop6>;
impl<'a, REG> Stop6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 6, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop6::Disabled)
    }
    #[doc = "The single address comparator 6, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop6::Enabled)
    }
}
#[doc = "Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop7 {
    #[doc = "0: The single address comparator 7, is not selected as a stop resource."]
    Disabled = 0,
    #[doc = "1: The single address comparator 7, is selected as a stop resource."]
    Enabled = 1,
}
impl From<Stop7> for bool {
    #[inline(always)]
    fn from(variant: Stop7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_7` reader - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop7R = crate::BitReader<Stop7>;
impl Stop7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop7 {
        match self.bits {
            false => Stop7::Disabled,
            true => Stop7::Enabled,
        }
    }
    #[doc = "The single address comparator 7, is not selected as a stop resource."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stop7::Disabled
    }
    #[doc = "The single address comparator 7, is selected as a stop resource."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stop7::Enabled
    }
}
#[doc = "Field `STOP_7` writer - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
pub type Stop7W<'a, REG> = crate::BitWriter<'a, REG, Stop7>;
impl<'a, REG> Stop7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single address comparator 7, is not selected as a stop resource."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop7::Disabled)
    }
    #[doc = "The single address comparator 7, is selected as a stop resource."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stop7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_0(&self) -> Start0R {
        Start0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_1(&self) -> Start1R {
        Start1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_2(&self) -> Start2R {
        Start2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_3(&self) -> Start3R {
        Start3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_4(&self) -> Start4R {
        Start4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_5(&self) -> Start5R {
        Start5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_6(&self) -> Start6R {
        Start6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_7(&self) -> Start7R {
        Start7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_0(&self) -> Stop0R {
        Stop0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_1(&self) -> Stop1R {
        Stop1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_2(&self) -> Stop2R {
        Stop2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_3(&self) -> Stop3R {
        Stop3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_4(&self) -> Stop4R {
        Stop4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_5(&self) -> Stop5R {
        Stop5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_6(&self) -> Stop6R {
        Stop6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_7(&self) -> Stop7R {
        Stop7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_0(&mut self) -> Start0W<'_, TrcvissctlrSpec> {
        Start0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_1(&mut self) -> Start1W<'_, TrcvissctlrSpec> {
        Start1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_2(&mut self) -> Start2W<'_, TrcvissctlrSpec> {
        Start2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_3(&mut self) -> Start3W<'_, TrcvissctlrSpec> {
        Start3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_4(&mut self) -> Start4W<'_, TrcvissctlrSpec> {
        Start4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_5(&mut self) -> Start5W<'_, TrcvissctlrSpec> {
        Start5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_6(&mut self) -> Start6W<'_, TrcvissctlrSpec> {
        Start6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of starting trace."]
    #[inline(always)]
    pub fn start_7(&mut self) -> Start7W<'_, TrcvissctlrSpec> {
        Start7W::new(self, 7)
    }
    #[doc = "Bit 16 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_0(&mut self) -> Stop0W<'_, TrcvissctlrSpec> {
        Stop0W::new(self, 16)
    }
    #[doc = "Bit 17 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_1(&mut self) -> Stop1W<'_, TrcvissctlrSpec> {
        Stop1W::new(self, 17)
    }
    #[doc = "Bit 18 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_2(&mut self) -> Stop2W<'_, TrcvissctlrSpec> {
        Stop2W::new(self, 18)
    }
    #[doc = "Bit 19 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_3(&mut self) -> Stop3W<'_, TrcvissctlrSpec> {
        Stop3W::new(self, 19)
    }
    #[doc = "Bit 20 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_4(&mut self) -> Stop4W<'_, TrcvissctlrSpec> {
        Stop4W::new(self, 20)
    }
    #[doc = "Bit 21 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_5(&mut self) -> Stop5W<'_, TrcvissctlrSpec> {
        Stop5W::new(self, 21)
    }
    #[doc = "Bit 22 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_6(&mut self) -> Stop6W<'_, TrcvissctlrSpec> {
        Stop6W::new(self, 22)
    }
    #[doc = "Bit 23 - Selects which single address comparators are in use with ViewInst start/stop control, for the purpose of stopping trace"]
    #[inline(always)]
    pub fn stop_7(&mut self) -> Stop7W<'_, TrcvissctlrSpec> {
        Stop7W::new(self, 23)
    }
}
#[doc = "Use this to set, or read, the single address comparators that control the ViewInst start/stop logic. The start/stop logic is active for an instruction which causes a start and remains active up to and including an instruction which causes a stop, and then the start/stop logic becomes inactive. Might ignore writes when the trace unit is enabled or not idle. If implemented then this register must be programmed.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcvissctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcvissctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcvissctlrSpec;
impl crate::RegisterSpec for TrcvissctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcvissctlr::R`](R) reader structure"]
impl crate::Readable for TrcvissctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcvissctlr::W`](W) writer structure"]
impl crate::Writable for TrcvissctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCVISSCTLR to value 0"]
impl crate::Resettable for TrcvissctlrSpec {}
