#[doc = "Register `REGIONENSET` reader"]
pub type R = crate::R<RegionensetSpec>;
#[doc = "Register `REGIONENSET` writer"]
pub type W = crate::W<RegionensetSpec>;
#[doc = "Enable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0wa {
    #[doc = "0: Write access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn0wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0WA` reader - Enable write access watch in region\\[0\\]"]
pub type Rgn0waR = crate::BitReader<Rgn0wa>;
impl Rgn0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn0wa {
        match self.bits {
            false => Rgn0wa::Disabled,
            true => Rgn0wa::Enabled,
        }
    }
    #[doc = "Write access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn0wa::Disabled
    }
    #[doc = "Write access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn0wa::Enabled
    }
}
#[doc = "Enable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0waWO {
    #[doc = "1: Enable write access watch in this region"]
    Set = 1,
}
impl From<Rgn0waWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0WA` writer - Enable write access watch in region\\[0\\]"]
pub type Rgn0waW<'a, REG> = crate::BitWriter<'a, REG, Rgn0waWO>;
impl<'a, REG> Rgn0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0waWO::Set)
    }
}
#[doc = "Enable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0ra {
    #[doc = "0: Read access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn0ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0RA` reader - Enable read access watch in region\\[0\\]"]
pub type Rgn0raR = crate::BitReader<Rgn0ra>;
impl Rgn0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn0ra {
        match self.bits {
            false => Rgn0ra::Disabled,
            true => Rgn0ra::Enabled,
        }
    }
    #[doc = "Read access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn0ra::Disabled
    }
    #[doc = "Read access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn0ra::Enabled
    }
}
#[doc = "Enable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0raWO {
    #[doc = "1: Enable read access watch in this region"]
    Set = 1,
}
impl From<Rgn0raWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0RA` writer - Enable read access watch in region\\[0\\]"]
pub type Rgn0raW<'a, REG> = crate::BitWriter<'a, REG, Rgn0raWO>;
impl<'a, REG> Rgn0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0raWO::Set)
    }
}
#[doc = "Enable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1wa {
    #[doc = "0: Write access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn1wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1WA` reader - Enable write access watch in region\\[1\\]"]
pub type Rgn1waR = crate::BitReader<Rgn1wa>;
impl Rgn1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn1wa {
        match self.bits {
            false => Rgn1wa::Disabled,
            true => Rgn1wa::Enabled,
        }
    }
    #[doc = "Write access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn1wa::Disabled
    }
    #[doc = "Write access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn1wa::Enabled
    }
}
#[doc = "Enable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1waWO {
    #[doc = "1: Enable write access watch in this region"]
    Set = 1,
}
impl From<Rgn1waWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1WA` writer - Enable write access watch in region\\[1\\]"]
pub type Rgn1waW<'a, REG> = crate::BitWriter<'a, REG, Rgn1waWO>;
impl<'a, REG> Rgn1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1waWO::Set)
    }
}
#[doc = "Enable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1ra {
    #[doc = "0: Read access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn1ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1RA` reader - Enable read access watch in region\\[1\\]"]
pub type Rgn1raR = crate::BitReader<Rgn1ra>;
impl Rgn1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn1ra {
        match self.bits {
            false => Rgn1ra::Disabled,
            true => Rgn1ra::Enabled,
        }
    }
    #[doc = "Read access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn1ra::Disabled
    }
    #[doc = "Read access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn1ra::Enabled
    }
}
#[doc = "Enable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1raWO {
    #[doc = "1: Enable read access watch in this region"]
    Set = 1,
}
impl From<Rgn1raWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1RA` writer - Enable read access watch in region\\[1\\]"]
pub type Rgn1raW<'a, REG> = crate::BitWriter<'a, REG, Rgn1raWO>;
impl<'a, REG> Rgn1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1raWO::Set)
    }
}
#[doc = "Enable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2wa {
    #[doc = "0: Write access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn2wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn2wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2WA` reader - Enable write access watch in region\\[2\\]"]
pub type Rgn2waR = crate::BitReader<Rgn2wa>;
impl Rgn2waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn2wa {
        match self.bits {
            false => Rgn2wa::Disabled,
            true => Rgn2wa::Enabled,
        }
    }
    #[doc = "Write access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn2wa::Disabled
    }
    #[doc = "Write access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn2wa::Enabled
    }
}
#[doc = "Enable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2waWO {
    #[doc = "1: Enable write access watch in this region"]
    Set = 1,
}
impl From<Rgn2waWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn2waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2WA` writer - Enable write access watch in region\\[2\\]"]
pub type Rgn2waW<'a, REG> = crate::BitWriter<'a, REG, Rgn2waWO>;
impl<'a, REG> Rgn2waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2waWO::Set)
    }
}
#[doc = "Enable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2ra {
    #[doc = "0: Read access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn2ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn2ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2RA` reader - Enable read access watch in region\\[2\\]"]
