#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on IN\\[0\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In0 {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<In0> for bool {
    #[inline(always)]
    fn from(variant: In0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` reader - Disable interrupt on IN\\[0\\] event."]
pub type In0R = crate::BitReader<In0>;
impl In0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In0 {
        match self.bits {
            false => In0::Disabled,
            true => In0::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In0::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In0::Enabled
    }
}
#[doc = "Disable interrupt on IN\\[0\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In0WO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<In0WO> for bool {
    #[inline(always)]
    fn from(variant: In0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` writer - Disable interrupt on IN\\[0\\] event."]
pub type In0W<'a, REG> = crate::BitWriter<'a, REG, In0WO>;
impl<'a, REG> In0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In0WO::Clear)
    }
}
#[doc = "Disable interrupt on IN\\[1\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In1 {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<In1> for bool {
    #[inline(always)]
    fn from(variant: In1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` reader - Disable interrupt on IN\\[1\\] event."]
pub type In1R = crate::BitReader<In1>;
impl In1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In1 {
        match self.bits {
            false => In1::Disabled,
            true => In1::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In1::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In1::Enabled
    }
}
#[doc = "Disable interrupt on IN\\[1\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In1WO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<In1WO> for bool {
    #[inline(always)]
    fn from(variant: In1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` writer - Disable interrupt on IN\\[1\\] event."]
pub type In1W<'a, REG> = crate::BitWriter<'a, REG, In1WO>;
impl<'a, REG> In1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In1WO::Clear)
    }
}
#[doc = "Disable interrupt on IN\\[2\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In2 {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<In2> for bool {
    #[inline(always)]
    fn from(variant: In2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` reader - Disable interrupt on IN\\[2\\] event."]
pub type In2R = crate::BitReader<In2>;
impl In2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In2 {
        match self.bits {
            false => In2::Disabled,
            true => In2::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In2::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In2::Enabled
    }
}
#[doc = "Disable interrupt on IN\\[2\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In2WO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<In2WO> for bool {
    #[inline(always)]
    fn from(variant: In2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` writer - Disable interrupt on IN\\[2\\] event."]
pub type In2W<'a, REG> = crate::BitWriter<'a, REG, In2WO>;
impl<'a, REG> In2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In2WO::Clear)
    }
}
#[doc = "Disable interrupt on IN\\[3\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In3 {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<In3> for bool {
    #[inline(always)]
    fn from(variant: In3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` reader - Disable interrupt on IN\\[3\\] event."]
pub type In3R = crate::BitReader<In3>;
impl In3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In3 {
        match self.bits {
            false => In3::Disabled,
            true => In3::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In3::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In3::Enabled
    }
}
#[doc = "Disable interrupt on IN\\[3\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In3WO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<In3WO> for bool {
    #[inline(always)]
    fn from(variant: In3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` writer - Disable interrupt on IN\\[3\\] event."]
pub type In3W<'a, REG> = crate::BitWriter<'a, REG, In3WO>;
impl<'a, REG> In3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In3WO::Clear)
    }
}
#[doc = "Disable interrupt on PORT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Port> for bool {
    #[inline(always)]
    fn from(variant: Port) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` reader - Disable interrupt on PORT event."]
pub type PortR = crate::BitReader<Port>;
impl PortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port {
        match self.bits {
            false => Port::Disabled,
            true => Port::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port::Enabled
    }
}
#[doc = "Disable interrupt on PORT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<PortWO> for bool {
    #[inline(always)]
    fn from(variant: PortWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` writer - Disable interrupt on PORT event."]
pub type PortW<'a, REG> = crate::BitWriter<'a, REG, PortWO>;
impl<'a, REG> PortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PortWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on IN\\[0\\] event."]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on IN\\[1\\] event."]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on IN\\[2\\] event."]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable interrupt on IN\\[3\\] event."]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable interrupt on PORT event."]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on IN\\[0\\] event."]
    #[inline(always)]
    pub fn in0(&mut self) -> In0W<'_, IntenclrSpec> {
        In0W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable interrupt on IN\\[1\\] event."]
    #[inline(always)]
    pub fn in1(&mut self) -> In1W<'_, IntenclrSpec> {
        In1W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable interrupt on IN\\[2\\] event."]
    #[inline(always)]
    pub fn in2(&mut self) -> In2W<'_, IntenclrSpec> {
        In2W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable interrupt on IN\\[3\\] event."]
    #[inline(always)]
    pub fn in3(&mut self) -> In3W<'_, IntenclrSpec> {
        In3W::new(self, 3)
    }
    #[doc = "Bit 31 - Disable interrupt on PORT event."]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, IntenclrSpec> {
        PortW::new(self, 31)
    }
}
#[doc = "Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
