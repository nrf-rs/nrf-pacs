#[doc = "Register `CTIGATE` reader"]
pub type R = crate::R<CtigateSpec>;
#[doc = "Register `CTIGATE` writer"]
pub type W = crate::W<CtigateSpec>;
#[doc = "Enable ctichout0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctigateen0 {
    #[doc = "1: Enable ctichout channel 0 propagation."]
    Enabled = 1,
    #[doc = "0: Disable ctichout channel 0 propagation."]
    Disabled = 0,
}
impl From<Ctigateen0> for bool {
    #[inline(always)]
    fn from(variant: Ctigateen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_0` reader - Enable ctichout0."]
pub type Ctigateen0R = crate::BitReader<Ctigateen0>;
impl Ctigateen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctigateen0 {
        match self.bits {
            true => Ctigateen0::Enabled,
            false => Ctigateen0::Disabled,
        }
    }
    #[doc = "Enable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctigateen0::Enabled
    }
    #[doc = "Disable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctigateen0::Disabled
    }
}
#[doc = "Field `CTIGATEEN_0` writer - Enable ctichout0."]
pub type Ctigateen0W<'a, REG> = crate::BitWriter<'a, REG, Ctigateen0>;
impl<'a, REG> Ctigateen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen0::Enabled)
    }
    #[doc = "Disable ctichout channel 0 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen0::Disabled)
    }
}
#[doc = "Enable ctichout1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctigateen1 {
    #[doc = "1: Enable ctichout channel 1 propagation."]
    Enabled = 1,
    #[doc = "0: Disable ctichout channel 1 propagation."]
    Disabled = 0,
}
impl From<Ctigateen1> for bool {
    #[inline(always)]
    fn from(variant: Ctigateen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_1` reader - Enable ctichout1."]
pub type Ctigateen1R = crate::BitReader<Ctigateen1>;
impl Ctigateen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctigateen1 {
        match self.bits {
            true => Ctigateen1::Enabled,
            false => Ctigateen1::Disabled,
        }
    }
    #[doc = "Enable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctigateen1::Enabled
    }
    #[doc = "Disable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctigateen1::Disabled
    }
}
#[doc = "Field `CTIGATEEN_1` writer - Enable ctichout1."]
pub type Ctigateen1W<'a, REG> = crate::BitWriter<'a, REG, Ctigateen1>;
impl<'a, REG> Ctigateen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen1::Enabled)
    }
    #[doc = "Disable ctichout channel 1 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen1::Disabled)
    }
}
#[doc = "Enable ctichout2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctigateen2 {
    #[doc = "1: Enable ctichout channel 2 propagation."]
    Enabled = 1,
    #[doc = "0: Disable ctichout channel 2 propagation."]
    Disabled = 0,
}
impl From<Ctigateen2> for bool {
    #[inline(always)]
    fn from(variant: Ctigateen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_2` reader - Enable ctichout2."]
pub type Ctigateen2R = crate::BitReader<Ctigateen2>;
impl Ctigateen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctigateen2 {
        match self.bits {
            true => Ctigateen2::Enabled,
            false => Ctigateen2::Disabled,
        }
    }
    #[doc = "Enable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctigateen2::Enabled
    }
    #[doc = "Disable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctigateen2::Disabled
    }
}
#[doc = "Field `CTIGATEEN_2` writer - Enable ctichout2."]
pub type Ctigateen2W<'a, REG> = crate::BitWriter<'a, REG, Ctigateen2>;
impl<'a, REG> Ctigateen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen2::Enabled)
    }
    #[doc = "Disable ctichout channel 2 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen2::Disabled)
    }
}
#[doc = "Enable ctichout3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctigateen3 {
    #[doc = "1: Enable ctichout channel 3 propagation."]
    Enabled = 1,
    #[doc = "0: Disable ctichout channel 3 propagation."]
    Disabled = 0,
}
impl From<Ctigateen3> for bool {
    #[inline(always)]
    fn from(variant: Ctigateen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIGATEEN_3` reader - Enable ctichout3."]
pub type Ctigateen3R = crate::BitReader<Ctigateen3>;
impl Ctigateen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctigateen3 {
        match self.bits {
            true => Ctigateen3::Enabled,
            false => Ctigateen3::Disabled,
        }
    }
    #[doc = "Enable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctigateen3::Enabled
    }
    #[doc = "Disable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctigateen3::Disabled
    }
}
#[doc = "Field `CTIGATEEN_3` writer - Enable ctichout3."]
pub type Ctigateen3W<'a, REG> = crate::BitWriter<'a, REG, Ctigateen3>;
impl<'a, REG> Ctigateen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen3::Enabled)
    }
    #[doc = "Disable ctichout channel 3 propagation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctigateen3::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&self) -> Ctigateen0R {
        Ctigateen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&self) -> Ctigateen1R {
        Ctigateen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&self) -> Ctigateen2R {
        Ctigateen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&self) -> Ctigateen3R {
        Ctigateen3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctichout0."]
    #[inline(always)]
    pub fn ctigateen_0(&mut self) -> Ctigateen0W<'_, CtigateSpec> {
        Ctigateen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ctichout1."]
    #[inline(always)]
    pub fn ctigateen_1(&mut self) -> Ctigateen1W<'_, CtigateSpec> {
        Ctigateen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable ctichout2."]
    #[inline(always)]
    pub fn ctigateen_2(&mut self) -> Ctigateen2W<'_, CtigateSpec> {
        Ctigateen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable ctichout3."]
    #[inline(always)]
    pub fn ctigateen_3(&mut self) -> Ctigateen3W<'_, CtigateSpec> {
        Ctigateen3W::new(self, 3)
    }
}
#[doc = "Enable CTI Channel Gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctigate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctigate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtigateSpec;
impl crate::RegisterSpec for CtigateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctigate::R`](R) reader structure"]
impl crate::Readable for CtigateSpec {}
#[doc = "`write(|w| ..)` method takes [`ctigate::W`](W) writer structure"]
impl crate::Writable for CtigateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIGATE to value 0x0f"]
impl crate::Resettable for CtigateSpec {
    const RESET_VALUE: u32 = 0x0f;
}
