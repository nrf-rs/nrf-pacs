#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to Disable interrupt for REGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region0wa> for bool {
    #[inline(always)]
    fn from(variant: Region0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` reader - Write '1' to Disable interrupt for REGION\\[0\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region0waWO> for bool {
    #[inline(always)]
    fn from(variant: Region0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` writer - Write '1' to Disable interrupt for REGION\\[0\\].WA event"]
pub type Region0waW<'a, REG> = crate::BitWriter<'a, REG, Region0waWO>;
impl<'a, REG> Region0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region0waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region0ra> for bool {
    #[inline(always)]
    fn from(variant: Region0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` reader - Write '1' to Disable interrupt for REGION\\[0\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region0raWO> for bool {
    #[inline(always)]
    fn from(variant: Region0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` writer - Write '1' to Disable interrupt for REGION\\[0\\].RA event"]
pub type Region0raW<'a, REG> = crate::BitWriter<'a, REG, Region0raWO>;
impl<'a, REG> Region0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region0raWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region1wa> for bool {
    #[inline(always)]
    fn from(variant: Region1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` reader - Write '1' to Disable interrupt for REGION\\[1\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region1waWO> for bool {
    #[inline(always)]
    fn from(variant: Region1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` writer - Write '1' to Disable interrupt for REGION\\[1\\].WA event"]
pub type Region1waW<'a, REG> = crate::BitWriter<'a, REG, Region1waWO>;
impl<'a, REG> Region1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region1waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region1ra> for bool {
    #[inline(always)]
    fn from(variant: Region1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` reader - Write '1' to Disable interrupt for REGION\\[1\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region1raWO> for bool {
    #[inline(always)]
    fn from(variant: Region1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` writer - Write '1' to Disable interrupt for REGION\\[1\\].RA event"]
pub type Region1raW<'a, REG> = crate::BitWriter<'a, REG, Region1raWO>;
impl<'a, REG> Region1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region1raWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[2\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region2wa> for bool {
    #[inline(always)]
    fn from(variant: Region2wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` reader - Write '1' to Disable interrupt for REGION\\[2\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[2\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region2waWO> for bool {
    #[inline(always)]
    fn from(variant: Region2waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` writer - Write '1' to Disable interrupt for REGION\\[2\\].WA event"]
pub type Region2waW<'a, REG> = crate::BitWriter<'a, REG, Region2waWO>;
impl<'a, REG> Region2waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region2waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[2\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region2ra> for bool {
    #[inline(always)]
    fn from(variant: Region2ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` reader - Write '1' to Disable interrupt for REGION\\[2\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[2\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region2raWO> for bool {
    #[inline(always)]
    fn from(variant: Region2raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` writer - Write '1' to Disable interrupt for REGION\\[2\\].RA event"]
pub type Region2raW<'a, REG> = crate::BitWriter<'a, REG, Region2raWO>;
impl<'a, REG> Region2raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region2raWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[3\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region3wa> for bool {
    #[inline(always)]
    fn from(variant: Region3wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` reader - Write '1' to Disable interrupt for REGION\\[3\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[3\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region3waWO> for bool {
    #[inline(always)]
    fn from(variant: Region3waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` writer - Write '1' to Disable interrupt for REGION\\[3\\].WA event"]
pub type Region3waW<'a, REG> = crate::BitWriter<'a, REG, Region3waWO>;
impl<'a, REG> Region3waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region3waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[3\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region3ra> for bool {
    #[inline(always)]
    fn from(variant: Region3ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` reader - Write '1' to Disable interrupt for REGION\\[3\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for REGION\\[3\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Region3raWO> for bool {
    #[inline(always)]
    fn from(variant: Region3raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` writer - Write '1' to Disable interrupt for REGION\\[3\\].RA event"]
pub type Region3raW<'a, REG> = crate::BitWriter<'a, REG, Region3raWO>;
impl<'a, REG> Region3raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Region3raWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion0wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` reader - Write '1' to Disable interrupt for PREGION\\[0\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Pregion0waWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` writer - Write '1' to Disable interrupt for PREGION\\[0\\].WA event"]
pub type Pregion0waW<'a, REG> = crate::BitWriter<'a, REG, Pregion0waWO>;
impl<'a, REG> Pregion0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion0ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` reader - Write '1' to Disable interrupt for PREGION\\[0\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Pregion0raWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` writer - Write '1' to Disable interrupt for PREGION\\[0\\].RA event"]
pub type Pregion0raW<'a, REG> = crate::BitWriter<'a, REG, Pregion0raWO>;
impl<'a, REG> Pregion0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0raWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion1wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` reader - Write '1' to Disable interrupt for PREGION\\[1\\].WA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1wa::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1waWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Pregion1waWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` writer - Write '1' to Disable interrupt for PREGION\\[1\\].WA event"]
pub type Pregion1waW<'a, REG> = crate::BitWriter<'a, REG, Pregion1waWO>;
impl<'a, REG> Pregion1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1waWO::Clear)
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion1ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` reader - Write '1' to Disable interrupt for PREGION\\[1\\].RA event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1ra::Enabled
    }
}
#[doc = "Write '1' to Disable interrupt for PREGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1raWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Pregion1raWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` writer - Write '1' to Disable interrupt for PREGION\\[1\\].RA event"]
pub type Pregion1raW<'a, REG> = crate::BitWriter<'a, REG, Pregion1raWO>;
impl<'a, REG> Pregion1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1raWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&self) -> Region0waR {
        Region0waR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&self) -> Region0raR {
        Region0raR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&self) -> Region1waR {
        Region1waR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&self) -> Region1raR {
        Region1raR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&self) -> Region2waR {
        Region2waR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&self) -> Region2raR {
        Region2raR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&self) -> Region3waR {
        Region3waR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&self) -> Region3raR {
        Region3raR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Write '1' to Disable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> Pregion0waR {
        Pregion0waR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write '1' to Disable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> Pregion0raR {
        Pregion0raR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to Disable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> Pregion1waR {
        Pregion1waR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to Disable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> Pregion1raR {
        Pregion1raR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> Region0waW<'_, IntenclrSpec> {
        Region0waW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> Region0raW<'_, IntenclrSpec> {
        Region0raW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> Region1waW<'_, IntenclrSpec> {
        Region1waW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> Region1raW<'_, IntenclrSpec> {
        Region1raW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> Region2waW<'_, IntenclrSpec> {
        Region2waW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> Region2raW<'_, IntenclrSpec> {
        Region2raW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> Region3waW<'_, IntenclrSpec> {
        Region3waW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> Region3raW<'_, IntenclrSpec> {
        Region3raW::new(self, 7)
    }
    #[doc = "Bit 24 - Write '1' to Disable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> Pregion0waW<'_, IntenclrSpec> {
        Pregion0waW::new(self, 24)
    }
    #[doc = "Bit 25 - Write '1' to Disable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> Pregion0raW<'_, IntenclrSpec> {
        Pregion0raW::new(self, 25)
    }
    #[doc = "Bit 26 - Write '1' to Disable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> Pregion1waW<'_, IntenclrSpec> {
        Pregion1waW::new(self, 26)
    }
    #[doc = "Bit 27 - Write '1' to Disable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> Pregion1raW<'_, IntenclrSpec> {
        Pregion1raW::new(self, 27)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
