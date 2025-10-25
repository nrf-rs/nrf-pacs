#[doc = "Register `MAXRX` reader"]
pub type R = crate::R<MaxrxSpec>;
#[doc = "Register `MAXRX` writer"]
pub type W = crate::W<MaxrxSpec>;
#[doc = "Field `MAXRX` reader - Maximum number of bytes in the receive buffer."]
pub type MaxrxR = crate::FieldReader;
#[doc = "Field `MAXRX` writer - Maximum number of bytes in the receive buffer."]
pub type MaxrxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&self) -> MaxrxR {
        MaxrxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&mut self) -> MaxrxW<'_, MaxrxSpec> {
        MaxrxW::new(self, 0)
    }
}
#[doc = "Maximum number of bytes in the receive buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxrxSpec;
impl crate::RegisterSpec for MaxrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxrx::R`](R) reader structure"]
impl crate::Readable for MaxrxSpec {}
#[doc = "`write(|w| ..)` method takes [`maxrx::W`](W) writer structure"]
impl crate::Writable for MaxrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXRX to value 0"]
impl crate::Resettable for MaxrxSpec {}
