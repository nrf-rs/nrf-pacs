#[doc = "Register `HFXODEBOUNCE` reader"]
pub type R = crate::R<HfxodebounceSpec>;
#[doc = "Register `HFXODEBOUNCE` writer"]
pub type W = crate::W<HfxodebounceSpec>;
#[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxodebounce {
    #[doc = "16: 256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    Db256us = 16,
    #[doc = "64: 1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    Db1024us = 64,
}
impl From<Hfxodebounce> for u8 {
    #[inline(always)]
    fn from(variant: Hfxodebounce) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxodebounce {
    type Ux = u8;
}
impl crate::IsEnum for Hfxodebounce {}
#[doc = "Field `HFXODEBOUNCE` reader - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub type HfxodebounceR = crate::FieldReader<Hfxodebounce>;
impl HfxodebounceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfxodebounce> {
        match self.bits {
            16 => Some(Hfxodebounce::Db256us),
            64 => Some(Hfxodebounce::Db1024us),
            _ => None,
        }
    }
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    #[inline(always)]
    pub fn is_db256us(&self) -> bool {
        *self == Hfxodebounce::Db256us
    }
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    #[inline(always)]
    pub fn is_db1024us(&self) -> bool {
        *self == Hfxodebounce::Db1024us
    }
}
#[doc = "Field `HFXODEBOUNCE` writer - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
pub type HfxodebounceW<'a, REG> = crate::FieldWriter<'a, REG, 8, Hfxodebounce>;
impl<'a, REG> HfxodebounceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    #[inline(always)]
    pub fn db256us(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxodebounce::Db256us)
    }
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    #[inline(always)]
    pub fn db1024us(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxodebounce::Db1024us)
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&self) -> HfxodebounceR {
        HfxodebounceR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn hfxodebounce(&mut self) -> HfxodebounceW<'_, HfxodebounceSpec> {
        HfxodebounceW::new(self, 0)
    }
}
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxodebounce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxodebounce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxodebounceSpec;
impl crate::RegisterSpec for HfxodebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxodebounce::R`](R) reader structure"]
impl crate::Readable for HfxodebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxodebounce::W`](W) writer structure"]
impl crate::Writable for HfxodebounceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXODEBOUNCE to value 0x10"]
impl crate::Resettable for HfxodebounceSpec {
    const RESET_VALUE: u32 = 0x10;
}
