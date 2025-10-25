#[doc = "Register `TRCQCTLR` reader"]
pub type R = crate::R<TrcqctlrSpec>;
#[doc = "Register `TRCQCTLR` writer"]
pub type W = crate::W<TrcqctlrSpec>;
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range0 {
    #[doc = "0: Address range comparator 0 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 0 is selected for use."]
    Enabled = 1,
}
impl From<Range0> for bool {
    #[inline(always)]
    fn from(variant: Range0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_0` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 0 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range0::Disabled
    }
    #[doc = "Address range comparator 0 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range0::Enabled
    }
}
#[doc = "Field `RANGE_0` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range0W<'a, REG> = crate::BitWriter<'a, REG, Range0>;
impl<'a, REG> Range0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 0 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::Disabled)
    }
    #[doc = "Address range comparator 0 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range1 {
    #[doc = "0: Address range comparator 1 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 1 is selected for use."]
    Enabled = 1,
}
impl From<Range1> for bool {
    #[inline(always)]
    fn from(variant: Range1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_1` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 1 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range1::Disabled
    }
    #[doc = "Address range comparator 1 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range1::Enabled
    }
}
#[doc = "Field `RANGE_1` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range1W<'a, REG> = crate::BitWriter<'a, REG, Range1>;
impl<'a, REG> Range1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 1 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range1::Disabled)
    }
    #[doc = "Address range comparator 1 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range1::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range2 {
    #[doc = "0: Address range comparator 2 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 2 is selected for use."]
    Enabled = 1,
}
impl From<Range2> for bool {
    #[inline(always)]
    fn from(variant: Range2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_2` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 2 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range2::Disabled
    }
    #[doc = "Address range comparator 2 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range2::Enabled
    }
}
#[doc = "Field `RANGE_2` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range2W<'a, REG> = crate::BitWriter<'a, REG, Range2>;
impl<'a, REG> Range2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 2 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range2::Disabled)
    }
    #[doc = "Address range comparator 2 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range2::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range3 {
    #[doc = "0: Address range comparator 3 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 3 is selected for use."]
    Enabled = 1,
}
impl From<Range3> for bool {
    #[inline(always)]
    fn from(variant: Range3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_3` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 3 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range3::Disabled
    }
    #[doc = "Address range comparator 3 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range3::Enabled
    }
}
#[doc = "Field `RANGE_3` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range3W<'a, REG> = crate::BitWriter<'a, REG, Range3>;
impl<'a, REG> Range3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 3 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range3::Disabled)
    }
    #[doc = "Address range comparator 3 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range3::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range4 {
    #[doc = "0: Address range comparator 4 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 4 is selected for use."]
    Enabled = 1,
}
impl From<Range4> for bool {
    #[inline(always)]
    fn from(variant: Range4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_4` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 4 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range4::Disabled
    }
    #[doc = "Address range comparator 4 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range4::Enabled
    }
}
#[doc = "Field `RANGE_4` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range4W<'a, REG> = crate::BitWriter<'a, REG, Range4>;
impl<'a, REG> Range4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 4 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range4::Disabled)
    }
    #[doc = "Address range comparator 4 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range4::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range5 {
    #[doc = "0: Address range comparator 5 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 5 is selected for use."]
    Enabled = 1,
}
impl From<Range5> for bool {
    #[inline(always)]
    fn from(variant: Range5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_5` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 5 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range5::Disabled
    }
    #[doc = "Address range comparator 5 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range5::Enabled
    }
}
#[doc = "Field `RANGE_5` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range5W<'a, REG> = crate::BitWriter<'a, REG, Range5>;
impl<'a, REG> Range5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 5 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range5::Disabled)
    }
    #[doc = "Address range comparator 5 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range5::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range6 {
    #[doc = "0: Address range comparator 6 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 6 is selected for use."]
    Enabled = 1,
}
impl From<Range6> for bool {
    #[inline(always)]
    fn from(variant: Range6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_6` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 6 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range6::Disabled
    }
    #[doc = "Address range comparator 6 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range6::Enabled
    }
}
#[doc = "Field `RANGE_6` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range6W<'a, REG> = crate::BitWriter<'a, REG, Range6>;
impl<'a, REG> Range6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 6 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range6::Disabled)
    }
    #[doc = "Address range comparator 6 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range6::Enabled)
    }
}
#[doc = "Specifies the address range comparators to be used for controlling Q elements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range7 {
    #[doc = "0: Address range comparator 7 is disabled."]
    Disabled = 0,
    #[doc = "1: Address range comparator 7 is selected for use."]
    Enabled = 1,
}
impl From<Range7> for bool {
    #[inline(always)]
    fn from(variant: Range7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE_7` reader - Specifies the address range comparators to be used for controlling Q elements."]
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
    #[doc = "Address range comparator 7 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Range7::Disabled
    }
    #[doc = "Address range comparator 7 is selected for use."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Range7::Enabled
    }
}
#[doc = "Field `RANGE_7` writer - Specifies the address range comparators to be used for controlling Q elements."]
pub type Range7W<'a, REG> = crate::BitWriter<'a, REG, Range7>;
impl<'a, REG> Range7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address range comparator 7 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range7::Disabled)
    }
    #[doc = "Address range comparator 7 is selected for use."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Range7::Enabled)
    }
}
#[doc = "Selects whether the address range comparators selected by the RANGE field indicate address ranges where the trace unit is permitted to generate Q elements or address ranges where the trace unit is not permitted to generate Q elements:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Exclude mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit cannot generate Q elements. If no ranges are selected, Q elements are permitted across the entire memory map."]
    Exclude = 0,
    #[doc = "1: Include mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit can generate Q elements. If all the implemented bits in RANGE are set to 0 then Q elements are disabled."]
    Include = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Selects whether the address range comparators selected by the RANGE field indicate address ranges where the trace unit is permitted to generate Q elements or address ranges where the trace unit is not permitted to generate Q elements:"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Exclude,
            true => Mode::Include,
        }
    }
    #[doc = "Exclude mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit cannot generate Q elements. If no ranges are selected, Q elements are permitted across the entire memory map."]
    #[inline(always)]
    pub fn is_exclude(&self) -> bool {
        *self == Mode::Exclude
    }
    #[doc = "Include mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit can generate Q elements. If all the implemented bits in RANGE are set to 0 then Q elements are disabled."]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Mode::Include
    }
}
#[doc = "Field `MODE` writer - Selects whether the address range comparators selected by the RANGE field indicate address ranges where the trace unit is permitted to generate Q elements or address ranges where the trace unit is not permitted to generate Q elements:"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exclude mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit cannot generate Q elements. If no ranges are selected, Q elements are permitted across the entire memory map."]
    #[inline(always)]
    pub fn exclude(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Exclude)
    }
    #[doc = "Include mode. The address range comparators selected by the RANGE field indicate address ranges where the trace unit can generate Q elements. If all the implemented bits in RANGE are set to 0 then Q elements are disabled."]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Include)
    }
}
impl R {
    #[doc = "Bit 0 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_0(&self) -> Range0R {
        Range0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_1(&self) -> Range1R {
        Range1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_2(&self) -> Range2R {
        Range2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_3(&self) -> Range3R {
        Range3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_4(&self) -> Range4R {
        Range4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_5(&self) -> Range5R {
        Range5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_6(&self) -> Range6R {
        Range6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_7(&self) -> Range7R {
        Range7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects whether the address range comparators selected by the RANGE field indicate address ranges where the trace unit is permitted to generate Q elements or address ranges where the trace unit is not permitted to generate Q elements:"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_0(&mut self) -> Range0W<'_, TrcqctlrSpec> {
        Range0W::new(self, 0)
    }
    #[doc = "Bit 1 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_1(&mut self) -> Range1W<'_, TrcqctlrSpec> {
        Range1W::new(self, 1)
    }
    #[doc = "Bit 2 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_2(&mut self) -> Range2W<'_, TrcqctlrSpec> {
        Range2W::new(self, 2)
    }
    #[doc = "Bit 3 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_3(&mut self) -> Range3W<'_, TrcqctlrSpec> {
        Range3W::new(self, 3)
    }
    #[doc = "Bit 4 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_4(&mut self) -> Range4W<'_, TrcqctlrSpec> {
        Range4W::new(self, 4)
    }
    #[doc = "Bit 5 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_5(&mut self) -> Range5W<'_, TrcqctlrSpec> {
        Range5W::new(self, 5)
    }
    #[doc = "Bit 6 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_6(&mut self) -> Range6W<'_, TrcqctlrSpec> {
        Range6W::new(self, 6)
    }
    #[doc = "Bit 7 - Specifies the address range comparators to be used for controlling Q elements."]
    #[inline(always)]
    pub fn range_7(&mut self) -> Range7W<'_, TrcqctlrSpec> {
        Range7W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects whether the address range comparators selected by the RANGE field indicate address ranges where the trace unit is permitted to generate Q elements or address ranges where the trace unit is not permitted to generate Q elements:"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, TrcqctlrSpec> {
        ModeW::new(self, 8)
    }
}
#[doc = "Controls when Q elements are enabled. Might ignore writes when the trace unit is enabled or not idle. This register must be programmed if it is implemented and TRCCONFIGR.QE is set to any value other than 0b00.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcqctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcqctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcqctlrSpec;
impl crate::RegisterSpec for TrcqctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcqctlr::R`](R) reader structure"]
impl crate::Readable for TrcqctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcqctlr::W`](W) writer structure"]
impl crate::Writable for TrcqctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCQCTLR to value 0"]
impl crate::Resettable for TrcqctlrSpec {}
