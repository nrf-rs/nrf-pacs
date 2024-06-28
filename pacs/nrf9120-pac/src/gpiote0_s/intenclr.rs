#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event IN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In0> for bool {
    #[inline(always)]
    fn from(variant: In0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` reader - Write '1' to disable interrupt for event IN\\[0\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In0::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In0WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In0WO> for bool {
    #[inline(always)]
    fn from(variant: In0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` writer - Write '1' to disable interrupt for event IN\\[0\\]"]
pub type In0W<'a, REG> = crate::BitWriter<'a, REG, In0WO>;
impl<'a, REG> In0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In0WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In1> for bool {
    #[inline(always)]
    fn from(variant: In1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` reader - Write '1' to disable interrupt for event IN\\[1\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In1::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In1WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In1WO> for bool {
    #[inline(always)]
    fn from(variant: In1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` writer - Write '1' to disable interrupt for event IN\\[1\\]"]
pub type In1W<'a, REG> = crate::BitWriter<'a, REG, In1WO>;
impl<'a, REG> In1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In1WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In2> for bool {
    #[inline(always)]
    fn from(variant: In2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` reader - Write '1' to disable interrupt for event IN\\[2\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In2::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In2WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In2WO> for bool {
    #[inline(always)]
    fn from(variant: In2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` writer - Write '1' to disable interrupt for event IN\\[2\\]"]
pub type In2W<'a, REG> = crate::BitWriter<'a, REG, In2WO>;
impl<'a, REG> In2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In2WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In3> for bool {
    #[inline(always)]
    fn from(variant: In3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` reader - Write '1' to disable interrupt for event IN\\[3\\]"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In3::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In3WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In3WO> for bool {
    #[inline(always)]
    fn from(variant: In3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` writer - Write '1' to disable interrupt for event IN\\[3\\]"]
pub type In3W<'a, REG> = crate::BitWriter<'a, REG, In3WO>;
impl<'a, REG> In3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In3WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In4 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In4> for bool {
    #[inline(always)]
    fn from(variant: In4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN4` reader - Write '1' to disable interrupt for event IN\\[4\\]"]
pub type In4R = crate::BitReader<In4>;
impl In4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In4 {
        match self.bits {
            false => In4::Disabled,
            true => In4::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In4::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In4::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In4WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In4WO> for bool {
    #[inline(always)]
    fn from(variant: In4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN4` writer - Write '1' to disable interrupt for event IN\\[4\\]"]
pub type In4W<'a, REG> = crate::BitWriter<'a, REG, In4WO>;
impl<'a, REG> In4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In4WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In5 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In5> for bool {
    #[inline(always)]
    fn from(variant: In5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN5` reader - Write '1' to disable interrupt for event IN\\[5\\]"]
pub type In5R = crate::BitReader<In5>;
impl In5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In5 {
        match self.bits {
            false => In5::Disabled,
            true => In5::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In5::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In5::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In5WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In5WO> for bool {
    #[inline(always)]
    fn from(variant: In5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN5` writer - Write '1' to disable interrupt for event IN\\[5\\]"]
pub type In5W<'a, REG> = crate::BitWriter<'a, REG, In5WO>;
impl<'a, REG> In5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In5WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In6 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In6> for bool {
    #[inline(always)]
    fn from(variant: In6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN6` reader - Write '1' to disable interrupt for event IN\\[6\\]"]
pub type In6R = crate::BitReader<In6>;
impl In6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In6 {
        match self.bits {
            false => In6::Disabled,
            true => In6::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In6::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In6::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In6WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In6WO> for bool {
    #[inline(always)]
    fn from(variant: In6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN6` writer - Write '1' to disable interrupt for event IN\\[6\\]"]
pub type In6W<'a, REG> = crate::BitWriter<'a, REG, In6WO>;
impl<'a, REG> In6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In6WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In7 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<In7> for bool {
    #[inline(always)]
    fn from(variant: In7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN7` reader - Write '1' to disable interrupt for event IN\\[7\\]"]
pub type In7R = crate::BitReader<In7>;
impl In7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In7 {
        match self.bits {
            false => In7::Disabled,
            true => In7::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == In7::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == In7::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event IN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In7WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<In7WO> for bool {
    #[inline(always)]
    fn from(variant: In7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN7` writer - Write '1' to disable interrupt for event IN\\[7\\]"]
pub type In7W<'a, REG> = crate::BitWriter<'a, REG, In7WO>;
impl<'a, REG> In7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(In7WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event PORT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Port {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Port> for bool {
    #[inline(always)]
    fn from(variant: Port) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` reader - Write '1' to disable interrupt for event PORT"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Port::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Port::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event PORT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<PortWO> for bool {
    #[inline(always)]
    fn from(variant: PortWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORT` writer - Write '1' to disable interrupt for event PORT"]
pub type PortW<'a, REG> = crate::BitWriter<'a, REG, PortWO>;
impl<'a, REG> PortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PortWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event IN\\[0\\]"]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event IN\\[1\\]"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event IN\\[2\\]"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event IN\\[3\\]"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event IN\\[4\\]"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event IN\\[5\\]"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event IN\\[6\\]"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event IN\\[7\\]"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Write '1' to disable interrupt for event PORT"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event IN\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in0(&mut self) -> In0W<IntenclrSpec> {
        In0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event IN\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in1(&mut self) -> In1W<IntenclrSpec> {
        In1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event IN\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in2(&mut self) -> In2W<IntenclrSpec> {
        In2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event IN\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in3(&mut self) -> In3W<IntenclrSpec> {
        In3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event IN\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in4(&mut self) -> In4W<IntenclrSpec> {
        In4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event IN\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in5(&mut self) -> In5W<IntenclrSpec> {
        In5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event IN\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in6(&mut self) -> In6W<IntenclrSpec> {
        In6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event IN\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn in7(&mut self) -> In7W<IntenclrSpec> {
        In7W::new(self, 7)
    }
    #[doc = "Bit 31 - Write '1' to disable interrupt for event PORT"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<IntenclrSpec> {
        PortW::new(self, 31)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
