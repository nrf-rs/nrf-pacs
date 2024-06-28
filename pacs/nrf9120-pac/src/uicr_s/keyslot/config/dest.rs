#[doc = "Register `DEST` reader"]
pub type R = crate::R<DestSpec>;
#[doc = "Register `DEST` writer"]
pub type W = crate::W<DestSpec>;
#[doc = "Field `DEST` reader - Secure APB destination address"]
pub type DestR = crate::FieldReader<u32>;
#[doc = "Field `DEST` writer - Secure APB destination address"]
pub type DestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    pub fn dest(&self) -> DestR {
        DestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    #[must_use]
    pub fn dest(&mut self) -> DestW<DestSpec> {
        DestW::new(self, 0)
    }
}
#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address must match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DestSpec;
impl crate::RegisterSpec for DestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dest::R`](R) reader structure"]
impl crate::Readable for DestSpec {}
#[doc = "`write(|w| ..)` method takes [`dest::W`](W) writer structure"]
impl crate::Writable for DestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEST to value 0xffff_ffff"]
impl crate::Resettable for DestSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
