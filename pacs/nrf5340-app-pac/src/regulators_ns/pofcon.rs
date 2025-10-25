#[doc = "Register `POFCON` reader"]
pub type R = crate::R<PofconSpec>;
#[doc = "Register `POFCON` writer"]
pub type W = crate::W<PofconSpec>;
#[doc = "Enable or disable power-fail comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pof {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pof> for bool {
    #[inline(always)]
    fn from(variant: Pof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Enable or disable power-fail comparator"]
pub type PofR = crate::BitReader<Pof>;
impl PofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pof {
        match self.bits {
            false => Pof::Disabled,
            true => Pof::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pof::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pof::Enabled
    }
}
#[doc = "Field `POF` writer - Enable or disable power-fail comparator"]
pub type PofW<'a, REG> = crate::BitWriter<'a, REG, Pof>;
impl<'a, REG> PofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Enabled)
    }
}
#[doc = "Power-fail comparator threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Threshold {
    #[doc = "6: Set threshold to 1.9 V"]
    V19 = 6,
    #[doc = "7: Set threshold to 2.0 V"]
    V20 = 7,
    #[doc = "8: Set threshold to 2.1 V"]
    V21 = 8,
    #[doc = "9: Set threshold to 2.2 V"]
    V22 = 9,
    #[doc = "10: Set threshold to 2.3 V"]
    V23 = 10,
    #[doc = "11: Set threshold to 2.4 V"]
    V24 = 11,
    #[doc = "12: Set threshold to 2.5 V"]
    V25 = 12,
    #[doc = "13: Set threshold to 2.6 V"]
    V26 = 13,
    #[doc = "14: Set threshold to 2.7 V"]
    V27 = 14,
    #[doc = "15: Set threshold to 2.8 V"]
    V28 = 15,
}
impl From<Threshold> for u8 {
    #[inline(always)]
    fn from(variant: Threshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Threshold {
    type Ux = u8;
}
impl crate::IsEnum for Threshold {}
#[doc = "Field `THRESHOLD` reader - Power-fail comparator threshold setting"]
pub type ThresholdR = crate::FieldReader<Threshold>;
impl ThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Threshold> {
        match self.bits {
            6 => Some(Threshold::V19),
            7 => Some(Threshold::V20),
            8 => Some(Threshold::V21),
            9 => Some(Threshold::V22),
            10 => Some(Threshold::V23),
            11 => Some(Threshold::V24),
            12 => Some(Threshold::V25),
            13 => Some(Threshold::V26),
            14 => Some(Threshold::V27),
            15 => Some(Threshold::V28),
            _ => None,
        }
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn is_v19(&self) -> bool {
        *self == Threshold::V19
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn is_v20(&self) -> bool {
        *self == Threshold::V20
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == Threshold::V21
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn is_v22(&self) -> bool {
        *self == Threshold::V22
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == Threshold::V23
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn is_v24(&self) -> bool {
        *self == Threshold::V24
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == Threshold::V25
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn is_v26(&self) -> bool {
        *self == Threshold::V26
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == Threshold::V27
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == Threshold::V28
    }
}
#[doc = "Field `THRESHOLD` writer - Power-fail comparator threshold setting"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Threshold>;
impl<'a, REG> ThresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn v19(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn v20(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn v21(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn v22(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn v23(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn v24(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn v26(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V28)
    }
}
#[doc = "Power-fail comparator threshold setting for voltage supply on VDDH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thresholdvddh {
    #[doc = "0: Set threshold to 2.7 V"]
    V27 = 0,
    #[doc = "1: Set threshold to 2.8 V"]
    V28 = 1,
    #[doc = "2: Set threshold to 2.9 V"]
    V29 = 2,
    #[doc = "3: Set threshold to 3.0 V"]
    V30 = 3,
    #[doc = "4: Set threshold to 3.1 V"]
    V31 = 4,
    #[doc = "5: Set threshold to 3.2 V"]
    V32 = 5,
    #[doc = "6: Set threshold to 3.3 V"]
    V33 = 6,
    #[doc = "7: Set threshold to 3.4 V"]
    V34 = 7,
    #[doc = "8: Set threshold to 3.5 V"]
    V35 = 8,
    #[doc = "9: Set threshold to 3.6 V"]
    V36 = 9,
    #[doc = "10: Set threshold to 3.7 V"]
    V37 = 10,
    #[doc = "11: Set threshold to 3.8 V"]
    V38 = 11,
    #[doc = "12: Set threshold to 3.9 V"]
    V39 = 12,
    #[doc = "13: Set threshold to 4.0 V"]
    V40 = 13,
    #[doc = "14: Set threshold to 4.1 V"]
    V41 = 14,
    #[doc = "15: Set threshold to 4.2 V"]
    V42 = 15,
}
impl From<Thresholdvddh> for u8 {
    #[inline(always)]
    fn from(variant: Thresholdvddh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thresholdvddh {
    type Ux = u8;
}
impl crate::IsEnum for Thresholdvddh {}
#[doc = "Field `THRESHOLDVDDH` reader - Power-fail comparator threshold setting for voltage supply on VDDH"]
pub type ThresholdvddhR = crate::FieldReader<Thresholdvddh>;
impl ThresholdvddhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thresholdvddh {
        match self.bits {
            0 => Thresholdvddh::V27,
            1 => Thresholdvddh::V28,
            2 => Thresholdvddh::V29,
            3 => Thresholdvddh::V30,
            4 => Thresholdvddh::V31,
            5 => Thresholdvddh::V32,
            6 => Thresholdvddh::V33,
            7 => Thresholdvddh::V34,
            8 => Thresholdvddh::V35,
            9 => Thresholdvddh::V36,
            10 => Thresholdvddh::V37,
            11 => Thresholdvddh::V38,
            12 => Thresholdvddh::V39,
            13 => Thresholdvddh::V40,
            14 => Thresholdvddh::V41,
            15 => Thresholdvddh::V42,
            _ => unreachable!(),
        }
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == Thresholdvddh::V27
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == Thresholdvddh::V28
    }
    #[doc = "Set threshold to 2.9 V"]
    #[inline(always)]
    pub fn is_v29(&self) -> bool {
        *self == Thresholdvddh::V29
    }
    #[doc = "Set threshold to 3.0 V"]
    #[inline(always)]
    pub fn is_v30(&self) -> bool {
        *self == Thresholdvddh::V30
    }
    #[doc = "Set threshold to 3.1 V"]
    #[inline(always)]
    pub fn is_v31(&self) -> bool {
        *self == Thresholdvddh::V31
    }
    #[doc = "Set threshold to 3.2 V"]
    #[inline(always)]
    pub fn is_v32(&self) -> bool {
        *self == Thresholdvddh::V32
    }
    #[doc = "Set threshold to 3.3 V"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == Thresholdvddh::V33
    }
    #[doc = "Set threshold to 3.4 V"]
    #[inline(always)]
    pub fn is_v34(&self) -> bool {
        *self == Thresholdvddh::V34
    }
    #[doc = "Set threshold to 3.5 V"]
    #[inline(always)]
    pub fn is_v35(&self) -> bool {
        *self == Thresholdvddh::V35
    }
    #[doc = "Set threshold to 3.6 V"]
    #[inline(always)]
    pub fn is_v36(&self) -> bool {
        *self == Thresholdvddh::V36
    }
    #[doc = "Set threshold to 3.7 V"]
    #[inline(always)]
    pub fn is_v37(&self) -> bool {
        *self == Thresholdvddh::V37
    }
    #[doc = "Set threshold to 3.8 V"]
    #[inline(always)]
    pub fn is_v38(&self) -> bool {
        *self == Thresholdvddh::V38
    }
    #[doc = "Set threshold to 3.9 V"]
    #[inline(always)]
    pub fn is_v39(&self) -> bool {
        *self == Thresholdvddh::V39
    }
    #[doc = "Set threshold to 4.0 V"]
    #[inline(always)]
    pub fn is_v40(&self) -> bool {
        *self == Thresholdvddh::V40
    }
    #[doc = "Set threshold to 4.1 V"]
    #[inline(always)]
    pub fn is_v41(&self) -> bool {
        *self == Thresholdvddh::V41
    }
    #[doc = "Set threshold to 4.2 V"]
    #[inline(always)]
    pub fn is_v42(&self) -> bool {
        *self == Thresholdvddh::V42
    }
}
#[doc = "Field `THRESHOLDVDDH` writer - Power-fail comparator threshold setting for voltage supply on VDDH"]
pub type ThresholdvddhW<'a, REG> = crate::FieldWriter<'a, REG, 4, Thresholdvddh, crate::Safe>;
impl<'a, REG> ThresholdvddhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V28)
    }
    #[doc = "Set threshold to 2.9 V"]
    #[inline(always)]
    pub fn v29(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V29)
    }
    #[doc = "Set threshold to 3.0 V"]
    #[inline(always)]
    pub fn v30(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V30)
    }
    #[doc = "Set threshold to 3.1 V"]
    #[inline(always)]
    pub fn v31(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V31)
    }
    #[doc = "Set threshold to 3.2 V"]
    #[inline(always)]
    pub fn v32(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V32)
    }
    #[doc = "Set threshold to 3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V33)
    }
    #[doc = "Set threshold to 3.4 V"]
    #[inline(always)]
    pub fn v34(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V34)
    }
    #[doc = "Set threshold to 3.5 V"]
    #[inline(always)]
    pub fn v35(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V35)
    }
    #[doc = "Set threshold to 3.6 V"]
    #[inline(always)]
    pub fn v36(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V36)
    }
    #[doc = "Set threshold to 3.7 V"]
    #[inline(always)]
    pub fn v37(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V37)
    }
    #[doc = "Set threshold to 3.8 V"]
    #[inline(always)]
    pub fn v38(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V38)
    }
    #[doc = "Set threshold to 3.9 V"]
    #[inline(always)]
    pub fn v39(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V39)
    }
    #[doc = "Set threshold to 4.0 V"]
    #[inline(always)]
    pub fn v40(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V40)
    }
    #[doc = "Set threshold to 4.1 V"]
    #[inline(always)]
    pub fn v41(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V41)
    }
    #[doc = "Set threshold to 4.2 V"]
    #[inline(always)]
    pub fn v42(self) -> &'a mut crate::W<REG> {
        self.variant(Thresholdvddh::V42)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable power-fail comparator"]
    #[inline(always)]
    pub fn pof(&self) -> PofR {
        PofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for voltage supply on VDDH"]
    #[inline(always)]
    pub fn thresholdvddh(&self) -> ThresholdvddhR {
        ThresholdvddhR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable power-fail comparator"]
    #[inline(always)]
    pub fn pof(&mut self) -> PofW<'_, PofconSpec> {
        PofW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, PofconSpec> {
        ThresholdW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for voltage supply on VDDH"]
    #[inline(always)]
    pub fn thresholdvddh(&mut self) -> ThresholdvddhW<'_, PofconSpec> {
        ThresholdvddhW::new(self, 8)
    }
}
#[doc = "Power-fail comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PofconSpec;
impl crate::RegisterSpec for PofconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pofcon::R`](R) reader structure"]
impl crate::Readable for PofconSpec {}
#[doc = "`write(|w| ..)` method takes [`pofcon::W`](W) writer structure"]
impl crate::Writable for PofconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for PofconSpec {}
