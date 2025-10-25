#[doc = "Register `TXPOWER` reader"]
pub type R = crate::R<TxpowerSpec>;
#[doc = "Register `TXPOWER` writer"]
pub type W = crate::W<TxpowerSpec>;
#[doc = "RADIO output power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txpower {
    #[doc = "4: +4 dBm"]
    Pos4dBm = 4,
    #[doc = "3: +3 dBm"]
    Pos3dBm = 3,
    #[doc = "0: 0 dBm"]
    _0dBm = 0,
    #[doc = "252: -4 dBm"]
    Neg4dBm = 252,
    #[doc = "248: -8 dBm"]
    Neg8dBm = 248,
    #[doc = "244: -12 dBm"]
    Neg12dBm = 244,
    #[doc = "240: -16 dBm"]
    Neg16dBm = 240,
    #[doc = "236: -20 dBm"]
    Neg20dBm = 236,
    #[doc = "216: -40 dBm"]
    Neg40dBm = 216,
}
impl From<Txpower> for u8 {
    #[inline(always)]
    fn from(variant: Txpower) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txpower {
    type Ux = u8;
}
impl crate::IsEnum for Txpower {}
#[doc = "Field `TXPOWER` reader - RADIO output power."]
pub type TxpowerR = crate::FieldReader<Txpower>;
impl TxpowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txpower> {
        match self.bits {
            4 => Some(Txpower::Pos4dBm),
            3 => Some(Txpower::Pos3dBm),
            0 => Some(Txpower::_0dBm),
            252 => Some(Txpower::Neg4dBm),
            248 => Some(Txpower::Neg8dBm),
            244 => Some(Txpower::Neg12dBm),
            240 => Some(Txpower::Neg16dBm),
            236 => Some(Txpower::Neg20dBm),
            216 => Some(Txpower::Neg40dBm),
            _ => None,
        }
    }
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == Txpower::Pos4dBm
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn is_pos3d_bm(&self) -> bool {
        *self == Txpower::Pos3dBm
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == Txpower::_0dBm
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == Txpower::Neg4dBm
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == Txpower::Neg8dBm
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == Txpower::Neg12dBm
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == Txpower::Neg16dBm
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == Txpower::Neg20dBm
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == Txpower::Neg40dBm
    }
}
#[doc = "Field `TXPOWER` writer - RADIO output power."]
pub type TxpowerW<'a, REG> = crate::FieldWriter<'a, REG, 8, Txpower>;
impl<'a, REG> TxpowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos4dBm)
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn pos3d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos3dBm)
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::_0dBm)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg4dBm)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg8dBm)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg12dBm)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg16dBm)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg20dBm)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg40dBm)
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    pub fn txpower(&self) -> TxpowerR {
        TxpowerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    pub fn txpower(&mut self) -> TxpowerW<'_, TxpowerSpec> {
        TxpowerW::new(self, 0)
    }
}
#[doc = "Output power\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpowerSpec;
impl crate::RegisterSpec for TxpowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpower::R`](R) reader structure"]
impl crate::Readable for TxpowerSpec {}
#[doc = "`write(|w| ..)` method takes [`txpower::W`](W) writer structure"]
impl crate::Writable for TxpowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TxpowerSpec {}
