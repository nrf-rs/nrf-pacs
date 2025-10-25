#[doc = "Register `MAXTX` reader"]
pub type R = crate::R<MaxtxSpec>;
#[doc = "Register `MAXTX` writer"]
pub type W = crate::W<MaxtxSpec>;
#[doc = "Field `MAXTX` reader - Maximum number of bytes in the transmit buffer."]
pub type MaxtxR = crate::FieldReader;
#[doc = "Field `MAXTX` writer - Maximum number of bytes in the transmit buffer."]
pub type MaxtxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&self) -> MaxtxR {
        MaxtxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&mut self) -> MaxtxW<'_, MaxtxSpec> {
        MaxtxW::new(self, 0)
    }
}
#[doc = "Maximum number of bytes in the transmit buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxtx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxtx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxtxSpec;
impl crate::RegisterSpec for MaxtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxtx::R`](R) reader structure"]
impl crate::Readable for MaxtxSpec {}
#[doc = "`write(|w| ..)` method takes [`maxtx::W`](W) writer structure"]
impl crate::Writable for MaxtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXTX to value 0"]
impl crate::Resettable for MaxtxSpec {}
