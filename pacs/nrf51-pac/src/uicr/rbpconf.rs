#[doc = "Register `RBPCONF` reader"]
pub type R = crate::R<RbpconfSpec>;
#[doc = "Register `RBPCONF` writer"]
pub type W = crate::W<RbpconfSpec>;
#[doc = "Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pr0 {
    #[doc = "255: Disabled."]
    Disabled = 255,
    #[doc = "0: Enabled."]
    Enabled = 0,
}
impl From<Pr0> for u8 {
    #[inline(always)]
    fn from(variant: Pr0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pr0 {
    type Ux = u8;
}
impl crate::IsEnum for Pr0 {}
#[doc = "Field `PR0` reader - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub type Pr0R = crate::FieldReader<Pr0>;
impl Pr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pr0> {
        match self.bits {
            255 => Some(Pr0::Disabled),
            0 => Some(Pr0::Enabled),
            _ => None,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pr0::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pr0::Enabled
    }
}
#[doc = "Field `PR0` writer - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
pub type Pr0W<'a, REG> = crate::FieldWriter<'a, REG, 8, Pr0>;
impl<'a, REG> Pr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pr0::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pr0::Enabled)
    }
}
#[doc = "Readback protect all code in the device.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pall {
    #[doc = "255: Disabled."]
    Disabled = 255,
    #[doc = "0: Enabled."]
    Enabled = 0,
}
impl From<Pall> for u8 {
    #[inline(always)]
    fn from(variant: Pall) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pall {
    type Ux = u8;
}
impl crate::IsEnum for Pall {}
#[doc = "Field `PALL` reader - Readback protect all code in the device."]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            255 => Some(Pall::Disabled),
            0 => Some(Pall::Enabled),
            _ => None,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pall::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pall::Enabled
    }
}
#[doc = "Field `PALL` writer - Readback protect all code in the device."]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 8, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline(always)]
    pub fn pr0(&self) -> Pr0R {
        Pr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip."]
    #[inline(always)]
    pub fn pr0(&mut self) -> Pr0W<'_, RbpconfSpec> {
        Pr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Readback protect all code in the device."]
    #[inline(always)]
    pub fn pall(&mut self) -> PallW<'_, RbpconfSpec> {
        PallW::new(self, 8)
    }
}
#[doc = "Readback protection configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbpconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbpconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbpconfSpec;
impl crate::RegisterSpec for RbpconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbpconf::R`](R) reader structure"]
impl crate::Readable for RbpconfSpec {}
#[doc = "`write(|w| ..)` method takes [`rbpconf::W`](W) writer structure"]
impl crate::Writable for RbpconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RBPCONF to value 0xffff_ffff"]
impl crate::Resettable for RbpconfSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
