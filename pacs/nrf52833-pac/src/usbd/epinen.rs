#[doc = "Register `EPINEN` reader"]
pub type R = crate::R<EpinenSpec>;
#[doc = "Register `EPINEN` writer"]
pub type W = crate::W<EpinenSpec>;
#[doc = "Enable IN endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In0 {
    #[doc = "0: Disable endpoint IN 0 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 0 (response to IN tokens)"]
    Enable = 1,
}
impl From<In0> for bool {
    #[inline(always)]
    fn from(variant: In0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN0` reader - Enable IN endpoint 0"]
pub type In0R = crate::BitReader<In0>;
impl In0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In0 {
        match self.bits {
            false => In0::Disable,
            true => In0::Enable,
        }
    }
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In0::Disable
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In0::Enable
    }
}
#[doc = "Field `IN0` writer - Enable IN endpoint 0"]
pub type In0W<'a, REG> = crate::BitWriter<'a, REG, In0>;
impl<'a, REG> In0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In0::Disable)
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In0::Enable)
    }
}
#[doc = "Enable IN endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In1 {
    #[doc = "0: Disable endpoint IN 1 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 1 (response to IN tokens)"]
    Enable = 1,
}
impl From<In1> for bool {
    #[inline(always)]
    fn from(variant: In1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN1` reader - Enable IN endpoint 1"]
pub type In1R = crate::BitReader<In1>;
impl In1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In1 {
        match self.bits {
            false => In1::Disable,
            true => In1::Enable,
        }
    }
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In1::Disable
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In1::Enable
    }
}
#[doc = "Field `IN1` writer - Enable IN endpoint 1"]
pub type In1W<'a, REG> = crate::BitWriter<'a, REG, In1>;
impl<'a, REG> In1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In1::Disable)
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In1::Enable)
    }
}
#[doc = "Enable IN endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In2 {
    #[doc = "0: Disable endpoint IN 2 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 2 (response to IN tokens)"]
    Enable = 1,
}
impl From<In2> for bool {
    #[inline(always)]
    fn from(variant: In2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN2` reader - Enable IN endpoint 2"]
pub type In2R = crate::BitReader<In2>;
impl In2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In2 {
        match self.bits {
            false => In2::Disable,
            true => In2::Enable,
        }
    }
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In2::Disable
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In2::Enable
    }
}
#[doc = "Field `IN2` writer - Enable IN endpoint 2"]
pub type In2W<'a, REG> = crate::BitWriter<'a, REG, In2>;
impl<'a, REG> In2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In2::Disable)
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In2::Enable)
    }
}
#[doc = "Enable IN endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In3 {
    #[doc = "0: Disable endpoint IN 3 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 3 (response to IN tokens)"]
    Enable = 1,
}
impl From<In3> for bool {
    #[inline(always)]
    fn from(variant: In3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN3` reader - Enable IN endpoint 3"]
pub type In3R = crate::BitReader<In3>;
impl In3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In3 {
        match self.bits {
            false => In3::Disable,
            true => In3::Enable,
        }
    }
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In3::Disable
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In3::Enable
    }
}
#[doc = "Field `IN3` writer - Enable IN endpoint 3"]
pub type In3W<'a, REG> = crate::BitWriter<'a, REG, In3>;
impl<'a, REG> In3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In3::Disable)
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In3::Enable)
    }
}
#[doc = "Enable IN endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In4 {
    #[doc = "0: Disable endpoint IN 4 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 4 (response to IN tokens)"]
    Enable = 1,
}
impl From<In4> for bool {
    #[inline(always)]
    fn from(variant: In4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN4` reader - Enable IN endpoint 4"]
pub type In4R = crate::BitReader<In4>;
impl In4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In4 {
        match self.bits {
            false => In4::Disable,
            true => In4::Enable,
        }
    }
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In4::Disable
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In4::Enable
    }
}
#[doc = "Field `IN4` writer - Enable IN endpoint 4"]
pub type In4W<'a, REG> = crate::BitWriter<'a, REG, In4>;
impl<'a, REG> In4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In4::Disable)
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In4::Enable)
    }
}
#[doc = "Enable IN endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In5 {
    #[doc = "0: Disable endpoint IN 5 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 5 (response to IN tokens)"]
    Enable = 1,
}
impl From<In5> for bool {
    #[inline(always)]
    fn from(variant: In5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN5` reader - Enable IN endpoint 5"]
pub type In5R = crate::BitReader<In5>;
impl In5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In5 {
        match self.bits {
            false => In5::Disable,
            true => In5::Enable,
        }
    }
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In5::Disable
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In5::Enable
    }
}
#[doc = "Field `IN5` writer - Enable IN endpoint 5"]
pub type In5W<'a, REG> = crate::BitWriter<'a, REG, In5>;
impl<'a, REG> In5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In5::Disable)
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In5::Enable)
    }
}
#[doc = "Enable IN endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In6 {
    #[doc = "0: Disable endpoint IN 6 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 6 (response to IN tokens)"]
    Enable = 1,
}
impl From<In6> for bool {
    #[inline(always)]
    fn from(variant: In6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN6` reader - Enable IN endpoint 6"]
pub type In6R = crate::BitReader<In6>;
impl In6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In6 {
        match self.bits {
            false => In6::Disable,
            true => In6::Enable,
        }
    }
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In6::Disable
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In6::Enable
    }
}
#[doc = "Field `IN6` writer - Enable IN endpoint 6"]
pub type In6W<'a, REG> = crate::BitWriter<'a, REG, In6>;
impl<'a, REG> In6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In6::Disable)
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In6::Enable)
    }
}
#[doc = "Enable IN endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum In7 {
    #[doc = "0: Disable endpoint IN 7 (no response to IN tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint IN 7 (response to IN tokens)"]
    Enable = 1,
}
impl From<In7> for bool {
    #[inline(always)]
    fn from(variant: In7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN7` reader - Enable IN endpoint 7"]
pub type In7R = crate::BitReader<In7>;
impl In7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> In7 {
        match self.bits {
            false => In7::Disable,
            true => In7::Enable,
        }
    }
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == In7::Disable
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == In7::Enable
    }
}
#[doc = "Field `IN7` writer - Enable IN endpoint 7"]
pub type In7W<'a, REG> = crate::BitWriter<'a, REG, In7>;
impl<'a, REG> In7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(In7::Disable)
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(In7::Enable)
    }
}
#[doc = "Enable ISO IN endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isoin {
    #[doc = "0: Disable ISO IN endpoint 8"]
    Disable = 0,
    #[doc = "1: Enable ISO IN endpoint 8"]
    Enable = 1,
}
impl From<Isoin> for bool {
    #[inline(always)]
    fn from(variant: Isoin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOIN` reader - Enable ISO IN endpoint"]
pub type IsoinR = crate::BitReader<Isoin>;
impl IsoinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isoin {
        match self.bits {
            false => Isoin::Disable,
            true => Isoin::Enable,
        }
    }
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Isoin::Disable
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Isoin::Enable
    }
}
#[doc = "Field `ISOIN` writer - Enable ISO IN endpoint"]
pub type IsoinW<'a, REG> = crate::BitWriter<'a, REG, Isoin>;
impl<'a, REG> IsoinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Isoin::Disable)
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Isoin::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&self) -> IsoinR {
        IsoinR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&mut self) -> In0W<'_, EpinenSpec> {
        In0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&mut self) -> In1W<'_, EpinenSpec> {
        In1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&mut self) -> In2W<'_, EpinenSpec> {
        In2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&mut self) -> In3W<'_, EpinenSpec> {
        In3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&mut self) -> In4W<'_, EpinenSpec> {
        In4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&mut self) -> In5W<'_, EpinenSpec> {
        In5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&mut self) -> In6W<'_, EpinenSpec> {
        In6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&mut self) -> In7W<'_, EpinenSpec> {
        In7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&mut self) -> IsoinW<'_, EpinenSpec> {
        IsoinW::new(self, 8)
    }
}
#[doc = "Endpoint IN enable\n\nYou can [`read`](crate::Reg::read) this register and get [`epinen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpinenSpec;
impl crate::RegisterSpec for EpinenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epinen::R`](R) reader structure"]
impl crate::Readable for EpinenSpec {}
#[doc = "`write(|w| ..)` method takes [`epinen::W`](W) writer structure"]
impl crate::Writable for EpinenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPINEN to value 0x01"]
impl crate::Resettable for EpinenSpec {
    const RESET_VALUE: u32 = 0x01;
}
