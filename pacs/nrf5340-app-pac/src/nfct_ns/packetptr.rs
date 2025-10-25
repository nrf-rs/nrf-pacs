#[doc = "Register `PACKETPTR` reader"]
pub type R = crate::R<PacketptrSpec>;
#[doc = "Register `PACKETPTR` writer"]
pub type W = crate::W<PacketptrSpec>;
#[doc = "Field `PTR` reader - Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte-aligned RAM address."]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte-aligned RAM address."]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte-aligned RAM address."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet pointer for TXD and RXD data storage in Data RAM. This address is a byte-aligned RAM address."]
    #[inline(always)]
    pub fn ptr(&mut self) -> PtrW<'_, PacketptrSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "Packet pointer for TXD and RXD data storage in Data RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketptrSpec;
impl crate::RegisterSpec for PacketptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packetptr::R`](R) reader structure"]
impl crate::Readable for PacketptrSpec {}
#[doc = "`write(|w| ..)` method takes [`packetptr::W`](W) writer structure"]
impl crate::Writable for PacketptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PACKETPTR to value 0"]
impl crate::Resettable for PacketptrSpec {}
