#[doc = "Register `RAMSTATUS` reader"]
pub type R = crate::R<RamstatusSpec>;
#[doc = "RAM block 0 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock0 {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<Ramblock0> for bool {
    #[inline(always)]
    fn from(variant: Ramblock0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK0` reader - RAM block 0 is on or off/powering up"]
pub type Ramblock0R = crate::BitReader<Ramblock0>;
impl Ramblock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock0 {
        match self.bits {
            false => Ramblock0::Off,
            true => Ramblock0::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock0::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock0::On
    }
}
#[doc = "RAM block 1 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramblock1 {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<Ramblock1> for bool {
    #[inline(always)]
    fn from(variant: Ramblock1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMBLOCK1` reader - RAM block 1 is on or off/powering up"]
pub type Ramblock1R = crate::BitReader<Ramblock1>;
impl Ramblock1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramblock1 {
        match self.bits {
            false => Ramblock1::Off,
            true => Ramblock1::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramblock1::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ramblock1::On
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 0 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock0(&self) -> Ramblock0R {
        Ramblock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM block 1 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock1(&self) -> Ramblock1R {
        Ramblock1R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Deprecated register - RAM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ramstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamstatusSpec;
impl crate::RegisterSpec for RamstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ramstatus::R`](R) reader structure"]
impl crate::Readable for RamstatusSpec {}
#[doc = "`reset()` method sets RAMSTATUS to value 0"]
impl crate::Resettable for RamstatusSpec {}
