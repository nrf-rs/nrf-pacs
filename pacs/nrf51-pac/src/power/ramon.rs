#[doc = "Register `RAMON` reader"]
pub type R = crate::R<RamonSpec>;
#[doc = "Register `RAMON` writer"]
pub type W = crate::W<RamonSpec>;
#[doc = "RAM block 0 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onram0 {
    #[doc = "0: RAM block 0 OFF in ON mode."]
    Ram0off = 0,
    #[doc = "1: RAM block 0 ON in ON mode."]
    Ram0on = 1,
}
impl From<Onram0> for bool {
    #[inline(always)]
    fn from(variant: Onram0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM0` reader - RAM block 0 behaviour in ON mode."]
pub type Onram0R = crate::BitReader<Onram0>;
impl Onram0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onram0 {
        match self.bits {
            false => Onram0::Ram0off,
            true => Onram0::Ram0on,
        }
    }
    #[doc = "RAM block 0 OFF in ON mode."]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == Onram0::Ram0off
    }
    #[doc = "RAM block 0 ON in ON mode."]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == Onram0::Ram0on
    }
}
#[doc = "Field `ONRAM0` writer - RAM block 0 behaviour in ON mode."]
pub type Onram0W<'a, REG> = crate::BitWriter<'a, REG, Onram0>;
impl<'a, REG> Onram0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 0 OFF in ON mode."]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut crate::W<REG> {
        self.variant(Onram0::Ram0off)
    }
    #[doc = "RAM block 0 ON in ON mode."]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut crate::W<REG> {
        self.variant(Onram0::Ram0on)
    }
}
#[doc = "RAM block 1 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onram1 {
    #[doc = "0: RAM block 1 OFF in ON mode."]
    Ram1off = 0,
    #[doc = "1: RAM block 1 ON in ON mode."]
    Ram1on = 1,
}
impl From<Onram1> for bool {
    #[inline(always)]
    fn from(variant: Onram1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONRAM1` reader - RAM block 1 behaviour in ON mode."]
pub type Onram1R = crate::BitReader<Onram1>;
impl Onram1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onram1 {
        match self.bits {
            false => Onram1::Ram1off,
            true => Onram1::Ram1on,
        }
    }
    #[doc = "RAM block 1 OFF in ON mode."]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == Onram1::Ram1off
    }
    #[doc = "RAM block 1 ON in ON mode."]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == Onram1::Ram1on
    }
}
#[doc = "Field `ONRAM1` writer - RAM block 1 behaviour in ON mode."]
pub type Onram1W<'a, REG> = crate::BitWriter<'a, REG, Onram1>;
impl<'a, REG> Onram1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 1 OFF in ON mode."]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut crate::W<REG> {
        self.variant(Onram1::Ram1off)
    }
    #[doc = "RAM block 1 ON in ON mode."]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut crate::W<REG> {
        self.variant(Onram1::Ram1on)
    }
}
#[doc = "RAM block 0 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offram0 {
    #[doc = "0: RAM block 0 OFF in OFF mode."]
    Ram0off = 0,
    #[doc = "1: RAM block 0 ON in OFF mode."]
    Ram0on = 1,
}
impl From<Offram0> for bool {
    #[inline(always)]
    fn from(variant: Offram0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM0` reader - RAM block 0 behaviour in OFF mode."]
pub type Offram0R = crate::BitReader<Offram0>;
impl Offram0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offram0 {
        match self.bits {
            false => Offram0::Ram0off,
            true => Offram0::Ram0on,
        }
    }
    #[doc = "RAM block 0 OFF in OFF mode."]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == Offram0::Ram0off
    }
    #[doc = "RAM block 0 ON in OFF mode."]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == Offram0::Ram0on
    }
}
#[doc = "Field `OFFRAM0` writer - RAM block 0 behaviour in OFF mode."]
pub type Offram0W<'a, REG> = crate::BitWriter<'a, REG, Offram0>;
impl<'a, REG> Offram0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 0 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut crate::W<REG> {
        self.variant(Offram0::Ram0off)
    }
    #[doc = "RAM block 0 ON in OFF mode."]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut crate::W<REG> {
        self.variant(Offram0::Ram0on)
    }
}
#[doc = "RAM block 1 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offram1 {
    #[doc = "0: RAM block 1 OFF in OFF mode."]
    Ram1off = 0,
    #[doc = "1: RAM block 1 ON in OFF mode."]
    Ram1on = 1,
}
impl From<Offram1> for bool {
    #[inline(always)]
    fn from(variant: Offram1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFRAM1` reader - RAM block 1 behaviour in OFF mode."]
pub type Offram1R = crate::BitReader<Offram1>;
impl Offram1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offram1 {
        match self.bits {
            false => Offram1::Ram1off,
            true => Offram1::Ram1on,
        }
    }
    #[doc = "RAM block 1 OFF in OFF mode."]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == Offram1::Ram1off
    }
    #[doc = "RAM block 1 ON in OFF mode."]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == Offram1::Ram1on
    }
}
#[doc = "Field `OFFRAM1` writer - RAM block 1 behaviour in OFF mode."]
pub type Offram1W<'a, REG> = crate::BitWriter<'a, REG, Offram1>;
impl<'a, REG> Offram1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAM block 1 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut crate::W<REG> {
        self.variant(Offram1::Ram1off)
    }
    #[doc = "RAM block 1 ON in OFF mode."]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut crate::W<REG> {
        self.variant(Offram1::Ram1on)
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 0 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram0(&self) -> Onram0R {
        Onram0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM block 1 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram1(&self) -> Onram1R {
        Onram1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM block 0 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram0(&self) -> Offram0R {
        Offram0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM block 1 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram1(&self) -> Offram1R {
        Offram1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM block 0 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram0(&mut self) -> Onram0W<'_, RamonSpec> {
        Onram0W::new(self, 0)
    }
    #[doc = "Bit 1 - RAM block 1 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram1(&mut self) -> Onram1W<'_, RamonSpec> {
        Onram1W::new(self, 1)
    }
    #[doc = "Bit 16 - RAM block 0 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram0(&mut self) -> Offram0W<'_, RamonSpec> {
        Offram0W::new(self, 16)
    }
    #[doc = "Bit 17 - RAM block 1 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram1(&mut self) -> Offram1W<'_, RamonSpec> {
        Offram1W::new(self, 17)
    }
}
#[doc = "Ram on/off.\n\nYou can [`read`](crate::Reg::read) this register and get [`ramon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ramon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamonSpec;
impl crate::RegisterSpec for RamonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramon::R`](R) reader structure"]
impl crate::Readable for RamonSpec {}
#[doc = "`write(|w| ..)` method takes [`ramon::W`](W) writer structure"]
impl crate::Writable for RamonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMON to value 0x03"]
impl crate::Resettable for RamonSpec {
    const RESET_VALUE: u32 = 0x03;
}
