#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invalidoperation {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Invalidoperation> for bool {
    #[inline(always)]
    fn from(variant: Invalidoperation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` reader - Enable or disable interrupt for event INVALIDOPERATION"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Invalidoperation::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Invalidoperation::Enabled
    }
}
#[doc = "Field `INVALIDOPERATION` writer - Enable or disable interrupt for event INVALIDOPERATION"]
pub type InvalidoperationW<'a, REG> = crate::BitWriter<'a, REG, Invalidoperation>;
impl<'a, REG> InvalidoperationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Invalidoperation::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Invalidoperation::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dividebyzero {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dividebyzero> for bool {
    #[inline(always)]
    fn from(variant: Dividebyzero) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` reader - Enable or disable interrupt for event DIVIDEBYZERO"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dividebyzero::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dividebyzero::Enabled
    }
}
#[doc = "Field `DIVIDEBYZERO` writer - Enable or disable interrupt for event DIVIDEBYZERO"]
pub type DividebyzeroW<'a, REG> = crate::BitWriter<'a, REG, Dividebyzero>;
impl<'a, REG> DividebyzeroW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dividebyzero::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dividebyzero::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Overflow> for bool {
    #[inline(always)]
    fn from(variant: Overflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - Enable or disable interrupt for event OVERFLOW"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Overflow::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Overflow::Enabled
    }
}
#[doc = "Field `OVERFLOW` writer - Enable or disable interrupt for event OVERFLOW"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG, Overflow>;
impl<'a, REG> OverflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Overflow::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underflow {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Underflow> for bool {
    #[inline(always)]
    fn from(variant: Underflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` reader - Enable or disable interrupt for event UNDERFLOW"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Underflow::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Underflow::Enabled
    }
}
#[doc = "Field `UNDERFLOW` writer - Enable or disable interrupt for event UNDERFLOW"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG, Underflow>;
impl<'a, REG> UnderflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Underflow::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Underflow::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inexact {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Inexact> for bool {
    #[inline(always)]
    fn from(variant: Inexact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` reader - Enable or disable interrupt for event INEXACT"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inexact::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inexact::Enabled
    }
}
#[doc = "Field `INEXACT` writer - Enable or disable interrupt for event INEXACT"]
pub type InexactW<'a, REG> = crate::BitWriter<'a, REG, Inexact>;
impl<'a, REG> InexactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inexact::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inexact::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Denormalinput {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Denormalinput> for bool {
    #[inline(always)]
    fn from(variant: Denormalinput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` reader - Enable or disable interrupt for event DENORMALINPUT"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Denormalinput::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Denormalinput::Enabled
    }
}
#[doc = "Field `DENORMALINPUT` writer - Enable or disable interrupt for event DENORMALINPUT"]
pub type DenormalinputW<'a, REG> = crate::BitWriter<'a, REG, Denormalinput>;
impl<'a, REG> DenormalinputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Denormalinput::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Denormalinput::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&self) -> InvalidoperationR {
        InvalidoperationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&self) -> DividebyzeroR {
        DividebyzeroR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&self) -> InexactR {
        InexactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&self) -> DenormalinputR {
        DenormalinputR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&mut self) -> InvalidoperationW<'_, IntenSpec> {
        InvalidoperationW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&mut self) -> DividebyzeroW<'_, IntenSpec> {
        DividebyzeroW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntenSpec> {
        OverflowW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, IntenSpec> {
        UnderflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&mut self) -> InexactW<'_, IntenSpec> {
        InexactW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&mut self) -> DenormalinputW<'_, IntenSpec> {
        DenormalinputW::new(self, 5)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
