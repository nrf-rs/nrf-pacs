#[doc = "Register `NFCID1_LAST` reader"]
pub type R = crate::R<Nfcid1LastSpec>;
#[doc = "Register `NFCID1_LAST` writer"]
pub type W = crate::W<Nfcid1LastSpec>;
#[doc = "Field `NFCID1_Z` reader - NFCID1 byte Z (very last byte sent)"]
pub type Nfcid1ZR = crate::FieldReader;
#[doc = "Field `NFCID1_Z` writer - NFCID1 byte Z (very last byte sent)"]
pub type Nfcid1ZW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_Y` reader - NFCID1 byte Y"]
pub type Nfcid1YR = crate::FieldReader;
#[doc = "Field `NFCID1_Y` writer - NFCID1 byte Y"]
pub type Nfcid1YW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_X` reader - NFCID1 byte X"]
pub type Nfcid1XR = crate::FieldReader;
#[doc = "Field `NFCID1_X` writer - NFCID1 byte X"]
pub type Nfcid1XW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NFCID1_W` reader - NFCID1 byte W"]
pub type Nfcid1WR = crate::FieldReader;
#[doc = "Field `NFCID1_W` writer - NFCID1 byte W"]
pub type Nfcid1WW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&self) -> Nfcid1ZR {
        Nfcid1ZR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&self) -> Nfcid1YR {
        Nfcid1YR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&self) -> Nfcid1XR {
        Nfcid1XR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&self) -> Nfcid1WR {
        Nfcid1WR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&mut self) -> Nfcid1ZW<'_, Nfcid1LastSpec> {
        Nfcid1ZW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&mut self) -> Nfcid1YW<'_, Nfcid1LastSpec> {
        Nfcid1YW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&mut self) -> Nfcid1XW<'_, Nfcid1LastSpec> {
        Nfcid1XW::new(self, 16)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&mut self) -> Nfcid1WW<'_, Nfcid1LastSpec> {
        Nfcid1WW::new(self, 24)
    }
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcid1_last::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcid1_last::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nfcid1LastSpec;
impl crate::RegisterSpec for Nfcid1LastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcid1_last::R`](R) reader structure"]
impl crate::Readable for Nfcid1LastSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcid1_last::W`](W) writer structure"]
impl crate::Writable for Nfcid1LastSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NFCID1_LAST to value 0x6363"]
impl crate::Resettable for Nfcid1LastSpec {
    const RESET_VALUE: u32 = 0x6363;
}
