#[doc = "Register `ITTRFLIN` reader"]
pub type R = crate::R<IttrflinSpec>;
#[doc = "Field `TRIGIN` reader - Read the value of trigin."]
pub type TriginR = crate::BitReader;
#[doc = "Field `FLUSHIN` reader - Read the value of flushin."]
pub type FlushinR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read the value of trigin."]
    #[inline(always)]
    pub fn trigin(&self) -> TriginR {
        TriginR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read the value of flushin."]
    #[inline(always)]
    pub fn flushin(&self) -> FlushinR {
        FlushinR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Integration Test Trigger In and Flush In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ittrflin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IttrflinSpec;
impl crate::RegisterSpec for IttrflinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ittrflin::R`](R) reader structure"]
impl crate::Readable for IttrflinSpec {}
#[doc = "`reset()` method sets ITTRFLIN to value 0"]
impl crate::Resettable for IttrflinSpec {}
