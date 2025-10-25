#[doc = "Register `TRCSEQEVR[%s]` reader"]
pub type R = crate::R<TrcseqevrSpec>;
#[doc = "Register `TRCSEQEVR[%s]` writer"]
pub type W = crate::W<TrcseqevrSpec>;
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F0 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F0> for bool {
    #[inline(always)]
    fn from(variant: F0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_0` reader - Forward field."]
pub type F0R = crate::BitReader<F0>;
impl F0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F0 {
        match self.bits {
            false => F0::Disabled,
            true => F0::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F0::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F0::Enabled
    }
}
#[doc = "Field `F_0` writer - Forward field."]
pub type F0W<'a, REG> = crate::BitWriter<'a, REG, F0>;
impl<'a, REG> F0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F0::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F0::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F1> for bool {
    #[inline(always)]
    fn from(variant: F1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_1` reader - Forward field."]
pub type F1R = crate::BitReader<F1>;
impl F1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F1 {
        match self.bits {
            false => F1::Disabled,
            true => F1::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F1::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F1::Enabled
    }
}
#[doc = "Field `F_1` writer - Forward field."]
pub type F1W<'a, REG> = crate::BitWriter<'a, REG, F1>;
impl<'a, REG> F1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F1::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F1::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F2 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F2> for bool {
    #[inline(always)]
    fn from(variant: F2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_2` reader - Forward field."]
pub type F2R = crate::BitReader<F2>;
impl F2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F2 {
        match self.bits {
            false => F2::Disabled,
            true => F2::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F2::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F2::Enabled
    }
}
#[doc = "Field `F_2` writer - Forward field."]
pub type F2W<'a, REG> = crate::BitWriter<'a, REG, F2>;
impl<'a, REG> F2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F2::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F2::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F3 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F3> for bool {
    #[inline(always)]
    fn from(variant: F3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_3` reader - Forward field."]
pub type F3R = crate::BitReader<F3>;
impl F3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F3 {
        match self.bits {
            false => F3::Disabled,
            true => F3::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F3::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F3::Enabled
    }
}
#[doc = "Field `F_3` writer - Forward field."]
pub type F3W<'a, REG> = crate::BitWriter<'a, REG, F3>;
impl<'a, REG> F3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F3::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F3::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F4 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F4> for bool {
    #[inline(always)]
    fn from(variant: F4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_4` reader - Forward field."]
pub type F4R = crate::BitReader<F4>;
impl F4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F4 {
        match self.bits {
            false => F4::Disabled,
            true => F4::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F4::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F4::Enabled
    }
}
#[doc = "Field `F_4` writer - Forward field."]
pub type F4W<'a, REG> = crate::BitWriter<'a, REG, F4>;
impl<'a, REG> F4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F4::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F4::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F5 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F5> for bool {
    #[inline(always)]
    fn from(variant: F5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_5` reader - Forward field."]
pub type F5R = crate::BitReader<F5>;
impl F5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F5 {
        match self.bits {
            false => F5::Disabled,
            true => F5::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F5::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F5::Enabled
    }
}
#[doc = "Field `F_5` writer - Forward field."]
pub type F5W<'a, REG> = crate::BitWriter<'a, REG, F5>;
impl<'a, REG> F5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F5::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F5::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F6 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F6> for bool {
    #[inline(always)]
    fn from(variant: F6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_6` reader - Forward field."]
pub type F6R = crate::BitReader<F6>;
impl F6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F6 {
        match self.bits {
            false => F6::Disabled,
            true => F6::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F6::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F6::Enabled
    }
}
#[doc = "Field `F_6` writer - Forward field."]
pub type F6W<'a, REG> = crate::BitWriter<'a, REG, F6>;
impl<'a, REG> F6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F6::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F6::Enabled)
    }
}
#[doc = "Forward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F7 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n to state n+1."]
    Enabled = 1,
}
impl From<F7> for bool {
    #[inline(always)]
    fn from(variant: F7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_7` reader - Forward field."]
pub type F7R = crate::BitReader<F7>;
impl F7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> F7 {
        match self.bits {
            false => F7::Disabled,
            true => F7::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == F7::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == F7::Enabled
    }
}
#[doc = "Field `F_7` writer - Forward field."]
pub type F7W<'a, REG> = crate::BitWriter<'a, REG, F7>;
impl<'a, REG> F7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(F7::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n to state n+1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(F7::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B0> for bool {
    #[inline(always)]
    fn from(variant: B0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_0` reader - Backward field."]
pub type B0R = crate::BitReader<B0>;
impl B0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B0 {
        match self.bits {
            false => B0::Disabled,
            true => B0::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B0::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B0::Enabled
    }
}
#[doc = "Field `B_0` writer - Backward field."]
pub type B0W<'a, REG> = crate::BitWriter<'a, REG, B0>;
impl<'a, REG> B0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B0::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B0::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B1> for bool {
    #[inline(always)]
    fn from(variant: B1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_1` reader - Backward field."]
pub type B1R = crate::BitReader<B1>;
impl B1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B1 {
        match self.bits {
            false => B1::Disabled,
            true => B1::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B1::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B1::Enabled
    }
}
#[doc = "Field `B_1` writer - Backward field."]
pub type B1W<'a, REG> = crate::BitWriter<'a, REG, B1>;
impl<'a, REG> B1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B1::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B1::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B2> for bool {
    #[inline(always)]
    fn from(variant: B2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_2` reader - Backward field."]
pub type B2R = crate::BitReader<B2>;
impl B2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B2 {
        match self.bits {
            false => B2::Disabled,
            true => B2::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B2::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B2::Enabled
    }
}
#[doc = "Field `B_2` writer - Backward field."]
pub type B2W<'a, REG> = crate::BitWriter<'a, REG, B2>;
impl<'a, REG> B2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B2::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B2::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B3 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B3> for bool {
    #[inline(always)]
    fn from(variant: B3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_3` reader - Backward field."]
pub type B3R = crate::BitReader<B3>;
impl B3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B3 {
        match self.bits {
            false => B3::Disabled,
            true => B3::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B3::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B3::Enabled
    }
}
#[doc = "Field `B_3` writer - Backward field."]
pub type B3W<'a, REG> = crate::BitWriter<'a, REG, B3>;
impl<'a, REG> B3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B3::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B3::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B4 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B4> for bool {
    #[inline(always)]
    fn from(variant: B4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_4` reader - Backward field."]
pub type B4R = crate::BitReader<B4>;
impl B4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B4 {
        match self.bits {
            false => B4::Disabled,
            true => B4::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B4::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B4::Enabled
    }
}
#[doc = "Field `B_4` writer - Backward field."]
pub type B4W<'a, REG> = crate::BitWriter<'a, REG, B4>;
impl<'a, REG> B4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B4::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B4::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B5 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B5> for bool {
    #[inline(always)]
    fn from(variant: B5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_5` reader - Backward field."]
pub type B5R = crate::BitReader<B5>;
impl B5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B5 {
        match self.bits {
            false => B5::Disabled,
            true => B5::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B5::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B5::Enabled
    }
}
#[doc = "Field `B_5` writer - Backward field."]
pub type B5W<'a, REG> = crate::BitWriter<'a, REG, B5>;
impl<'a, REG> B5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B5::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B5::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B6 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B6> for bool {
    #[inline(always)]
    fn from(variant: B6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_6` reader - Backward field."]
pub type B6R = crate::BitReader<B6>;
impl B6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B6 {
        match self.bits {
            false => B6::Disabled,
            true => B6::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B6::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B6::Enabled
    }
}
#[doc = "Field `B_6` writer - Backward field."]
pub type B6W<'a, REG> = crate::BitWriter<'a, REG, B6>;
impl<'a, REG> B6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B6::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B6::Enabled)
    }
}
#[doc = "Backward field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B7 {
    #[doc = "0: The trace event does not affect the sequencer."]
    Disabled = 0,
    #[doc = "1: When the event occurs then the sequencer state moves from state n+1 to state n."]
    Enabled = 1,
}
impl From<B7> for bool {
    #[inline(always)]
    fn from(variant: B7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_7` reader - Backward field."]
pub type B7R = crate::BitReader<B7>;
impl B7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B7 {
        match self.bits {
            false => B7::Disabled,
            true => B7::Enabled,
        }
    }
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B7::Disabled
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B7::Enabled
    }
}
#[doc = "Field `B_7` writer - Backward field."]
pub type B7W<'a, REG> = crate::BitWriter<'a, REG, B7>;
impl<'a, REG> B7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace event does not affect the sequencer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B7::Disabled)
    }
    #[doc = "When the event occurs then the sequencer state moves from state n+1 to state n."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Forward field."]
    #[inline(always)]
    pub fn f_0(&self) -> F0R {
        F0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Forward field."]
    #[inline(always)]
    pub fn f_1(&self) -> F1R {
        F1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Forward field."]
    #[inline(always)]
    pub fn f_2(&self) -> F2R {
        F2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Forward field."]
    #[inline(always)]
    pub fn f_3(&self) -> F3R {
        F3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forward field."]
    #[inline(always)]
    pub fn f_4(&self) -> F4R {
        F4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Forward field."]
    #[inline(always)]
    pub fn f_5(&self) -> F5R {
        F5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward field."]
    #[inline(always)]
    pub fn f_6(&self) -> F6R {
        F6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward field."]
    #[inline(always)]
    pub fn f_7(&self) -> F7R {
        F7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Backward field."]
    #[inline(always)]
    pub fn b_0(&self) -> B0R {
        B0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Backward field."]
    #[inline(always)]
    pub fn b_1(&self) -> B1R {
        B1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Backward field."]
    #[inline(always)]
    pub fn b_2(&self) -> B2R {
        B2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Backward field."]
    #[inline(always)]
    pub fn b_3(&self) -> B3R {
        B3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Backward field."]
    #[inline(always)]
    pub fn b_4(&self) -> B4R {
        B4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Backward field."]
    #[inline(always)]
    pub fn b_5(&self) -> B5R {
        B5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Backward field."]
    #[inline(always)]
    pub fn b_6(&self) -> B6R {
        B6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Backward field."]
    #[inline(always)]
    pub fn b_7(&self) -> B7R {
        B7R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Forward field."]
    #[inline(always)]
    pub fn f_0(&mut self) -> F0W<'_, TrcseqevrSpec> {
        F0W::new(self, 0)
    }
    #[doc = "Bit 1 - Forward field."]
    #[inline(always)]
    pub fn f_1(&mut self) -> F1W<'_, TrcseqevrSpec> {
        F1W::new(self, 1)
    }
    #[doc = "Bit 2 - Forward field."]
    #[inline(always)]
    pub fn f_2(&mut self) -> F2W<'_, TrcseqevrSpec> {
        F2W::new(self, 2)
    }
    #[doc = "Bit 3 - Forward field."]
    #[inline(always)]
    pub fn f_3(&mut self) -> F3W<'_, TrcseqevrSpec> {
        F3W::new(self, 3)
    }
    #[doc = "Bit 4 - Forward field."]
    #[inline(always)]
    pub fn f_4(&mut self) -> F4W<'_, TrcseqevrSpec> {
        F4W::new(self, 4)
    }
    #[doc = "Bit 5 - Forward field."]
    #[inline(always)]
    pub fn f_5(&mut self) -> F5W<'_, TrcseqevrSpec> {
        F5W::new(self, 5)
    }
    #[doc = "Bit 6 - Forward field."]
    #[inline(always)]
    pub fn f_6(&mut self) -> F6W<'_, TrcseqevrSpec> {
        F6W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward field."]
    #[inline(always)]
    pub fn f_7(&mut self) -> F7W<'_, TrcseqevrSpec> {
        F7W::new(self, 7)
    }
    #[doc = "Bit 8 - Backward field."]
    #[inline(always)]
    pub fn b_0(&mut self) -> B0W<'_, TrcseqevrSpec> {
        B0W::new(self, 8)
    }
    #[doc = "Bit 9 - Backward field."]
    #[inline(always)]
    pub fn b_1(&mut self) -> B1W<'_, TrcseqevrSpec> {
        B1W::new(self, 9)
    }
    #[doc = "Bit 10 - Backward field."]
    #[inline(always)]
    pub fn b_2(&mut self) -> B2W<'_, TrcseqevrSpec> {
        B2W::new(self, 10)
    }
    #[doc = "Bit 11 - Backward field."]
    #[inline(always)]
    pub fn b_3(&mut self) -> B3W<'_, TrcseqevrSpec> {
        B3W::new(self, 11)
    }
    #[doc = "Bit 12 - Backward field."]
    #[inline(always)]
    pub fn b_4(&mut self) -> B4W<'_, TrcseqevrSpec> {
        B4W::new(self, 12)
    }
    #[doc = "Bit 13 - Backward field."]
    #[inline(always)]
    pub fn b_5(&mut self) -> B5W<'_, TrcseqevrSpec> {
        B5W::new(self, 13)
    }
    #[doc = "Bit 14 - Backward field."]
    #[inline(always)]
    pub fn b_6(&mut self) -> B6W<'_, TrcseqevrSpec> {
        B6W::new(self, 14)
    }
    #[doc = "Bit 15 - Backward field."]
    #[inline(always)]
    pub fn b_7(&mut self) -> B7W<'_, TrcseqevrSpec> {
        B7W::new(self, 15)
    }
}
#[doc = "Description collection: Moves the sequencer state according to programmed events. Might ignore writes when the trace unit is enabled or not idle. When the sequencer is used, all sequencer state transitions must be programmed with a valid event.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcseqevr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcseqevr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcseqevrSpec;
impl crate::RegisterSpec for TrcseqevrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcseqevr::R`](R) reader structure"]
impl crate::Readable for TrcseqevrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcseqevr::W`](W) writer structure"]
impl crate::Writable for TrcseqevrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSEQEVR[%s] to value 0"]
impl crate::Resettable for TrcseqevrSpec {}
