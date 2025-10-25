#[doc = "Register `RAMONB` reader"]
pub type R = crate::R<RamonbSpec>;
#[doc = "Register `RAMONB` writer"]
pub type W = crate::W<RamonbSpec>;
#[doc = "RAM block 2 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onram2 {
    #[doc = "0: RAM block 2 OFF in ON mode."]
    Ram2off = 0,
    #[doc = "1: RAM block 2 ON in ON mode."]
    Ram2on = 1,
}
impl From<Onram2> for bool {
    #[inline(always)]
    fn from(variant: Onram2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM2` reader - RAM block 2 behaviour in ON mode."]
pub type Onram2R = crate::BitReader<Onram2>;
impl Onram2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onram2 {
        match self.bits {
            false => Onram2::Ram2off,
            true => Onram2::Ram2on,
        }
    }
    #[doc = "RAM block 2 OFF in ON mode."]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == Onram2::Ram2off
    }
    #[doc = "RAM block 2 ON in ON mode."]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == Onram2::Ram2on
    }
}
#[doc = "Field `ONRAM2` writer - RAM block 2 behaviour in ON mode."]
pub type Onram2W<'a, REG> = crate::BitWriter<'a, REG, Onram2>;
impl<'a, REG> Onram2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 2 OFF in ON mode."]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut crate::W<REG> {
        self.variant(Onram2::Ram2off)
    }
    #[doc = "RAM block 2 ON in ON mode."]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut crate::W<REG> {
        self.variant(Onram2::Ram2on)
    }
}
#[doc = "RAM block 3 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onram3 {
    #[doc = "0: RAM block 33 OFF in ON mode."]
    Ram3off = 0,
    #[doc = "1: RAM block 3 ON in ON mode."]
    Ram3on = 1,
}
impl From<Onram3> for bool {
    #[inline(always)]
    fn from(variant: Onram3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM3` reader - RAM block 3 behaviour in ON mode."]
pub type Onram3R = crate::BitReader<Onram3>;
impl Onram3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onram3 {
        match self.bits {
            false => Onram3::Ram3off,
            true => Onram3::Ram3on,
        }
    }
    #[doc = "RAM block 33 OFF in ON mode."]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == Onram3::Ram3off
    }
    #[doc = "RAM block 3 ON in ON mode."]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == Onram3::Ram3on
    }
}
#[doc = "Field `ONRAM3` writer - RAM block 3 behaviour in ON mode."]
pub type Onram3W<'a, REG> = crate::BitWriter<'a, REG, Onram3>;
impl<'a, REG> Onram3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 33 OFF in ON mode."]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut crate::W<REG> {
        self.variant(Onram3::Ram3off)
    }
    #[doc = "RAM block 3 ON in ON mode."]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut crate::W<REG> {
        self.variant(Onram3::Ram3on)
    }
}
#[doc = "RAM block 2 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offram2 {
    #[doc = "0: RAM block 2 OFF in OFF mode."]
    Ram2off = 0,
    #[doc = "1: RAM block 2 ON in OFF mode."]
    Ram2on = 1,
}
impl From<Offram2> for bool {
    #[inline(always)]
    fn from(variant: Offram2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM2` reader - RAM block 2 behaviour in OFF mode."]
pub type Offram2R = crate::BitReader<Offram2>;
impl Offram2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offram2 {
        match self.bits {
            false => Offram2::Ram2off,
            true => Offram2::Ram2on,
        }
    }
    #[doc = "RAM block 2 OFF in OFF mode."]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == Offram2::Ram2off
    }
    #[doc = "RAM block 2 ON in OFF mode."]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == Offram2::Ram2on
    }
}
#[doc = "Field `OFFRAM2` writer - RAM block 2 behaviour in OFF mode."]
pub type Offram2W<'a, REG> = crate::BitWriter<'a, REG, Offram2>;
impl<'a, REG> Offram2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 2 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut crate::W<REG> {
        self.variant(Offram2::Ram2off)
    }
    #[doc = "RAM block 2 ON in OFF mode."]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut crate::W<REG> {
        self.variant(Offram2::Ram2on)
    }
}
#[doc = "RAM block 3 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offram3 {
    #[doc = "0: RAM block 3 OFF in OFF mode."]
    Ram3off = 0,
    #[doc = "1: RAM block 3 ON in OFF mode."]
    Ram3on = 1,
}
impl From<Offram3> for bool {
    #[inline(always)]
    fn from(variant: Offram3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM3` reader - RAM block 3 behaviour in OFF mode."]
pub type Offram3R = crate::BitReader<Offram3>;
impl Offram3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offram3 {
        match self.bits {
            false => Offram3::Ram3off,
            true => Offram3::Ram3on,
        }
    }
    #[doc = "RAM block 3 OFF in OFF mode."]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == Offram3::Ram3off
    }
    #[doc = "RAM block 3 ON in OFF mode."]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == Offram3::Ram3on
    }
}
#[doc = "Field `OFFRAM3` writer - RAM block 3 behaviour in OFF mode."]
pub type Offram3W<'a, REG> = crate::BitWriter<'a, REG, Offram3>;
impl<'a, REG> Offram3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 3 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut crate::W<REG> {
        self.variant(Offram3::Ram3off)
    }
    #[doc = "RAM block 3 ON in OFF mode."]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut crate::W<REG> {
        self.variant(Offram3::Ram3on)
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram2(&self) -> Onram2R {
        Onram2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram3(&self) -> Onram3R {
        Onram3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram2(&self) -> Offram2R {
        Offram2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram3(&self) -> Offram3R {
        Offram3R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram2(&mut self) -> Onram2W<'_, RamonbSpec> {
        Onram2W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram3(&mut self) -> Onram3W<'_, RamonbSpec> {
        Onram3W::new(self, 1)
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram2(&mut self) -> Offram2W<'_, RamonbSpec> {
        Offram2W::new(self, 16)
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram3(&mut self) -> Offram3W<'_, RamonbSpec> {
        Offram3W::new(self, 17)
    }
}
#[doc = "Ram on/off.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramonb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramonb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamonbSpec;
impl crate::RegisterSpec for RamonbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramonb::R`](R) reader structure"]
impl crate::Readable for RamonbSpec {}
#[doc = "`write(|w| ..)` method takes [`ramonb::W`](W) writer structure"]
impl crate::Writable for RamonbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMONB to value 0x03"]
impl crate::Resettable for RamonbSpec {
    const RESET_VALUE: u32 = 0x03;
}
