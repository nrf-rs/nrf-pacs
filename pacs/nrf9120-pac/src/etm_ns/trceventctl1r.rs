#[doc = "Register `TRCEVENTCTL1R` reader"]
pub type R = crate::R<Trceventctl1rSpec>;
#[doc = "Register `TRCEVENTCTL1R` writer"]
pub type W = crate::W<Trceventctl1rSpec>;
#[doc = "Instruction event enable field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insten0 {
    #[doc = "0: The trace unit does not generate an Event element."]
    Disabled = 0,
    #[doc = "1: The trace unit generates an Event element for event 0, in the instruction trace stream."]
    Enabled = 1,
}
impl From<Insten0> for bool {
    #[inline(always)]
    fn from(variant: Insten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSTEN_0` reader - Instruction event enable field."]
pub type Insten0R = crate::BitReader<Insten0>;
impl Insten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insten0 {
        match self.bits {
            false => Insten0::Disabled,
            true => Insten0::Enabled,
        }
    }
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Insten0::Disabled
    }
    #[doc = "The trace unit generates an Event element for event 0, in the instruction trace stream."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Insten0::Enabled
    }
}
#[doc = "Field `INSTEN_0` writer - Instruction event enable field."]
pub type Insten0W<'a, REG> = crate::BitWriter<'a, REG, Insten0>;
impl<'a, REG> Insten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten0::Disabled)
    }
    #[doc = "The trace unit generates an Event element for event 0, in the instruction trace stream."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten0::Enabled)
    }
}
#[doc = "Instruction event enable field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insten1 {
    #[doc = "0: The trace unit does not generate an Event element."]
    Disabled = 0,
    #[doc = "1: The trace unit generates an Event element for event 1, in the instruction trace stream."]
    Enabled = 1,
}
impl From<Insten1> for bool {
    #[inline(always)]
    fn from(variant: Insten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSTEN_1` reader - Instruction event enable field."]
pub type Insten1R = crate::BitReader<Insten1>;
impl Insten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insten1 {
        match self.bits {
            false => Insten1::Disabled,
            true => Insten1::Enabled,
        }
    }
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Insten1::Disabled
    }
    #[doc = "The trace unit generates an Event element for event 1, in the instruction trace stream."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Insten1::Enabled
    }
}
#[doc = "Field `INSTEN_1` writer - Instruction event enable field."]
pub type Insten1W<'a, REG> = crate::BitWriter<'a, REG, Insten1>;
impl<'a, REG> Insten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten1::Disabled)
    }
    #[doc = "The trace unit generates an Event element for event 1, in the instruction trace stream."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten1::Enabled)
    }
}
#[doc = "Instruction event enable field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insten2 {
    #[doc = "0: The trace unit does not generate an Event element."]
    Disabled = 0,
    #[doc = "1: The trace unit generates an Event element for event 2, in the instruction trace stream."]
    Enabled = 1,
}
impl From<Insten2> for bool {
    #[inline(always)]
    fn from(variant: Insten2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSTEN_2` reader - Instruction event enable field."]
pub type Insten2R = crate::BitReader<Insten2>;
impl Insten2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insten2 {
        match self.bits {
            false => Insten2::Disabled,
            true => Insten2::Enabled,
        }
    }
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Insten2::Disabled
    }
    #[doc = "The trace unit generates an Event element for event 2, in the instruction trace stream."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Insten2::Enabled
    }
}
#[doc = "Field `INSTEN_2` writer - Instruction event enable field."]
pub type Insten2W<'a, REG> = crate::BitWriter<'a, REG, Insten2>;
impl<'a, REG> Insten2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten2::Disabled)
    }
    #[doc = "The trace unit generates an Event element for event 2, in the instruction trace stream."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten2::Enabled)
    }
}
#[doc = "Instruction event enable field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insten3 {
    #[doc = "0: The trace unit does not generate an Event element."]
    Disabled = 0,
    #[doc = "1: The trace unit generates an Event element for event 3, in the instruction trace stream."]
    Enabled = 1,
}
impl From<Insten3> for bool {
    #[inline(always)]
    fn from(variant: Insten3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSTEN_3` reader - Instruction event enable field."]
pub type Insten3R = crate::BitReader<Insten3>;
impl Insten3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insten3 {
        match self.bits {
            false => Insten3::Disabled,
            true => Insten3::Enabled,
        }
    }
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Insten3::Disabled
    }
    #[doc = "The trace unit generates an Event element for event 3, in the instruction trace stream."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Insten3::Enabled
    }
}
#[doc = "Field `INSTEN_3` writer - Instruction event enable field."]
pub type Insten3W<'a, REG> = crate::BitWriter<'a, REG, Insten3>;
impl<'a, REG> Insten3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate an Event element."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten3::Disabled)
    }
    #[doc = "The trace unit generates an Event element for event 3, in the instruction trace stream."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Insten3::Enabled)
    }
}
#[doc = "Data event enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataen {
    #[doc = "0: The trace unit does not generate an Event element if event 0 occurs."]
    Disabled = 0,
    #[doc = "1: The trace unit generates an Event element in the data trace stream if event 0 occurs."]
    Enabled = 1,
}
impl From<Dataen> for bool {
    #[inline(always)]
    fn from(variant: Dataen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAEN` reader - Data event enable bit."]
pub type DataenR = crate::BitReader<Dataen>;
impl DataenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataen {
        match self.bits {
            false => Dataen::Disabled,
            true => Dataen::Enabled,
        }
    }
    #[doc = "The trace unit does not generate an Event element if event 0 occurs."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dataen::Disabled
    }
    #[doc = "The trace unit generates an Event element in the data trace stream if event 0 occurs."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dataen::Enabled
    }
}
#[doc = "Field `DATAEN` writer - Data event enable bit."]
pub type DataenW<'a, REG> = crate::BitWriter<'a, REG, Dataen>;
impl<'a, REG> DataenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trace unit does not generate an Event element if event 0 occurs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dataen::Disabled)
    }
    #[doc = "The trace unit generates an Event element in the data trace stream if event 0 occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dataen::Enabled)
    }
}
#[doc = "AMBA Trace Bus (ATB) trigger enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atb {
    #[doc = "0: ATB trigger is disabled."]
    Disabled = 0,
    #[doc = "1: ATB trigger is enabled. If a CoreSight ATB interface is implemented then when event 0 occurs the trace unit generates an ATB event."]
    Enabled = 1,
}
impl From<Atb> for bool {
    #[inline(always)]
    fn from(variant: Atb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATB` reader - AMBA Trace Bus (ATB) trigger enable bit."]
pub type AtbR = crate::BitReader<Atb>;
impl AtbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atb {
        match self.bits {
            false => Atb::Disabled,
            true => Atb::Enabled,
        }
    }
    #[doc = "ATB trigger is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Atb::Disabled
    }
    #[doc = "ATB trigger is enabled. If a CoreSight ATB interface is implemented then when event 0 occurs the trace unit generates an ATB event."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Atb::Enabled
    }
}
#[doc = "Field `ATB` writer - AMBA Trace Bus (ATB) trigger enable bit."]
pub type AtbW<'a, REG> = crate::BitWriter<'a, REG, Atb>;
impl<'a, REG> AtbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ATB trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Atb::Disabled)
    }
    #[doc = "ATB trigger is enabled. If a CoreSight ATB interface is implemented then when event 0 occurs the trace unit generates an ATB event."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Atb::Enabled)
    }
}
#[doc = "Low-power state behavior override bit. Controls how a trace unit behaves in low-power state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpoverride {
    #[doc = "0: Trace unit low-power state behavior is not affected. That is, the trace unit is enabled to enter low-power state."]
    Disabled = 0,
    #[doc = "1: Trace unit low-power state behavior is overridden. That is, entry to a low-power state does not affect the trace unit resources or trace generation."]
    Enabled = 1,
}
impl From<Lpoverride> for bool {
    #[inline(always)]
    fn from(variant: Lpoverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOVERRIDE` reader - Low-power state behavior override bit. Controls how a trace unit behaves in low-power state."]
pub type LpoverrideR = crate::BitReader<Lpoverride>;
impl LpoverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpoverride {
        match self.bits {
            false => Lpoverride::Disabled,
            true => Lpoverride::Enabled,
        }
    }
    #[doc = "Trace unit low-power state behavior is not affected. That is, the trace unit is enabled to enter low-power state."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lpoverride::Disabled
    }
    #[doc = "Trace unit low-power state behavior is overridden. That is, entry to a low-power state does not affect the trace unit resources or trace generation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lpoverride::Enabled
    }
}
#[doc = "Field `LPOVERRIDE` writer - Low-power state behavior override bit. Controls how a trace unit behaves in low-power state."]
pub type LpoverrideW<'a, REG> = crate::BitWriter<'a, REG, Lpoverride>;
impl<'a, REG> LpoverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trace unit low-power state behavior is not affected. That is, the trace unit is enabled to enter low-power state."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpoverride::Disabled)
    }
    #[doc = "Trace unit low-power state behavior is overridden. That is, entry to a low-power state does not affect the trace unit resources or trace generation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lpoverride::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_0(&self) -> Insten0R {
        Insten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_1(&self) -> Insten1R {
        Insten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_2(&self) -> Insten2R {
        Insten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_3(&self) -> Insten3R {
        Insten3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data event enable bit."]
    #[inline(always)]
    pub fn dataen(&self) -> DataenR {
        DataenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - AMBA Trace Bus (ATB) trigger enable bit."]
    #[inline(always)]
    pub fn atb(&self) -> AtbR {
        AtbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Low-power state behavior override bit. Controls how a trace unit behaves in low-power state."]
    #[inline(always)]
    pub fn lpoverride(&self) -> LpoverrideR {
        LpoverrideR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_0(&mut self) -> Insten0W<'_, Trceventctl1rSpec> {
        Insten0W::new(self, 0)
    }
    #[doc = "Bit 1 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_1(&mut self) -> Insten1W<'_, Trceventctl1rSpec> {
        Insten1W::new(self, 1)
    }
    #[doc = "Bit 2 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_2(&mut self) -> Insten2W<'_, Trceventctl1rSpec> {
        Insten2W::new(self, 2)
    }
    #[doc = "Bit 3 - Instruction event enable field."]
    #[inline(always)]
    pub fn insten_3(&mut self) -> Insten3W<'_, Trceventctl1rSpec> {
        Insten3W::new(self, 3)
    }
    #[doc = "Bit 4 - Data event enable bit."]
    #[inline(always)]
    pub fn dataen(&mut self) -> DataenW<'_, Trceventctl1rSpec> {
        DataenW::new(self, 4)
    }
    #[doc = "Bit 11 - AMBA Trace Bus (ATB) trigger enable bit."]
    #[inline(always)]
    pub fn atb(&mut self) -> AtbW<'_, Trceventctl1rSpec> {
        AtbW::new(self, 11)
    }
    #[doc = "Bit 12 - Low-power state behavior override bit. Controls how a trace unit behaves in low-power state."]
    #[inline(always)]
    pub fn lpoverride(&mut self) -> LpoverrideW<'_, Trceventctl1rSpec> {
        LpoverrideW::new(self, 12)
    }
}
#[doc = "Controls the behavior of the events that TRCEVENTCTL0R selects. This register must always be programmed as part of trace unit initialization. Might ignore writes when the trace unit is enabled or not idle.\n\nYou can [`read`](crate::Reg::read) this register and get [`trceventctl1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trceventctl1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trceventctl1rSpec;
impl crate::RegisterSpec for Trceventctl1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trceventctl1r::R`](R) reader structure"]
impl crate::Readable for Trceventctl1rSpec {}
#[doc = "`write(|w| ..)` method takes [`trceventctl1r::W`](W) writer structure"]
impl crate::Writable for Trceventctl1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRCEVENTCTL1R to value 0"]
impl crate::Resettable for Trceventctl1rSpec {}
