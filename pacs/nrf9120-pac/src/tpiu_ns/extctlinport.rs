#[doc = "Register `EXTCTLINPORT` reader"]
pub type R = crate::R<ExtctlinportSpec>;
#[doc = "Register `EXTCTLINPORT` writer"]
pub type W = crate::W<ExtctlinportSpec>;
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin0 {
    #[doc = "0: Input EXTCTL0 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL0 is high."]
    High = 1,
}
impl From<Extctlin0> for bool {
    #[inline(always)]
    fn from(variant: Extctlin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_0` reader - EXTCTL inputs."]
pub type Extctlin0R = crate::BitReader<Extctlin0>;
impl Extctlin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin0 {
        match self.bits {
            false => Extctlin0::Low,
            true => Extctlin0::High,
        }
    }
    #[doc = "Input EXTCTL0 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin0::Low
    }
    #[doc = "Input EXTCTL0 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin0::High
    }
}
#[doc = "Field `EXTCTLIN_0` writer - EXTCTL inputs."]
pub type Extctlin0W<'a, REG> = crate::BitWriter<'a, REG, Extctlin0>;
impl<'a, REG> Extctlin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL0 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin0::Low)
    }
    #[doc = "Input EXTCTL0 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin0::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin1 {
    #[doc = "0: Input EXTCTL1 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL1 is high."]
    High = 1,
}
impl From<Extctlin1> for bool {
    #[inline(always)]
    fn from(variant: Extctlin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_1` reader - EXTCTL inputs."]
pub type Extctlin1R = crate::BitReader<Extctlin1>;
impl Extctlin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin1 {
        match self.bits {
            false => Extctlin1::Low,
            true => Extctlin1::High,
        }
    }
    #[doc = "Input EXTCTL1 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin1::Low
    }
    #[doc = "Input EXTCTL1 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin1::High
    }
}
#[doc = "Field `EXTCTLIN_1` writer - EXTCTL inputs."]
pub type Extctlin1W<'a, REG> = crate::BitWriter<'a, REG, Extctlin1>;
impl<'a, REG> Extctlin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL1 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin1::Low)
    }
    #[doc = "Input EXTCTL1 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin1::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin2 {
    #[doc = "0: Input EXTCTL2 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL2 is high."]
    High = 1,
}
impl From<Extctlin2> for bool {
    #[inline(always)]
    fn from(variant: Extctlin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_2` reader - EXTCTL inputs."]
pub type Extctlin2R = crate::BitReader<Extctlin2>;
impl Extctlin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin2 {
        match self.bits {
            false => Extctlin2::Low,
            true => Extctlin2::High,
        }
    }
    #[doc = "Input EXTCTL2 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin2::Low
    }
    #[doc = "Input EXTCTL2 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin2::High
    }
}
#[doc = "Field `EXTCTLIN_2` writer - EXTCTL inputs."]
pub type Extctlin2W<'a, REG> = crate::BitWriter<'a, REG, Extctlin2>;
impl<'a, REG> Extctlin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL2 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin2::Low)
    }
    #[doc = "Input EXTCTL2 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin2::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin3 {
    #[doc = "0: Input EXTCTL3 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL3 is high."]
    High = 1,
}
impl From<Extctlin3> for bool {
    #[inline(always)]
    fn from(variant: Extctlin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_3` reader - EXTCTL inputs."]
pub type Extctlin3R = crate::BitReader<Extctlin3>;
impl Extctlin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin3 {
        match self.bits {
            false => Extctlin3::Low,
            true => Extctlin3::High,
        }
    }
    #[doc = "Input EXTCTL3 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin3::Low
    }
    #[doc = "Input EXTCTL3 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin3::High
    }
}
#[doc = "Field `EXTCTLIN_3` writer - EXTCTL inputs."]
pub type Extctlin3W<'a, REG> = crate::BitWriter<'a, REG, Extctlin3>;
impl<'a, REG> Extctlin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL3 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin3::Low)
    }
    #[doc = "Input EXTCTL3 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin3::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin4 {
    #[doc = "0: Input EXTCTL4 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL4 is high."]
    High = 1,
}
impl From<Extctlin4> for bool {
    #[inline(always)]
    fn from(variant: Extctlin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_4` reader - EXTCTL inputs."]
pub type Extctlin4R = crate::BitReader<Extctlin4>;
impl Extctlin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin4 {
        match self.bits {
            false => Extctlin4::Low,
            true => Extctlin4::High,
        }
    }
    #[doc = "Input EXTCTL4 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin4::Low
    }
    #[doc = "Input EXTCTL4 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin4::High
    }
}
#[doc = "Field `EXTCTLIN_4` writer - EXTCTL inputs."]
pub type Extctlin4W<'a, REG> = crate::BitWriter<'a, REG, Extctlin4>;
impl<'a, REG> Extctlin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL4 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin4::Low)
    }
    #[doc = "Input EXTCTL4 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin4::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin5 {
    #[doc = "0: Input EXTCTL5 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL5 is high."]
    High = 1,
}
impl From<Extctlin5> for bool {
    #[inline(always)]
    fn from(variant: Extctlin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_5` reader - EXTCTL inputs."]
pub type Extctlin5R = crate::BitReader<Extctlin5>;
impl Extctlin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin5 {
        match self.bits {
            false => Extctlin5::Low,
            true => Extctlin5::High,
        }
    }
    #[doc = "Input EXTCTL5 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin5::Low
    }
    #[doc = "Input EXTCTL5 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin5::High
    }
}
#[doc = "Field `EXTCTLIN_5` writer - EXTCTL inputs."]
pub type Extctlin5W<'a, REG> = crate::BitWriter<'a, REG, Extctlin5>;
impl<'a, REG> Extctlin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL5 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin5::Low)
    }
    #[doc = "Input EXTCTL5 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin5::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin6 {
    #[doc = "0: Input EXTCTL6 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL6 is high."]
    High = 1,
}
impl From<Extctlin6> for bool {
    #[inline(always)]
    fn from(variant: Extctlin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_6` reader - EXTCTL inputs."]
pub type Extctlin6R = crate::BitReader<Extctlin6>;
impl Extctlin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin6 {
        match self.bits {
            false => Extctlin6::Low,
            true => Extctlin6::High,
        }
    }
    #[doc = "Input EXTCTL6 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin6::Low
    }
    #[doc = "Input EXTCTL6 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin6::High
    }
}
#[doc = "Field `EXTCTLIN_6` writer - EXTCTL inputs."]
pub type Extctlin6W<'a, REG> = crate::BitWriter<'a, REG, Extctlin6>;
impl<'a, REG> Extctlin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL6 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin6::Low)
    }
    #[doc = "Input EXTCTL6 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin6::High)
    }
}
#[doc = "EXTCTL inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extctlin7 {
    #[doc = "0: Input EXTCTL7 is low."]
    Low = 0,
    #[doc = "1: Input EXTCTL7 is high."]
    High = 1,
}
impl From<Extctlin7> for bool {
    #[inline(always)]
    fn from(variant: Extctlin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTCTLIN_7` reader - EXTCTL inputs."]
pub type Extctlin7R = crate::BitReader<Extctlin7>;
impl Extctlin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extctlin7 {
        match self.bits {
            false => Extctlin7::Low,
            true => Extctlin7::High,
        }
    }
    #[doc = "Input EXTCTL7 is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Extctlin7::Low
    }
    #[doc = "Input EXTCTL7 is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Extctlin7::High
    }
}
#[doc = "Field `EXTCTLIN_7` writer - EXTCTL inputs."]
pub type Extctlin7W<'a, REG> = crate::BitWriter<'a, REG, Extctlin7>;
impl<'a, REG> Extctlin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input EXTCTL7 is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin7::Low)
    }
    #[doc = "Input EXTCTL7 is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Extctlin7::High)
    }
}
impl R {
    #[doc = "Bit 0 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_0(&self) -> Extctlin0R {
        Extctlin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_1(&self) -> Extctlin1R {
        Extctlin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_2(&self) -> Extctlin2R {
        Extctlin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_3(&self) -> Extctlin3R {
        Extctlin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_4(&self) -> Extctlin4R {
        Extctlin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_5(&self) -> Extctlin5R {
        Extctlin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_6(&self) -> Extctlin6R {
        Extctlin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_7(&self) -> Extctlin7R {
        Extctlin7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_0(&mut self) -> Extctlin0W<'_, ExtctlinportSpec> {
        Extctlin0W::new(self, 0)
    }
    #[doc = "Bit 1 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_1(&mut self) -> Extctlin1W<'_, ExtctlinportSpec> {
        Extctlin1W::new(self, 1)
    }
    #[doc = "Bit 2 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_2(&mut self) -> Extctlin2W<'_, ExtctlinportSpec> {
        Extctlin2W::new(self, 2)
    }
    #[doc = "Bit 3 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_3(&mut self) -> Extctlin3W<'_, ExtctlinportSpec> {
        Extctlin3W::new(self, 3)
    }
    #[doc = "Bit 4 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_4(&mut self) -> Extctlin4W<'_, ExtctlinportSpec> {
        Extctlin4W::new(self, 4)
    }
    #[doc = "Bit 5 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_5(&mut self) -> Extctlin5W<'_, ExtctlinportSpec> {
        Extctlin5W::new(self, 5)
    }
    #[doc = "Bit 6 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_6(&mut self) -> Extctlin6W<'_, ExtctlinportSpec> {
        Extctlin6W::new(self, 6)
    }
    #[doc = "Bit 7 - EXTCTL inputs."]
    #[inline(always)]
    pub fn extctlin_7(&mut self) -> Extctlin7W<'_, ExtctlinportSpec> {
        Extctlin7W::new(self, 7)
    }
}
#[doc = "Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexers, or other solutions that might be added to the trace output pins either for pin control or a high-speed trace port solution.\n\nYou can [`read`](crate::Reg::read) this register and get [`extctlinport::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctlinport::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtctlinportSpec;
impl crate::RegisterSpec for ExtctlinportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extctlinport::R`](R) reader structure"]
impl crate::Readable for ExtctlinportSpec {}
#[doc = "`write(|w| ..)` method takes [`extctlinport::W`](W) writer structure"]
impl crate::Writable for ExtctlinportSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTCTLINPORT to value 0"]
impl crate::Resettable for ExtctlinportSpec {}
