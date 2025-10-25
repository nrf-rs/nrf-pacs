#[doc = "Register `COUNTERTOP` reader"]
pub type R = crate::R<CountertopSpec>;
#[doc = "Register `COUNTERTOP` writer"]
pub type W = crate::W<CountertopSpec>;
#[doc = "Field `COUNTERTOP` reader - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
pub type CountertopR = crate::FieldReader<u16>;
#[doc = "Field `COUNTERTOP` writer - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
pub type CountertopW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub fn countertop(&self) -> CountertopR {
        CountertopR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub fn countertop(&mut self) -> CountertopW<'_, CountertopSpec> {
        CountertopW::new(self, 0)
    }
}
#[doc = "Value up to which the pulse generator counter counts\n\nYou can [`read`](crate::Reg::read) this register and get [`countertop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`countertop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountertopSpec;
impl crate::RegisterSpec for CountertopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`countertop::R`](R) reader structure"]
impl crate::Readable for CountertopSpec {}
#[doc = "`write(|w| ..)` method takes [`countertop::W`](W) writer structure"]
impl crate::Writable for CountertopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNTERTOP to value 0x03ff"]
impl crate::Resettable for CountertopSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
