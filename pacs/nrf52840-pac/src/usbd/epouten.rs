#[doc = "Register `EPOUTEN` reader"]
pub type R = crate::R<EpoutenSpec>;
#[doc = "Register `EPOUTEN` writer"]
pub type W = crate::W<EpoutenSpec>;
#[doc = "Enable OUT endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out0 {
    #[doc = "0: Disable endpoint OUT 0 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 0 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out0> for bool {
    #[inline(always)]
    fn from(variant: Out0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT0` reader - Enable OUT endpoint 0"]
pub type Out0R = crate::BitReader<Out0>;
impl Out0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out0 {
        match self.bits {
            false => Out0::Disable,
            true => Out0::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out0::Disable
    }
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out0::Enable
    }
}
#[doc = "Field `OUT0` writer - Enable OUT endpoint 0"]
pub type Out0W<'a, REG> = crate::BitWriter<'a, REG, Out0>;
impl<'a, REG> Out0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 0 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Disable)
    }
    #[doc = "Enable endpoint OUT 0 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out0::Enable)
    }
}
#[doc = "Enable OUT endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out1 {
    #[doc = "0: Disable endpoint OUT 1 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 1 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out1> for bool {
    #[inline(always)]
    fn from(variant: Out1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT1` reader - Enable OUT endpoint 1"]
pub type Out1R = crate::BitReader<Out1>;
impl Out1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out1 {
        match self.bits {
            false => Out1::Disable,
            true => Out1::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out1::Disable
    }
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out1::Enable
    }
}
#[doc = "Field `OUT1` writer - Enable OUT endpoint 1"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG, Out1>;
impl<'a, REG> Out1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 1 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Disable)
    }
    #[doc = "Enable endpoint OUT 1 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out1::Enable)
    }
}
#[doc = "Enable OUT endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out2 {
    #[doc = "0: Disable endpoint OUT 2 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 2 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out2> for bool {
    #[inline(always)]
    fn from(variant: Out2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2` reader - Enable OUT endpoint 2"]
pub type Out2R = crate::BitReader<Out2>;
impl Out2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out2 {
        match self.bits {
            false => Out2::Disable,
            true => Out2::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out2::Disable
    }
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out2::Enable
    }
}
#[doc = "Field `OUT2` writer - Enable OUT endpoint 2"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG, Out2>;
impl<'a, REG> Out2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 2 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Disable)
    }
    #[doc = "Enable endpoint OUT 2 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out2::Enable)
    }
}
#[doc = "Enable OUT endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out3 {
    #[doc = "0: Disable endpoint OUT 3 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 3 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out3> for bool {
    #[inline(always)]
    fn from(variant: Out3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT3` reader - Enable OUT endpoint 3"]
pub type Out3R = crate::BitReader<Out3>;
impl Out3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out3 {
        match self.bits {
            false => Out3::Disable,
            true => Out3::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out3::Disable
    }
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out3::Enable
    }
}
#[doc = "Field `OUT3` writer - Enable OUT endpoint 3"]
pub type Out3W<'a, REG> = crate::BitWriter<'a, REG, Out3>;
impl<'a, REG> Out3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 3 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out3::Disable)
    }
    #[doc = "Enable endpoint OUT 3 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out3::Enable)
    }
}
#[doc = "Enable OUT endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out4 {
    #[doc = "0: Disable endpoint OUT 4 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 4 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out4> for bool {
    #[inline(always)]
    fn from(variant: Out4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT4` reader - Enable OUT endpoint 4"]
pub type Out4R = crate::BitReader<Out4>;
impl Out4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out4 {
        match self.bits {
            false => Out4::Disable,
            true => Out4::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out4::Disable
    }
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out4::Enable
    }
}
#[doc = "Field `OUT4` writer - Enable OUT endpoint 4"]
pub type Out4W<'a, REG> = crate::BitWriter<'a, REG, Out4>;
impl<'a, REG> Out4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 4 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out4::Disable)
    }
    #[doc = "Enable endpoint OUT 4 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out4::Enable)
    }
}
#[doc = "Enable OUT endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out5 {
    #[doc = "0: Disable endpoint OUT 5 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 5 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out5> for bool {
    #[inline(always)]
    fn from(variant: Out5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT5` reader - Enable OUT endpoint 5"]
pub type Out5R = crate::BitReader<Out5>;
impl Out5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out5 {
        match self.bits {
            false => Out5::Disable,
            true => Out5::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out5::Disable
    }
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out5::Enable
    }
}
#[doc = "Field `OUT5` writer - Enable OUT endpoint 5"]
pub type Out5W<'a, REG> = crate::BitWriter<'a, REG, Out5>;
impl<'a, REG> Out5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 5 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out5::Disable)
    }
    #[doc = "Enable endpoint OUT 5 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out5::Enable)
    }
}
#[doc = "Enable OUT endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out6 {
    #[doc = "0: Disable endpoint OUT 6 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 6 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out6> for bool {
    #[inline(always)]
    fn from(variant: Out6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT6` reader - Enable OUT endpoint 6"]
pub type Out6R = crate::BitReader<Out6>;
impl Out6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out6 {
        match self.bits {
            false => Out6::Disable,
            true => Out6::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out6::Disable
    }
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out6::Enable
    }
}
#[doc = "Field `OUT6` writer - Enable OUT endpoint 6"]
pub type Out6W<'a, REG> = crate::BitWriter<'a, REG, Out6>;
impl<'a, REG> Out6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 6 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out6::Disable)
    }
    #[doc = "Enable endpoint OUT 6 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out6::Enable)
    }
}
#[doc = "Enable OUT endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out7 {
    #[doc = "0: Disable endpoint OUT 7 (no response to OUT tokens)"]
    Disable = 0,
    #[doc = "1: Enable endpoint OUT 7 (response to OUT tokens)"]
    Enable = 1,
}
impl From<Out7> for bool {
    #[inline(always)]
    fn from(variant: Out7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT7` reader - Enable OUT endpoint 7"]
pub type Out7R = crate::BitReader<Out7>;
impl Out7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out7 {
        match self.bits {
            false => Out7::Disable,
            true => Out7::Enable,
        }
    }
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Out7::Disable
    }
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Out7::Enable
    }
}
#[doc = "Field `OUT7` writer - Enable OUT endpoint 7"]
pub type Out7W<'a, REG> = crate::BitWriter<'a, REG, Out7>;
impl<'a, REG> Out7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable endpoint OUT 7 (no response to OUT tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Out7::Disable)
    }
    #[doc = "Enable endpoint OUT 7 (response to OUT tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Out7::Enable)
    }
}
#[doc = "Enable ISO OUT endpoint 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isoout {
    #[doc = "0: Disable ISO OUT endpoint 8"]
    Disable = 0,
    #[doc = "1: Enable ISO OUT endpoint 8"]
    Enable = 1,
}
impl From<Isoout> for bool {
    #[inline(always)]
    fn from(variant: Isoout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOOUT` reader - Enable ISO OUT endpoint 8"]
pub type IsooutR = crate::BitReader<Isoout>;
impl IsooutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isoout {
        match self.bits {
            false => Isoout::Disable,
            true => Isoout::Enable,
        }
    }
    #[doc = "Disable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Isoout::Disable
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Isoout::Enable
    }
}
#[doc = "Field `ISOOUT` writer - Enable ISO OUT endpoint 8"]
pub type IsooutW<'a, REG> = crate::BitWriter<'a, REG, Isoout>;
impl<'a, REG> IsooutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Isoout::Disable)
    }
    #[doc = "Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Isoout::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline(always)]
    pub fn out3(&self) -> Out3R {
        Out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline(always)]
    pub fn out4(&self) -> Out4R {
        Out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline(always)]
    pub fn out5(&self) -> Out5R {
        Out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline(always)]
    pub fn out6(&self) -> Out6R {
        Out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline(always)]
    pub fn out7(&self) -> Out7R {
        Out7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn isoout(&self) -> IsooutR {
        IsooutR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OUT endpoint 0"]
    #[inline(always)]
    pub fn out0(&mut self) -> Out0W<'_, EpoutenSpec> {
        Out0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable OUT endpoint 1"]
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<'_, EpoutenSpec> {
        Out1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable OUT endpoint 2"]
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<'_, EpoutenSpec> {
        Out2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable OUT endpoint 3"]
    #[inline(always)]
    pub fn out3(&mut self) -> Out3W<'_, EpoutenSpec> {
        Out3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable OUT endpoint 4"]
    #[inline(always)]
    pub fn out4(&mut self) -> Out4W<'_, EpoutenSpec> {
        Out4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable OUT endpoint 5"]
    #[inline(always)]
    pub fn out5(&mut self) -> Out5W<'_, EpoutenSpec> {
        Out5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable OUT endpoint 6"]
    #[inline(always)]
    pub fn out6(&mut self) -> Out6W<'_, EpoutenSpec> {
        Out6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable OUT endpoint 7"]
    #[inline(always)]
    pub fn out7(&mut self) -> Out7W<'_, EpoutenSpec> {
        Out7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable ISO OUT endpoint 8"]
    #[inline(always)]
    pub fn isoout(&mut self) -> IsooutW<'_, EpoutenSpec> {
        IsooutW::new(self, 8)
    }
}
#[doc = "Endpoint OUT enable\n\nYou can [`read`](crate::Reg::read) this register and get [`epouten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epouten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpoutenSpec;
impl crate::RegisterSpec for EpoutenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epouten::R`](R) reader structure"]
impl crate::Readable for EpoutenSpec {}
#[doc = "`write(|w| ..)` method takes [`epouten::W`](W) writer structure"]
impl crate::Writable for EpoutenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPOUTEN to value 0x01"]
impl crate::Resettable for EpoutenSpec {
    const RESET_VALUE: u32 = 0x01;
}
