#[doc = "Register `MATCH` reader"]
pub type R = crate::R<MatchSpec>;
#[doc = "Field `MATCH` reader - Indication of which address in ADDRESS that matched the incoming address"]
pub type MatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indication of which address in ADDRESS that matched the incoming address"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register indicating which address had a match\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchSpec;
impl crate::RegisterSpec for MatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_::R`](R) reader structure"]
impl crate::Readable for MatchSpec {}
#[doc = "`reset()` method sets MATCH to value 0"]
impl crate::Resettable for MatchSpec {}
