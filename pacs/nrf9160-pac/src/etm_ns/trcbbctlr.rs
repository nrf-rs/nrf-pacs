#[doc = "Register `TRCBBCTLR` reader"]
pub type R = crate::R<TrcbbctlrSpec>;
#[doc = "Register `TRCBBCTLR` writer"]
pub type W = crate::W<TrcbbctlrSpec>;
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[0\\] controls the selection of address range comparator pair 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range0 {
    #[doc = "0: The address range that address range comparator pair 0 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range0> for bool {
    #[inline(always)]
    fn from(variant: Range0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_0` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[0\\] controls the selection of address range comparator pair 0."]
pub type Range0R = crate::BitReader<Range0>;
impl Range0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range0 {
        match self.bits {
            false => Range0::Disabled,
            true => Range0::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 0 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range0::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range0::Enabled
    }
}
#[doc = "Field `RANGE_0` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[0\\] controls the selection of address range comparator pair 0."]
pub type Range0W<'a, REG> = crate::BitWriter<'a, REG, Range0>;
impl<'a, REG> Range0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 0 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[1\\] controls the selection of address range comparator pair 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range1 {
    #[doc = "0: The address range that address range comparator pair 1 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range1> for bool {
    #[inline(always)]
    fn from(variant: Range1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_1` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[1\\] controls the selection of address range comparator pair 1."]
pub type Range1R = crate::BitReader<Range1>;
impl Range1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range1 {
        match self.bits {
            false => Range1::Disabled,
            true => Range1::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 1 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range1::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range1::Enabled
    }
}
#[doc = "Field `RANGE_1` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[1\\] controls the selection of address range comparator pair 1."]
pub type Range1W<'a, REG> = crate::BitWriter<'a, REG, Range1>;
impl<'a, REG> Range1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 1 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range1::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range1::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[2\\] controls the selection of address range comparator pair 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range2 {
    #[doc = "0: The address range that address range comparator pair 2 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range2> for bool {
    #[inline(always)]
    fn from(variant: Range2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_2` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[2\\] controls the selection of address range comparator pair 2."]
pub type Range2R = crate::BitReader<Range2>;
impl Range2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range2 {
        match self.bits {
            false => Range2::Disabled,
            true => Range2::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 2 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range2::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range2::Enabled
    }
}
#[doc = "Field `RANGE_2` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[2\\] controls the selection of address range comparator pair 2."]
pub type Range2W<'a, REG> = crate::BitWriter<'a, REG, Range2>;
impl<'a, REG> Range2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 2 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range2::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range2::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[3\\] controls the selection of address range comparator pair 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range3 {
    #[doc = "0: The address range that address range comparator pair 3 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range3> for bool {
    #[inline(always)]
    fn from(variant: Range3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_3` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[3\\] controls the selection of address range comparator pair 3."]
pub type Range3R = crate::BitReader<Range3>;
impl Range3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range3 {
        match self.bits {
            false => Range3::Disabled,
            true => Range3::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 3 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range3::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range3::Enabled
    }
}
#[doc = "Field `RANGE_3` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[3\\] controls the selection of address range comparator pair 3."]
pub type Range3W<'a, REG> = crate::BitWriter<'a, REG, Range3>;
impl<'a, REG> Range3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 3 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range3::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range3::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[4\\] controls the selection of address range comparator pair 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range4 {
    #[doc = "0: The address range that address range comparator pair 4 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range4> for bool {
    #[inline(always)]
    fn from(variant: Range4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_4` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[4\\] controls the selection of address range comparator pair 4."]
pub type Range4R = crate::BitReader<Range4>;
impl Range4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range4 {
        match self.bits {
            false => Range4::Disabled,
            true => Range4::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 4 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range4::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range4::Enabled
    }
}
#[doc = "Field `RANGE_4` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[4\\] controls the selection of address range comparator pair 4."]
pub type Range4W<'a, REG> = crate::BitWriter<'a, REG, Range4>;
impl<'a, REG> Range4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 4 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range4::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range4::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[5\\] controls the selection of address range comparator pair 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range5 {
    #[doc = "0: The address range that address range comparator pair 5 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range5> for bool {
    #[inline(always)]
    fn from(variant: Range5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_5` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[5\\] controls the selection of address range comparator pair 5."]
pub type Range5R = crate::BitReader<Range5>;
impl Range5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range5 {
        match self.bits {
            false => Range5::Disabled,
            true => Range5::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 5 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range5::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range5::Enabled
    }
}
#[doc = "Field `RANGE_5` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[5\\] controls the selection of address range comparator pair 5."]
pub type Range5W<'a, REG> = crate::BitWriter<'a, REG, Range5>;
impl<'a, REG> Range5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 5 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range5::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range5::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[6\\] controls the selection of address range comparator pair 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range6 {
    #[doc = "0: The address range that address range comparator pair 6 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range6> for bool {
    #[inline(always)]
    fn from(variant: Range6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_6` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[6\\] controls the selection of address range comparator pair 6."]
pub type Range6R = crate::BitReader<Range6>;
impl Range6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range6 {
        match self.bits {
            false => Range6::Disabled,
            true => Range6::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 6 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range6::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range6::Enabled
    }
}
#[doc = "Field `RANGE_6` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[6\\] controls the selection of address range comparator pair 6."]
pub type Range6W<'a, REG> = crate::BitWriter<'a, REG, Range6>;
impl<'a, REG> Range6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 6 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range6::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range6::Enabled)
    }
}
#[doc = "Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[7\\] controls the selection of address range comparator pair 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range7 {
    #[doc = "0: The address range that address range comparator pair 7 defines, is not selected."]
    Disabled = 0,
    #[doc = "1: The address range that address range comparator pair n defines, is selected."]
    Enabled = 1,
}
impl From<Range7> for bool {
    #[inline(always)]
    fn from(variant: Range7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_7` reader - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[7\\] controls the selection of address range comparator pair 7."]
pub type Range7R = crate::BitReader<Range7>;
impl Range7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range7 {
        match self.bits {
            false => Range7::Disabled,
            true => Range7::Enabled,
        }
    }
    #[doc = "The address range that address range comparator pair 7 defines, is not selected."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range7::Disabled
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range7::Enabled
    }
}
#[doc = "Field `RANGE_7` writer - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[7\\] controls the selection of address range comparator pair 7."]
pub type Range7W<'a, REG> = crate::BitWriter<'a, REG, Range7>;
impl<'a, REG> Range7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The address range that address range comparator pair 7 defines, is not selected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range7::Disabled)
    }
    #[doc = "The address range that address range comparator pair n defines, is selected."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[0\\] controls the selection of address range comparator pair 0."]
    #[inline(always)]
    pub fn range_0(&self) -> Range0R {
        Range0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[1\\] controls the selection of address range comparator pair 1."]
    #[inline(always)]
    pub fn range_1(&self) -> Range1R {
        Range1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[2\\] controls the selection of address range comparator pair 2."]
    #[inline(always)]
    pub fn range_2(&self) -> Range2R {
        Range2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[3\\] controls the selection of address range comparator pair 3."]
    #[inline(always)]
    pub fn range_3(&self) -> Range3R {
        Range3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[4\\] controls the selection of address range comparator pair 4."]
    #[inline(always)]
    pub fn range_4(&self) -> Range4R {
        Range4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[5\\] controls the selection of address range comparator pair 5."]
    #[inline(always)]
    pub fn range_5(&self) -> Range5R {
        Range5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[6\\] controls the selection of address range comparator pair 6."]
    #[inline(always)]
    pub fn range_6(&self) -> Range6R {
        Range6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[7\\] controls the selection of address range comparator pair 7."]
    #[inline(always)]
    pub fn range_7(&self) -> Range7R {
        Range7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[0\\] controls the selection of address range comparator pair 0."]
    #[inline(always)]
    pub fn range_0(&mut self) -> Range0W<'_, TrcbbctlrSpec> {
        Range0W::new(self, 0)
    }
    #[doc = "Bit 1 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[1\\] controls the selection of address range comparator pair 1."]
    #[inline(always)]
    pub fn range_1(&mut self) -> Range1W<'_, TrcbbctlrSpec> {
        Range1W::new(self, 1)
    }
    #[doc = "Bit 2 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[2\\] controls the selection of address range comparator pair 2."]
    #[inline(always)]
    pub fn range_2(&mut self) -> Range2W<'_, TrcbbctlrSpec> {
        Range2W::new(self, 2)
    }
    #[doc = "Bit 3 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[3\\] controls the selection of address range comparator pair 3."]
    #[inline(always)]
    pub fn range_3(&mut self) -> Range3W<'_, TrcbbctlrSpec> {
        Range3W::new(self, 3)
    }
    #[doc = "Bit 4 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[4\\] controls the selection of address range comparator pair 4."]
    #[inline(always)]
    pub fn range_4(&mut self) -> Range4W<'_, TrcbbctlrSpec> {
        Range4W::new(self, 4)
    }
    #[doc = "Bit 5 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[5\\] controls the selection of address range comparator pair 5."]
    #[inline(always)]
    pub fn range_5(&mut self) -> Range5W<'_, TrcbbctlrSpec> {
        Range5W::new(self, 5)
    }
    #[doc = "Bit 6 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[6\\] controls the selection of address range comparator pair 6."]
    #[inline(always)]
    pub fn range_6(&mut self) -> Range6W<'_, TrcbbctlrSpec> {
        Range6W::new(self, 6)
    }
    #[doc = "Bit 7 - Address range field. Selects which address range comparator pairs are in use with branch broadcasting. Each field represents an address range comparator pair, so field\\[7\\] controls the selection of address range comparator pair 7."]
    #[inline(always)]
    pub fn range_7(&mut self) -> Range7W<'_, TrcbbctlrSpec> {
        Range7W::new(self, 7)
    }
}
#[doc = "Controls which regions in the memory map are enabled to use branch broadcasting. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCCONFIGR.BB == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcbbctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcbbctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcbbctlrSpec;
impl crate::RegisterSpec for TrcbbctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcbbctlr::R`](R) reader structure"]
impl crate::Readable for TrcbbctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcbbctlr::W`](W) writer structure"]
impl crate::Writable for TrcbbctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCBBCTLR to value 0"]
impl crate::Resettable for TrcbbctlrSpec {}
