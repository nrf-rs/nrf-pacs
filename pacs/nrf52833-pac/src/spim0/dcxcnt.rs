#[doc = "Register `DCXCNT` reader"]
pub type R = crate::R<DcxcntSpec>;
#[doc = "Register `DCXCNT` writer"]
pub type W = crate::W<DcxcntSpec>;
#[doc = "Field `DCXCNT` reader - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
pub type DcxcntR = crate::FieldReader;
#[doc = "Field `DCXCNT` writer - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
pub type DcxcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&self) -> DcxcntR {
        DcxcntR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn dcxcnt(&mut self) -> DcxcntW<'_, DcxcntSpec> {
        DcxcntW::new(self, 0)
    }
}
#[doc = "DCX configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dcxcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcxcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxcntSpec;
impl crate::RegisterSpec for DcxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcxcnt::R`](R) reader structure"]
impl crate::Readable for DcxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dcxcnt::W`](W) writer structure"]
impl crate::Writable for DcxcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCXCNT to value 0"]
impl crate::Resettable for DcxcntSpec {}
