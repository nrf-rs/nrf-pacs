#[doc = "Register `IR[%s]` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Field `IR` reader - Identity root, word n"]
pub type IrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Identity root, word n"]
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(self.bits)
    }
}
#[doc = "Description collection: Identity root, word n\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`reset()` method sets IR[%s] to value 0xffff_ffff"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
