#[doc = "Register `ITATBDATA0` reader"]
pub type R = crate::R<Itatbdata0Spec>;
#[doc = "Register `ITATBDATA0` writer"]
pub type W = crate::W<Itatbdata0Spec>;
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata0 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata0> for bool {
    #[inline(always)]
    fn from(variant: Atdata0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_0` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata0R = crate::BitReader<Atdata0>;
impl Atdata0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata0 {
        match self.bits {
            false => Atdata0::Low,
            true => Atdata0::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata0::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata0::High
    }
}
#[doc = "Field `ATDATA_0` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata0W<'a, REG> = crate::BitWriter<'a, REG, Atdata0>;
impl<'a, REG> Atdata0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata0::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata0::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata1 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata1> for bool {
    #[inline(always)]
    fn from(variant: Atdata1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_1` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata1R = crate::BitReader<Atdata1>;
impl Atdata1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata1 {
        match self.bits {
            false => Atdata1::Low,
            true => Atdata1::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata1::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata1::High
    }
}
#[doc = "Field `ATDATA_1` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata1W<'a, REG> = crate::BitWriter<'a, REG, Atdata1>;
impl<'a, REG> Atdata1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata1::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata1::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata2 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata2> for bool {
    #[inline(always)]
    fn from(variant: Atdata2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_2` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata2R = crate::BitReader<Atdata2>;
impl Atdata2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata2 {
        match self.bits {
            false => Atdata2::Low,
            true => Atdata2::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata2::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata2::High
    }
}
#[doc = "Field `ATDATA_2` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata2W<'a, REG> = crate::BitWriter<'a, REG, Atdata2>;
impl<'a, REG> Atdata2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata2::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata2::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata3 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata3> for bool {
    #[inline(always)]
    fn from(variant: Atdata3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_3` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata3R = crate::BitReader<Atdata3>;
impl Atdata3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata3 {
        match self.bits {
            false => Atdata3::Low,
            true => Atdata3::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata3::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata3::High
    }
}
#[doc = "Field `ATDATA_3` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata3W<'a, REG> = crate::BitWriter<'a, REG, Atdata3>;
impl<'a, REG> Atdata3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata3::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata3::High)
    }
}
#[doc = "A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atdata4 {
    #[doc = "0: Pin is logic 0."]
    Low = 0,
    #[doc = "1: Pin is logic 1."]
    High = 1,
}
impl From<Atdata4> for bool {
    #[inline(always)]
    fn from(variant: Atdata4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATDATA_4` reader - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata4R = crate::BitReader<Atdata4>;
impl Atdata4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atdata4 {
        match self.bits {
            false => Atdata4::Low,
            true => Atdata4::High,
        }
    }
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Atdata4::Low
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Atdata4::High
    }
}
#[doc = "Field `ATDATA_4` writer - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
pub type Atdata4W<'a, REG> = crate::BitWriter<'a, REG, Atdata4>;
impl<'a, REG> Atdata4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata4::Low)
    }
    #[doc = "Pin is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Atdata4::High)
    }
}
impl R {
    #[doc = "Bit 0 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_0(&self) -> Atdata0R {
        Atdata0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_1(&self) -> Atdata1R {
        Atdata1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_2(&self) -> Atdata2R {
        Atdata2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_3(&self) -> Atdata3R {
        Atdata3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_4(&self) -> Atdata4R {
        Atdata4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_0(&mut self) -> Atdata0W<'_, Itatbdata0Spec> {
        Atdata0W::new(self, 0)
    }
    #[doc = "Bit 1 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_1(&mut self) -> Atdata1W<'_, Itatbdata0Spec> {
        Atdata1W::new(self, 1)
    }
    #[doc = "Bit 2 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_2(&mut self) -> Atdata2W<'_, Itatbdata0Spec> {
        Atdata2W::new(self, 2)
    }
    #[doc = "Bit 3 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_3(&mut self) -> Atdata3W<'_, Itatbdata0Spec> {
        Atdata3W::new(self, 3)
    }
    #[doc = "Bit 4 - A read access returns the value of a pin on atdatas_x of the enabled port. A write access writes to the corresponding atdatam pin of the enabled port."]
    #[inline(always)]
    pub fn atdata_4(&mut self) -> Atdata4W<'_, Itatbdata0Spec> {
        Atdata4W::new(self, 4)
    }
}
#[doc = "The ITATBDATA0 register contains the value of the atdatas inputs to the TPIU. The values are valid only when atvalids is HIGH.\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itatbdata0Spec;
impl crate::RegisterSpec for Itatbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itatbdata0::R`](R) reader structure"]
impl crate::Readable for Itatbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`itatbdata0::W`](W) writer structure"]
impl crate::Writable for Itatbdata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ITATBDATA0 to value 0"]
impl crate::Resettable for Itatbdata0Spec {}
