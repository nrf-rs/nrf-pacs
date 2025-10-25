#[doc = "Register `AHBM_HNONSEC` reader"]
pub type R = crate::R<AhbmHnonsecSpec>;
#[doc = "Register `AHBM_HNONSEC` writer"]
pub type W = crate::W<AhbmHnonsecSpec>;
#[doc = "Field `AHB_WRITE_HNONSEC` reader - The AHB HNONSEC value for write transaction."]
pub type AhbWriteHnonsecR = crate::BitReader;
#[doc = "Field `AHB_WRITE_HNONSEC` writer - The AHB HNONSEC value for write transaction."]
pub type AhbWriteHnonsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_READ_HNONSEC` reader - The AHB HNONSEC value for read transaction."]
pub type AhbReadHnonsecR = crate::BitReader;
#[doc = "Field `AHB_READ_HNONSEC` writer - The AHB HNONSEC value for read transaction."]
pub type AhbReadHnonsecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The AHB HNONSEC value for write transaction."]
    #[inline(always)]
    pub fn ahb_write_hnonsec(&self) -> AhbWriteHnonsecR {
        AhbWriteHnonsecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The AHB HNONSEC value for read transaction."]
    #[inline(always)]
    pub fn ahb_read_hnonsec(&self) -> AhbReadHnonsecR {
        AhbReadHnonsecR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The AHB HNONSEC value for write transaction."]
    #[inline(always)]
    pub fn ahb_write_hnonsec(&mut self) -> AhbWriteHnonsecW<'_, AhbmHnonsecSpec> {
        AhbWriteHnonsecW::new(self, 0)
    }
    #[doc = "Bit 1 - The AHB HNONSEC value for read transaction."]
    #[inline(always)]
    pub fn ahb_read_hnonsec(&mut self) -> AhbReadHnonsecW<'_, AhbmHnonsecSpec> {
        AhbReadHnonsecW::new(self, 1)
    }
}
#[doc = "This register holds AHB HNONSEC value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hnonsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hnonsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmHnonsecSpec;
impl crate::RegisterSpec for AhbmHnonsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbm_hnonsec::R`](R) reader structure"]
impl crate::Readable for AhbmHnonsecSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbm_hnonsec::W`](W) writer structure"]
impl crate::Writable for AhbmHnonsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBM_HNONSEC to value 0"]
impl crate::Resettable for AhbmHnonsecSpec {}
