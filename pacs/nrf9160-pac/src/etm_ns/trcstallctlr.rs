#[doc = "Register `TRCSTALLCTLR` reader"]
pub type R = crate::R<TrcstallctlrSpec>;
#[doc = "Register `TRCSTALLCTLR` writer"]
pub type W = crate::W<TrcstallctlrSpec>;
#[doc = "Threshold level field. If LEVEL is nonzero then a trace unit might suppress the generation of: Global timestamps in the instruction trace stream and the data trace stream. Cycle counting in the instruction trace stream, although the cumulative cycle count remains correct.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    #[doc = "0: Zero invasion. This setting has a greater risk of a FIFO overflow"]
    Min = 0,
    #[doc = "15: Maximum invasion occurs but there is less risk of a FIFO overflow."]
    Max = 15,
}
impl From<Level> for u8 {
    #[inline(always)]
    fn from(variant: Level) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Level {
    type Ux = u8;
}
impl crate::IsEnum for Level {}
#[doc = "Field `LEVEL` reader - Threshold level field. If LEVEL is nonzero then a trace unit might suppress the generation of: Global timestamps in the instruction trace stream and the data trace stream. Cycle counting in the instruction trace stream, although the cumulative cycle count remains correct."]
pub type LevelR = crate::FieldReader<Level>;
impl LevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Level> {
        match self.bits {
            0 => Some(Level::Min),
            15 => Some(Level::Max),
            _ => None,
        }
    }
    #[doc = "Zero invasion. This setting has a greater risk of a FIFO overflow"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Level::Min
    }
    #[doc = "Maximum invasion occurs but there is less risk of a FIFO overflow."]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Level::Max
    }
}
#[doc = "Field `LEVEL` writer - Threshold level field. If LEVEL is nonzero then a trace unit might suppress the generation of: Global timestamps in the instruction trace stream and the data trace stream. Cycle counting in the instruction trace stream, although the cumulative cycle count remains correct."]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Level>;
impl<'a, REG> LevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero invasion. This setting has a greater risk of a FIFO overflow"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Min)
    }
    #[doc = "Maximum invasion occurs but there is less risk of a FIFO overflow."]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Level::Max)
    }
}
#[doc = "Instruction stall bit. Controls if a trace unit can stall the PE when the instruction trace buffer space is less than LEVEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Istall {
    #[doc = "0: The trace unit must not stall the PE."]
    Disabled = 0,
    #[doc = "1: The trace unit can stall the PE."]
    Enabled = 1,
}
impl From<Istall> for bool {
    #[inline(always)]
    fn from(variant: Istall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISTALL` reader - Instruction stall bit. Controls if a trace unit can stall the PE when the instruction trace buffer space is less than LEVEL."]
pub type IstallR = crate::BitReader<Istall>;
impl IstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Istall {
        match self.bits {
            false => Istall::Disabled,
            true => Istall::Enabled,
        }
    }
    #[doc = "The trace unit must not stall the PE."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Istall::Disabled
    }
    #[doc = "The trace unit can stall the PE."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Istall::Enabled
    }
}
#[doc = "Field `ISTALL` writer - Instruction stall bit. Controls if a trace unit can stall the PE when the instruction trace buffer space is less than LEVEL."]
pub type IstallW<'a, REG> = crate::BitWriter<'a, REG, Istall>;
impl<'a, REG> IstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit must not stall the PE."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Istall::Disabled)
    }
    #[doc = "The trace unit can stall the PE."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Istall::Enabled)
    }
}
#[doc = "Data stall bit. Controls if a trace unit can stall the PE when the data trace buffer space is less than LEVEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstall {
    #[doc = "0: The trace unit must not stall the PE."]
    Disabled = 0,
    #[doc = "1: The trace unit can stall the PE."]
    Enabled = 1,
}
impl From<Dstall> for bool {
    #[inline(always)]
    fn from(variant: Dstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTALL` reader - Data stall bit. Controls if a trace unit can stall the PE when the data trace buffer space is less than LEVEL."]
pub type DstallR = crate::BitReader<Dstall>;
impl DstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstall {
        match self.bits {
            false => Dstall::Disabled,
            true => Dstall::Enabled,
        }
    }
    #[doc = "The trace unit must not stall the PE."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dstall::Disabled
    }
    #[doc = "The trace unit can stall the PE."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dstall::Enabled
    }
}
#[doc = "Field `DSTALL` writer - Data stall bit. Controls if a trace unit can stall the PE when the data trace buffer space is less than LEVEL."]
pub type DstallW<'a, REG> = crate::BitWriter<'a, REG, Dstall>;
impl<'a, REG> DstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit must not stall the PE."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dstall::Disabled)
    }
    #[doc = "The trace unit can stall the PE."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dstall::Enabled)
    }
}
#[doc = "Prioritize instruction trace bit. Controls if a trace unit can prioritize instruction trace when the instruction trace buffer space is less than LEVEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instpriority {
    #[doc = "0: The trace unit must not prioritize instruction trace."]
    Disabled = 0,
    #[doc = "1: The trace unit can prioritize instruction trace. A trace unit might prioritize instruction trace by preventing output of data trace, or other means which ensure that the instruction trace has a higher priority than the data trace."]
    Enabled = 1,
}
impl From<Instpriority> for bool {
    #[inline(always)]
    fn from(variant: Instpriority) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSTPRIORITY` reader - Prioritize instruction trace bit. Controls if a trace unit can prioritize instruction trace when the instruction trace buffer space is less than LEVEL."]
pub type InstpriorityR = crate::BitReader<Instpriority>;
impl InstpriorityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Instpriority {
        match self.bits {
            false => Instpriority::Disabled,
            true => Instpriority::Enabled,
        }
    }
    #[doc = "The trace unit must not prioritize instruction trace."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Instpriority::Disabled
    }
    #[doc = "The trace unit can prioritize instruction trace. A trace unit might prioritize instruction trace by preventing output of data trace, or other means which ensure that the instruction trace has a higher priority than the data trace."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Instpriority::Enabled
    }
}
#[doc = "Field `INSTPRIORITY` writer - Prioritize instruction trace bit. Controls if a trace unit can prioritize instruction trace when the instruction trace buffer space is less than LEVEL."]
pub type InstpriorityW<'a, REG> = crate::BitWriter<'a, REG, Instpriority>;
impl<'a, REG> InstpriorityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit must not prioritize instruction trace."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Instpriority::Disabled)
    }
    #[doc = "The trace unit can prioritize instruction trace. A trace unit might prioritize instruction trace by preventing output of data trace, or other means which ensure that the instruction trace has a higher priority than the data trace."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Instpriority::Enabled)
    }
}
#[doc = "Data discard field. Controls if a trace unit can discard data trace elements on a load when the data trace buffer space is less than LEVEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datadiscardload {
    #[doc = "0: The trace unit must not discard any data trace elements."]
    Disabled = 0,
    #[doc = "1: The trace unit can discard P1 and P2 elements associated with data loads."]
    Enabled = 1,
}
impl From<Datadiscardload> for bool {
    #[inline(always)]
    fn from(variant: Datadiscardload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATADISCARDLOAD` reader - Data discard field. Controls if a trace unit can discard data trace elements on a load when the data trace buffer space is less than LEVEL."]
pub type DatadiscardloadR = crate::BitReader<Datadiscardload>;
impl DatadiscardloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datadiscardload {
        match self.bits {
            false => Datadiscardload::Disabled,
            true => Datadiscardload::Enabled,
        }
    }
    #[doc = "The trace unit must not discard any data trace elements."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Datadiscardload::Disabled
    }
    #[doc = "The trace unit can discard P1 and P2 elements associated with data loads."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datadiscardload::Enabled
    }
}
#[doc = "Field `DATADISCARDLOAD` writer - Data discard field. Controls if a trace unit can discard data trace elements on a load when the data trace buffer space is less than LEVEL."]
pub type DatadiscardloadW<'a, REG> = crate::BitWriter<'a, REG, Datadiscardload>;
impl<'a, REG> DatadiscardloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit must not discard any data trace elements."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datadiscardload::Disabled)
    }
    #[doc = "The trace unit can discard P1 and P2 elements associated with data loads."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datadiscardload::Enabled)
    }
}
#[doc = "Data discard field. Controls if a trace unit can discard data trace elements on a store when the data trace buffer space is less than LEVEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datadiscardstore {
    #[doc = "0: The trace unit must not discard any data trace elements."]
    Disabled = 0,
    #[doc = "1: The trace unit can discard P1 and P2 elements associated with data stores."]
    Enabled = 1,
}
impl From<Datadiscardstore> for bool {
    #[inline(always)]
    fn from(variant: Datadiscardstore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATADISCARDSTORE` reader - Data discard field. Controls if a trace unit can discard data trace elements on a store when the data trace buffer space is less than LEVEL."]
pub type DatadiscardstoreR = crate::BitReader<Datadiscardstore>;
impl DatadiscardstoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datadiscardstore {
        match self.bits {
            false => Datadiscardstore::Disabled,
            true => Datadiscardstore::Enabled,
        }
    }
    #[doc = "The trace unit must not discard any data trace elements."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Datadiscardstore::Disabled
    }
    #[doc = "The trace unit can discard P1 and P2 elements associated with data stores."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Datadiscardstore::Enabled
    }
}
#[doc = "Field `DATADISCARDSTORE` writer - Data discard field. Controls if a trace unit can discard data trace elements on a store when the data trace buffer space is less than LEVEL."]
pub type DatadiscardstoreW<'a, REG> = crate::BitWriter<'a, REG, Datadiscardstore>;
impl<'a, REG> DatadiscardstoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit must not discard any data trace elements."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datadiscardstore::Disabled)
    }
    #[doc = "The trace unit can discard P1 and P2 elements associated with data stores."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Datadiscardstore::Enabled)
    }
}
#[doc = "Trace overflow prevention bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nooverflow {
    #[doc = "0: Trace overflow prevention is disabled."]
    Disabled = 0,
    #[doc = "1: Trace overflow prevention is enabled. This might cause a significant performance impact."]
    Enabled = 1,
}
impl From<Nooverflow> for bool {
    #[inline(always)]
    fn from(variant: Nooverflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOOVERFLOW` reader - Trace overflow prevention bit."]
pub type NooverflowR = crate::BitReader<Nooverflow>;
impl NooverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nooverflow {
        match self.bits {
            false => Nooverflow::Disabled,
            true => Nooverflow::Enabled,
        }
    }
    #[doc = "Trace overflow prevention is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nooverflow::Disabled
    }
    #[doc = "Trace overflow prevention is enabled. This might cause a significant performance impact."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nooverflow::Enabled
    }
}
#[doc = "Field `NOOVERFLOW` writer - Trace overflow prevention bit."]
pub type NooverflowW<'a, REG> = crate::BitWriter<'a, REG, Nooverflow>;
impl<'a, REG> NooverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trace overflow prevention is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nooverflow::Disabled)
    }
    #[doc = "Trace overflow prevention is enabled. This might cause a significant performance impact."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nooverflow::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Threshold level field. If LEVEL is nonzero then a trace unit might suppress the generation of: Global timestamps in the instruction trace stream and the data trace stream. Cycle counting in the instruction trace stream, although the cumulative cycle count remains correct."]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Instruction stall bit. Controls if a trace unit can stall the PE when the instruction trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn istall(&self) -> IstallR {
        IstallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data stall bit. Controls if a trace unit can stall the PE when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn dstall(&self) -> DstallR {
        DstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Prioritize instruction trace bit. Controls if a trace unit can prioritize instruction trace when the instruction trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn instpriority(&self) -> InstpriorityR {
        InstpriorityR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data discard field. Controls if a trace unit can discard data trace elements on a load when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn datadiscardload(&self) -> DatadiscardloadR {
        DatadiscardloadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data discard field. Controls if a trace unit can discard data trace elements on a store when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn datadiscardstore(&self) -> DatadiscardstoreR {
        DatadiscardstoreR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Trace overflow prevention bit."]
    #[inline(always)]
    pub fn nooverflow(&self) -> NooverflowR {
        NooverflowR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Threshold level field. If LEVEL is nonzero then a trace unit might suppress the generation of: Global timestamps in the instruction trace stream and the data trace stream. Cycle counting in the instruction trace stream, although the cumulative cycle count remains correct."]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, TrcstallctlrSpec> {
        LevelW::new(self, 0)
    }
    #[doc = "Bit 8 - Instruction stall bit. Controls if a trace unit can stall the PE when the instruction trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn istall(&mut self) -> IstallW<'_, TrcstallctlrSpec> {
        IstallW::new(self, 8)
    }
    #[doc = "Bit 9 - Data stall bit. Controls if a trace unit can stall the PE when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn dstall(&mut self) -> DstallW<'_, TrcstallctlrSpec> {
        DstallW::new(self, 9)
    }
    #[doc = "Bit 10 - Prioritize instruction trace bit. Controls if a trace unit can prioritize instruction trace when the instruction trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn instpriority(&mut self) -> InstpriorityW<'_, TrcstallctlrSpec> {
        InstpriorityW::new(self, 10)
    }
    #[doc = "Bit 11 - Data discard field. Controls if a trace unit can discard data trace elements on a load when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn datadiscardload(&mut self) -> DatadiscardloadW<'_, TrcstallctlrSpec> {
        DatadiscardloadW::new(self, 11)
    }
    #[doc = "Bit 12 - Data discard field. Controls if a trace unit can discard data trace elements on a store when the data trace buffer space is less than LEVEL."]
    #[inline(always)]
    pub fn datadiscardstore(&mut self) -> DatadiscardstoreW<'_, TrcstallctlrSpec> {
        DatadiscardstoreW::new(self, 12)
    }
    #[doc = "Bit 13 - Trace overflow prevention bit."]
    #[inline(always)]
    pub fn nooverflow(&mut self) -> NooverflowW<'_, TrcstallctlrSpec> {
        NooverflowW::new(self, 13)
    }
}
#[doc = "Enables trace unit functionality that prevents trace unit buffer overflows. Might ignore writes when the trace unit is enabled or not idle. Must be programmed if TRCIDR3.STALLCTL == 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcstallctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcstallctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcstallctlrSpec;
impl crate::RegisterSpec for TrcstallctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcstallctlr::R`](R) reader structure"]
impl crate::Readable for TrcstallctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcstallctlr::W`](W) writer structure"]
impl crate::Writable for TrcstallctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCSTALLCTLR to value 0"]
impl crate::Resettable for TrcstallctlrSpec {}
