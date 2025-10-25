#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Result of last compare. Decision point SAMPLE task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Result {
    #[doc = "0: Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    Below = 0,
    #[doc = "1: Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    Above = 1,
}
impl From<Result> for bool {
    #[inline(always)]
    fn from(variant: Result) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULT` reader - Result of last compare. Decision point SAMPLE task."]
pub type ResultR = crate::BitReader<Result>;
impl ResultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Result {
        match self.bits {
            false => Result::Below,
            true => Result::Above,
        }
    }
    #[doc = "Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Result::Below
    }
    #[doc = "Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Result::Above
    }
}
impl R {
    #[doc = "Bit 0 - Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 1) != 0)
    }
}
#[doc = "Compare result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {}
