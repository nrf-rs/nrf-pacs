#[doc = "Register `NMIEN` reader"]
pub type R = crate::R<NmienSpec>;
#[doc = "Register `NMIEN` writer"]
pub type W = crate::W<NmienSpec>;
#[doc = "Enable or disable interrupt for event REGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region0wa> for bool {
    #[inline(always)]
    fn from(variant: Region0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` reader - Enable or disable interrupt for event REGION0WA"]
pub type Region0waR = crate::BitReader<Region0wa>;
impl Region0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region0wa {
        match self.bits {
            false => Region0wa::Disabled,
            true => Region0wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0wa::Enabled
    }
}
#[doc = "Field `REGION0WA` writer - Enable or disable interrupt for event REGION0WA"]
pub type Region0waW<'a, REG> = crate::BitWriter<'a, REG, Region0wa>;
impl<'a, REG> Region0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region0ra> for bool {
    #[inline(always)]
    fn from(variant: Region0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` reader - Enable or disable interrupt for event REGION0RA"]
pub type Region0raR = crate::BitReader<Region0ra>;
impl Region0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region0ra {
        match self.bits {
            false => Region0ra::Disabled,
            true => Region0ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0ra::Enabled
    }
}
#[doc = "Field `REGION0RA` writer - Enable or disable interrupt for event REGION0RA"]
pub type Region0raW<'a, REG> = crate::BitWriter<'a, REG, Region0ra>;
impl<'a, REG> Region0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region0ra::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region1wa> for bool {
    #[inline(always)]
    fn from(variant: Region1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` reader - Enable or disable interrupt for event REGION1WA"]
pub type Region1waR = crate::BitReader<Region1wa>;
impl Region1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region1wa {
        match self.bits {
            false => Region1wa::Disabled,
            true => Region1wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1wa::Enabled
    }
}
#[doc = "Field `REGION1WA` writer - Enable or disable interrupt for event REGION1WA"]
pub type Region1waW<'a, REG> = crate::BitWriter<'a, REG, Region1wa>;
impl<'a, REG> Region1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region1ra> for bool {
    #[inline(always)]
    fn from(variant: Region1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` reader - Enable or disable interrupt for event REGION1RA"]
pub type Region1raR = crate::BitReader<Region1ra>;
impl Region1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region1ra {
        match self.bits {
            false => Region1ra::Disabled,
            true => Region1ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1ra::Enabled
    }
}
#[doc = "Field `REGION1RA` writer - Enable or disable interrupt for event REGION1RA"]
pub type Region1raW<'a, REG> = crate::BitWriter<'a, REG, Region1ra>;
impl<'a, REG> Region1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region1ra::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION2WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region2wa> for bool {
    #[inline(always)]
    fn from(variant: Region2wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` reader - Enable or disable interrupt for event REGION2WA"]
pub type Region2waR = crate::BitReader<Region2wa>;
impl Region2waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region2wa {
        match self.bits {
            false => Region2wa::Disabled,
            true => Region2wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2wa::Enabled
    }
}
#[doc = "Field `REGION2WA` writer - Enable or disable interrupt for event REGION2WA"]
pub type Region2waW<'a, REG> = crate::BitWriter<'a, REG, Region2wa>;
impl<'a, REG> Region2waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION2RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region2ra> for bool {
    #[inline(always)]
    fn from(variant: Region2ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` reader - Enable or disable interrupt for event REGION2RA"]
pub type Region2raR = crate::BitReader<Region2ra>;
impl Region2raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region2ra {
        match self.bits {
            false => Region2ra::Disabled,
            true => Region2ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2ra::Enabled
    }
}
#[doc = "Field `REGION2RA` writer - Enable or disable interrupt for event REGION2RA"]
pub type Region2raW<'a, REG> = crate::BitWriter<'a, REG, Region2ra>;
impl<'a, REG> Region2raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region2ra::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION3WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region3wa> for bool {
    #[inline(always)]
    fn from(variant: Region3wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` reader - Enable or disable interrupt for event REGION3WA"]
pub type Region3waR = crate::BitReader<Region3wa>;
impl Region3waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region3wa {
        match self.bits {
            false => Region3wa::Disabled,
            true => Region3wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3wa::Enabled
    }
}
#[doc = "Field `REGION3WA` writer - Enable or disable interrupt for event REGION3WA"]
pub type Region3waW<'a, REG> = crate::BitWriter<'a, REG, Region3wa>;
impl<'a, REG> Region3waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event REGION3RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Region3ra> for bool {
    #[inline(always)]
    fn from(variant: Region3ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` reader - Enable or disable interrupt for event REGION3RA"]
pub type Region3raR = crate::BitReader<Region3ra>;
impl Region3raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region3ra {
        match self.bits {
            false => Region3ra::Disabled,
            true => Region3ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3ra::Enabled
    }
}
#[doc = "Field `REGION3RA` writer - Enable or disable interrupt for event REGION3RA"]
pub type Region3raW<'a, REG> = crate::BitWriter<'a, REG, Region3ra>;
impl<'a, REG> Region3raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Region3ra::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PREGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pregion0wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` reader - Enable or disable interrupt for event PREGION0WA"]
pub type Pregion0waR = crate::BitReader<Pregion0wa>;
impl Pregion0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion0wa {
        match self.bits {
            false => Pregion0wa::Disabled,
            true => Pregion0wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0wa::Enabled
    }
}
#[doc = "Field `PREGION0WA` writer - Enable or disable interrupt for event PREGION0WA"]
pub type Pregion0waW<'a, REG> = crate::BitWriter<'a, REG, Pregion0wa>;
impl<'a, REG> Pregion0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PREGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pregion0ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` reader - Enable or disable interrupt for event PREGION0RA"]
pub type Pregion0raR = crate::BitReader<Pregion0ra>;
impl Pregion0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion0ra {
        match self.bits {
            false => Pregion0ra::Disabled,
            true => Pregion0ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0ra::Enabled
    }
}
#[doc = "Field `PREGION0RA` writer - Enable or disable interrupt for event PREGION0RA"]
pub type Pregion0raW<'a, REG> = crate::BitWriter<'a, REG, Pregion0ra>;
impl<'a, REG> Pregion0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0ra::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PREGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1wa {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pregion1wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` reader - Enable or disable interrupt for event PREGION1WA"]
pub type Pregion1waR = crate::BitReader<Pregion1wa>;
impl Pregion1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion1wa {
        match self.bits {
            false => Pregion1wa::Disabled,
            true => Pregion1wa::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1wa::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1wa::Enabled
    }
}
#[doc = "Field `PREGION1WA` writer - Enable or disable interrupt for event PREGION1WA"]
pub type Pregion1waW<'a, REG> = crate::BitWriter<'a, REG, Pregion1wa>;
impl<'a, REG> Pregion1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1wa::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1wa::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PREGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1ra {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pregion1ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` reader - Enable or disable interrupt for event PREGION1RA"]
pub type Pregion1raR = crate::BitReader<Pregion1ra>;
impl Pregion1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion1ra {
        match self.bits {
            false => Pregion1ra::Disabled,
            true => Pregion1ra::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1ra::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1ra::Enabled
    }
}
#[doc = "Field `PREGION1RA` writer - Enable or disable interrupt for event PREGION1RA"]
pub type Pregion1raW<'a, REG> = crate::BitWriter<'a, REG, Pregion1ra>;
impl<'a, REG> Pregion1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1ra::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1ra::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&self) -> Region0waR {
        Region0waR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&self) -> Region0raR {
        Region0raR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&self) -> Region1waR {
        Region1waR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&self) -> Region1raR {
        Region1raR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&self) -> Region2waR {
        Region2waR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&self) -> Region2raR {
        Region2raR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&self) -> Region3waR {
        Region3waR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&self) -> Region3raR {
        Region3raR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> Pregion0waR {
        Pregion0waR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> Pregion0raR {
        Pregion0raR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> Pregion1waR {
        Pregion1waR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> Pregion1raR {
        Pregion1raR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> Region0waW<'_, NmienSpec> {
        Region0waW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> Region0raW<'_, NmienSpec> {
        Region0raW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> Region1waW<'_, NmienSpec> {
        Region1waW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> Region1raW<'_, NmienSpec> {
        Region1raW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> Region2waW<'_, NmienSpec> {
        Region2waW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> Region2raW<'_, NmienSpec> {
        Region2raW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> Region3waW<'_, NmienSpec> {
        Region3waW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> Region3raW<'_, NmienSpec> {
        Region3raW::new(self, 7)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> Pregion0waW<'_, NmienSpec> {
        Pregion0waW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> Pregion0raW<'_, NmienSpec> {
        Pregion0raW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> Pregion1waW<'_, NmienSpec> {
        Pregion1waW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> Pregion1raW<'_, NmienSpec> {
        Pregion1raW::new(self, 27)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nmien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmienSpec;
impl crate::RegisterSpec for NmienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmien::R`](R) reader structure"]
impl crate::Readable for NmienSpec {}
#[doc = "`write(|w| ..)` method takes [`nmien::W`](W) writer structure"]
impl crate::Writable for NmienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIEN to value 0"]
impl crate::Resettable for NmienSpec {}
