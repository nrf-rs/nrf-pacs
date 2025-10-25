#[doc = "Register `CRCPOLY` reader"]
pub type R = crate::R<CrcpolySpec>;
#[doc = "Register `CRCPOLY` writer"]
pub type W = crate::W<CrcpolySpec>;
#[doc = "Field `CRCPOLY` reader - CRC polynomial"]
pub type CrcpolyR = crate::FieldReader<u32>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial"]
pub type CrcpolyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CRC polynomial"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CrcpolyR {
        CrcpolyR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CRC polynomial"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CrcpolyW<'_, CrcpolySpec> {
        CrcpolyW::new(self, 0)
    }
}
#[doc = "CRC polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcpolySpec;
impl crate::RegisterSpec for CrcpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcpoly::R`](R) reader structure"]
impl crate::Readable for CrcpolySpec {}
#[doc = "`write(|w| ..)` method takes [`crcpoly::W`](W) writer structure"]
impl crate::Writable for CrcpolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCPOLY to value 0"]
impl crate::Resettable for CrcpolySpec {}
