#[doc = "Register `CURRENTPORTSIZE` reader"]
pub type R = crate::R<CurrentportsizeSpec>;
#[doc = "Register `CURRENTPORTSIZE` writer"]
pub type W = crate::W<CurrentportsizeSpec>;
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize1 {
    #[doc = "0: Port size 1 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 1 is selected."]
    Selected = 1,
}
impl From<PortSize1> for bool {
    #[inline(always)]
    fn from(variant: PortSize1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_1` reader - Indicates which port size is currently selected."]
pub type PortSize1R = crate::BitReader<PortSize1>;
impl PortSize1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize1 {
        match self.bits {
            false => PortSize1::NotSelected,
            true => PortSize1::Selected,
        }
    }
    #[doc = "Port size 1 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize1::NotSelected
    }
    #[doc = "Port size 1 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize1::Selected
    }
}
#[doc = "Field `PORT_SIZE_1` writer - Indicates which port size is currently selected."]
pub type PortSize1W<'a, REG> = crate::BitWriter<'a, REG, PortSize1>;
impl<'a, REG> PortSize1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 1 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize1::NotSelected)
    }
    #[doc = "Port size 1 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize1::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize2 {
    #[doc = "0: Port size 2 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 2 is selected."]
    Selected = 1,
}
impl From<PortSize2> for bool {
    #[inline(always)]
    fn from(variant: PortSize2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_2` reader - Indicates which port size is currently selected."]
pub type PortSize2R = crate::BitReader<PortSize2>;
impl PortSize2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize2 {
        match self.bits {
            false => PortSize2::NotSelected,
            true => PortSize2::Selected,
        }
    }
    #[doc = "Port size 2 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize2::NotSelected
    }
    #[doc = "Port size 2 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize2::Selected
    }
}
#[doc = "Field `PORT_SIZE_2` writer - Indicates which port size is currently selected."]
pub type PortSize2W<'a, REG> = crate::BitWriter<'a, REG, PortSize2>;
impl<'a, REG> PortSize2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 2 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize2::NotSelected)
    }
    #[doc = "Port size 2 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize2::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize3 {
    #[doc = "0: Port size 3 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 3 is selected."]
    Selected = 1,
}
impl From<PortSize3> for bool {
    #[inline(always)]
    fn from(variant: PortSize3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_3` reader - Indicates which port size is currently selected."]
pub type PortSize3R = crate::BitReader<PortSize3>;
impl PortSize3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize3 {
        match self.bits {
            false => PortSize3::NotSelected,
            true => PortSize3::Selected,
        }
    }
    #[doc = "Port size 3 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize3::NotSelected
    }
    #[doc = "Port size 3 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize3::Selected
    }
}
#[doc = "Field `PORT_SIZE_3` writer - Indicates which port size is currently selected."]
pub type PortSize3W<'a, REG> = crate::BitWriter<'a, REG, PortSize3>;
impl<'a, REG> PortSize3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 3 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize3::NotSelected)
    }
    #[doc = "Port size 3 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize3::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize4 {
    #[doc = "0: Port size 4 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 4 is selected."]
    Selected = 1,
}
impl From<PortSize4> for bool {
    #[inline(always)]
    fn from(variant: PortSize4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_4` reader - Indicates which port size is currently selected."]
pub type PortSize4R = crate::BitReader<PortSize4>;
impl PortSize4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize4 {
        match self.bits {
            false => PortSize4::NotSelected,
            true => PortSize4::Selected,
        }
    }
    #[doc = "Port size 4 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize4::NotSelected
    }
    #[doc = "Port size 4 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize4::Selected
    }
}
#[doc = "Field `PORT_SIZE_4` writer - Indicates which port size is currently selected."]
pub type PortSize4W<'a, REG> = crate::BitWriter<'a, REG, PortSize4>;
impl<'a, REG> PortSize4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 4 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize4::NotSelected)
    }
    #[doc = "Port size 4 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize4::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize5 {
    #[doc = "0: Port size 5 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 5 is selected."]
    Selected = 1,
}
impl From<PortSize5> for bool {
    #[inline(always)]
    fn from(variant: PortSize5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_5` reader - Indicates which port size is currently selected."]
pub type PortSize5R = crate::BitReader<PortSize5>;
impl PortSize5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize5 {
        match self.bits {
            false => PortSize5::NotSelected,
            true => PortSize5::Selected,
        }
    }
    #[doc = "Port size 5 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize5::NotSelected
    }
    #[doc = "Port size 5 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize5::Selected
    }
}
#[doc = "Field `PORT_SIZE_5` writer - Indicates which port size is currently selected."]
pub type PortSize5W<'a, REG> = crate::BitWriter<'a, REG, PortSize5>;
impl<'a, REG> PortSize5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 5 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize5::NotSelected)
    }
    #[doc = "Port size 5 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize5::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize6 {
    #[doc = "0: Port size 6 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 6 is selected."]
    Selected = 1,
}
impl From<PortSize6> for bool {
    #[inline(always)]
    fn from(variant: PortSize6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_6` reader - Indicates which port size is currently selected."]
pub type PortSize6R = crate::BitReader<PortSize6>;
impl PortSize6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize6 {
        match self.bits {
            false => PortSize6::NotSelected,
            true => PortSize6::Selected,
        }
    }
    #[doc = "Port size 6 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize6::NotSelected
    }
    #[doc = "Port size 6 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize6::Selected
    }
}
#[doc = "Field `PORT_SIZE_6` writer - Indicates which port size is currently selected."]
pub type PortSize6W<'a, REG> = crate::BitWriter<'a, REG, PortSize6>;
impl<'a, REG> PortSize6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 6 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize6::NotSelected)
    }
    #[doc = "Port size 6 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize6::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize7 {
    #[doc = "0: Port size 7 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 7 is selected."]
    Selected = 1,
}
impl From<PortSize7> for bool {
    #[inline(always)]
    fn from(variant: PortSize7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_7` reader - Indicates which port size is currently selected."]
pub type PortSize7R = crate::BitReader<PortSize7>;
impl PortSize7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize7 {
        match self.bits {
            false => PortSize7::NotSelected,
            true => PortSize7::Selected,
        }
    }
    #[doc = "Port size 7 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize7::NotSelected
    }
    #[doc = "Port size 7 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize7::Selected
    }
}
#[doc = "Field `PORT_SIZE_7` writer - Indicates which port size is currently selected."]
pub type PortSize7W<'a, REG> = crate::BitWriter<'a, REG, PortSize7>;
impl<'a, REG> PortSize7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 7 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize7::NotSelected)
    }
    #[doc = "Port size 7 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize7::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize8 {
    #[doc = "0: Port size 8 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 8 is selected."]
    Selected = 1,
}
impl From<PortSize8> for bool {
    #[inline(always)]
    fn from(variant: PortSize8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_8` reader - Indicates which port size is currently selected."]
pub type PortSize8R = crate::BitReader<PortSize8>;
impl PortSize8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize8 {
        match self.bits {
            false => PortSize8::NotSelected,
            true => PortSize8::Selected,
        }
    }
    #[doc = "Port size 8 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize8::NotSelected
    }
    #[doc = "Port size 8 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize8::Selected
    }
}
#[doc = "Field `PORT_SIZE_8` writer - Indicates which port size is currently selected."]
pub type PortSize8W<'a, REG> = crate::BitWriter<'a, REG, PortSize8>;
impl<'a, REG> PortSize8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 8 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize8::NotSelected)
    }
    #[doc = "Port size 8 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize8::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize9 {
    #[doc = "0: Port size 9 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 9 is selected."]
    Selected = 1,
}
impl From<PortSize9> for bool {
    #[inline(always)]
    fn from(variant: PortSize9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_9` reader - Indicates which port size is currently selected."]
pub type PortSize9R = crate::BitReader<PortSize9>;
impl PortSize9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize9 {
        match self.bits {
            false => PortSize9::NotSelected,
            true => PortSize9::Selected,
        }
    }
    #[doc = "Port size 9 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize9::NotSelected
    }
    #[doc = "Port size 9 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize9::Selected
    }
}
#[doc = "Field `PORT_SIZE_9` writer - Indicates which port size is currently selected."]
pub type PortSize9W<'a, REG> = crate::BitWriter<'a, REG, PortSize9>;
impl<'a, REG> PortSize9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 9 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize9::NotSelected)
    }
    #[doc = "Port size 9 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize9::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize10 {
    #[doc = "0: Port size 10 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 10 is selected."]
    Selected = 1,
}
impl From<PortSize10> for bool {
    #[inline(always)]
    fn from(variant: PortSize10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_10` reader - Indicates which port size is currently selected."]
pub type PortSize10R = crate::BitReader<PortSize10>;
impl PortSize10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize10 {
        match self.bits {
            false => PortSize10::NotSelected,
            true => PortSize10::Selected,
        }
    }
    #[doc = "Port size 10 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize10::NotSelected
    }
    #[doc = "Port size 10 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize10::Selected
    }
}
#[doc = "Field `PORT_SIZE_10` writer - Indicates which port size is currently selected."]
pub type PortSize10W<'a, REG> = crate::BitWriter<'a, REG, PortSize10>;
impl<'a, REG> PortSize10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 10 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize10::NotSelected)
    }
    #[doc = "Port size 10 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize10::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize11 {
    #[doc = "0: Port size 11 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 11 is selected."]
    Selected = 1,
}
impl From<PortSize11> for bool {
    #[inline(always)]
    fn from(variant: PortSize11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_11` reader - Indicates which port size is currently selected."]
pub type PortSize11R = crate::BitReader<PortSize11>;
impl PortSize11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize11 {
        match self.bits {
            false => PortSize11::NotSelected,
            true => PortSize11::Selected,
        }
    }
    #[doc = "Port size 11 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize11::NotSelected
    }
    #[doc = "Port size 11 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize11::Selected
    }
}
#[doc = "Field `PORT_SIZE_11` writer - Indicates which port size is currently selected."]
pub type PortSize11W<'a, REG> = crate::BitWriter<'a, REG, PortSize11>;
impl<'a, REG> PortSize11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 11 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize11::NotSelected)
    }
    #[doc = "Port size 11 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize11::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize12 {
    #[doc = "0: Port size 12 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 12 is selected."]
    Selected = 1,
}
impl From<PortSize12> for bool {
    #[inline(always)]
    fn from(variant: PortSize12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_12` reader - Indicates which port size is currently selected."]
pub type PortSize12R = crate::BitReader<PortSize12>;
impl PortSize12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize12 {
        match self.bits {
            false => PortSize12::NotSelected,
            true => PortSize12::Selected,
        }
    }
    #[doc = "Port size 12 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize12::NotSelected
    }
    #[doc = "Port size 12 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize12::Selected
    }
}
#[doc = "Field `PORT_SIZE_12` writer - Indicates which port size is currently selected."]
pub type PortSize12W<'a, REG> = crate::BitWriter<'a, REG, PortSize12>;
impl<'a, REG> PortSize12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 12 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize12::NotSelected)
    }
    #[doc = "Port size 12 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize12::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize13 {
    #[doc = "0: Port size 13 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 13 is selected."]
    Selected = 1,
}
impl From<PortSize13> for bool {
    #[inline(always)]
    fn from(variant: PortSize13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_13` reader - Indicates which port size is currently selected."]
pub type PortSize13R = crate::BitReader<PortSize13>;
impl PortSize13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize13 {
        match self.bits {
            false => PortSize13::NotSelected,
            true => PortSize13::Selected,
        }
    }
    #[doc = "Port size 13 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize13::NotSelected
    }
    #[doc = "Port size 13 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize13::Selected
    }
}
#[doc = "Field `PORT_SIZE_13` writer - Indicates which port size is currently selected."]
pub type PortSize13W<'a, REG> = crate::BitWriter<'a, REG, PortSize13>;
impl<'a, REG> PortSize13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 13 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize13::NotSelected)
    }
    #[doc = "Port size 13 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize13::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize14 {
    #[doc = "0: Port size 14 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 14 is selected."]
    Selected = 1,
}
impl From<PortSize14> for bool {
    #[inline(always)]
    fn from(variant: PortSize14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_14` reader - Indicates which port size is currently selected."]
pub type PortSize14R = crate::BitReader<PortSize14>;
impl PortSize14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize14 {
        match self.bits {
            false => PortSize14::NotSelected,
            true => PortSize14::Selected,
        }
    }
    #[doc = "Port size 14 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize14::NotSelected
    }
    #[doc = "Port size 14 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize14::Selected
    }
}
#[doc = "Field `PORT_SIZE_14` writer - Indicates which port size is currently selected."]
pub type PortSize14W<'a, REG> = crate::BitWriter<'a, REG, PortSize14>;
impl<'a, REG> PortSize14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 14 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize14::NotSelected)
    }
    #[doc = "Port size 14 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize14::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize15 {
    #[doc = "0: Port size 15 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 15 is selected."]
    Selected = 1,
}
impl From<PortSize15> for bool {
    #[inline(always)]
    fn from(variant: PortSize15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_15` reader - Indicates which port size is currently selected."]
pub type PortSize15R = crate::BitReader<PortSize15>;
impl PortSize15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize15 {
        match self.bits {
            false => PortSize15::NotSelected,
            true => PortSize15::Selected,
        }
    }
    #[doc = "Port size 15 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize15::NotSelected
    }
    #[doc = "Port size 15 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize15::Selected
    }
}
#[doc = "Field `PORT_SIZE_15` writer - Indicates which port size is currently selected."]
pub type PortSize15W<'a, REG> = crate::BitWriter<'a, REG, PortSize15>;
impl<'a, REG> PortSize15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 15 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize15::NotSelected)
    }
    #[doc = "Port size 15 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize15::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize16 {
    #[doc = "0: Port size 16 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 16 is selected."]
    Selected = 1,
}
impl From<PortSize16> for bool {
    #[inline(always)]
    fn from(variant: PortSize16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_16` reader - Indicates which port size is currently selected."]
pub type PortSize16R = crate::BitReader<PortSize16>;
impl PortSize16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize16 {
        match self.bits {
            false => PortSize16::NotSelected,
            true => PortSize16::Selected,
        }
    }
    #[doc = "Port size 16 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize16::NotSelected
    }
    #[doc = "Port size 16 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize16::Selected
    }
}
#[doc = "Field `PORT_SIZE_16` writer - Indicates which port size is currently selected."]
pub type PortSize16W<'a, REG> = crate::BitWriter<'a, REG, PortSize16>;
impl<'a, REG> PortSize16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 16 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize16::NotSelected)
    }
    #[doc = "Port size 16 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize16::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize17 {
    #[doc = "0: Port size 17 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 17 is selected."]
    Selected = 1,
}
impl From<PortSize17> for bool {
    #[inline(always)]
    fn from(variant: PortSize17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_17` reader - Indicates which port size is currently selected."]
pub type PortSize17R = crate::BitReader<PortSize17>;
impl PortSize17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize17 {
        match self.bits {
            false => PortSize17::NotSelected,
            true => PortSize17::Selected,
        }
    }
    #[doc = "Port size 17 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize17::NotSelected
    }
    #[doc = "Port size 17 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize17::Selected
    }
}
#[doc = "Field `PORT_SIZE_17` writer - Indicates which port size is currently selected."]
pub type PortSize17W<'a, REG> = crate::BitWriter<'a, REG, PortSize17>;
impl<'a, REG> PortSize17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 17 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize17::NotSelected)
    }
    #[doc = "Port size 17 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize17::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize18 {
    #[doc = "0: Port size 18 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 18 is selected."]
    Selected = 1,
}
impl From<PortSize18> for bool {
    #[inline(always)]
    fn from(variant: PortSize18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_18` reader - Indicates which port size is currently selected."]
pub type PortSize18R = crate::BitReader<PortSize18>;
impl PortSize18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize18 {
        match self.bits {
            false => PortSize18::NotSelected,
            true => PortSize18::Selected,
        }
    }
    #[doc = "Port size 18 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize18::NotSelected
    }
    #[doc = "Port size 18 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize18::Selected
    }
}
#[doc = "Field `PORT_SIZE_18` writer - Indicates which port size is currently selected."]
pub type PortSize18W<'a, REG> = crate::BitWriter<'a, REG, PortSize18>;
impl<'a, REG> PortSize18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 18 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize18::NotSelected)
    }
    #[doc = "Port size 18 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize18::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize19 {
    #[doc = "0: Port size 19 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 19 is selected."]
    Selected = 1,
}
impl From<PortSize19> for bool {
    #[inline(always)]
    fn from(variant: PortSize19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_19` reader - Indicates which port size is currently selected."]
pub type PortSize19R = crate::BitReader<PortSize19>;
impl PortSize19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize19 {
        match self.bits {
            false => PortSize19::NotSelected,
            true => PortSize19::Selected,
        }
    }
    #[doc = "Port size 19 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize19::NotSelected
    }
    #[doc = "Port size 19 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize19::Selected
    }
}
#[doc = "Field `PORT_SIZE_19` writer - Indicates which port size is currently selected."]
pub type PortSize19W<'a, REG> = crate::BitWriter<'a, REG, PortSize19>;
impl<'a, REG> PortSize19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 19 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize19::NotSelected)
    }
    #[doc = "Port size 19 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize19::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize20 {
    #[doc = "0: Port size 20 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 20 is selected."]
    Selected = 1,
}
impl From<PortSize20> for bool {
    #[inline(always)]
    fn from(variant: PortSize20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_20` reader - Indicates which port size is currently selected."]
pub type PortSize20R = crate::BitReader<PortSize20>;
impl PortSize20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize20 {
        match self.bits {
            false => PortSize20::NotSelected,
            true => PortSize20::Selected,
        }
    }
    #[doc = "Port size 20 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize20::NotSelected
    }
    #[doc = "Port size 20 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize20::Selected
    }
}
#[doc = "Field `PORT_SIZE_20` writer - Indicates which port size is currently selected."]
pub type PortSize20W<'a, REG> = crate::BitWriter<'a, REG, PortSize20>;
impl<'a, REG> PortSize20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 20 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize20::NotSelected)
    }
    #[doc = "Port size 20 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize20::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize21 {
    #[doc = "0: Port size 21 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 21 is selected."]
    Selected = 1,
}
impl From<PortSize21> for bool {
    #[inline(always)]
    fn from(variant: PortSize21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_21` reader - Indicates which port size is currently selected."]
pub type PortSize21R = crate::BitReader<PortSize21>;
impl PortSize21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize21 {
        match self.bits {
            false => PortSize21::NotSelected,
            true => PortSize21::Selected,
        }
    }
    #[doc = "Port size 21 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize21::NotSelected
    }
    #[doc = "Port size 21 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize21::Selected
    }
}
#[doc = "Field `PORT_SIZE_21` writer - Indicates which port size is currently selected."]
pub type PortSize21W<'a, REG> = crate::BitWriter<'a, REG, PortSize21>;
impl<'a, REG> PortSize21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 21 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize21::NotSelected)
    }
    #[doc = "Port size 21 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize21::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize22 {
    #[doc = "0: Port size 22 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 22 is selected."]
    Selected = 1,
}
impl From<PortSize22> for bool {
    #[inline(always)]
    fn from(variant: PortSize22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_22` reader - Indicates which port size is currently selected."]
pub type PortSize22R = crate::BitReader<PortSize22>;
impl PortSize22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize22 {
        match self.bits {
            false => PortSize22::NotSelected,
            true => PortSize22::Selected,
        }
    }
    #[doc = "Port size 22 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize22::NotSelected
    }
    #[doc = "Port size 22 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize22::Selected
    }
}
#[doc = "Field `PORT_SIZE_22` writer - Indicates which port size is currently selected."]
pub type PortSize22W<'a, REG> = crate::BitWriter<'a, REG, PortSize22>;
impl<'a, REG> PortSize22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 22 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize22::NotSelected)
    }
    #[doc = "Port size 22 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize22::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize23 {
    #[doc = "0: Port size 23 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 23 is selected."]
    Selected = 1,
}
impl From<PortSize23> for bool {
    #[inline(always)]
    fn from(variant: PortSize23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_23` reader - Indicates which port size is currently selected."]
pub type PortSize23R = crate::BitReader<PortSize23>;
impl PortSize23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize23 {
        match self.bits {
            false => PortSize23::NotSelected,
            true => PortSize23::Selected,
        }
    }
    #[doc = "Port size 23 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize23::NotSelected
    }
    #[doc = "Port size 23 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize23::Selected
    }
}
#[doc = "Field `PORT_SIZE_23` writer - Indicates which port size is currently selected."]
pub type PortSize23W<'a, REG> = crate::BitWriter<'a, REG, PortSize23>;
impl<'a, REG> PortSize23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 23 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize23::NotSelected)
    }
    #[doc = "Port size 23 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize23::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize24 {
    #[doc = "0: Port size 24 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 24 is selected."]
    Selected = 1,
}
impl From<PortSize24> for bool {
    #[inline(always)]
    fn from(variant: PortSize24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_24` reader - Indicates which port size is currently selected."]
pub type PortSize24R = crate::BitReader<PortSize24>;
impl PortSize24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize24 {
        match self.bits {
            false => PortSize24::NotSelected,
            true => PortSize24::Selected,
        }
    }
    #[doc = "Port size 24 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize24::NotSelected
    }
    #[doc = "Port size 24 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize24::Selected
    }
}
#[doc = "Field `PORT_SIZE_24` writer - Indicates which port size is currently selected."]
pub type PortSize24W<'a, REG> = crate::BitWriter<'a, REG, PortSize24>;
impl<'a, REG> PortSize24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 24 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize24::NotSelected)
    }
    #[doc = "Port size 24 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize24::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize25 {
    #[doc = "0: Port size 25 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 25 is selected."]
    Selected = 1,
}
impl From<PortSize25> for bool {
    #[inline(always)]
    fn from(variant: PortSize25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_25` reader - Indicates which port size is currently selected."]
pub type PortSize25R = crate::BitReader<PortSize25>;
impl PortSize25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize25 {
        match self.bits {
            false => PortSize25::NotSelected,
            true => PortSize25::Selected,
        }
    }
    #[doc = "Port size 25 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize25::NotSelected
    }
    #[doc = "Port size 25 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize25::Selected
    }
}
#[doc = "Field `PORT_SIZE_25` writer - Indicates which port size is currently selected."]
pub type PortSize25W<'a, REG> = crate::BitWriter<'a, REG, PortSize25>;
impl<'a, REG> PortSize25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 25 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize25::NotSelected)
    }
    #[doc = "Port size 25 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize25::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize26 {
    #[doc = "0: Port size 26 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 26 is selected."]
    Selected = 1,
}
impl From<PortSize26> for bool {
    #[inline(always)]
    fn from(variant: PortSize26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_26` reader - Indicates which port size is currently selected."]
pub type PortSize26R = crate::BitReader<PortSize26>;
impl PortSize26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize26 {
        match self.bits {
            false => PortSize26::NotSelected,
            true => PortSize26::Selected,
        }
    }
    #[doc = "Port size 26 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize26::NotSelected
    }
    #[doc = "Port size 26 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize26::Selected
    }
}
#[doc = "Field `PORT_SIZE_26` writer - Indicates which port size is currently selected."]
pub type PortSize26W<'a, REG> = crate::BitWriter<'a, REG, PortSize26>;
impl<'a, REG> PortSize26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 26 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize26::NotSelected)
    }
    #[doc = "Port size 26 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize26::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize27 {
    #[doc = "0: Port size 27 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 27 is selected."]
    Selected = 1,
}
impl From<PortSize27> for bool {
    #[inline(always)]
    fn from(variant: PortSize27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_27` reader - Indicates which port size is currently selected."]
pub type PortSize27R = crate::BitReader<PortSize27>;
impl PortSize27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize27 {
        match self.bits {
            false => PortSize27::NotSelected,
            true => PortSize27::Selected,
        }
    }
    #[doc = "Port size 27 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize27::NotSelected
    }
    #[doc = "Port size 27 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize27::Selected
    }
}
#[doc = "Field `PORT_SIZE_27` writer - Indicates which port size is currently selected."]
pub type PortSize27W<'a, REG> = crate::BitWriter<'a, REG, PortSize27>;
impl<'a, REG> PortSize27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 27 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize27::NotSelected)
    }
    #[doc = "Port size 27 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize27::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize28 {
    #[doc = "0: Port size 28 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 28 is selected."]
    Selected = 1,
}
impl From<PortSize28> for bool {
    #[inline(always)]
    fn from(variant: PortSize28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_28` reader - Indicates which port size is currently selected."]
pub type PortSize28R = crate::BitReader<PortSize28>;
impl PortSize28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize28 {
        match self.bits {
            false => PortSize28::NotSelected,
            true => PortSize28::Selected,
        }
    }
    #[doc = "Port size 28 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize28::NotSelected
    }
    #[doc = "Port size 28 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize28::Selected
    }
}
#[doc = "Field `PORT_SIZE_28` writer - Indicates which port size is currently selected."]
pub type PortSize28W<'a, REG> = crate::BitWriter<'a, REG, PortSize28>;
impl<'a, REG> PortSize28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 28 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize28::NotSelected)
    }
    #[doc = "Port size 28 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize28::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize29 {
    #[doc = "0: Port size 29 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 29 is selected."]
    Selected = 1,
}
impl From<PortSize29> for bool {
    #[inline(always)]
    fn from(variant: PortSize29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_29` reader - Indicates which port size is currently selected."]
pub type PortSize29R = crate::BitReader<PortSize29>;
impl PortSize29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize29 {
        match self.bits {
            false => PortSize29::NotSelected,
            true => PortSize29::Selected,
        }
    }
    #[doc = "Port size 29 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize29::NotSelected
    }
    #[doc = "Port size 29 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize29::Selected
    }
}
#[doc = "Field `PORT_SIZE_29` writer - Indicates which port size is currently selected."]
pub type PortSize29W<'a, REG> = crate::BitWriter<'a, REG, PortSize29>;
impl<'a, REG> PortSize29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 29 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize29::NotSelected)
    }
    #[doc = "Port size 29 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize29::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize30 {
    #[doc = "0: Port size 30 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 30 is selected."]
    Selected = 1,
}
impl From<PortSize30> for bool {
    #[inline(always)]
    fn from(variant: PortSize30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_30` reader - Indicates which port size is currently selected."]
pub type PortSize30R = crate::BitReader<PortSize30>;
impl PortSize30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize30 {
        match self.bits {
            false => PortSize30::NotSelected,
            true => PortSize30::Selected,
        }
    }
    #[doc = "Port size 30 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize30::NotSelected
    }
    #[doc = "Port size 30 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize30::Selected
    }
}
#[doc = "Field `PORT_SIZE_30` writer - Indicates which port size is currently selected."]
pub type PortSize30W<'a, REG> = crate::BitWriter<'a, REG, PortSize30>;
impl<'a, REG> PortSize30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 30 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize30::NotSelected)
    }
    #[doc = "Port size 30 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize30::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize31 {
    #[doc = "0: Port size 31 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 31 is selected."]
    Selected = 1,
}
impl From<PortSize31> for bool {
    #[inline(always)]
    fn from(variant: PortSize31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_31` reader - Indicates which port size is currently selected."]
pub type PortSize31R = crate::BitReader<PortSize31>;
impl PortSize31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize31 {
        match self.bits {
            false => PortSize31::NotSelected,
            true => PortSize31::Selected,
        }
    }
    #[doc = "Port size 31 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize31::NotSelected
    }
    #[doc = "Port size 31 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize31::Selected
    }
}
#[doc = "Field `PORT_SIZE_31` writer - Indicates which port size is currently selected."]
pub type PortSize31W<'a, REG> = crate::BitWriter<'a, REG, PortSize31>;
impl<'a, REG> PortSize31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 31 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize31::NotSelected)
    }
    #[doc = "Port size 31 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize31::Selected)
    }
}
#[doc = "Indicates which port size is currently selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSize32 {
    #[doc = "0: Port size 32 is not selected."]
    NotSelected = 0,
    #[doc = "1: Port size 32 is selected."]
    Selected = 1,
}
impl From<PortSize32> for bool {
    #[inline(always)]
    fn from(variant: PortSize32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT_SIZE_32` reader - Indicates which port size is currently selected."]
pub type PortSize32R = crate::BitReader<PortSize32>;
impl PortSize32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortSize32 {
        match self.bits {
            false => PortSize32::NotSelected,
            true => PortSize32::Selected,
        }
    }
    #[doc = "Port size 32 is not selected."]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == PortSize32::NotSelected
    }
    #[doc = "Port size 32 is selected."]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == PortSize32::Selected
    }
}
#[doc = "Field `PORT_SIZE_32` writer - Indicates which port size is currently selected."]
pub type PortSize32W<'a, REG> = crate::BitWriter<'a, REG, PortSize32>;
impl<'a, REG> PortSize32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port size 32 is not selected."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize32::NotSelected)
    }
    #[doc = "Port size 32 is selected."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(PortSize32::Selected)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_1(&self) -> PortSize1R {
        PortSize1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_2(&self) -> PortSize2R {
        PortSize2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_3(&self) -> PortSize3R {
        PortSize3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_4(&self) -> PortSize4R {
        PortSize4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_5(&self) -> PortSize5R {
        PortSize5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_6(&self) -> PortSize6R {
        PortSize6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_7(&self) -> PortSize7R {
        PortSize7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_8(&self) -> PortSize8R {
        PortSize8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_9(&self) -> PortSize9R {
        PortSize9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_10(&self) -> PortSize10R {
        PortSize10R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_11(&self) -> PortSize11R {
        PortSize11R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_12(&self) -> PortSize12R {
        PortSize12R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_13(&self) -> PortSize13R {
        PortSize13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_14(&self) -> PortSize14R {
        PortSize14R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_15(&self) -> PortSize15R {
        PortSize15R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_16(&self) -> PortSize16R {
        PortSize16R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_17(&self) -> PortSize17R {
        PortSize17R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_18(&self) -> PortSize18R {
        PortSize18R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_19(&self) -> PortSize19R {
        PortSize19R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_20(&self) -> PortSize20R {
        PortSize20R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_21(&self) -> PortSize21R {
        PortSize21R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_22(&self) -> PortSize22R {
        PortSize22R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_23(&self) -> PortSize23R {
        PortSize23R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_24(&self) -> PortSize24R {
        PortSize24R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_25(&self) -> PortSize25R {
        PortSize25R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_26(&self) -> PortSize26R {
        PortSize26R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_27(&self) -> PortSize27R {
        PortSize27R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_28(&self) -> PortSize28R {
        PortSize28R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_29(&self) -> PortSize29R {
        PortSize29R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_30(&self) -> PortSize30R {
        PortSize30R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_31(&self) -> PortSize31R {
        PortSize31R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_32(&self) -> PortSize32R {
        PortSize32R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_1(&mut self) -> PortSize1W<'_, CurrentportsizeSpec> {
        PortSize1W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_2(&mut self) -> PortSize2W<'_, CurrentportsizeSpec> {
        PortSize2W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_3(&mut self) -> PortSize3W<'_, CurrentportsizeSpec> {
        PortSize3W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_4(&mut self) -> PortSize4W<'_, CurrentportsizeSpec> {
        PortSize4W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_5(&mut self) -> PortSize5W<'_, CurrentportsizeSpec> {
        PortSize5W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_6(&mut self) -> PortSize6W<'_, CurrentportsizeSpec> {
        PortSize6W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_7(&mut self) -> PortSize7W<'_, CurrentportsizeSpec> {
        PortSize7W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_8(&mut self) -> PortSize8W<'_, CurrentportsizeSpec> {
        PortSize8W::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_9(&mut self) -> PortSize9W<'_, CurrentportsizeSpec> {
        PortSize9W::new(self, 8)
    }
    #[doc = "Bit 9 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_10(&mut self) -> PortSize10W<'_, CurrentportsizeSpec> {
        PortSize10W::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_11(&mut self) -> PortSize11W<'_, CurrentportsizeSpec> {
        PortSize11W::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_12(&mut self) -> PortSize12W<'_, CurrentportsizeSpec> {
        PortSize12W::new(self, 11)
    }
    #[doc = "Bit 12 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_13(&mut self) -> PortSize13W<'_, CurrentportsizeSpec> {
        PortSize13W::new(self, 12)
    }
    #[doc = "Bit 13 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_14(&mut self) -> PortSize14W<'_, CurrentportsizeSpec> {
        PortSize14W::new(self, 13)
    }
    #[doc = "Bit 14 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_15(&mut self) -> PortSize15W<'_, CurrentportsizeSpec> {
        PortSize15W::new(self, 14)
    }
    #[doc = "Bit 15 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_16(&mut self) -> PortSize16W<'_, CurrentportsizeSpec> {
        PortSize16W::new(self, 15)
    }
    #[doc = "Bit 16 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_17(&mut self) -> PortSize17W<'_, CurrentportsizeSpec> {
        PortSize17W::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_18(&mut self) -> PortSize18W<'_, CurrentportsizeSpec> {
        PortSize18W::new(self, 17)
    }
    #[doc = "Bit 18 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_19(&mut self) -> PortSize19W<'_, CurrentportsizeSpec> {
        PortSize19W::new(self, 18)
    }
    #[doc = "Bit 19 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_20(&mut self) -> PortSize20W<'_, CurrentportsizeSpec> {
        PortSize20W::new(self, 19)
    }
    #[doc = "Bit 20 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_21(&mut self) -> PortSize21W<'_, CurrentportsizeSpec> {
        PortSize21W::new(self, 20)
    }
    #[doc = "Bit 21 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_22(&mut self) -> PortSize22W<'_, CurrentportsizeSpec> {
        PortSize22W::new(self, 21)
    }
    #[doc = "Bit 22 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_23(&mut self) -> PortSize23W<'_, CurrentportsizeSpec> {
        PortSize23W::new(self, 22)
    }
    #[doc = "Bit 23 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_24(&mut self) -> PortSize24W<'_, CurrentportsizeSpec> {
        PortSize24W::new(self, 23)
    }
    #[doc = "Bit 24 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_25(&mut self) -> PortSize25W<'_, CurrentportsizeSpec> {
        PortSize25W::new(self, 24)
    }
    #[doc = "Bit 25 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_26(&mut self) -> PortSize26W<'_, CurrentportsizeSpec> {
        PortSize26W::new(self, 25)
    }
    #[doc = "Bit 26 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_27(&mut self) -> PortSize27W<'_, CurrentportsizeSpec> {
        PortSize27W::new(self, 26)
    }
    #[doc = "Bit 27 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_28(&mut self) -> PortSize28W<'_, CurrentportsizeSpec> {
        PortSize28W::new(self, 27)
    }
    #[doc = "Bit 28 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_29(&mut self) -> PortSize29W<'_, CurrentportsizeSpec> {
        PortSize29W::new(self, 28)
    }
    #[doc = "Bit 29 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_30(&mut self) -> PortSize30W<'_, CurrentportsizeSpec> {
        PortSize30W::new(self, 29)
    }
    #[doc = "Bit 30 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_31(&mut self) -> PortSize31W<'_, CurrentportsizeSpec> {
        PortSize31W::new(self, 30)
    }
    #[doc = "Bit 31 - Indicates which port size is currently selected."]
    #[inline(always)]
    pub fn port_size_32(&mut self) -> PortSize32W<'_, CurrentportsizeSpec> {
        PortSize32W::new(self, 31)
    }
}
#[doc = "Each bit location is a single port size. One bit can be set, and indicates the current port size.\n\nYou can [`read`](crate::Reg::read) this register and get [`currentportsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`currentportsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentportsizeSpec;
impl crate::RegisterSpec for CurrentportsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`currentportsize::R`](R) reader structure"]
impl crate::Readable for CurrentportsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`currentportsize::W`](W) writer structure"]
impl crate::Writable for CurrentportsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CURRENTPORTSIZE to value 0"]
impl crate::Resettable for CurrentportsizeSpec {}
