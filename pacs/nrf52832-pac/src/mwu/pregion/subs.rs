#[doc = "Register `SUBS` reader"]
pub type R = crate::R<SubsSpec>;
#[doc = "Register `SUBS` writer"]
pub type W = crate::W<SubsSpec>;
#[doc = "Include or exclude subregion 0 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr0 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr0> for bool {
    #[inline(always)]
    fn from(variant: Sr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR0` reader - Include or exclude subregion 0 in region"]
pub type Sr0R = crate::BitReader<Sr0>;
impl Sr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr0 {
        match self.bits {
            false => Sr0::Exclude,
            true => Sr0::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr0::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr0::Include
    }
}
#[doc = "Field `SR0` writer - Include or exclude subregion 0 in region"]
pub type Sr0W<'a, REG> = crate::BitWriter<'a, REG, Sr0>;
impl<'a, REG> Sr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr0::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr0::Include)
    }
}
#[doc = "Include or exclude subregion 1 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr1 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr1> for bool {
    #[inline(always)]
    fn from(variant: Sr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR1` reader - Include or exclude subregion 1 in region"]
pub type Sr1R = crate::BitReader<Sr1>;
impl Sr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr1 {
        match self.bits {
            false => Sr1::Exclude,
            true => Sr1::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr1::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr1::Include
    }
}
#[doc = "Field `SR1` writer - Include or exclude subregion 1 in region"]
pub type Sr1W<'a, REG> = crate::BitWriter<'a, REG, Sr1>;
impl<'a, REG> Sr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr1::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr1::Include)
    }
}
#[doc = "Include or exclude subregion 2 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr2 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr2> for bool {
    #[inline(always)]
    fn from(variant: Sr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR2` reader - Include or exclude subregion 2 in region"]
pub type Sr2R = crate::BitReader<Sr2>;
impl Sr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr2 {
        match self.bits {
            false => Sr2::Exclude,
            true => Sr2::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr2::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr2::Include
    }
}
#[doc = "Field `SR2` writer - Include or exclude subregion 2 in region"]
pub type Sr2W<'a, REG> = crate::BitWriter<'a, REG, Sr2>;
impl<'a, REG> Sr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr2::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr2::Include)
    }
}
#[doc = "Include or exclude subregion 3 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr3 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr3> for bool {
    #[inline(always)]
    fn from(variant: Sr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR3` reader - Include or exclude subregion 3 in region"]
pub type Sr3R = crate::BitReader<Sr3>;
impl Sr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr3 {
        match self.bits {
            false => Sr3::Exclude,
            true => Sr3::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr3::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr3::Include
    }
}
#[doc = "Field `SR3` writer - Include or exclude subregion 3 in region"]
pub type Sr3W<'a, REG> = crate::BitWriter<'a, REG, Sr3>;
impl<'a, REG> Sr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr3::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr3::Include)
    }
}
#[doc = "Include or exclude subregion 4 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr4 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr4> for bool {
    #[inline(always)]
    fn from(variant: Sr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR4` reader - Include or exclude subregion 4 in region"]
pub type Sr4R = crate::BitReader<Sr4>;
impl Sr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr4 {
        match self.bits {
            false => Sr4::Exclude,
            true => Sr4::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr4::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr4::Include
    }
}
#[doc = "Field `SR4` writer - Include or exclude subregion 4 in region"]
pub type Sr4W<'a, REG> = crate::BitWriter<'a, REG, Sr4>;
impl<'a, REG> Sr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr4::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr4::Include)
    }
}
#[doc = "Include or exclude subregion 5 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr5 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr5> for bool {
    #[inline(always)]
    fn from(variant: Sr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR5` reader - Include or exclude subregion 5 in region"]
pub type Sr5R = crate::BitReader<Sr5>;
impl Sr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr5 {
        match self.bits {
            false => Sr5::Exclude,
            true => Sr5::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr5::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr5::Include
    }
}
#[doc = "Field `SR5` writer - Include or exclude subregion 5 in region"]
pub type Sr5W<'a, REG> = crate::BitWriter<'a, REG, Sr5>;
impl<'a, REG> Sr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr5::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr5::Include)
    }
}
#[doc = "Include or exclude subregion 6 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr6 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr6> for bool {
    #[inline(always)]
    fn from(variant: Sr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR6` reader - Include or exclude subregion 6 in region"]
pub type Sr6R = crate::BitReader<Sr6>;
impl Sr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr6 {
        match self.bits {
            false => Sr6::Exclude,
            true => Sr6::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr6::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr6::Include
    }
}
#[doc = "Field `SR6` writer - Include or exclude subregion 6 in region"]
pub type Sr6W<'a, REG> = crate::BitWriter<'a, REG, Sr6>;
impl<'a, REG> Sr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr6::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr6::Include)
    }
}
#[doc = "Include or exclude subregion 7 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr7 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr7> for bool {
    #[inline(always)]
    fn from(variant: Sr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR7` reader - Include or exclude subregion 7 in region"]
pub type Sr7R = crate::BitReader<Sr7>;
impl Sr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr7 {
        match self.bits {
            false => Sr7::Exclude,
            true => Sr7::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr7::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr7::Include
    }
}
#[doc = "Field `SR7` writer - Include or exclude subregion 7 in region"]
pub type Sr7W<'a, REG> = crate::BitWriter<'a, REG, Sr7>;
impl<'a, REG> Sr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr7::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr7::Include)
    }
}
#[doc = "Include or exclude subregion 8 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr8 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr8> for bool {
    #[inline(always)]
    fn from(variant: Sr8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR8` reader - Include or exclude subregion 8 in region"]
pub type Sr8R = crate::BitReader<Sr8>;
impl Sr8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr8 {
        match self.bits {
            false => Sr8::Exclude,
            true => Sr8::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr8::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr8::Include
    }
}
#[doc = "Field `SR8` writer - Include or exclude subregion 8 in region"]
pub type Sr8W<'a, REG> = crate::BitWriter<'a, REG, Sr8>;
impl<'a, REG> Sr8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr8::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr8::Include)
    }
}
#[doc = "Include or exclude subregion 9 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr9 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr9> for bool {
    #[inline(always)]
    fn from(variant: Sr9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR9` reader - Include or exclude subregion 9 in region"]
pub type Sr9R = crate::BitReader<Sr9>;
impl Sr9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr9 {
        match self.bits {
            false => Sr9::Exclude,
            true => Sr9::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr9::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr9::Include
    }
}
#[doc = "Field `SR9` writer - Include or exclude subregion 9 in region"]
pub type Sr9W<'a, REG> = crate::BitWriter<'a, REG, Sr9>;
impl<'a, REG> Sr9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr9::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr9::Include)
    }
}
#[doc = "Include or exclude subregion 10 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr10 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr10> for bool {
    #[inline(always)]
    fn from(variant: Sr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR10` reader - Include or exclude subregion 10 in region"]
pub type Sr10R = crate::BitReader<Sr10>;
impl Sr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr10 {
        match self.bits {
            false => Sr10::Exclude,
            true => Sr10::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr10::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr10::Include
    }
}
#[doc = "Field `SR10` writer - Include or exclude subregion 10 in region"]
pub type Sr10W<'a, REG> = crate::BitWriter<'a, REG, Sr10>;
impl<'a, REG> Sr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr10::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr10::Include)
    }
}
#[doc = "Include or exclude subregion 11 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr11 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr11> for bool {
    #[inline(always)]
    fn from(variant: Sr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR11` reader - Include or exclude subregion 11 in region"]
pub type Sr11R = crate::BitReader<Sr11>;
impl Sr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr11 {
        match self.bits {
            false => Sr11::Exclude,
            true => Sr11::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr11::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr11::Include
    }
}
#[doc = "Field `SR11` writer - Include or exclude subregion 11 in region"]
pub type Sr11W<'a, REG> = crate::BitWriter<'a, REG, Sr11>;
impl<'a, REG> Sr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr11::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr11::Include)
    }
}
#[doc = "Include or exclude subregion 12 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr12 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr12> for bool {
    #[inline(always)]
    fn from(variant: Sr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR12` reader - Include or exclude subregion 12 in region"]
pub type Sr12R = crate::BitReader<Sr12>;
impl Sr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr12 {
        match self.bits {
            false => Sr12::Exclude,
            true => Sr12::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr12::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr12::Include
    }
}
#[doc = "Field `SR12` writer - Include or exclude subregion 12 in region"]
pub type Sr12W<'a, REG> = crate::BitWriter<'a, REG, Sr12>;
impl<'a, REG> Sr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr12::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr12::Include)
    }
}
#[doc = "Include or exclude subregion 13 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr13 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr13> for bool {
    #[inline(always)]
    fn from(variant: Sr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR13` reader - Include or exclude subregion 13 in region"]
pub type Sr13R = crate::BitReader<Sr13>;
impl Sr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr13 {
        match self.bits {
            false => Sr13::Exclude,
            true => Sr13::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr13::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr13::Include
    }
}
#[doc = "Field `SR13` writer - Include or exclude subregion 13 in region"]
pub type Sr13W<'a, REG> = crate::BitWriter<'a, REG, Sr13>;
impl<'a, REG> Sr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr13::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr13::Include)
    }
}
#[doc = "Include or exclude subregion 14 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr14 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr14> for bool {
    #[inline(always)]
    fn from(variant: Sr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR14` reader - Include or exclude subregion 14 in region"]
pub type Sr14R = crate::BitReader<Sr14>;
impl Sr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr14 {
        match self.bits {
            false => Sr14::Exclude,
            true => Sr14::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr14::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr14::Include
    }
}
#[doc = "Field `SR14` writer - Include or exclude subregion 14 in region"]
pub type Sr14W<'a, REG> = crate::BitWriter<'a, REG, Sr14>;
impl<'a, REG> Sr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr14::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr14::Include)
    }
}
#[doc = "Include or exclude subregion 15 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr15 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr15> for bool {
    #[inline(always)]
    fn from(variant: Sr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR15` reader - Include or exclude subregion 15 in region"]
pub type Sr15R = crate::BitReader<Sr15>;
impl Sr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr15 {
        match self.bits {
            false => Sr15::Exclude,
            true => Sr15::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr15::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr15::Include
    }
}
#[doc = "Field `SR15` writer - Include or exclude subregion 15 in region"]
pub type Sr15W<'a, REG> = crate::BitWriter<'a, REG, Sr15>;
impl<'a, REG> Sr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr15::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr15::Include)
    }
}
#[doc = "Include or exclude subregion 16 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr16 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr16> for bool {
    #[inline(always)]
    fn from(variant: Sr16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR16` reader - Include or exclude subregion 16 in region"]
pub type Sr16R = crate::BitReader<Sr16>;
impl Sr16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr16 {
        match self.bits {
            false => Sr16::Exclude,
            true => Sr16::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr16::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr16::Include
    }
}
#[doc = "Field `SR16` writer - Include or exclude subregion 16 in region"]
pub type Sr16W<'a, REG> = crate::BitWriter<'a, REG, Sr16>;
impl<'a, REG> Sr16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr16::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr16::Include)
    }
}
#[doc = "Include or exclude subregion 17 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr17 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr17> for bool {
    #[inline(always)]
    fn from(variant: Sr17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR17` reader - Include or exclude subregion 17 in region"]
pub type Sr17R = crate::BitReader<Sr17>;
impl Sr17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr17 {
        match self.bits {
            false => Sr17::Exclude,
            true => Sr17::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr17::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr17::Include
    }
}
#[doc = "Field `SR17` writer - Include or exclude subregion 17 in region"]
pub type Sr17W<'a, REG> = crate::BitWriter<'a, REG, Sr17>;
impl<'a, REG> Sr17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr17::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr17::Include)
    }
}
#[doc = "Include or exclude subregion 18 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr18 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr18> for bool {
    #[inline(always)]
    fn from(variant: Sr18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR18` reader - Include or exclude subregion 18 in region"]
pub type Sr18R = crate::BitReader<Sr18>;
impl Sr18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr18 {
        match self.bits {
            false => Sr18::Exclude,
            true => Sr18::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr18::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr18::Include
    }
}
#[doc = "Field `SR18` writer - Include or exclude subregion 18 in region"]
pub type Sr18W<'a, REG> = crate::BitWriter<'a, REG, Sr18>;
impl<'a, REG> Sr18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr18::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr18::Include)
    }
}
#[doc = "Include or exclude subregion 19 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr19 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr19> for bool {
    #[inline(always)]
    fn from(variant: Sr19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR19` reader - Include or exclude subregion 19 in region"]
pub type Sr19R = crate::BitReader<Sr19>;
impl Sr19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr19 {
        match self.bits {
            false => Sr19::Exclude,
            true => Sr19::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr19::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr19::Include
    }
}
#[doc = "Field `SR19` writer - Include or exclude subregion 19 in region"]
pub type Sr19W<'a, REG> = crate::BitWriter<'a, REG, Sr19>;
impl<'a, REG> Sr19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr19::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr19::Include)
    }
}
#[doc = "Include or exclude subregion 20 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr20 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr20> for bool {
    #[inline(always)]
    fn from(variant: Sr20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR20` reader - Include or exclude subregion 20 in region"]
pub type Sr20R = crate::BitReader<Sr20>;
impl Sr20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr20 {
        match self.bits {
            false => Sr20::Exclude,
            true => Sr20::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr20::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr20::Include
    }
}
#[doc = "Field `SR20` writer - Include or exclude subregion 20 in region"]
pub type Sr20W<'a, REG> = crate::BitWriter<'a, REG, Sr20>;
impl<'a, REG> Sr20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr20::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr20::Include)
    }
}
#[doc = "Include or exclude subregion 21 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr21 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr21> for bool {
    #[inline(always)]
    fn from(variant: Sr21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR21` reader - Include or exclude subregion 21 in region"]
pub type Sr21R = crate::BitReader<Sr21>;
impl Sr21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr21 {
        match self.bits {
            false => Sr21::Exclude,
            true => Sr21::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr21::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr21::Include
    }
}
#[doc = "Field `SR21` writer - Include or exclude subregion 21 in region"]
pub type Sr21W<'a, REG> = crate::BitWriter<'a, REG, Sr21>;
impl<'a, REG> Sr21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr21::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr21::Include)
    }
}
#[doc = "Include or exclude subregion 22 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr22 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr22> for bool {
    #[inline(always)]
    fn from(variant: Sr22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR22` reader - Include or exclude subregion 22 in region"]
pub type Sr22R = crate::BitReader<Sr22>;
impl Sr22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr22 {
        match self.bits {
            false => Sr22::Exclude,
            true => Sr22::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr22::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr22::Include
    }
}
#[doc = "Field `SR22` writer - Include or exclude subregion 22 in region"]
pub type Sr22W<'a, REG> = crate::BitWriter<'a, REG, Sr22>;
impl<'a, REG> Sr22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr22::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr22::Include)
    }
}
#[doc = "Include or exclude subregion 23 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr23 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr23> for bool {
    #[inline(always)]
    fn from(variant: Sr23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR23` reader - Include or exclude subregion 23 in region"]
pub type Sr23R = crate::BitReader<Sr23>;
impl Sr23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr23 {
        match self.bits {
            false => Sr23::Exclude,
            true => Sr23::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr23::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr23::Include
    }
}
#[doc = "Field `SR23` writer - Include or exclude subregion 23 in region"]
pub type Sr23W<'a, REG> = crate::BitWriter<'a, REG, Sr23>;
impl<'a, REG> Sr23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr23::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr23::Include)
    }
}
#[doc = "Include or exclude subregion 24 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr24 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr24> for bool {
    #[inline(always)]
    fn from(variant: Sr24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR24` reader - Include or exclude subregion 24 in region"]
pub type Sr24R = crate::BitReader<Sr24>;
impl Sr24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr24 {
        match self.bits {
            false => Sr24::Exclude,
            true => Sr24::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr24::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr24::Include
    }
}
#[doc = "Field `SR24` writer - Include or exclude subregion 24 in region"]
pub type Sr24W<'a, REG> = crate::BitWriter<'a, REG, Sr24>;
impl<'a, REG> Sr24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr24::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr24::Include)
    }
}
#[doc = "Include or exclude subregion 25 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr25 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr25> for bool {
    #[inline(always)]
    fn from(variant: Sr25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR25` reader - Include or exclude subregion 25 in region"]
pub type Sr25R = crate::BitReader<Sr25>;
impl Sr25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr25 {
        match self.bits {
            false => Sr25::Exclude,
            true => Sr25::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr25::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr25::Include
    }
}
#[doc = "Field `SR25` writer - Include or exclude subregion 25 in region"]
pub type Sr25W<'a, REG> = crate::BitWriter<'a, REG, Sr25>;
impl<'a, REG> Sr25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr25::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr25::Include)
    }
}
#[doc = "Include or exclude subregion 26 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr26 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr26> for bool {
    #[inline(always)]
    fn from(variant: Sr26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR26` reader - Include or exclude subregion 26 in region"]
pub type Sr26R = crate::BitReader<Sr26>;
impl Sr26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr26 {
        match self.bits {
            false => Sr26::Exclude,
            true => Sr26::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr26::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr26::Include
    }
}
#[doc = "Field `SR26` writer - Include or exclude subregion 26 in region"]
pub type Sr26W<'a, REG> = crate::BitWriter<'a, REG, Sr26>;
impl<'a, REG> Sr26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr26::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr26::Include)
    }
}
#[doc = "Include or exclude subregion 27 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr27 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr27> for bool {
    #[inline(always)]
    fn from(variant: Sr27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR27` reader - Include or exclude subregion 27 in region"]
pub type Sr27R = crate::BitReader<Sr27>;
impl Sr27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr27 {
        match self.bits {
            false => Sr27::Exclude,
            true => Sr27::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr27::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr27::Include
    }
}
#[doc = "Field `SR27` writer - Include or exclude subregion 27 in region"]
pub type Sr27W<'a, REG> = crate::BitWriter<'a, REG, Sr27>;
impl<'a, REG> Sr27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr27::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr27::Include)
    }
}
#[doc = "Include or exclude subregion 28 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr28 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr28> for bool {
    #[inline(always)]
    fn from(variant: Sr28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR28` reader - Include or exclude subregion 28 in region"]
pub type Sr28R = crate::BitReader<Sr28>;
impl Sr28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr28 {
        match self.bits {
            false => Sr28::Exclude,
            true => Sr28::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr28::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr28::Include
    }
}
#[doc = "Field `SR28` writer - Include or exclude subregion 28 in region"]
pub type Sr28W<'a, REG> = crate::BitWriter<'a, REG, Sr28>;
impl<'a, REG> Sr28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr28::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr28::Include)
    }
}
#[doc = "Include or exclude subregion 29 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr29 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr29> for bool {
    #[inline(always)]
    fn from(variant: Sr29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR29` reader - Include or exclude subregion 29 in region"]
pub type Sr29R = crate::BitReader<Sr29>;
impl Sr29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr29 {
        match self.bits {
            false => Sr29::Exclude,
            true => Sr29::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr29::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr29::Include
    }
}
#[doc = "Field `SR29` writer - Include or exclude subregion 29 in region"]
pub type Sr29W<'a, REG> = crate::BitWriter<'a, REG, Sr29>;
impl<'a, REG> Sr29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr29::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr29::Include)
    }
}
#[doc = "Include or exclude subregion 30 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr30 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr30> for bool {
    #[inline(always)]
    fn from(variant: Sr30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR30` reader - Include or exclude subregion 30 in region"]
pub type Sr30R = crate::BitReader<Sr30>;
impl Sr30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr30 {
        match self.bits {
            false => Sr30::Exclude,
            true => Sr30::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr30::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr30::Include
    }
}
#[doc = "Field `SR30` writer - Include or exclude subregion 30 in region"]
pub type Sr30W<'a, REG> = crate::BitWriter<'a, REG, Sr30>;
impl<'a, REG> Sr30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr30::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr30::Include)
    }
}
#[doc = "Include or exclude subregion 31 in region\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr31 {
    #[doc = "0: Exclude"]
    Exclude = 0,
    #[doc = "1: Include"]
    Include = 1,
}
impl From<Sr31> for bool {
    #[inline(always)]
    fn from(variant: Sr31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR31` reader - Include or exclude subregion 31 in region"]
pub type Sr31R = crate::BitReader<Sr31>;
impl Sr31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr31 {
        match self.bits {
            false => Sr31::Exclude,
            true => Sr31::Include,
        }
    }
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Sr31::Exclude
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Sr31::Include
    }
}
#[doc = "Field `SR31` writer - Include or exclude subregion 31 in region"]
pub type Sr31W<'a, REG> = crate::BitWriter<'a, REG, Sr31>;
impl<'a, REG> Sr31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Sr31::Exclude)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Sr31::Include)
    }
}
impl R {
    #[doc = "Bit 0 - Include or exclude subregion 0 in region"]
    #[inline(always)]
    pub fn sr0(&self) -> Sr0R {
        Sr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Include or exclude subregion 1 in region"]
    #[inline(always)]
    pub fn sr1(&self) -> Sr1R {
        Sr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Include or exclude subregion 2 in region"]
    #[inline(always)]
    pub fn sr2(&self) -> Sr2R {
        Sr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Include or exclude subregion 3 in region"]
    #[inline(always)]
    pub fn sr3(&self) -> Sr3R {
        Sr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Include or exclude subregion 4 in region"]
    #[inline(always)]
    pub fn sr4(&self) -> Sr4R {
        Sr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Include or exclude subregion 5 in region"]
    #[inline(always)]
    pub fn sr5(&self) -> Sr5R {
        Sr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Include or exclude subregion 6 in region"]
    #[inline(always)]
    pub fn sr6(&self) -> Sr6R {
        Sr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Include or exclude subregion 7 in region"]
    #[inline(always)]
    pub fn sr7(&self) -> Sr7R {
        Sr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Include or exclude subregion 8 in region"]
    #[inline(always)]
    pub fn sr8(&self) -> Sr8R {
        Sr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include or exclude subregion 9 in region"]
    #[inline(always)]
    pub fn sr9(&self) -> Sr9R {
        Sr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Include or exclude subregion 10 in region"]
    #[inline(always)]
    pub fn sr10(&self) -> Sr10R {
        Sr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Include or exclude subregion 11 in region"]
    #[inline(always)]
    pub fn sr11(&self) -> Sr11R {
        Sr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Include or exclude subregion 12 in region"]
    #[inline(always)]
    pub fn sr12(&self) -> Sr12R {
        Sr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Include or exclude subregion 13 in region"]
    #[inline(always)]
    pub fn sr13(&self) -> Sr13R {
        Sr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Include or exclude subregion 14 in region"]
    #[inline(always)]
    pub fn sr14(&self) -> Sr14R {
        Sr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Include or exclude subregion 15 in region"]
    #[inline(always)]
    pub fn sr15(&self) -> Sr15R {
        Sr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Include or exclude subregion 16 in region"]
    #[inline(always)]
    pub fn sr16(&self) -> Sr16R {
        Sr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Include or exclude subregion 17 in region"]
    #[inline(always)]
    pub fn sr17(&self) -> Sr17R {
        Sr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Include or exclude subregion 18 in region"]
    #[inline(always)]
    pub fn sr18(&self) -> Sr18R {
        Sr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Include or exclude subregion 19 in region"]
    #[inline(always)]
    pub fn sr19(&self) -> Sr19R {
        Sr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Include or exclude subregion 20 in region"]
    #[inline(always)]
    pub fn sr20(&self) -> Sr20R {
        Sr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Include or exclude subregion 21 in region"]
    #[inline(always)]
    pub fn sr21(&self) -> Sr21R {
        Sr21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Include or exclude subregion 22 in region"]
    #[inline(always)]
    pub fn sr22(&self) -> Sr22R {
        Sr22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Include or exclude subregion 23 in region"]
    #[inline(always)]
    pub fn sr23(&self) -> Sr23R {
        Sr23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Include or exclude subregion 24 in region"]
    #[inline(always)]
    pub fn sr24(&self) -> Sr24R {
        Sr24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Include or exclude subregion 25 in region"]
    #[inline(always)]
    pub fn sr25(&self) -> Sr25R {
        Sr25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Include or exclude subregion 26 in region"]
    #[inline(always)]
    pub fn sr26(&self) -> Sr26R {
        Sr26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Include or exclude subregion 27 in region"]
    #[inline(always)]
    pub fn sr27(&self) -> Sr27R {
        Sr27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Include or exclude subregion 28 in region"]
    #[inline(always)]
    pub fn sr28(&self) -> Sr28R {
        Sr28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Include or exclude subregion 29 in region"]
    #[inline(always)]
    pub fn sr29(&self) -> Sr29R {
        Sr29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Include or exclude subregion 30 in region"]
    #[inline(always)]
    pub fn sr30(&self) -> Sr30R {
        Sr30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Include or exclude subregion 31 in region"]
    #[inline(always)]
    pub fn sr31(&self) -> Sr31R {
        Sr31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Include or exclude subregion 0 in region"]
    #[inline(always)]
    pub fn sr0(&mut self) -> Sr0W<'_, SubsSpec> {
        Sr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Include or exclude subregion 1 in region"]
    #[inline(always)]
    pub fn sr1(&mut self) -> Sr1W<'_, SubsSpec> {
        Sr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Include or exclude subregion 2 in region"]
    #[inline(always)]
    pub fn sr2(&mut self) -> Sr2W<'_, SubsSpec> {
        Sr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Include or exclude subregion 3 in region"]
    #[inline(always)]
    pub fn sr3(&mut self) -> Sr3W<'_, SubsSpec> {
        Sr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Include or exclude subregion 4 in region"]
    #[inline(always)]
    pub fn sr4(&mut self) -> Sr4W<'_, SubsSpec> {
        Sr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Include or exclude subregion 5 in region"]
    #[inline(always)]
    pub fn sr5(&mut self) -> Sr5W<'_, SubsSpec> {
        Sr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Include or exclude subregion 6 in region"]
    #[inline(always)]
    pub fn sr6(&mut self) -> Sr6W<'_, SubsSpec> {
        Sr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Include or exclude subregion 7 in region"]
    #[inline(always)]
    pub fn sr7(&mut self) -> Sr7W<'_, SubsSpec> {
        Sr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Include or exclude subregion 8 in region"]
    #[inline(always)]
    pub fn sr8(&mut self) -> Sr8W<'_, SubsSpec> {
        Sr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Include or exclude subregion 9 in region"]
    #[inline(always)]
    pub fn sr9(&mut self) -> Sr9W<'_, SubsSpec> {
        Sr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Include or exclude subregion 10 in region"]
    #[inline(always)]
    pub fn sr10(&mut self) -> Sr10W<'_, SubsSpec> {
        Sr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Include or exclude subregion 11 in region"]
    #[inline(always)]
    pub fn sr11(&mut self) -> Sr11W<'_, SubsSpec> {
        Sr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Include or exclude subregion 12 in region"]
    #[inline(always)]
    pub fn sr12(&mut self) -> Sr12W<'_, SubsSpec> {
        Sr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Include or exclude subregion 13 in region"]
    #[inline(always)]
    pub fn sr13(&mut self) -> Sr13W<'_, SubsSpec> {
        Sr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Include or exclude subregion 14 in region"]
    #[inline(always)]
    pub fn sr14(&mut self) -> Sr14W<'_, SubsSpec> {
        Sr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Include or exclude subregion 15 in region"]
    #[inline(always)]
    pub fn sr15(&mut self) -> Sr15W<'_, SubsSpec> {
        Sr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Include or exclude subregion 16 in region"]
    #[inline(always)]
    pub fn sr16(&mut self) -> Sr16W<'_, SubsSpec> {
        Sr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Include or exclude subregion 17 in region"]
    #[inline(always)]
    pub fn sr17(&mut self) -> Sr17W<'_, SubsSpec> {
        Sr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Include or exclude subregion 18 in region"]
    #[inline(always)]
    pub fn sr18(&mut self) -> Sr18W<'_, SubsSpec> {
        Sr18W::new(self, 18)
    }
    #[doc = "Bit 19 - Include or exclude subregion 19 in region"]
    #[inline(always)]
    pub fn sr19(&mut self) -> Sr19W<'_, SubsSpec> {
        Sr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Include or exclude subregion 20 in region"]
    #[inline(always)]
    pub fn sr20(&mut self) -> Sr20W<'_, SubsSpec> {
        Sr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Include or exclude subregion 21 in region"]
    #[inline(always)]
    pub fn sr21(&mut self) -> Sr21W<'_, SubsSpec> {
        Sr21W::new(self, 21)
    }
    #[doc = "Bit 22 - Include or exclude subregion 22 in region"]
    #[inline(always)]
    pub fn sr22(&mut self) -> Sr22W<'_, SubsSpec> {
        Sr22W::new(self, 22)
    }
    #[doc = "Bit 23 - Include or exclude subregion 23 in region"]
    #[inline(always)]
    pub fn sr23(&mut self) -> Sr23W<'_, SubsSpec> {
        Sr23W::new(self, 23)
    }
    #[doc = "Bit 24 - Include or exclude subregion 24 in region"]
    #[inline(always)]
    pub fn sr24(&mut self) -> Sr24W<'_, SubsSpec> {
        Sr24W::new(self, 24)
    }
    #[doc = "Bit 25 - Include or exclude subregion 25 in region"]
    #[inline(always)]
    pub fn sr25(&mut self) -> Sr25W<'_, SubsSpec> {
        Sr25W::new(self, 25)
    }
    #[doc = "Bit 26 - Include or exclude subregion 26 in region"]
    #[inline(always)]
    pub fn sr26(&mut self) -> Sr26W<'_, SubsSpec> {
        Sr26W::new(self, 26)
    }
    #[doc = "Bit 27 - Include or exclude subregion 27 in region"]
    #[inline(always)]
    pub fn sr27(&mut self) -> Sr27W<'_, SubsSpec> {
        Sr27W::new(self, 27)
    }
    #[doc = "Bit 28 - Include or exclude subregion 28 in region"]
    #[inline(always)]
    pub fn sr28(&mut self) -> Sr28W<'_, SubsSpec> {
        Sr28W::new(self, 28)
    }
    #[doc = "Bit 29 - Include or exclude subregion 29 in region"]
    #[inline(always)]
    pub fn sr29(&mut self) -> Sr29W<'_, SubsSpec> {
        Sr29W::new(self, 29)
    }
    #[doc = "Bit 30 - Include or exclude subregion 30 in region"]
    #[inline(always)]
    pub fn sr30(&mut self) -> Sr30W<'_, SubsSpec> {
        Sr30W::new(self, 30)
    }
    #[doc = "Bit 31 - Include or exclude subregion 31 in region"]
    #[inline(always)]
    pub fn sr31(&mut self) -> Sr31W<'_, SubsSpec> {
        Sr31W::new(self, 31)
    }
}
#[doc = "Description cluster\\[0\\]: Subregions of region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`subs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsSpec;
impl crate::RegisterSpec for SubsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subs::R`](R) reader structure"]
impl crate::Readable for SubsSpec {}
#[doc = "`write(|w| ..)` method takes [`subs::W`](W) writer structure"]
impl crate::Writable for SubsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUBS to value 0"]
impl crate::Resettable for SubsSpec {}
