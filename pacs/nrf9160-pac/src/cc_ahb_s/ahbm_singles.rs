#[doc = "Register `AHBM_SINGLES` reader"]
pub type R = crate::R<AhbmSinglesSpec>;
#[doc = "Register `AHBM_SINGLES` writer"]
pub type W = crate::W<AhbmSinglesSpec>;
#[doc = "Field `AHB_SINGLES` reader - Force AHB singles"]
pub type AhbSinglesR = crate::BitReader;
#[doc = "Field `AHB_SINGLES` writer - Force AHB singles"]
pub type AhbSinglesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force AHB singles"]
    #[inline(always)]
    pub fn ahb_singles(&self) -> AhbSinglesR {
        AhbSinglesR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force AHB singles"]
    #[inline(always)]
    pub fn ahb_singles(&mut self) -> AhbSinglesW<'_, AhbmSinglesSpec> {
        AhbSinglesW::new(self, 0)
    }
}
#[doc = "This register forces the AHB transactions from CRYPTOCELL master to be always singles.\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_singles::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_singles::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmSinglesSpec;
impl crate::RegisterSpec for AhbmSinglesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbm_singles::R`](R) reader structure"]
impl crate::Readable for AhbmSinglesSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbm_singles::W`](W) writer structure"]
impl crate::Writable for AhbmSinglesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBM_SINGLES to value 0"]
impl crate::Resettable for AhbmSinglesSpec {}
