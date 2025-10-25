#[doc = "Register `BCC` reader"]
pub type R = crate::R<BccSpec>;
#[doc = "Register `BCC` writer"]
pub type W = crate::W<BccSpec>;
#[doc = "Field `BCC` reader - Bit counter compare"]
pub type BccR = crate::FieldReader<u32>;
#[doc = "Field `BCC` writer - Bit counter compare"]
pub type BccW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    pub fn bcc(&self) -> BccR {
        BccR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    pub fn bcc(&mut self) -> BccW<'_, BccSpec> {
        BccW::new(self, 0)
    }
}
#[doc = "Bit counter compare\n\nYou can [`read`](crate::Reg::read) this register and get [`bcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BccSpec;
impl crate::RegisterSpec for BccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcc::R`](R) reader structure"]
impl crate::Readable for BccSpec {}
#[doc = "`write(|w| ..)` method takes [`bcc::W`](W) writer structure"]
impl crate::Writable for BccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCC to value 0"]
impl crate::Resettable for BccSpec {}
