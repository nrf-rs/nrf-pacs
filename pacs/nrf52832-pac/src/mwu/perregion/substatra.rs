#[doc = "Register `SUBSTATRA` reader"]
pub type R = crate::R<SubstatraSpec>;
#[doc = "Register `SUBSTATRA` writer"]
pub type W = crate::W<SubstatraSpec>;
#[doc = "Subregion 0 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr0 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr0> for bool {
    #[inline(always)]
    fn from(variant: Sr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR0` reader - Subregion 0 in region 0 (write '1' to clear)"]
pub type Sr0R = crate::BitReader<Sr0>;
impl Sr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr0 {
        match self.bits {
            false => Sr0::NoAccess,
            true => Sr0::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr0::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr0::Access
    }
}
#[doc = "Field `SR0` writer - Subregion 0 in region 0 (write '1' to clear)"]
pub type Sr0W<'a, REG> = crate::BitWriter1C<'a, REG, Sr0>;
impl<'a, REG> Sr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr0::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr0::Access)
    }
}
#[doc = "Subregion 1 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr1 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr1> for bool {
    #[inline(always)]
    fn from(variant: Sr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR1` reader - Subregion 1 in region 0 (write '1' to clear)"]
pub type Sr1R = crate::BitReader<Sr1>;
impl Sr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr1 {
        match self.bits {
            false => Sr1::NoAccess,
            true => Sr1::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr1::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr1::Access
    }
}
#[doc = "Field `SR1` writer - Subregion 1 in region 0 (write '1' to clear)"]
pub type Sr1W<'a, REG> = crate::BitWriter1C<'a, REG, Sr1>;
impl<'a, REG> Sr1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr1::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr1::Access)
    }
}
#[doc = "Subregion 2 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr2 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr2> for bool {
    #[inline(always)]
    fn from(variant: Sr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR2` reader - Subregion 2 in region 0 (write '1' to clear)"]
pub type Sr2R = crate::BitReader<Sr2>;
impl Sr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr2 {
        match self.bits {
            false => Sr2::NoAccess,
            true => Sr2::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr2::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr2::Access
    }
}
#[doc = "Field `SR2` writer - Subregion 2 in region 0 (write '1' to clear)"]
pub type Sr2W<'a, REG> = crate::BitWriter1C<'a, REG, Sr2>;
impl<'a, REG> Sr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr2::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr2::Access)
    }
}
#[doc = "Subregion 3 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr3 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr3> for bool {
    #[inline(always)]
    fn from(variant: Sr3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR3` reader - Subregion 3 in region 0 (write '1' to clear)"]
pub type Sr3R = crate::BitReader<Sr3>;
impl Sr3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr3 {
        match self.bits {
            false => Sr3::NoAccess,
            true => Sr3::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr3::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr3::Access
    }
}
#[doc = "Field `SR3` writer - Subregion 3 in region 0 (write '1' to clear)"]
pub type Sr3W<'a, REG> = crate::BitWriter1C<'a, REG, Sr3>;
impl<'a, REG> Sr3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr3::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr3::Access)
    }
}
#[doc = "Subregion 4 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr4 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr4> for bool {
    #[inline(always)]
    fn from(variant: Sr4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR4` reader - Subregion 4 in region 0 (write '1' to clear)"]
pub type Sr4R = crate::BitReader<Sr4>;
impl Sr4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr4 {
        match self.bits {
            false => Sr4::NoAccess,
            true => Sr4::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr4::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr4::Access
    }
}
#[doc = "Field `SR4` writer - Subregion 4 in region 0 (write '1' to clear)"]
pub type Sr4W<'a, REG> = crate::BitWriter1C<'a, REG, Sr4>;
impl<'a, REG> Sr4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr4::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr4::Access)
    }
}
#[doc = "Subregion 5 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr5 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr5> for bool {
    #[inline(always)]
    fn from(variant: Sr5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR5` reader - Subregion 5 in region 0 (write '1' to clear)"]
pub type Sr5R = crate::BitReader<Sr5>;
impl Sr5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr5 {
        match self.bits {
            false => Sr5::NoAccess,
            true => Sr5::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr5::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr5::Access
    }
}
#[doc = "Field `SR5` writer - Subregion 5 in region 0 (write '1' to clear)"]
pub type Sr5W<'a, REG> = crate::BitWriter1C<'a, REG, Sr5>;
impl<'a, REG> Sr5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr5::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr5::Access)
    }
}
#[doc = "Subregion 6 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr6 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr6> for bool {
    #[inline(always)]
    fn from(variant: Sr6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR6` reader - Subregion 6 in region 0 (write '1' to clear)"]
pub type Sr6R = crate::BitReader<Sr6>;
impl Sr6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr6 {
        match self.bits {
            false => Sr6::NoAccess,
            true => Sr6::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr6::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr6::Access
    }
}
#[doc = "Field `SR6` writer - Subregion 6 in region 0 (write '1' to clear)"]
pub type Sr6W<'a, REG> = crate::BitWriter1C<'a, REG, Sr6>;
impl<'a, REG> Sr6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr6::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr6::Access)
    }
}
#[doc = "Subregion 7 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr7 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr7> for bool {
    #[inline(always)]
    fn from(variant: Sr7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR7` reader - Subregion 7 in region 0 (write '1' to clear)"]
pub type Sr7R = crate::BitReader<Sr7>;
impl Sr7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr7 {
        match self.bits {
            false => Sr7::NoAccess,
            true => Sr7::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr7::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr7::Access
    }
}
#[doc = "Field `SR7` writer - Subregion 7 in region 0 (write '1' to clear)"]
pub type Sr7W<'a, REG> = crate::BitWriter1C<'a, REG, Sr7>;
impl<'a, REG> Sr7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr7::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr7::Access)
    }
}
#[doc = "Subregion 8 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr8 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr8> for bool {
    #[inline(always)]
    fn from(variant: Sr8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR8` reader - Subregion 8 in region 0 (write '1' to clear)"]
pub type Sr8R = crate::BitReader<Sr8>;
impl Sr8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr8 {
        match self.bits {
            false => Sr8::NoAccess,
            true => Sr8::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr8::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr8::Access
    }
}
#[doc = "Field `SR8` writer - Subregion 8 in region 0 (write '1' to clear)"]
pub type Sr8W<'a, REG> = crate::BitWriter1C<'a, REG, Sr8>;
impl<'a, REG> Sr8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr8::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr8::Access)
    }
}
#[doc = "Subregion 9 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr9 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr9> for bool {
    #[inline(always)]
    fn from(variant: Sr9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR9` reader - Subregion 9 in region 0 (write '1' to clear)"]
pub type Sr9R = crate::BitReader<Sr9>;
impl Sr9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr9 {
        match self.bits {
            false => Sr9::NoAccess,
            true => Sr9::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr9::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr9::Access
    }
}
#[doc = "Field `SR9` writer - Subregion 9 in region 0 (write '1' to clear)"]
pub type Sr9W<'a, REG> = crate::BitWriter1C<'a, REG, Sr9>;
impl<'a, REG> Sr9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr9::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr9::Access)
    }
}
#[doc = "Subregion 10 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr10 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr10> for bool {
    #[inline(always)]
    fn from(variant: Sr10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR10` reader - Subregion 10 in region 0 (write '1' to clear)"]
pub type Sr10R = crate::BitReader<Sr10>;
impl Sr10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr10 {
        match self.bits {
            false => Sr10::NoAccess,
            true => Sr10::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr10::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr10::Access
    }
}
#[doc = "Field `SR10` writer - Subregion 10 in region 0 (write '1' to clear)"]
pub type Sr10W<'a, REG> = crate::BitWriter1C<'a, REG, Sr10>;
impl<'a, REG> Sr10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr10::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr10::Access)
    }
}
#[doc = "Subregion 11 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr11 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr11> for bool {
    #[inline(always)]
    fn from(variant: Sr11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR11` reader - Subregion 11 in region 0 (write '1' to clear)"]
pub type Sr11R = crate::BitReader<Sr11>;
impl Sr11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr11 {
        match self.bits {
            false => Sr11::NoAccess,
            true => Sr11::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr11::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr11::Access
    }
}
#[doc = "Field `SR11` writer - Subregion 11 in region 0 (write '1' to clear)"]
pub type Sr11W<'a, REG> = crate::BitWriter1C<'a, REG, Sr11>;
impl<'a, REG> Sr11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr11::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr11::Access)
    }
}
#[doc = "Subregion 12 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr12 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr12> for bool {
    #[inline(always)]
    fn from(variant: Sr12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR12` reader - Subregion 12 in region 0 (write '1' to clear)"]
pub type Sr12R = crate::BitReader<Sr12>;
impl Sr12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr12 {
        match self.bits {
            false => Sr12::NoAccess,
            true => Sr12::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr12::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr12::Access
    }
}
#[doc = "Field `SR12` writer - Subregion 12 in region 0 (write '1' to clear)"]
pub type Sr12W<'a, REG> = crate::BitWriter1C<'a, REG, Sr12>;
impl<'a, REG> Sr12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr12::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr12::Access)
    }
}
#[doc = "Subregion 13 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr13 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr13> for bool {
    #[inline(always)]
    fn from(variant: Sr13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR13` reader - Subregion 13 in region 0 (write '1' to clear)"]
pub type Sr13R = crate::BitReader<Sr13>;
impl Sr13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr13 {
        match self.bits {
            false => Sr13::NoAccess,
            true => Sr13::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr13::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr13::Access
    }
}
#[doc = "Field `SR13` writer - Subregion 13 in region 0 (write '1' to clear)"]
pub type Sr13W<'a, REG> = crate::BitWriter1C<'a, REG, Sr13>;
impl<'a, REG> Sr13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr13::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr13::Access)
    }
}
#[doc = "Subregion 14 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr14 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr14> for bool {
    #[inline(always)]
    fn from(variant: Sr14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR14` reader - Subregion 14 in region 0 (write '1' to clear)"]
pub type Sr14R = crate::BitReader<Sr14>;
impl Sr14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr14 {
        match self.bits {
            false => Sr14::NoAccess,
            true => Sr14::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr14::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr14::Access
    }
}
#[doc = "Field `SR14` writer - Subregion 14 in region 0 (write '1' to clear)"]
pub type Sr14W<'a, REG> = crate::BitWriter1C<'a, REG, Sr14>;
impl<'a, REG> Sr14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr14::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr14::Access)
    }
}
#[doc = "Subregion 15 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr15 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr15> for bool {
    #[inline(always)]
    fn from(variant: Sr15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR15` reader - Subregion 15 in region 0 (write '1' to clear)"]
pub type Sr15R = crate::BitReader<Sr15>;
impl Sr15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr15 {
        match self.bits {
            false => Sr15::NoAccess,
            true => Sr15::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr15::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr15::Access
    }
}
#[doc = "Field `SR15` writer - Subregion 15 in region 0 (write '1' to clear)"]
pub type Sr15W<'a, REG> = crate::BitWriter1C<'a, REG, Sr15>;
impl<'a, REG> Sr15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr15::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr15::Access)
    }
}
#[doc = "Subregion 16 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr16 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr16> for bool {
    #[inline(always)]
    fn from(variant: Sr16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR16` reader - Subregion 16 in region 0 (write '1' to clear)"]
pub type Sr16R = crate::BitReader<Sr16>;
impl Sr16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr16 {
        match self.bits {
            false => Sr16::NoAccess,
            true => Sr16::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr16::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr16::Access
    }
}
#[doc = "Field `SR16` writer - Subregion 16 in region 0 (write '1' to clear)"]
pub type Sr16W<'a, REG> = crate::BitWriter1C<'a, REG, Sr16>;
impl<'a, REG> Sr16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr16::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr16::Access)
    }
}
#[doc = "Subregion 17 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr17 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr17> for bool {
    #[inline(always)]
    fn from(variant: Sr17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR17` reader - Subregion 17 in region 0 (write '1' to clear)"]
pub type Sr17R = crate::BitReader<Sr17>;
impl Sr17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr17 {
        match self.bits {
            false => Sr17::NoAccess,
            true => Sr17::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr17::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr17::Access
    }
}
#[doc = "Field `SR17` writer - Subregion 17 in region 0 (write '1' to clear)"]
pub type Sr17W<'a, REG> = crate::BitWriter1C<'a, REG, Sr17>;
impl<'a, REG> Sr17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr17::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr17::Access)
    }
}
#[doc = "Subregion 18 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr18 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr18> for bool {
    #[inline(always)]
    fn from(variant: Sr18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR18` reader - Subregion 18 in region 0 (write '1' to clear)"]
pub type Sr18R = crate::BitReader<Sr18>;
impl Sr18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr18 {
        match self.bits {
            false => Sr18::NoAccess,
            true => Sr18::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr18::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr18::Access
    }
}
#[doc = "Field `SR18` writer - Subregion 18 in region 0 (write '1' to clear)"]
pub type Sr18W<'a, REG> = crate::BitWriter1C<'a, REG, Sr18>;
impl<'a, REG> Sr18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr18::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr18::Access)
    }
}
#[doc = "Subregion 19 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr19 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr19> for bool {
    #[inline(always)]
    fn from(variant: Sr19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR19` reader - Subregion 19 in region 0 (write '1' to clear)"]
pub type Sr19R = crate::BitReader<Sr19>;
impl Sr19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr19 {
        match self.bits {
            false => Sr19::NoAccess,
            true => Sr19::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr19::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr19::Access
    }
}
#[doc = "Field `SR19` writer - Subregion 19 in region 0 (write '1' to clear)"]
pub type Sr19W<'a, REG> = crate::BitWriter1C<'a, REG, Sr19>;
impl<'a, REG> Sr19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr19::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr19::Access)
    }
}
#[doc = "Subregion 20 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr20 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr20> for bool {
    #[inline(always)]
    fn from(variant: Sr20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR20` reader - Subregion 20 in region 0 (write '1' to clear)"]
pub type Sr20R = crate::BitReader<Sr20>;
impl Sr20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr20 {
        match self.bits {
            false => Sr20::NoAccess,
            true => Sr20::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr20::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr20::Access
    }
}
#[doc = "Field `SR20` writer - Subregion 20 in region 0 (write '1' to clear)"]
pub type Sr20W<'a, REG> = crate::BitWriter1C<'a, REG, Sr20>;
impl<'a, REG> Sr20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr20::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr20::Access)
    }
}
#[doc = "Subregion 21 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr21 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr21> for bool {
    #[inline(always)]
    fn from(variant: Sr21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR21` reader - Subregion 21 in region 0 (write '1' to clear)"]
pub type Sr21R = crate::BitReader<Sr21>;
impl Sr21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr21 {
        match self.bits {
            false => Sr21::NoAccess,
            true => Sr21::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr21::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr21::Access
    }
}
#[doc = "Field `SR21` writer - Subregion 21 in region 0 (write '1' to clear)"]
pub type Sr21W<'a, REG> = crate::BitWriter1C<'a, REG, Sr21>;
impl<'a, REG> Sr21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr21::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr21::Access)
    }
}
#[doc = "Subregion 22 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr22 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr22> for bool {
    #[inline(always)]
    fn from(variant: Sr22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR22` reader - Subregion 22 in region 0 (write '1' to clear)"]
pub type Sr22R = crate::BitReader<Sr22>;
impl Sr22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr22 {
        match self.bits {
            false => Sr22::NoAccess,
            true => Sr22::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr22::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr22::Access
    }
}
#[doc = "Field `SR22` writer - Subregion 22 in region 0 (write '1' to clear)"]
pub type Sr22W<'a, REG> = crate::BitWriter1C<'a, REG, Sr22>;
impl<'a, REG> Sr22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr22::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr22::Access)
    }
}
#[doc = "Subregion 23 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr23 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr23> for bool {
    #[inline(always)]
    fn from(variant: Sr23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR23` reader - Subregion 23 in region 0 (write '1' to clear)"]
pub type Sr23R = crate::BitReader<Sr23>;
impl Sr23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr23 {
        match self.bits {
            false => Sr23::NoAccess,
            true => Sr23::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr23::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr23::Access
    }
}
#[doc = "Field `SR23` writer - Subregion 23 in region 0 (write '1' to clear)"]
pub type Sr23W<'a, REG> = crate::BitWriter1C<'a, REG, Sr23>;
impl<'a, REG> Sr23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr23::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr23::Access)
    }
}
#[doc = "Subregion 24 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr24 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr24> for bool {
    #[inline(always)]
    fn from(variant: Sr24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR24` reader - Subregion 24 in region 0 (write '1' to clear)"]
pub type Sr24R = crate::BitReader<Sr24>;
impl Sr24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr24 {
        match self.bits {
            false => Sr24::NoAccess,
            true => Sr24::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr24::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr24::Access
    }
}
#[doc = "Field `SR24` writer - Subregion 24 in region 0 (write '1' to clear)"]
pub type Sr24W<'a, REG> = crate::BitWriter1C<'a, REG, Sr24>;
impl<'a, REG> Sr24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr24::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr24::Access)
    }
}
#[doc = "Subregion 25 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr25 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr25> for bool {
    #[inline(always)]
    fn from(variant: Sr25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR25` reader - Subregion 25 in region 0 (write '1' to clear)"]
pub type Sr25R = crate::BitReader<Sr25>;
impl Sr25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr25 {
        match self.bits {
            false => Sr25::NoAccess,
            true => Sr25::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr25::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr25::Access
    }
}
#[doc = "Field `SR25` writer - Subregion 25 in region 0 (write '1' to clear)"]
pub type Sr25W<'a, REG> = crate::BitWriter1C<'a, REG, Sr25>;
impl<'a, REG> Sr25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr25::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr25::Access)
    }
}
#[doc = "Subregion 26 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr26 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr26> for bool {
    #[inline(always)]
    fn from(variant: Sr26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR26` reader - Subregion 26 in region 0 (write '1' to clear)"]
pub type Sr26R = crate::BitReader<Sr26>;
impl Sr26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr26 {
        match self.bits {
            false => Sr26::NoAccess,
            true => Sr26::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr26::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr26::Access
    }
}
#[doc = "Field `SR26` writer - Subregion 26 in region 0 (write '1' to clear)"]
pub type Sr26W<'a, REG> = crate::BitWriter1C<'a, REG, Sr26>;
impl<'a, REG> Sr26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr26::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr26::Access)
    }
}
#[doc = "Subregion 27 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr27 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr27> for bool {
    #[inline(always)]
    fn from(variant: Sr27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR27` reader - Subregion 27 in region 0 (write '1' to clear)"]
pub type Sr27R = crate::BitReader<Sr27>;
impl Sr27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr27 {
        match self.bits {
            false => Sr27::NoAccess,
            true => Sr27::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr27::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr27::Access
    }
}
#[doc = "Field `SR27` writer - Subregion 27 in region 0 (write '1' to clear)"]
pub type Sr27W<'a, REG> = crate::BitWriter1C<'a, REG, Sr27>;
impl<'a, REG> Sr27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr27::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr27::Access)
    }
}
#[doc = "Subregion 28 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr28 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr28> for bool {
    #[inline(always)]
    fn from(variant: Sr28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR28` reader - Subregion 28 in region 0 (write '1' to clear)"]
pub type Sr28R = crate::BitReader<Sr28>;
impl Sr28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr28 {
        match self.bits {
            false => Sr28::NoAccess,
            true => Sr28::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr28::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr28::Access
    }
}
#[doc = "Field `SR28` writer - Subregion 28 in region 0 (write '1' to clear)"]
pub type Sr28W<'a, REG> = crate::BitWriter1C<'a, REG, Sr28>;
impl<'a, REG> Sr28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr28::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr28::Access)
    }
}
#[doc = "Subregion 29 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr29 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr29> for bool {
    #[inline(always)]
    fn from(variant: Sr29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR29` reader - Subregion 29 in region 0 (write '1' to clear)"]
pub type Sr29R = crate::BitReader<Sr29>;
impl Sr29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr29 {
        match self.bits {
            false => Sr29::NoAccess,
            true => Sr29::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr29::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr29::Access
    }
}
#[doc = "Field `SR29` writer - Subregion 29 in region 0 (write '1' to clear)"]
pub type Sr29W<'a, REG> = crate::BitWriter1C<'a, REG, Sr29>;
impl<'a, REG> Sr29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr29::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr29::Access)
    }
}
#[doc = "Subregion 30 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr30 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr30> for bool {
    #[inline(always)]
    fn from(variant: Sr30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR30` reader - Subregion 30 in region 0 (write '1' to clear)"]
pub type Sr30R = crate::BitReader<Sr30>;
impl Sr30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr30 {
        match self.bits {
            false => Sr30::NoAccess,
            true => Sr30::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr30::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr30::Access
    }
}
#[doc = "Field `SR30` writer - Subregion 30 in region 0 (write '1' to clear)"]
pub type Sr30W<'a, REG> = crate::BitWriter1C<'a, REG, Sr30>;
impl<'a, REG> Sr30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr30::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr30::Access)
    }
}
#[doc = "Subregion 31 in region 0 (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr31 {
    #[doc = "0: No read access occurred in this subregion"]
    NoAccess = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    Access = 1,
}
impl From<Sr31> for bool {
    #[inline(always)]
    fn from(variant: Sr31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR31` reader - Subregion 31 in region 0 (write '1' to clear)"]
pub type Sr31R = crate::BitReader<Sr31>;
impl Sr31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr31 {
        match self.bits {
            false => Sr31::NoAccess,
            true => Sr31::Access,
        }
    }
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == Sr31::NoAccess
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == Sr31::Access
    }
}
#[doc = "Field `SR31` writer - Subregion 31 in region 0 (write '1' to clear)"]
pub type Sr31W<'a, REG> = crate::BitWriter1C<'a, REG, Sr31>;
impl<'a, REG> Sr31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr31::NoAccess)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut crate::W<REG> {
        self.variant(Sr31::Access)
    }
}
impl R {
    #[doc = "Bit 0 - Subregion 0 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr0(&self) -> Sr0R {
        Sr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Subregion 1 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr1(&self) -> Sr1R {
        Sr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Subregion 2 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr2(&self) -> Sr2R {
        Sr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subregion 3 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr3(&self) -> Sr3R {
        Sr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Subregion 4 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr4(&self) -> Sr4R {
        Sr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Subregion 5 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr5(&self) -> Sr5R {
        Sr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Subregion 6 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr6(&self) -> Sr6R {
        Sr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Subregion 7 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr7(&self) -> Sr7R {
        Sr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Subregion 8 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr8(&self) -> Sr8R {
        Sr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Subregion 9 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr9(&self) -> Sr9R {
        Sr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Subregion 10 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr10(&self) -> Sr10R {
        Sr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subregion 11 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr11(&self) -> Sr11R {
        Sr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Subregion 12 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr12(&self) -> Sr12R {
        Sr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Subregion 13 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr13(&self) -> Sr13R {
        Sr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Subregion 14 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr14(&self) -> Sr14R {
        Sr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Subregion 15 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr15(&self) -> Sr15R {
        Sr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Subregion 16 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr16(&self) -> Sr16R {
        Sr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Subregion 17 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr17(&self) -> Sr17R {
        Sr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Subregion 18 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr18(&self) -> Sr18R {
        Sr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subregion 19 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr19(&self) -> Sr19R {
        Sr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Subregion 20 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr20(&self) -> Sr20R {
        Sr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Subregion 21 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr21(&self) -> Sr21R {
        Sr21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Subregion 22 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr22(&self) -> Sr22R {
        Sr22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Subregion 23 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr23(&self) -> Sr23R {
        Sr23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Subregion 24 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr24(&self) -> Sr24R {
        Sr24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Subregion 25 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr25(&self) -> Sr25R {
        Sr25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Subregion 26 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr26(&self) -> Sr26R {
        Sr26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subregion 27 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr27(&self) -> Sr27R {
        Sr27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Subregion 28 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr28(&self) -> Sr28R {
        Sr28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Subregion 29 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr29(&self) -> Sr29R {
        Sr29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Subregion 30 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr30(&self) -> Sr30R {
        Sr30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Subregion 31 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr31(&self) -> Sr31R {
        Sr31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Subregion 0 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr0(&mut self) -> Sr0W<'_, SubstatraSpec> {
        Sr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Subregion 1 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr1(&mut self) -> Sr1W<'_, SubstatraSpec> {
        Sr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Subregion 2 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr2(&mut self) -> Sr2W<'_, SubstatraSpec> {
        Sr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Subregion 3 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr3(&mut self) -> Sr3W<'_, SubstatraSpec> {
        Sr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Subregion 4 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr4(&mut self) -> Sr4W<'_, SubstatraSpec> {
        Sr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Subregion 5 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr5(&mut self) -> Sr5W<'_, SubstatraSpec> {
        Sr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Subregion 6 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr6(&mut self) -> Sr6W<'_, SubstatraSpec> {
        Sr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Subregion 7 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr7(&mut self) -> Sr7W<'_, SubstatraSpec> {
        Sr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Subregion 8 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr8(&mut self) -> Sr8W<'_, SubstatraSpec> {
        Sr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Subregion 9 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr9(&mut self) -> Sr9W<'_, SubstatraSpec> {
        Sr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Subregion 10 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr10(&mut self) -> Sr10W<'_, SubstatraSpec> {
        Sr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Subregion 11 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr11(&mut self) -> Sr11W<'_, SubstatraSpec> {
        Sr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Subregion 12 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr12(&mut self) -> Sr12W<'_, SubstatraSpec> {
        Sr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Subregion 13 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr13(&mut self) -> Sr13W<'_, SubstatraSpec> {
        Sr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Subregion 14 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr14(&mut self) -> Sr14W<'_, SubstatraSpec> {
        Sr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Subregion 15 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr15(&mut self) -> Sr15W<'_, SubstatraSpec> {
        Sr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Subregion 16 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr16(&mut self) -> Sr16W<'_, SubstatraSpec> {
        Sr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Subregion 17 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr17(&mut self) -> Sr17W<'_, SubstatraSpec> {
        Sr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Subregion 18 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr18(&mut self) -> Sr18W<'_, SubstatraSpec> {
        Sr18W::new(self, 18)
    }
    #[doc = "Bit 19 - Subregion 19 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr19(&mut self) -> Sr19W<'_, SubstatraSpec> {
        Sr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Subregion 20 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr20(&mut self) -> Sr20W<'_, SubstatraSpec> {
        Sr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Subregion 21 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr21(&mut self) -> Sr21W<'_, SubstatraSpec> {
        Sr21W::new(self, 21)
    }
    #[doc = "Bit 22 - Subregion 22 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr22(&mut self) -> Sr22W<'_, SubstatraSpec> {
        Sr22W::new(self, 22)
    }
    #[doc = "Bit 23 - Subregion 23 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr23(&mut self) -> Sr23W<'_, SubstatraSpec> {
        Sr23W::new(self, 23)
    }
    #[doc = "Bit 24 - Subregion 24 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr24(&mut self) -> Sr24W<'_, SubstatraSpec> {
        Sr24W::new(self, 24)
    }
    #[doc = "Bit 25 - Subregion 25 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr25(&mut self) -> Sr25W<'_, SubstatraSpec> {
        Sr25W::new(self, 25)
    }
    #[doc = "Bit 26 - Subregion 26 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr26(&mut self) -> Sr26W<'_, SubstatraSpec> {
        Sr26W::new(self, 26)
    }
    #[doc = "Bit 27 - Subregion 27 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr27(&mut self) -> Sr27W<'_, SubstatraSpec> {
        Sr27W::new(self, 27)
    }
    #[doc = "Bit 28 - Subregion 28 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr28(&mut self) -> Sr28W<'_, SubstatraSpec> {
        Sr28W::new(self, 28)
    }
    #[doc = "Bit 29 - Subregion 29 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr29(&mut self) -> Sr29W<'_, SubstatraSpec> {
        Sr29W::new(self, 29)
    }
    #[doc = "Bit 30 - Subregion 30 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr30(&mut self) -> Sr30W<'_, SubstatraSpec> {
        Sr30W::new(self, 30)
    }
    #[doc = "Bit 31 - Subregion 31 in region 0 (write '1' to clear)"]
    #[inline(always)]
    pub fn sr31(&mut self) -> Sr31W<'_, SubstatraSpec> {
        Sr31W::new(self, 31)
    }
}
#[doc = "Description cluster\\[0\\]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching\n\nYou can [`read`](crate::Reg::read) this register and get [`substatra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`substatra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubstatraSpec;
impl crate::RegisterSpec for SubstatraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`substatra::R`](R) reader structure"]
impl crate::Readable for SubstatraSpec {}
#[doc = "`write(|w| ..)` method takes [`substatra::W`](W) writer structure"]
impl crate::Writable for SubstatraSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets SUBSTATRA to value 0"]
impl crate::Resettable for SubstatraSpec {}