pub type Rgn2raR = crate::BitReader<Rgn2ra>;
impl Rgn2raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn2ra {
        match self.bits {
            false => Rgn2ra::Disabled,
            true => Rgn2ra::Enabled,
        }
    }
    #[doc = "Read access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn2ra::Disabled
    }
    #[doc = "Read access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn2ra::Enabled
    }
}
#[doc = "Enable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2raWO {
    #[doc = "1: Enable read access watch in this region"]
    Set = 1,
}
impl From<Rgn2raWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn2raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2RA` writer - Enable read access watch in region\\[2\\]"]
pub type Rgn2raW<'a, REG> = crate::BitWriter<'a, REG, Rgn2raWO>;
impl<'a, REG> Rgn2raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2raWO::Set)
    }
}
#[doc = "Enable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3wa {
    #[doc = "0: Write access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn3wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn3wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3WA` reader - Enable write access watch in region\\[3\\]"]
pub type Rgn3waR = crate::BitReader<Rgn3wa>;
impl Rgn3waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn3wa {
        match self.bits {
            false => Rgn3wa::Disabled,
            true => Rgn3wa::Enabled,
        }
    }
    #[doc = "Write access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn3wa::Disabled
    }
    #[doc = "Write access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn3wa::Enabled
    }
}
#[doc = "Enable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3waWO {
    #[doc = "1: Enable write access watch in this region"]
    Set = 1,
}
impl From<Rgn3waWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn3waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3WA` writer - Enable write access watch in region\\[3\\]"]
pub type Rgn3waW<'a, REG> = crate::BitWriter<'a, REG, Rgn3waWO>;
impl<'a, REG> Rgn3waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3waWO::Set)
    }
}
#[doc = "Enable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3ra {
    #[doc = "0: Read access watch in this region is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    Enabled = 1,
}
impl From<Rgn3ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn3ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3RA` reader - Enable read access watch in region\\[3\\]"]
pub type Rgn3raR = crate::BitReader<Rgn3ra>;
impl Rgn3raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn3ra {
        match self.bits {
            false => Rgn3ra::Disabled,
            true => Rgn3ra::Enabled,
        }
    }
    #[doc = "Read access watch in this region is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rgn3ra::Disabled
    }
    #[doc = "Read access watch in this region is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rgn3ra::Enabled
    }
}
#[doc = "Enable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3raWO {
    #[doc = "1: Enable read access watch in this region"]
    Set = 1,
}
impl From<Rgn3raWO> for bool {
    #[inline(always)]
    fn from(variant: Rgn3raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3RA` writer - Enable read access watch in region\\[3\\]"]
pub type Rgn3raW<'a, REG> = crate::BitWriter<'a, REG, Rgn3raWO>;
impl<'a, REG> Rgn3raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3raWO::Set)
    }
}
#[doc = "Enable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0wa {
    #[doc = "0: Write access watch in this PREGION is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this PREGION is enabled"]
    Enabled = 1,
}
impl From<Prgn0wa> for bool {
    #[inline(always)]
    fn from(variant: Prgn0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0WA` reader - Enable write access watch in PREGION\\[0\\]"]
pub type Prgn0waR = crate::BitReader<Prgn0wa>;
impl Prgn0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn0wa {
        match self.bits {
            false => Prgn0wa::Disabled,
            true => Prgn0wa::Enabled,
        }
    }
    #[doc = "Write access watch in this PREGION is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prgn0wa::Disabled
    }
    #[doc = "Write access watch in this PREGION is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Prgn0wa::Enabled
    }
}
#[doc = "Enable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0waWO {
    #[doc = "1: Enable write access watch in this PREGION"]
    Set = 1,
}
impl From<Prgn0waWO> for bool {
    #[inline(always)]
    fn from(variant: Prgn0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0WA` writer - Enable write access watch in PREGION\\[0\\]"]
pub type Prgn0waW<'a, REG> = crate::BitWriter<'a, REG, Prgn0waWO>;
impl<'a, REG> Prgn0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0waWO::Set)
    }
}
#[doc = "Enable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0ra {
    #[doc = "0: Read access watch in this PREGION is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this PREGION is enabled"]
    Enabled = 1,
}
impl From<Prgn0ra> for bool {
    #[inline(always)]
    fn from(variant: Prgn0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0RA` reader - Enable read access watch in PREGION\\[0\\]"]
pub type Prgn0raR = crate::BitReader<Prgn0ra>;
impl Prgn0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn0ra {
        match self.bits {
            false => Prgn0ra::Disabled,
            true => Prgn0ra::Enabled,
        }
    }
    #[doc = "Read access watch in this PREGION is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prgn0ra::Disabled
    }
    #[doc = "Read access watch in this PREGION is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Prgn0ra::Enabled
    }
}
#[doc = "Enable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0raWO {
    #[doc = "1: Enable read access watch in this PREGION"]
    Set = 1,
}
impl From<Prgn0raWO> for bool {
    #[inline(always)]
    fn from(variant: Prgn0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0RA` writer - Enable read access watch in PREGION\\[0\\]"]
pub type Prgn0raW<'a, REG> = crate::BitWriter<'a, REG, Prgn0raWO>;
impl<'a, REG> Prgn0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0raWO::Set)
    }
}
#[doc = "Enable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1wa {
    #[doc = "0: Write access watch in this PREGION is disabled"]
    Disabled = 0,
    #[doc = "1: Write access watch in this PREGION is enabled"]
    Enabled = 1,
}
impl From<Prgn1wa> for bool {
    #[inline(always)]
    fn from(variant: Prgn1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1WA` reader - Enable write access watch in PREGION\\[1\\]"]
pub type Prgn1waR = crate::BitReader<Prgn1wa>;
impl Prgn1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn1wa {
        match self.bits {
            false => Prgn1wa::Disabled,
            true => Prgn1wa::Enabled,
        }
    }
    #[doc = "Write access watch in this PREGION is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prgn1wa::Disabled
    }
    #[doc = "Write access watch in this PREGION is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Prgn1wa::Enabled
    }
}
#[doc = "Enable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1waWO {
    #[doc = "1: Enable write access watch in this PREGION"]
    Set = 1,
}
impl From<Prgn1waWO> for bool {
    #[inline(always)]
    fn from(variant: Prgn1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1WA` writer - Enable write access watch in PREGION\\[1\\]"]
pub type Prgn1waW<'a, REG> = crate::BitWriter<'a, REG, Prgn1waWO>;
impl<'a, REG> Prgn1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1waWO::Set)
    }
}
#[doc = "Enable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1ra {
    #[doc = "0: Read access watch in this PREGION is disabled"]
    Disabled = 0,
    #[doc = "1: Read access watch in this PREGION is enabled"]
    Enabled = 1,
}
impl From<Prgn1ra> for bool {
    #[inline(always)]
    fn from(variant: Prgn1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1RA` reader - Enable read access watch in PREGION\\[1\\]"]
pub type Prgn1raR = crate::BitReader<Prgn1ra>;
impl Prgn1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn1ra {
        match self.bits {
            false => Prgn1ra::Disabled,
            true => Prgn1ra::Enabled,
        }
    }
    #[doc = "Read access watch in this PREGION is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prgn1ra::Disabled
    }
    #[doc = "Read access watch in this PREGION is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Prgn1ra::Enabled
    }
}
#[doc = "Enable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1raWO {
    #[doc = "1: Enable read access watch in this PREGION"]
    Set = 1,
}
impl From<Prgn1raWO> for bool {
    #[inline(always)]
    fn from(variant: Prgn1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1RA` writer - Enable read access watch in PREGION\\[1\\]"]
pub type Prgn1raW<'a, REG> = crate::BitWriter<'a, REG, Prgn1raWO>;
impl<'a, REG> Prgn1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1raWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&self) -> Rgn0waR {
        Rgn0waR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&self) -> Rgn0raR {
        Rgn0raR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&self) -> Rgn1waR {
        Rgn1waR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&self) -> Rgn1raR {
        Rgn1raR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&self) -> Rgn2waR {
        Rgn2waR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&self) -> Rgn2raR {
        Rgn2raR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&self) -> Rgn3waR {
        Rgn3waR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&self) -> Rgn3raR {
        Rgn3raR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&self) -> Prgn0waR {
        Prgn0waR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&self) -> Prgn0raR {
        Prgn0raR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&self) -> Prgn1waR {
        Prgn1waR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&self) -> Prgn1raR {
        Prgn1raR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&mut self) -> Rgn0waW<'_, RegionensetSpec> {
        Rgn0waW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&mut self) -> Rgn0raW<'_, RegionensetSpec> {
        Rgn0raW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&mut self) -> Rgn1waW<'_, RegionensetSpec> {
        Rgn1waW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&mut self) -> Rgn1raW<'_, RegionensetSpec> {
        Rgn1raW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&mut self) -> Rgn2waW<'_, RegionensetSpec> {
        Rgn2waW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&mut self) -> Rgn2raW<'_, RegionensetSpec> {
        Rgn2raW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&mut self) -> Rgn3waW<'_, RegionensetSpec> {
        Rgn3waW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&mut self) -> Rgn3raW<'_, RegionensetSpec> {
        Rgn3raW::new(self, 7)
    }
    #[doc = "Bit 24 - Enable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&mut self) -> Prgn0waW<'_, RegionensetSpec> {
        Prgn0waW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&mut self) -> Prgn0raW<'_, RegionensetSpec> {
        Prgn0raW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&mut self) -> Prgn1waW<'_, RegionensetSpec> {
        Prgn1waW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&mut self) -> Prgn1raW<'_, RegionensetSpec> {
        Prgn1raW::new(self, 27)
    }
}
#[doc = "Enable regions watch\n\nYou can [`read`](crate::Reg::read) this register and get [`regionenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regionenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegionensetSpec;
impl crate::RegisterSpec for RegionensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regionenset::R`](R) reader structure"]
impl crate::Readable for RegionensetSpec {}
#[doc = "`write(|w| ..)` method takes [`regionenset::W`](W) writer structure"]
impl crate::Writable for RegionensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGIONENSET to value 0"]
impl crate::Resettable for RegionensetSpec {}
