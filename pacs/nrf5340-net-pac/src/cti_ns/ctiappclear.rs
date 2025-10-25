#[doc = "Register `CTIAPPCLEAR` writer"]
pub type W = crate::W<CtiappclearSpec>;
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appclear0 {
    #[doc = "1: Clears the event for channel 0."]
    Clear = 1,
}
impl From<Appclear0> for bool {
    #[inline(always)]
    fn from(variant: Appclear0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_0` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type Appclear0W<'a, REG> = crate::BitWriter<'a, REG, Appclear0>;
impl<'a, REG> Appclear0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the event for channel 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Appclear0::Clear)
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appclear1 {
    #[doc = "1: Clears the event for channel 1."]
    Clear = 1,
}
impl From<Appclear1> for bool {
    #[inline(always)]
    fn from(variant: Appclear1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_1` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type Appclear1W<'a, REG> = crate::BitWriter<'a, REG, Appclear1>;
impl<'a, REG> Appclear1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the event for channel 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Appclear1::Clear)
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appclear2 {
    #[doc = "1: Clears the event for channel 2."]
    Clear = 1,
}
impl From<Appclear2> for bool {
    #[inline(always)]
    fn from(variant: Appclear2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_2` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type Appclear2W<'a, REG> = crate::BitWriter<'a, REG, Appclear2>;
impl<'a, REG> Appclear2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the event for channel 2."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Appclear2::Clear)
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appclear3 {
    #[doc = "1: Clears the event for channel 3."]
    Clear = 1,
}
impl From<Appclear3> for bool {
    #[inline(always)]
    fn from(variant: Appclear3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_3` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub type Appclear3W<'a, REG> = crate::BitWriter<'a, REG, Appclear3>;
impl<'a, REG> Appclear3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the event for channel 3."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Appclear3::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_0(&mut self) -> Appclear0W<'_, CtiappclearSpec> {
        Appclear0W::new(self, 0)
    }
    #[doc = "Bit 1 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_1(&mut self) -> Appclear1W<'_, CtiappclearSpec> {
        Appclear1W::new(self, 1)
    }
    #[doc = "Bit 2 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_2(&mut self) -> Appclear2W<'_, CtiappclearSpec> {
        Appclear2W::new(self, 2)
    }
    #[doc = "Bit 3 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_3(&mut self) -> Appclear3W<'_, CtiappclearSpec> {
        Appclear3W::new(self, 3)
    }
}
#[doc = "CTI Application Trigger Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctiappclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtiappclearSpec;
impl crate::RegisterSpec for CtiappclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctiappclear::W`](W) writer structure"]
impl crate::Writable for CtiappclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIAPPCLEAR to value 0"]
impl crate::Resettable for CtiappclearSpec {}
