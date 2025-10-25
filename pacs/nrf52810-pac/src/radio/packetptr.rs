#[doc = "Register `PACKETPTR` reader"]
pub type R = crate::R<PacketptrSpec>;
#[doc = "Register `PACKETPTR` writer"]
pub type W = crate::W<PacketptrSpec>;
#[doc = "Field `PACKETPTR` reader - Packet pointer"]
pub type PacketptrR = crate::FieldReader<u32>;
#[doc = "Field `PACKETPTR` writer - Packet pointer"]
pub type PacketptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&self) -> PacketptrR {
        PacketptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&mut self) -> PacketptrW<'_, PacketptrSpec> {
        PacketptrW::new(self, 0)
    }
}
#[doc = "Packet pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
