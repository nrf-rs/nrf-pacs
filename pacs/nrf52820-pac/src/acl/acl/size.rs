#[doc = "Register `SIZE` reader"]
pub type R = crate::R<SizeSpec>;
#[doc = "Register `SIZE` writer"]
pub type W = crate::W<SizeSpec>;
#[doc = "Field `SIZE` reader - Size of flash region n in bytes. Must be a multiple of the flash page size."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - Size of flash region n in bytes. Must be a multiple of the flash page size."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Size of flash region n in bytes. Must be a multiple of the flash page size."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of flash region n in bytes. Must be a multiple of the flash page size."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, SizeSpec> {
        SizeW::new(self, 0)
    }
}
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeSpec;
impl crate::RegisterSpec for SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`size::W`](W) writer structure"]
impl crate::Writable for SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIZE to value 0"]
impl crate::Resettable for SizeSpec {}
