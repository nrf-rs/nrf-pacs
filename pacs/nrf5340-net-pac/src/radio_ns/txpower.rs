#[doc = "Register `TXPOWER` reader"]
pub type R = crate::R<TxpowerSpec>;
#[doc = "Register `TXPOWER` writer"]
pub type W = crate::W<TxpowerSpec>;
#[doc = "RADIO output power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txpower {
    #[doc = "0: 0 dBm"]
    _0dBm = 0,
    #[doc = "255: -1 dBm"]
    Neg1dBm = 255,
    #[doc = "254: -2 dBm"]
    Neg2dBm = 254,
    #[doc = "253: -3 dBm"]
    Neg3dBm = 253,
    #[doc = "252: -4 dBm"]
    Neg4dBm = 252,
    #[doc = "251: -5 dBm"]
    Neg5dBm = 251,
    #[doc = "250: -6 dBm"]
    Neg6dBm = 250,
    #[doc = "249: -7 dBm"]
    Neg7dBm = 249,
    #[doc = "248: -8 dBm"]
    Neg8dBm = 248,
    #[doc = "244: -12 dBm"]
    Neg12dBm = 244,
    #[doc = "240: -16 dBm"]
    Neg16dBm = 240,
    #[doc = "236: -20 dBm"]
    Neg20dBm = 236,
    #[doc = "226: Deprecated enumerator - -40 dBm"]
    Neg30dBm = 226,
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
#[doc = "Field `TXPOWER` reader - RADIO output power"]
pub type TxpowerR = crate::FieldReader<Txpower>;
impl TxpowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txpower> {
        match self.bits {
            0 => Some(Txpower::_0dBm),
            255 => Some(Txpower::Neg1dBm),
            254 => Some(Txpower::Neg2dBm),
            253 => Some(Txpower::Neg3dBm),
            252 => Some(Txpower::Neg4dBm),
            251 => Some(Txpower::Neg5dBm),
            250 => Some(Txpower::Neg6dBm),
            249 => Some(Txpower::Neg7dBm),
            248 => Some(Txpower::Neg8dBm),
            244 => Some(Txpower::Neg12dBm),
            240 => Some(Txpower::Neg16dBm),
            236 => Some(Txpower::Neg20dBm),
            226 => Some(Txpower::Neg30dBm),
            216 => Some(Txpower::Neg40dBm),
            _ => None,
        }
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == Txpower::_0dBm
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn is_neg1d_bm(&self) -> bool {
        *self == Txpower::Neg1dBm
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn is_neg2d_bm(&self) -> bool {
        *self == Txpower::Neg2dBm
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn is_neg3d_bm(&self) -> bool {
        *self == Txpower::Neg3dBm
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == Txpower::Neg4dBm
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn is_neg5d_bm(&self) -> bool {
        *self == Txpower::Neg5dBm
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn is_neg6d_bm(&self) -> bool {
        *self == Txpower::Neg6dBm
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn is_neg7d_bm(&self) -> bool {
        *self == Txpower::Neg7dBm
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
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        *self == Txpower::Neg30dBm
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == Txpower::Neg40dBm
    }
}
#[doc = "Field `TXPOWER` writer - RADIO output power"]
pub type TxpowerW<'a, REG> = crate::FieldWriter<'a, REG, 8, Txpower>;
impl<'a, REG> TxpowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::_0dBm)
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn neg1d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg1dBm)
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn neg2d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg2dBm)
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn neg3d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg3dBm)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg4dBm)
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn neg5d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg5dBm)
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn neg6d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg6dBm)
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn neg7d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg7dBm)
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
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg30dBm)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg40dBm)
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&self) -> TxpowerR {
        TxpowerR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power"]
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
