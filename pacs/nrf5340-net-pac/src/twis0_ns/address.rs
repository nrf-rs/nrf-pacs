#[doc = "Register `ADDRESS[%s]` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Register `ADDRESS[%s]` writer"]
pub type W = crate::W<AddressSpec>;
#[doc = "Field `ADDRESS` reader - TWI slave address"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - TWI slave address"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - TWI slave address"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TWI slave address"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, AddressSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Description collection: TWI slave address n\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for AddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRESS[%s] to value 0"]
impl crate::Resettable for AddressSpec {}
