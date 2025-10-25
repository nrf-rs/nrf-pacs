#[doc = "Register `AHBM_HPROT` reader"]
pub type R = crate::R<AhbmHprotSpec>;
#[doc = "Register `AHBM_HPROT` writer"]
pub type W = crate::W<AhbmHprotSpec>;
#[doc = "Field `AHB_HPROT` reader - The AHB HPROT value"]
pub type AhbHprotR = crate::FieldReader;
#[doc = "Field `AHB_HPROT` writer - The AHB HPROT value"]
pub type AhbHprotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The AHB HPROT value"]
    #[inline(always)]
    pub fn ahb_hprot(&self) -> AhbHprotR {
        AhbHprotR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The AHB HPROT value"]
    #[inline(always)]
    pub fn ahb_hprot(&mut self) -> AhbHprotW<'_, AhbmHprotSpec> {
        AhbHprotW::new(self, 0)
    }
}
#[doc = "This register holds the AHB HPROT value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmHprotSpec;
impl crate::RegisterSpec for AhbmHprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbm_hprot::R`](R) reader structure"]
impl crate::Readable for AhbmHprotSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbm_hprot::W`](W) writer structure"]
impl crate::Writable for AhbmHprotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBM_HPROT to value 0"]
impl crate::Resettable for AhbmHprotSpec {}
