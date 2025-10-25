#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Bit order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    #[doc = "0: Most significant bit shifted out first"]
    MsbFirst = 0,
    #[doc = "1: Least significant bit shifted out first"]
    LsbFirst = 1,
}
impl From<Order> for bool {
    #[inline(always)]
    fn from(variant: Order) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORDER` reader - Bit order"]
pub type OrderR = crate::BitReader<Order>;
impl OrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Order {
        match self.bits {
            false => Order::MsbFirst,
            true => Order::LsbFirst,
        }
    }
    #[doc = "Most significant bit shifted out first"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == Order::MsbFirst
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == Order::LsbFirst
    }
}
#[doc = "Field `ORDER` writer - Bit order"]
pub type OrderW<'a, REG> = crate::BitWriter<'a, REG, Order>;
impl<'a, REG> OrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Most significant bit shifted out first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Order::MsbFirst)
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Order::LsbFirst)
    }
}
#[doc = "Serial clock (SCK) phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Sample on leading edge of clock, shift serial data on trailing edge"]
    Leading = 0,
    #[doc = "1: Sample on trailing edge of clock, shift serial data on leading edge"]
    Trailing = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Serial clock (SCK) phase"]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::Leading,
            true => Cpha::Trailing,
        }
    }
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline(always)]
    pub fn is_leading(&self) -> bool {
        *self == Cpha::Leading
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline(always)]
    pub fn is_trailing(&self) -> bool {
        *self == Cpha::Trailing
    }
}
#[doc = "Field `CPHA` writer - Serial clock (SCK) phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline(always)]
    pub fn leading(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::Leading)
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline(always)]
    pub fn trailing(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::Trailing)
    }
}
#[doc = "Serial clock (SCK) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Active high"]
    ActiveHigh = 0,
    #[doc = "1: Active low"]
    ActiveLow = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Serial clock (SCK) polarity"]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::ActiveHigh,
            true => Cpol::ActiveLow,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Cpol::ActiveHigh
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Cpol::ActiveLow
    }
}
#[doc = "Field `CPOL` writer - Serial clock (SCK) polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::ActiveHigh)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::ActiveLow)
    }
}
impl R {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&self) -> OrderR {
        OrderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&mut self) -> OrderW<'_, ConfigSpec> {
        OrderW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, ConfigSpec> {
        CphaW::new(self, 1)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, ConfigSpec> {
        CpolW::new(self, 2)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
