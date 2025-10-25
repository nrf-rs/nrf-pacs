#[doc = "Register `EXTCTLOUTPORT` reader"]
pub type R = crate::R<ExtctloutportSpec>;
#[doc = "Register `EXTCTLOUTPORT` writer"]
pub type W = crate::W<ExtctloutportSpec>;
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout0 {
    #[doc = "0: Output EXTCTL0 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL0 is high."]
    High = 1,
}
impl From<Extctlout0> for bool {
    #[inline(always)]
    fn from(variant: Extctlout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_0` reader - EXTCTL outputs."]
pub type Extctlout0R = crate::BitReader<Extctlout0>;
impl Extctlout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout0 {
        match self.bits {
            false => Extctlout0::Low,
            true => Extctlout0::High,
        }
    }
    #[doc = "Output EXTCTL0 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout0::Low
    }
    #[doc = "Output EXTCTL0 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout0::High
    }
}
#[doc = "Field `EXTCTLOUT_0` writer - EXTCTL outputs."]
pub type Extctlout0W<'a, REG> = crate::BitWriter<'a, REG, Extctlout0>;
impl<'a, REG> Extctlout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL0 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout0::Low)
    }
    #[doc = "Output EXTCTL0 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout0::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout1 {
    #[doc = "0: Output EXTCTL1 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL1 is high."]
    High = 1,
}
impl From<Extctlout1> for bool {
    #[inline(always)]
    fn from(variant: Extctlout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_1` reader - EXTCTL outputs."]
pub type Extctlout1R = crate::BitReader<Extctlout1>;
impl Extctlout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout1 {
        match self.bits {
            false => Extctlout1::Low,
            true => Extctlout1::High,
        }
    }
    #[doc = "Output EXTCTL1 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout1::Low
    }
    #[doc = "Output EXTCTL1 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout1::High
    }
}
#[doc = "Field `EXTCTLOUT_1` writer - EXTCTL outputs."]
pub type Extctlout1W<'a, REG> = crate::BitWriter<'a, REG, Extctlout1>;
impl<'a, REG> Extctlout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL1 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout1::Low)
    }
    #[doc = "Output EXTCTL1 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout1::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout2 {
    #[doc = "0: Output EXTCTL2 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL2 is high."]
    High = 1,
}
impl From<Extctlout2> for bool {
    #[inline(always)]
    fn from(variant: Extctlout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_2` reader - EXTCTL outputs."]
pub type Extctlout2R = crate::BitReader<Extctlout2>;
impl Extctlout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout2 {
        match self.bits {
            false => Extctlout2::Low,
            true => Extctlout2::High,
        }
    }
    #[doc = "Output EXTCTL2 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout2::Low
    }
    #[doc = "Output EXTCTL2 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout2::High
    }
}
#[doc = "Field `EXTCTLOUT_2` writer - EXTCTL outputs."]
pub type Extctlout2W<'a, REG> = crate::BitWriter<'a, REG, Extctlout2>;
impl<'a, REG> Extctlout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL2 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout2::Low)
    }
    #[doc = "Output EXTCTL2 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout2::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout3 {
    #[doc = "0: Output EXTCTL3 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL3 is high."]
    High = 1,
}
impl From<Extctlout3> for bool {
    #[inline(always)]
    fn from(variant: Extctlout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_3` reader - EXTCTL outputs."]
pub type Extctlout3R = crate::BitReader<Extctlout3>;
impl Extctlout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout3 {
        match self.bits {
            false => Extctlout3::Low,
            true => Extctlout3::High,
        }
    }
    #[doc = "Output EXTCTL3 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout3::Low
    }
    #[doc = "Output EXTCTL3 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout3::High
    }
}
#[doc = "Field `EXTCTLOUT_3` writer - EXTCTL outputs."]
pub type Extctlout3W<'a, REG> = crate::BitWriter<'a, REG, Extctlout3>;
impl<'a, REG> Extctlout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL3 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout3::Low)
    }
    #[doc = "Output EXTCTL3 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout3::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout4 {
    #[doc = "0: Output EXTCTL4 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL4 is high."]
    High = 1,
}
impl From<Extctlout4> for bool {
    #[inline(always)]
    fn from(variant: Extctlout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_4` reader - EXTCTL outputs."]
pub type Extctlout4R = crate::BitReader<Extctlout4>;
impl Extctlout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout4 {
        match self.bits {
            false => Extctlout4::Low,
            true => Extctlout4::High,
        }
    }
    #[doc = "Output EXTCTL4 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout4::Low
    }
    #[doc = "Output EXTCTL4 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout4::High
    }
}
#[doc = "Field `EXTCTLOUT_4` writer - EXTCTL outputs."]
pub type Extctlout4W<'a, REG> = crate::BitWriter<'a, REG, Extctlout4>;
impl<'a, REG> Extctlout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL4 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout4::Low)
    }
    #[doc = "Output EXTCTL4 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout4::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout5 {
    #[doc = "0: Output EXTCTL5 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL5 is high."]
    High = 1,
}
impl From<Extctlout5> for bool {
    #[inline(always)]
    fn from(variant: Extctlout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_5` reader - EXTCTL outputs."]
pub type Extctlout5R = crate::BitReader<Extctlout5>;
impl Extctlout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout5 {
        match self.bits {
            false => Extctlout5::Low,
            true => Extctlout5::High,
        }
    }
    #[doc = "Output EXTCTL5 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout5::Low
    }
    #[doc = "Output EXTCTL5 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout5::High
    }
}
#[doc = "Field `EXTCTLOUT_5` writer - EXTCTL outputs."]
pub type Extctlout5W<'a, REG> = crate::BitWriter<'a, REG, Extctlout5>;
impl<'a, REG> Extctlout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL5 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout5::Low)
    }
    #[doc = "Output EXTCTL5 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout5::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout6 {
    #[doc = "0: Output EXTCTL6 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL6 is high."]
    High = 1,
}
impl From<Extctlout6> for bool {
    #[inline(always)]
    fn from(variant: Extctlout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_6` reader - EXTCTL outputs."]
pub type Extctlout6R = crate::BitReader<Extctlout6>;
impl Extctlout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout6 {
        match self.bits {
            false => Extctlout6::Low,
            true => Extctlout6::High,
        }
    }
    #[doc = "Output EXTCTL6 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout6::Low
    }
    #[doc = "Output EXTCTL6 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout6::High
    }
}
#[doc = "Field `EXTCTLOUT_6` writer - EXTCTL outputs."]
pub type Extctlout6W<'a, REG> = crate::BitWriter<'a, REG, Extctlout6>;
impl<'a, REG> Extctlout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL6 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout6::Low)
    }
    #[doc = "Output EXTCTL6 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout6::High)
    }
}
#[doc = "EXTCTL outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlout7 {
    #[doc = "0: Output EXTCTL7 is low."]
    Low = 0,
    #[doc = "1: Output EXTCTL7 is high."]
    High = 1,
}
impl From<Extctlout7> for bool {
    #[inline(always)]
    fn from(variant: Extctlout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLOUT_7` reader - EXTCTL outputs."]
pub type Extctlout7R = crate::BitReader<Extctlout7>;
impl Extctlout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlout7 {
        match self.bits {
            false => Extctlout7::Low,
            true => Extctlout7::High,
        }
    }
    #[doc = "Output EXTCTL7 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlout7::Low
    }
    #[doc = "Output EXTCTL7 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlout7::High
    }
}
#[doc = "Field `EXTCTLOUT_7` writer - EXTCTL outputs."]
pub type Extctlout7W<'a, REG> = crate::BitWriter<'a, REG, Extctlout7>;
impl<'a, REG> Extctlout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output EXTCTL7 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout7::Low)
    }
    #[doc = "Output EXTCTL7 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlout7::High)
    }
}
impl R {
    #[doc = "Bit 0 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_0(&self) -> Extctlout0R {
        Extctlout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_1(&self) -> Extctlout1R {
        Extctlout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_2(&self) -> Extctlout2R {
        Extctlout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_3(&self) -> Extctlout3R {
        Extctlout3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_4(&self) -> Extctlout4R {
        Extctlout4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_5(&self) -> Extctlout5R {
        Extctlout5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_6(&self) -> Extctlout6R {
        Extctlout6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_7(&self) -> Extctlout7R {
        Extctlout7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_0(&mut self) -> Extctlout0W<'_, ExtctloutportSpec> {
        Extctlout0W::new(self, 0)
    }
    #[doc = "Bit 1 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_1(&mut self) -> Extctlout1W<'_, ExtctloutportSpec> {
        Extctlout1W::new(self, 1)
    }
    #[doc = "Bit 2 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_2(&mut self) -> Extctlout2W<'_, ExtctloutportSpec> {
        Extctlout2W::new(self, 2)
    }
    #[doc = "Bit 3 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_3(&mut self) -> Extctlout3W<'_, ExtctloutportSpec> {
        Extctlout3W::new(self, 3)
    }
    #[doc = "Bit 4 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_4(&mut self) -> Extctlout4W<'_, ExtctloutportSpec> {
        Extctlout4W::new(self, 4)
    }
    #[doc = "Bit 5 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_5(&mut self) -> Extctlout5W<'_, ExtctloutportSpec> {
        Extctlout5W::new(self, 5)
    }
    #[doc = "Bit 6 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_6(&mut self) -> Extctlout6W<'_, ExtctloutportSpec> {
        Extctlout6W::new(self, 6)
    }
    #[doc = "Bit 7 - EXTCTL outputs."]
    #[inline(always)]
    pub fn extctlout_7(&mut self) -> Extctlout7W<'_, ExtctloutportSpec> {
        Extctlout7W::new(self, 7)
    }
}
#[doc = "Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`extctloutport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctloutport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtctloutportSpec;
impl crate::RegisterSpec for ExtctloutportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extctloutport::R`](R) reader structure"]
impl crate::Readable for ExtctloutportSpec {}
#[doc = "`write(|w| ..)` method takes [`extctloutport::W`](W) writer structure"]
impl crate::Writable for ExtctloutportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTCTLOUTPORT to value 0"]
impl crate::Resettable for ExtctloutportSpec {}
