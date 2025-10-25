#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invalidoperation {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Invalidoperation> for bool {
    #[inline(always)]
    fn from(variant: Invalidoperation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` reader - Write '1' to disable interrupt for event INVALIDOPERATION"]
pub type InvalidoperationR = crate::BitReader<Invalidoperation>;
impl InvalidoperationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invalidoperation {
        match self.bits {
            false => Invalidoperation::Disabled,
            true => Invalidoperation::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Invalidoperation::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Invalidoperation::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidoperationWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<InvalidoperationWO> for bool {
    #[inline(always)]
    fn from(variant: InvalidoperationWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` writer - Write '1' to disable interrupt for event INVALIDOPERATION"]
pub type InvalidoperationW<'a, REG> = crate::BitWriter<'a, REG, InvalidoperationWO>;
impl<'a, REG> InvalidoperationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(InvalidoperationWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dividebyzero {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dividebyzero> for bool {
    #[inline(always)]
    fn from(variant: Dividebyzero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` reader - Write '1' to disable interrupt for event DIVIDEBYZERO"]
pub type DividebyzeroR = crate::BitReader<Dividebyzero>;
impl DividebyzeroR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dividebyzero {
        match self.bits {
            false => Dividebyzero::Disabled,
            true => Dividebyzero::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dividebyzero::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dividebyzero::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DividebyzeroWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DividebyzeroWO> for bool {
    #[inline(always)]
    fn from(variant: DividebyzeroWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` writer - Write '1' to disable interrupt for event DIVIDEBYZERO"]
pub type DividebyzeroW<'a, REG> = crate::BitWriter<'a, REG, DividebyzeroWO>;
impl<'a, REG> DividebyzeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DividebyzeroWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Overflow> for bool {
    #[inline(always)]
    fn from(variant: Overflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - Write '1' to disable interrupt for event OVERFLOW"]
pub type OverflowR = crate::BitReader<Overflow>;
impl OverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overflow {
        match self.bits {
            false => Overflow::Disabled,
            true => Overflow::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Overflow::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Overflow::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<OverflowWO> for bool {
    #[inline(always)]
    fn from(variant: OverflowWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - Write '1' to disable interrupt for event OVERFLOW"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG, OverflowWO>;
impl<'a, REG> OverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underflow {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Underflow> for bool {
    #[inline(always)]
    fn from(variant: Underflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` reader - Write '1' to disable interrupt for event UNDERFLOW"]
pub type UnderflowR = crate::BitReader<Underflow>;
impl UnderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Underflow {
        match self.bits {
            false => Underflow::Disabled,
            true => Underflow::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Underflow::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Underflow::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnderflowWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<UnderflowWO> for bool {
    #[inline(always)]
    fn from(variant: UnderflowWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` writer - Write '1' to disable interrupt for event UNDERFLOW"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG, UnderflowWO>;
impl<'a, REG> UnderflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UnderflowWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inexact {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Inexact> for bool {
    #[inline(always)]
    fn from(variant: Inexact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` reader - Write '1' to disable interrupt for event INEXACT"]
pub type InexactR = crate::BitReader<Inexact>;
impl InexactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inexact {
        match self.bits {
            false => Inexact::Disabled,
            true => Inexact::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inexact::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inexact::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InexactWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<InexactWO> for bool {
    #[inline(always)]
    fn from(variant: InexactWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` writer - Write '1' to disable interrupt for event INEXACT"]
pub type InexactW<'a, REG> = crate::BitWriter<'a, REG, InexactWO>;
impl<'a, REG> InexactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(InexactWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Denormalinput {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Denormalinput> for bool {
    #[inline(always)]
    fn from(variant: Denormalinput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` reader - Write '1' to disable interrupt for event DENORMALINPUT"]
pub type DenormalinputR = crate::BitReader<Denormalinput>;
impl DenormalinputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Denormalinput {
        match self.bits {
            false => Denormalinput::Disabled,
            true => Denormalinput::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Denormalinput::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Denormalinput::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenormalinputWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<DenormalinputWO> for bool {
    #[inline(always)]
    fn from(variant: DenormalinputWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` writer - Write '1' to disable interrupt for event DENORMALINPUT"]
pub type DenormalinputW<'a, REG> = crate::BitWriter<'a, REG, DenormalinputWO>;
impl<'a, REG> DenormalinputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DenormalinputWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&self) -> InvalidoperationR {
        InvalidoperationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&self) -> DividebyzeroR {
        DividebyzeroR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&self) -> InexactR {
        InexactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&self) -> DenormalinputR {
        DenormalinputR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&mut self) -> InvalidoperationW<'_, IntenclrSpec> {
        InvalidoperationW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&mut self) -> DividebyzeroW<'_, IntenclrSpec> {
        DividebyzeroW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntenclrSpec> {
        OverflowW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, IntenclrSpec> {
        UnderflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&mut self) -> InexactW<'_, IntenclrSpec> {
        InexactW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&mut self) -> DenormalinputW<'_, IntenclrSpec> {
        DenormalinputW::new(self, 5)
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
