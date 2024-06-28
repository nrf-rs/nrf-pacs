#[doc = "Register `PTR` reader"]
pub type R = crate::R<PtrSpec>;
#[doc = "Register `PTR` writer"]
pub type W = crate::W<PtrSpec>;
#[doc = "Field `PTR` reader - Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address."]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address."]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive buffer Data RAM start address. When receiving, words containing samples will be written to this address. This address is a word aligned Data RAM address."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<PtrSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "Receive buffer RAM start address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtrSpec;
impl crate::RegisterSpec for PtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptr::R`](R) reader structure"]
impl crate::Readable for PtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ptr::W`](W) writer structure"]
impl crate::Writable for PtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PtrSpec {
    const RESET_VALUE: u32 = 0;
}
