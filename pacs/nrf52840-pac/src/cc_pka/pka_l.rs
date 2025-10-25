#[doc = "Register `PKA_L[%s]` reader"]
pub type R = crate::R<PkaLSpec>;
#[doc = "Register `PKA_L[%s]` writer"]
pub type W = crate::W<PkaLSpec>;
#[doc = "Field `OpSize` reader - Operand bit size."]
pub type OpSizeR = crate::FieldReader<u16>;
#[doc = "Field `OpSize` writer - Operand bit size."]
pub type OpSizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Operand bit size."]
    #[inline(always)]
    pub fn op_size(&self) -> OpSizeR {
        OpSizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Operand bit size."]
    #[inline(always)]
    pub fn op_size(&mut self) -> OpSizeW<'_, PkaLSpec> {
        OpSizeW::new(self, 0)
    }
}
#[doc = "Description collection: This register holds the operands bit size.\n\nYou can [`read`](crate::Reg::read) this register and get [`pka_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaLSpec;
impl crate::RegisterSpec for PkaLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_l::R`](R) reader structure"]
impl crate::Readable for PkaLSpec {}
#[doc = "`write(|w| ..)` method takes [`pka_l::W`](W) writer structure"]
impl crate::Writable for PkaLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKA_L[%s] to value 0"]
impl crate::Resettable for PkaLSpec {}
