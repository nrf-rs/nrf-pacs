#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Keep RAM section S0 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S0power> for bool {
    #[inline(always)]
    fn from(variant: S0power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` reader - Keep RAM section S0 on or off in System ON mode."]
pub type S0powerR = crate::BitReader<S0power>;
impl S0powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0power {
        match self.bits {
            false => S0power::Off,
            true => S0power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0power::On
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 on or off in System ON mode."]
pub type S0powerW<'a, REG> = crate::BitWriter<'a, REG, S0power>;
impl<'a, REG> S0powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S0power::On)
    }
}
#[doc = "Keep RAM section S1 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S1power> for bool {
    #[inline(always)]
    fn from(variant: S1power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` reader - Keep RAM section S1 on or off in System ON mode."]
pub type S1powerR = crate::BitReader<S1power>;
impl S1powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1power {
        match self.bits {
            false => S1power::Off,
            true => S1power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1power::On
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 on or off in System ON mode."]
pub type S1powerW<'a, REG> = crate::BitWriter<'a, REG, S1power>;
impl<'a, REG> S1powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S1power::On)
    }
}
#[doc = "Keep RAM section S2 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S2power> for bool {
    #[inline(always)]
    fn from(variant: S2power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` reader - Keep RAM section S2 on or off in System ON mode."]
pub type S2powerR = crate::BitReader<S2power>;
impl S2powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2power {
        match self.bits {
            false => S2power::Off,
            true => S2power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2power::On
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 on or off in System ON mode."]
pub type S2powerW<'a, REG> = crate::BitWriter<'a, REG, S2power>;
impl<'a, REG> S2powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S2power::On)
    }
}
#[doc = "Keep RAM section S3 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S3power> for bool {
    #[inline(always)]
    fn from(variant: S3power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` reader - Keep RAM section S3 on or off in System ON mode."]
pub type S3powerR = crate::BitReader<S3power>;
impl S3powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3power {
        match self.bits {
            false => S3power::Off,
            true => S3power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3power::On
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 on or off in System ON mode."]
pub type S3powerW<'a, REG> = crate::BitWriter<'a, REG, S3power>;
impl<'a, REG> S3powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S3power::On)
    }
}
#[doc = "Keep RAM section S4 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S4power> for bool {
    #[inline(always)]
    fn from(variant: S4power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4POWER` reader - Keep RAM section S4 on or off in System ON mode."]
pub type S4powerR = crate::BitReader<S4power>;
impl S4powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4power {
        match self.bits {
            false => S4power::Off,
            true => S4power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S4power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S4power::On
    }
}
#[doc = "Field `S4POWER` writer - Keep RAM section S4 on or off in System ON mode."]
pub type S4powerW<'a, REG> = crate::BitWriter<'a, REG, S4power>;
impl<'a, REG> S4powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S4power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S4power::On)
    }
}
#[doc = "Keep RAM section S5 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S5power> for bool {
    #[inline(always)]
    fn from(variant: S5power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5POWER` reader - Keep RAM section S5 on or off in System ON mode."]
pub type S5powerR = crate::BitReader<S5power>;
impl S5powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5power {
        match self.bits {
            false => S5power::Off,
            true => S5power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S5power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S5power::On
    }
}
#[doc = "Field `S5POWER` writer - Keep RAM section S5 on or off in System ON mode."]
pub type S5powerW<'a, REG> = crate::BitWriter<'a, REG, S5power>;
impl<'a, REG> S5powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S5power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S5power::On)
    }
}
#[doc = "Keep RAM section S6 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S6power> for bool {
    #[inline(always)]
    fn from(variant: S6power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6POWER` reader - Keep RAM section S6 on or off in System ON mode."]
pub type S6powerR = crate::BitReader<S6power>;
impl S6powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6power {
        match self.bits {
            false => S6power::Off,
            true => S6power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S6power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S6power::On
    }
}
#[doc = "Field `S6POWER` writer - Keep RAM section S6 on or off in System ON mode."]
pub type S6powerW<'a, REG> = crate::BitWriter<'a, REG, S6power>;
impl<'a, REG> S6powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S6power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S6power::On)
    }
}
#[doc = "Keep RAM section S7 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S7power> for bool {
    #[inline(always)]
    fn from(variant: S7power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7POWER` reader - Keep RAM section S7 on or off in System ON mode."]
pub type S7powerR = crate::BitReader<S7power>;
impl S7powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7power {
        match self.bits {
            false => S7power::Off,
            true => S7power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S7power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S7power::On
    }
}
#[doc = "Field `S7POWER` writer - Keep RAM section S7 on or off in System ON mode."]
pub type S7powerW<'a, REG> = crate::BitWriter<'a, REG, S7power>;
impl<'a, REG> S7powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S7power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S7power::On)
    }
}
#[doc = "Keep RAM section S8 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S8power> for bool {
    #[inline(always)]
    fn from(variant: S8power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8POWER` reader - Keep RAM section S8 on or off in System ON mode."]
pub type S8powerR = crate::BitReader<S8power>;
impl S8powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8power {
        match self.bits {
            false => S8power::Off,
            true => S8power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S8power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S8power::On
    }
}
#[doc = "Field `S8POWER` writer - Keep RAM section S8 on or off in System ON mode."]
pub type S8powerW<'a, REG> = crate::BitWriter<'a, REG, S8power>;
impl<'a, REG> S8powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S8power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S8power::On)
    }
}
#[doc = "Keep RAM section S9 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S9power> for bool {
    #[inline(always)]
    fn from(variant: S9power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9POWER` reader - Keep RAM section S9 on or off in System ON mode."]
pub type S9powerR = crate::BitReader<S9power>;
impl S9powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9power {
        match self.bits {
            false => S9power::Off,
            true => S9power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S9power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S9power::On
    }
}
#[doc = "Field `S9POWER` writer - Keep RAM section S9 on or off in System ON mode."]
pub type S9powerW<'a, REG> = crate::BitWriter<'a, REG, S9power>;
impl<'a, REG> S9powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S9power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S9power::On)
    }
}
#[doc = "Keep RAM section S10 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S10power> for bool {
    #[inline(always)]
    fn from(variant: S10power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10POWER` reader - Keep RAM section S10 on or off in System ON mode."]
pub type S10powerR = crate::BitReader<S10power>;
impl S10powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10power {
        match self.bits {
            false => S10power::Off,
            true => S10power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S10power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S10power::On
    }
}
#[doc = "Field `S10POWER` writer - Keep RAM section S10 on or off in System ON mode."]
pub type S10powerW<'a, REG> = crate::BitWriter<'a, REG, S10power>;
impl<'a, REG> S10powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S10power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S10power::On)
    }
}
#[doc = "Keep RAM section S11 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S11power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S11power> for bool {
    #[inline(always)]
    fn from(variant: S11power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11POWER` reader - Keep RAM section S11 on or off in System ON mode."]
pub type S11powerR = crate::BitReader<S11power>;
impl S11powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S11power {
        match self.bits {
            false => S11power::Off,
            true => S11power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S11power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S11power::On
    }
}
#[doc = "Field `S11POWER` writer - Keep RAM section S11 on or off in System ON mode."]
pub type S11powerW<'a, REG> = crate::BitWriter<'a, REG, S11power>;
impl<'a, REG> S11powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S11power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S11power::On)
    }
}
#[doc = "Keep RAM section S12 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S12power> for bool {
    #[inline(always)]
    fn from(variant: S12power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12POWER` reader - Keep RAM section S12 on or off in System ON mode."]
pub type S12powerR = crate::BitReader<S12power>;
impl S12powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S12power {
        match self.bits {
            false => S12power::Off,
            true => S12power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S12power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S12power::On
    }
}
#[doc = "Field `S12POWER` writer - Keep RAM section S12 on or off in System ON mode."]
pub type S12powerW<'a, REG> = crate::BitWriter<'a, REG, S12power>;
impl<'a, REG> S12powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S12power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S12power::On)
    }
}
#[doc = "Keep RAM section S13 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S13power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S13power> for bool {
    #[inline(always)]
    fn from(variant: S13power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13POWER` reader - Keep RAM section S13 on or off in System ON mode."]
pub type S13powerR = crate::BitReader<S13power>;
impl S13powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S13power {
        match self.bits {
            false => S13power::Off,
            true => S13power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S13power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S13power::On
    }
}
#[doc = "Field `S13POWER` writer - Keep RAM section S13 on or off in System ON mode."]
pub type S13powerW<'a, REG> = crate::BitWriter<'a, REG, S13power>;
impl<'a, REG> S13powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S13power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S13power::On)
    }
}
#[doc = "Keep RAM section S14 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S14power> for bool {
    #[inline(always)]
    fn from(variant: S14power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14POWER` reader - Keep RAM section S14 on or off in System ON mode."]
pub type S14powerR = crate::BitReader<S14power>;
impl S14powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S14power {
        match self.bits {
            false => S14power::Off,
            true => S14power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S14power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S14power::On
    }
}
#[doc = "Field `S14POWER` writer - Keep RAM section S14 on or off in System ON mode."]
pub type S14powerW<'a, REG> = crate::BitWriter<'a, REG, S14power>;
impl<'a, REG> S14powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S14power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S14power::On)
    }
}
#[doc = "Keep RAM section S15 on or off in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S15power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S15power> for bool {
    #[inline(always)]
    fn from(variant: S15power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15POWER` reader - Keep RAM section S15 on or off in System ON mode."]
pub type S15powerR = crate::BitReader<S15power>;
impl S15powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S15power {
        match self.bits {
            false => S15power::Off,
            true => S15power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S15power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S15power::On
    }
}
#[doc = "Field `S15POWER` writer - Keep RAM section S15 on or off in System ON mode."]
pub type S15powerW<'a, REG> = crate::BitWriter<'a, REG, S15power>;
impl<'a, REG> S15powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S15power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S15power::On)
    }
}
#[doc = "Keep retention on RAM section S0 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S0retention> for bool {
    #[inline(always)]
    fn from(variant: S0retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` reader - Keep retention on RAM section S0 when RAM section is off"]
pub type S0retentionR = crate::BitReader<S0retention>;
impl S0retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0retention {
        match self.bits {
            false => S0retention::Off,
            true => S0retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0retention::On
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 when RAM section is off"]
pub type S0retentionW<'a, REG> = crate::BitWriter<'a, REG, S0retention>;
impl<'a, REG> S0retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S0retention::On)
    }
}
#[doc = "Keep retention on RAM section S1 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S1retention> for bool {
    #[inline(always)]
    fn from(variant: S1retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` reader - Keep retention on RAM section S1 when RAM section is off"]
pub type S1retentionR = crate::BitReader<S1retention>;
impl S1retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1retention {
        match self.bits {
            false => S1retention::Off,
            true => S1retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1retention::On
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 when RAM section is off"]
pub type S1retentionW<'a, REG> = crate::BitWriter<'a, REG, S1retention>;
impl<'a, REG> S1retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S1retention::On)
    }
}
#[doc = "Keep retention on RAM section S2 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S2retention> for bool {
    #[inline(always)]
    fn from(variant: S2retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` reader - Keep retention on RAM section S2 when RAM section is off"]
pub type S2retentionR = crate::BitReader<S2retention>;
impl S2retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2retention {
        match self.bits {
            false => S2retention::Off,
            true => S2retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2retention::On
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 when RAM section is off"]
pub type S2retentionW<'a, REG> = crate::BitWriter<'a, REG, S2retention>;
impl<'a, REG> S2retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S2retention::On)
    }
}
#[doc = "Keep retention on RAM section S3 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S3retention> for bool {
    #[inline(always)]
    fn from(variant: S3retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` reader - Keep retention on RAM section S3 when RAM section is off"]
pub type S3retentionR = crate::BitReader<S3retention>;
impl S3retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3retention {
        match self.bits {
            false => S3retention::Off,
            true => S3retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3retention::On
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 when RAM section is off"]
pub type S3retentionW<'a, REG> = crate::BitWriter<'a, REG, S3retention>;
impl<'a, REG> S3retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S3retention::On)
    }
}
#[doc = "Keep retention on RAM section S4 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S4retention> for bool {
    #[inline(always)]
    fn from(variant: S4retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4RETENTION` reader - Keep retention on RAM section S4 when RAM section is off"]
pub type S4retentionR = crate::BitReader<S4retention>;
impl S4retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S4retention {
        match self.bits {
            false => S4retention::Off,
            true => S4retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S4retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S4retention::On
    }
}
#[doc = "Field `S4RETENTION` writer - Keep retention on RAM section S4 when RAM section is off"]
pub type S4retentionW<'a, REG> = crate::BitWriter<'a, REG, S4retention>;
impl<'a, REG> S4retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S4retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S4retention::On)
    }
}
#[doc = "Keep retention on RAM section S5 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S5retention> for bool {
    #[inline(always)]
    fn from(variant: S5retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5RETENTION` reader - Keep retention on RAM section S5 when RAM section is off"]
pub type S5retentionR = crate::BitReader<S5retention>;
impl S5retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S5retention {
        match self.bits {
            false => S5retention::Off,
            true => S5retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S5retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S5retention::On
    }
}
#[doc = "Field `S5RETENTION` writer - Keep retention on RAM section S5 when RAM section is off"]
pub type S5retentionW<'a, REG> = crate::BitWriter<'a, REG, S5retention>;
impl<'a, REG> S5retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S5retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S5retention::On)
    }
}
#[doc = "Keep retention on RAM section S6 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S6retention> for bool {
    #[inline(always)]
    fn from(variant: S6retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6RETENTION` reader - Keep retention on RAM section S6 when RAM section is off"]
pub type S6retentionR = crate::BitReader<S6retention>;
impl S6retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S6retention {
        match self.bits {
            false => S6retention::Off,
            true => S6retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S6retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S6retention::On
    }
}
#[doc = "Field `S6RETENTION` writer - Keep retention on RAM section S6 when RAM section is off"]
pub type S6retentionW<'a, REG> = crate::BitWriter<'a, REG, S6retention>;
impl<'a, REG> S6retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S6retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S6retention::On)
    }
}
#[doc = "Keep retention on RAM section S7 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S7retention> for bool {
    #[inline(always)]
    fn from(variant: S7retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7RETENTION` reader - Keep retention on RAM section S7 when RAM section is off"]
pub type S7retentionR = crate::BitReader<S7retention>;
impl S7retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S7retention {
        match self.bits {
            false => S7retention::Off,
            true => S7retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S7retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S7retention::On
    }
}
#[doc = "Field `S7RETENTION` writer - Keep retention on RAM section S7 when RAM section is off"]
pub type S7retentionW<'a, REG> = crate::BitWriter<'a, REG, S7retention>;
impl<'a, REG> S7retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S7retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S7retention::On)
    }
}
#[doc = "Keep retention on RAM section S8 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S8retention> for bool {
    #[inline(always)]
    fn from(variant: S8retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8RETENTION` reader - Keep retention on RAM section S8 when RAM section is off"]
pub type S8retentionR = crate::BitReader<S8retention>;
impl S8retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S8retention {
        match self.bits {
            false => S8retention::Off,
            true => S8retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S8retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S8retention::On
    }
}
#[doc = "Field `S8RETENTION` writer - Keep retention on RAM section S8 when RAM section is off"]
pub type S8retentionW<'a, REG> = crate::BitWriter<'a, REG, S8retention>;
impl<'a, REG> S8retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S8retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S8retention::On)
    }
}
#[doc = "Keep retention on RAM section S9 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S9retention> for bool {
    #[inline(always)]
    fn from(variant: S9retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9RETENTION` reader - Keep retention on RAM section S9 when RAM section is off"]
pub type S9retentionR = crate::BitReader<S9retention>;
impl S9retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S9retention {
        match self.bits {
            false => S9retention::Off,
            true => S9retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S9retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S9retention::On
    }
}
#[doc = "Field `S9RETENTION` writer - Keep retention on RAM section S9 when RAM section is off"]
pub type S9retentionW<'a, REG> = crate::BitWriter<'a, REG, S9retention>;
impl<'a, REG> S9retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S9retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S9retention::On)
    }
}
#[doc = "Keep retention on RAM section S10 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S10retention> for bool {
    #[inline(always)]
    fn from(variant: S10retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10RETENTION` reader - Keep retention on RAM section S10 when RAM section is off"]
pub type S10retentionR = crate::BitReader<S10retention>;
impl S10retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S10retention {
        match self.bits {
            false => S10retention::Off,
            true => S10retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S10retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S10retention::On
    }
}
#[doc = "Field `S10RETENTION` writer - Keep retention on RAM section S10 when RAM section is off"]
pub type S10retentionW<'a, REG> = crate::BitWriter<'a, REG, S10retention>;
impl<'a, REG> S10retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S10retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S10retention::On)
    }
}
#[doc = "Keep retention on RAM section S11 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S11retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S11retention> for bool {
    #[inline(always)]
    fn from(variant: S11retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11RETENTION` reader - Keep retention on RAM section S11 when RAM section is off"]
pub type S11retentionR = crate::BitReader<S11retention>;
impl S11retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S11retention {
        match self.bits {
            false => S11retention::Off,
            true => S11retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S11retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S11retention::On
    }
}
#[doc = "Field `S11RETENTION` writer - Keep retention on RAM section S11 when RAM section is off"]
pub type S11retentionW<'a, REG> = crate::BitWriter<'a, REG, S11retention>;
impl<'a, REG> S11retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S11retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S11retention::On)
    }
}
#[doc = "Keep retention on RAM section S12 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S12retention> for bool {
    #[inline(always)]
    fn from(variant: S12retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12RETENTION` reader - Keep retention on RAM section S12 when RAM section is off"]
pub type S12retentionR = crate::BitReader<S12retention>;
impl S12retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S12retention {
        match self.bits {
            false => S12retention::Off,
            true => S12retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S12retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S12retention::On
    }
}
#[doc = "Field `S12RETENTION` writer - Keep retention on RAM section S12 when RAM section is off"]
pub type S12retentionW<'a, REG> = crate::BitWriter<'a, REG, S12retention>;
impl<'a, REG> S12retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S12retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S12retention::On)
    }
}
#[doc = "Keep retention on RAM section S13 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S13retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S13retention> for bool {
    #[inline(always)]
    fn from(variant: S13retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13RETENTION` reader - Keep retention on RAM section S13 when RAM section is off"]
pub type S13retentionR = crate::BitReader<S13retention>;
impl S13retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S13retention {
        match self.bits {
            false => S13retention::Off,
            true => S13retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S13retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S13retention::On
    }
}
#[doc = "Field `S13RETENTION` writer - Keep retention on RAM section S13 when RAM section is off"]
pub type S13retentionW<'a, REG> = crate::BitWriter<'a, REG, S13retention>;
impl<'a, REG> S13retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S13retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S13retention::On)
    }
}
#[doc = "Keep retention on RAM section S14 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S14retention> for bool {
    #[inline(always)]
    fn from(variant: S14retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14RETENTION` reader - Keep retention on RAM section S14 when RAM section is off"]
pub type S14retentionR = crate::BitReader<S14retention>;
impl S14retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S14retention {
        match self.bits {
            false => S14retention::Off,
            true => S14retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S14retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S14retention::On
    }
}
#[doc = "Field `S14RETENTION` writer - Keep retention on RAM section S14 when RAM section is off"]
pub type S14retentionW<'a, REG> = crate::BitWriter<'a, REG, S14retention>;
impl<'a, REG> S14retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S14retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S14retention::On)
    }
}
#[doc = "Keep retention on RAM section S15 when RAM section is off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S15retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S15retention> for bool {
    #[inline(always)]
    fn from(variant: S15retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15RETENTION` reader - Keep retention on RAM section S15 when RAM section is off"]
pub type S15retentionR = crate::BitReader<S15retention>;
impl S15retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S15retention {
        match self.bits {
            false => S15retention::Off,
            true => S15retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S15retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S15retention::On
    }
}
#[doc = "Field `S15RETENTION` writer - Keep retention on RAM section S15 when RAM section is off"]
pub type S15retentionW<'a, REG> = crate::BitWriter<'a, REG, S15retention>;
impl<'a, REG> S15retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S15retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S15retention::On)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub fn s0power(&self) -> S0powerR {
        S0powerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub fn s1power(&self) -> S1powerR {
        S1powerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub fn s2power(&self) -> S2powerR {
        S2powerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub fn s3power(&self) -> S3powerR {
        S3powerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub fn s4power(&self) -> S4powerR {
        S4powerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub fn s5power(&self) -> S5powerR {
        S5powerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub fn s6power(&self) -> S6powerR {
        S6powerR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub fn s7power(&self) -> S7powerR {
        S7powerR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub fn s8power(&self) -> S8powerR {
        S8powerR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub fn s9power(&self) -> S9powerR {
        S9powerR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub fn s10power(&self) -> S10powerR {
        S10powerR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub fn s11power(&self) -> S11powerR {
        S11powerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub fn s12power(&self) -> S12powerR {
        S12powerR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub fn s13power(&self) -> S13powerR {
        S13powerR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub fn s14power(&self) -> S14powerR {
        S14powerR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub fn s15power(&self) -> S15powerR {
        S15powerR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0retentionR {
        S0retentionR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1retentionR {
        S1retentionR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub fn s2retention(&self) -> S2retentionR {
        S2retentionR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub fn s3retention(&self) -> S3retentionR {
        S3retentionR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub fn s4retention(&self) -> S4retentionR {
        S4retentionR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub fn s5retention(&self) -> S5retentionR {
        S5retentionR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub fn s6retention(&self) -> S6retentionR {
        S6retentionR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub fn s7retention(&self) -> S7retentionR {
        S7retentionR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub fn s8retention(&self) -> S8retentionR {
        S8retentionR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub fn s9retention(&self) -> S9retentionR {
        S9retentionR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub fn s10retention(&self) -> S10retentionR {
        S10retentionR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub fn s11retention(&self) -> S11retentionR {
        S11retentionR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub fn s12retention(&self) -> S12retentionR {
        S12retentionR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub fn s13retention(&self) -> S13retentionR {
        S13retentionR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub fn s14retention(&self) -> S14retentionR {
        S14retentionR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub fn s15retention(&self) -> S15retentionR {
        S15retentionR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0powerW<'_, PowerSpec> {
        S0powerW::new(self, 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1powerW<'_, PowerSpec> {
        S1powerW::new(self, 1)
    }
    #[doc = "Bit 2 - Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub fn s2power(&mut self) -> S2powerW<'_, PowerSpec> {
        S2powerW::new(self, 2)
    }
    #[doc = "Bit 3 - Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub fn s3power(&mut self) -> S3powerW<'_, PowerSpec> {
        S3powerW::new(self, 3)
    }
    #[doc = "Bit 4 - Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub fn s4power(&mut self) -> S4powerW<'_, PowerSpec> {
        S4powerW::new(self, 4)
    }
    #[doc = "Bit 5 - Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub fn s5power(&mut self) -> S5powerW<'_, PowerSpec> {
        S5powerW::new(self, 5)
    }
    #[doc = "Bit 6 - Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub fn s6power(&mut self) -> S6powerW<'_, PowerSpec> {
        S6powerW::new(self, 6)
    }
    #[doc = "Bit 7 - Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub fn s7power(&mut self) -> S7powerW<'_, PowerSpec> {
        S7powerW::new(self, 7)
    }
    #[doc = "Bit 8 - Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub fn s8power(&mut self) -> S8powerW<'_, PowerSpec> {
        S8powerW::new(self, 8)
    }
    #[doc = "Bit 9 - Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub fn s9power(&mut self) -> S9powerW<'_, PowerSpec> {
        S9powerW::new(self, 9)
    }
    #[doc = "Bit 10 - Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub fn s10power(&mut self) -> S10powerW<'_, PowerSpec> {
        S10powerW::new(self, 10)
    }
    #[doc = "Bit 11 - Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub fn s11power(&mut self) -> S11powerW<'_, PowerSpec> {
        S11powerW::new(self, 11)
    }
    #[doc = "Bit 12 - Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub fn s12power(&mut self) -> S12powerW<'_, PowerSpec> {
        S12powerW::new(self, 12)
    }
    #[doc = "Bit 13 - Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub fn s13power(&mut self) -> S13powerW<'_, PowerSpec> {
        S13powerW::new(self, 13)
    }
    #[doc = "Bit 14 - Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub fn s14power(&mut self) -> S14powerW<'_, PowerSpec> {
        S14powerW::new(self, 14)
    }
    #[doc = "Bit 15 - Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub fn s15power(&mut self) -> S15powerW<'_, PowerSpec> {
        S15powerW::new(self, 15)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0retentionW<'_, PowerSpec> {
        S0retentionW::new(self, 16)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1retentionW<'_, PowerSpec> {
        S1retentionW::new(self, 17)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub fn s2retention(&mut self) -> S2retentionW<'_, PowerSpec> {
        S2retentionW::new(self, 18)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub fn s3retention(&mut self) -> S3retentionW<'_, PowerSpec> {
        S3retentionW::new(self, 19)
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub fn s4retention(&mut self) -> S4retentionW<'_, PowerSpec> {
        S4retentionW::new(self, 20)
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub fn s5retention(&mut self) -> S5retentionW<'_, PowerSpec> {
        S5retentionW::new(self, 21)
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub fn s6retention(&mut self) -> S6retentionW<'_, PowerSpec> {
        S6retentionW::new(self, 22)
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub fn s7retention(&mut self) -> S7retentionW<'_, PowerSpec> {
        S7retentionW::new(self, 23)
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub fn s8retention(&mut self) -> S8retentionW<'_, PowerSpec> {
        S8retentionW::new(self, 24)
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub fn s9retention(&mut self) -> S9retentionW<'_, PowerSpec> {
        S9retentionW::new(self, 25)
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub fn s10retention(&mut self) -> S10retentionW<'_, PowerSpec> {
        S10retentionW::new(self, 26)
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub fn s11retention(&mut self) -> S11retentionW<'_, PowerSpec> {
        S11retentionW::new(self, 27)
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub fn s12retention(&mut self) -> S12retentionW<'_, PowerSpec> {
        S12retentionW::new(self, 28)
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub fn s13retention(&mut self) -> S13retentionW<'_, PowerSpec> {
        S13retentionW::new(self, 29)
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub fn s14retention(&mut self) -> S14retentionW<'_, PowerSpec> {
        S14retentionW::new(self, 30)
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub fn s15retention(&mut self) -> S15retentionW<'_, PowerSpec> {
        S15retentionW::new(self, 31)
    }
}
#[doc = "Description cluster: RAMn power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0xffff"]
impl crate::Resettable for PowerSpec {
    const RESET_VALUE: u32 = 0xffff;
}
