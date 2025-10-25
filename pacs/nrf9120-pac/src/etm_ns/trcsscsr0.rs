#[doc = "Register `TRCSSCSR0` reader"]
pub type R = crate::R<Trcsscsr0Spec>;
#[doc = "Register `TRCSSCSR0` writer"]
pub type W = crate::W<Trcsscsr0Spec>;
#[doc = "Instruction address comparator support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inst {
    #[doc = "0: Single-shot instruction address comparisons not supported."]
    False = 0,
    #[doc = "1: Single-shot instruction address comparisons supported."]
    True = 1,
}
impl From<Inst> for bool {
    #[inline(always)]
    fn from(variant: Inst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INST` reader - Instruction address comparator support"]
pub type InstR = crate::BitReader<Inst>;
impl InstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inst {
        match self.bits {
            false => Inst::False,
            true => Inst::True,
        }
    }
    #[doc = "Single-shot instruction address comparisons not supported."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Inst::False
    }
    #[doc = "Single-shot instruction address comparisons supported."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Inst::True
    }
}
#[doc = "Field `INST` writer - Instruction address comparator support"]
pub type InstW<'a, REG> = crate::BitWriter<'a, REG, Inst>;
impl<'a, REG> InstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-shot instruction address comparisons not supported."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Inst::False)
    }
    #[doc = "Single-shot instruction address comparisons supported."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Inst::True)
    }
}
#[doc = "Data address comparator support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Da {
    #[doc = "0: Data address comparisons not supported."]
    False = 0,
    #[doc = "1: Data address comparisons supported."]
    True = 1,
}
impl From<Da> for bool {
    #[inline(always)]
    fn from(variant: Da) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DA` reader - Data address comparator support"]
pub type DaR = crate::BitReader<Da>;
impl DaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Da {
        match self.bits {
            false => Da::False,
            true => Da::True,
        }
    }
    #[doc = "Data address comparisons not supported."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Da::False
    }
    #[doc = "Data address comparisons supported."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Da::True
    }
}
#[doc = "Field `DA` writer - Data address comparator support"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG, Da>;
impl<'a, REG> DaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data address comparisons not supported."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Da::False)
    }
    #[doc = "Data address comparisons supported."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Da::True)
    }
}
#[doc = "Data value comparator support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dv {
    #[doc = "0: Data value comparisons not supported."]
    False = 0,
    #[doc = "1: Data value comparisons supported."]
    True = 1,
}
impl From<Dv> for bool {
    #[inline(always)]
    fn from(variant: Dv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DV` reader - Data value comparator support"]
pub type DvR = crate::BitReader<Dv>;
impl DvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dv {
        match self.bits {
            false => Dv::False,
            true => Dv::True,
        }
    }
    #[doc = "Data value comparisons not supported."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Dv::False
    }
    #[doc = "Data value comparisons supported."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Dv::True
    }
}
#[doc = "Field `DV` writer - Data value comparator support"]
pub type DvW<'a, REG> = crate::BitWriter<'a, REG, Dv>;
impl<'a, REG> DvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data value comparisons not supported."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Dv::False)
    }
    #[doc = "Data value comparisons supported."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Dv::True)
    }
}
#[doc = "Process counter value comparator support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc {
    #[doc = "0: Process counter value comparisons not supported."]
    False = 0,
    #[doc = "1: Process counter value comparisons supported."]
    True = 1,
}
impl From<Pc> for bool {
    #[inline(always)]
    fn from(variant: Pc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC` reader - Process counter value comparator support"]
pub type PcR = crate::BitReader<Pc>;
impl PcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc {
        match self.bits {
            false => Pc::False,
            true => Pc::True,
        }
    }
    #[doc = "Process counter value comparisons not supported."]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Pc::False
    }
    #[doc = "Process counter value comparisons supported."]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Pc::True
    }
}
#[doc = "Field `PC` writer - Process counter value comparator support"]
pub type PcW<'a, REG> = crate::BitWriter<'a, REG, Pc>;
impl<'a, REG> PcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Process counter value comparisons not supported."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::False)
    }
    #[doc = "Process counter value comparisons supported."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::True)
    }
}
#[doc = "Single-shot status. This indicates whether any of the selected comparators have matched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Match has not occurred."]
    NoMatch = 0,
    #[doc = "1: Match has occurred at least once."]
    Match = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Single-shot status. This indicates whether any of the selected comparators have matched."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::NoMatch,
            true => Status::Match,
        }
    }
    #[doc = "Match has not occurred."]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == Status::NoMatch
    }
    #[doc = "Match has occurred at least once."]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == Status::Match
    }
}
#[doc = "Field `STATUS` writer - Single-shot status. This indicates whether any of the selected comparators have matched."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG, Status>;
impl<'a, REG> StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match has not occurred."]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut crate::W<REG> {
        self.variant(Status::NoMatch)
    }
    #[doc = "Match has occurred at least once."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Match)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction address comparator support"]
    #[inline(always)]
    pub fn inst(&self) -> InstR {
        InstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data address comparator support"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data value comparator support"]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Process counter value comparator support"]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Single-shot status. This indicates whether any of the selected comparators have matched."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction address comparator support"]
    #[inline(always)]
    pub fn inst(&mut self) -> InstW<'_, Trcsscsr0Spec> {
        InstW::new(self, 0)
    }
    #[doc = "Bit 1 - Data address comparator support"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, Trcsscsr0Spec> {
        DaW::new(self, 1)
    }
    #[doc = "Bit 2 - Data value comparator support"]
    #[inline(always)]
    pub fn dv(&mut self) -> DvW<'_, Trcsscsr0Spec> {
        DvW::new(self, 2)
    }
    #[doc = "Bit 3 - Process counter value comparator support"]
    #[inline(always)]
    pub fn pc(&mut self) -> PcW<'_, Trcsscsr0Spec> {
        PcW::new(self, 3)
    }
    #[doc = "Bit 31 - Single-shot status. This indicates whether any of the selected comparators have matched."]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, Trcsscsr0Spec> {
        StatusW::new(self, 31)
    }
}
#[doc = "Indicates the status of the single-shot comparators. TRCSSCSR0 is sensitive toinstruction addresses.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcsscsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcsscsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trcsscsr0Spec;
impl crate::RegisterSpec for Trcsscsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcsscsr0::R`](R) reader structure"]
impl crate::Readable for Trcsscsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`trcsscsr0::W`](W) writer structure"]
impl crate::Writable for Trcsscsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSSCSR0 to value 0"]
impl crate::Resettable for Trcsscsr0Spec {}
