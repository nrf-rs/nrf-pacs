#[doc = "Register `TINSTANCE` reader"]
pub type R = crate::R<TinstanceSpec>;
#[doc = "Register `TINSTANCE` writer"]
pub type W = crate::W<TinstanceSpec>;
#[doc = "Field `TINSTANCE` reader - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
pub type TinstanceR = crate::FieldReader;
#[doc = "Field `TINSTANCE` writer - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
pub type TinstanceW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&self) -> TinstanceR {
        TinstanceR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&mut self) -> TinstanceW<'_, TinstanceSpec> {
        TinstanceW::new(self, 28)
    }
}
#[doc = "SW-DP Target instance\n\nYou can [`read`](crate::Reg::read) this register and get [`tinstance::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tinstance::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TinstanceSpec;
impl crate::RegisterSpec for TinstanceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tinstance::R`](R) reader structure"]
impl crate::Readable for TinstanceSpec {}
#[doc = "`write(|w| ..)` method takes [`tinstance::W`](W) writer structure"]
impl crate::Writable for TinstanceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TINSTANCE to value 0xffff_ffff"]
impl crate::Resettable for TinstanceSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
