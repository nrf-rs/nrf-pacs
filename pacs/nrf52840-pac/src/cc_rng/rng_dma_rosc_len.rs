#[doc = "Register `RNG_DMA_ROSC_LEN` reader"]
pub type R = crate::R<RngDmaRoscLenSpec>;
#[doc = "Register `RNG_DMA_ROSC_LEN` writer"]
pub type W = crate::W<RngDmaRoscLenSpec>;
#[doc = "Use shortest ROSC1 ring oscillator configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rosc1 {
    #[doc = "0: Disable ROSC1"]
    Disable = 0,
    #[doc = "1: Enable ROSC1"]
    Enable = 1,
}
impl From<Rosc1> for bool {
    #[inline(always)]
    fn from(variant: Rosc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROSC1` reader - Use shortest ROSC1 ring oscillator configuration."]
pub type Rosc1R = crate::BitReader<Rosc1>;
impl Rosc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rosc1 {
        match self.bits {
            false => Rosc1::Disable,
            true => Rosc1::Enable,
        }
    }
    #[doc = "Disable ROSC1"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rosc1::Disable
    }
    #[doc = "Enable ROSC1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rosc1::Enable
    }
}
#[doc = "Field `ROSC1` writer - Use shortest ROSC1 ring oscillator configuration."]
pub type Rosc1W<'a, REG> = crate::BitWriter<'a, REG, Rosc1>;
impl<'a, REG> Rosc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ROSC1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc1::Disable)
    }
    #[doc = "Enable ROSC1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc1::Enable)
    }
}
#[doc = "Use ROSC2 ring oscillator configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rosc2 {
    #[doc = "0: Disable ROSC2"]
    Disable = 0,
    #[doc = "1: Enable ROSC2"]
    Enable = 1,
}
impl From<Rosc2> for bool {
    #[inline(always)]
    fn from(variant: Rosc2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROSC2` reader - Use ROSC2 ring oscillator configuration."]
pub type Rosc2R = crate::BitReader<Rosc2>;
impl Rosc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rosc2 {
        match self.bits {
            false => Rosc2::Disable,
            true => Rosc2::Enable,
        }
    }
    #[doc = "Disable ROSC2"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rosc2::Disable
    }
    #[doc = "Enable ROSC2"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rosc2::Enable
    }
}
#[doc = "Field `ROSC2` writer - Use ROSC2 ring oscillator configuration."]
pub type Rosc2W<'a, REG> = crate::BitWriter<'a, REG, Rosc2>;
impl<'a, REG> Rosc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ROSC2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc2::Disable)
    }
    #[doc = "Enable ROSC2"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc2::Enable)
    }
}
#[doc = "Use ROSC3 ring oscillator configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rosc3 {
    #[doc = "0: Disable ROSC3"]
    Disable = 0,
    #[doc = "1: Enable ROSC3"]
    Enable = 1,
}
impl From<Rosc3> for bool {
    #[inline(always)]
    fn from(variant: Rosc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROSC3` reader - Use ROSC3 ring oscillator configuration."]
pub type Rosc3R = crate::BitReader<Rosc3>;
impl Rosc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rosc3 {
        match self.bits {
            false => Rosc3::Disable,
            true => Rosc3::Enable,
        }
    }
    #[doc = "Disable ROSC3"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rosc3::Disable
    }
    #[doc = "Enable ROSC3"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rosc3::Enable
    }
}
#[doc = "Field `ROSC3` writer - Use ROSC3 ring oscillator configuration."]
pub type Rosc3W<'a, REG> = crate::BitWriter<'a, REG, Rosc3>;
impl<'a, REG> Rosc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ROSC3"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc3::Disable)
    }
    #[doc = "Enable ROSC3"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc3::Enable)
    }
}
#[doc = "Use longest ROSC4 ring oscillator configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rosc4 {
    #[doc = "0: Disable ROSC4"]
    Disable = 0,
    #[doc = "1: Enable ROSC4"]
    Enable = 1,
}
impl From<Rosc4> for bool {
    #[inline(always)]
    fn from(variant: Rosc4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROSC4` reader - Use longest ROSC4 ring oscillator configuration."]
pub type Rosc4R = crate::BitReader<Rosc4>;
impl Rosc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rosc4 {
        match self.bits {
            false => Rosc4::Disable,
            true => Rosc4::Enable,
        }
    }
    #[doc = "Disable ROSC4"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rosc4::Disable
    }
    #[doc = "Enable ROSC4"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rosc4::Enable
    }
}
#[doc = "Field `ROSC4` writer - Use longest ROSC4 ring oscillator configuration."]
pub type Rosc4W<'a, REG> = crate::BitWriter<'a, REG, Rosc4>;
impl<'a, REG> Rosc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ROSC4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc4::Disable)
    }
    #[doc = "Enable ROSC4"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rosc4::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Use shortest ROSC1 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc1(&self) -> Rosc1R {
        Rosc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use ROSC2 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc2(&self) -> Rosc2R {
        Rosc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Use ROSC3 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc3(&self) -> Rosc3R {
        Rosc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use longest ROSC4 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc4(&self) -> Rosc4R {
        Rosc4R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use shortest ROSC1 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc1(&mut self) -> Rosc1W<'_, RngDmaRoscLenSpec> {
        Rosc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Use ROSC2 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc2(&mut self) -> Rosc2W<'_, RngDmaRoscLenSpec> {
        Rosc2W::new(self, 1)
    }
    #[doc = "Bit 2 - Use ROSC3 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc3(&mut self) -> Rosc3W<'_, RngDmaRoscLenSpec> {
        Rosc3W::new(self, 2)
    }
    #[doc = "Bit 3 - Use longest ROSC4 ring oscillator configuration."]
    #[inline(always)]
    pub fn rosc4(&mut self) -> Rosc4W<'_, RngDmaRoscLenSpec> {
        Rosc4W::new(self, 3)
    }
}
#[doc = "This register defines which ring oscillator length configuration should be used when using the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_rosc_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_rosc_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDmaRoscLenSpec;
impl crate::RegisterSpec for RngDmaRoscLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dma_rosc_len::R`](R) reader structure"]
impl crate::Readable for RngDmaRoscLenSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_dma_rosc_len::W`](W) writer structure"]
impl crate::Writable for RngDmaRoscLenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_DMA_ROSC_LEN to value 0"]
impl crate::Resettable for RngDmaRoscLenSpec {}
