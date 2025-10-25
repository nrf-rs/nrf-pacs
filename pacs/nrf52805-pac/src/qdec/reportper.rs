#[doc = "Register `REPORTPER` reader"]
pub type R = crate::R<ReportperSpec>;
#[doc = "Register `REPORTPER` writer"]
pub type W = crate::W<ReportperSpec>;
#[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reportper {
    #[doc = "0: 10 samples/report"]
    _10smpl = 0,
    #[doc = "1: 40 samples/report"]
    _40smpl = 1,
    #[doc = "2: 80 samples/report"]
    _80smpl = 2,
    #[doc = "3: 120 samples/report"]
    _120smpl = 3,
    #[doc = "4: 160 samples/report"]
    _160smpl = 4,
    #[doc = "5: 200 samples/report"]
    _200smpl = 5,
    #[doc = "6: 240 samples/report"]
    _240smpl = 6,
    #[doc = "7: 280 samples/report"]
    _280smpl = 7,
    #[doc = "8: 1 sample/report"]
    _1smpl = 8,
}
impl From<Reportper> for u8 {
    #[inline(always)]
    fn from(variant: Reportper) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reportper {
    type Ux = u8;
}
impl crate::IsEnum for Reportper {}
#[doc = "Field `REPORTPER` reader - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
pub type ReportperR = crate::FieldReader<Reportper>;
impl ReportperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Reportper> {
        match self.bits {
            0 => Some(Reportper::_10smpl),
            1 => Some(Reportper::_40smpl),
            2 => Some(Reportper::_80smpl),
            3 => Some(Reportper::_120smpl),
            4 => Some(Reportper::_160smpl),
            5 => Some(Reportper::_200smpl),
            6 => Some(Reportper::_240smpl),
            7 => Some(Reportper::_280smpl),
            8 => Some(Reportper::_1smpl),
            _ => None,
        }
    }
    #[doc = "10 samples/report"]
    #[inline(always)]
    pub fn is_10smpl(&self) -> bool {
        *self == Reportper::_10smpl
    }
    #[doc = "40 samples/report"]
    #[inline(always)]
    pub fn is_40smpl(&self) -> bool {
        *self == Reportper::_40smpl
    }
    #[doc = "80 samples/report"]
    #[inline(always)]
    pub fn is_80smpl(&self) -> bool {
        *self == Reportper::_80smpl
    }
    #[doc = "120 samples/report"]
    #[inline(always)]
    pub fn is_120smpl(&self) -> bool {
        *self == Reportper::_120smpl
    }
    #[doc = "160 samples/report"]
    #[inline(always)]
    pub fn is_160smpl(&self) -> bool {
        *self == Reportper::_160smpl
    }
    #[doc = "200 samples/report"]
    #[inline(always)]
    pub fn is_200smpl(&self) -> bool {
        *self == Reportper::_200smpl
    }
    #[doc = "240 samples/report"]
    #[inline(always)]
    pub fn is_240smpl(&self) -> bool {
        *self == Reportper::_240smpl
    }
    #[doc = "280 samples/report"]
    #[inline(always)]
    pub fn is_280smpl(&self) -> bool {
        *self == Reportper::_280smpl
    }
    #[doc = "1 sample/report"]
    #[inline(always)]
    pub fn is_1smpl(&self) -> bool {
        *self == Reportper::_1smpl
    }
}
#[doc = "Field `REPORTPER` writer - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
pub type ReportperW<'a, REG> = crate::FieldWriter<'a, REG, 4, Reportper>;
impl<'a, REG> ReportperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10 samples/report"]
    #[inline(always)]
    pub fn _10smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_10smpl)
    }
    #[doc = "40 samples/report"]
    #[inline(always)]
    pub fn _40smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_40smpl)
    }
    #[doc = "80 samples/report"]
    #[inline(always)]
    pub fn _80smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_80smpl)
    }
    #[doc = "120 samples/report"]
    #[inline(always)]
    pub fn _120smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_120smpl)
    }
    #[doc = "160 samples/report"]
    #[inline(always)]
    pub fn _160smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_160smpl)
    }
    #[doc = "200 samples/report"]
    #[inline(always)]
    pub fn _200smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_200smpl)
    }
    #[doc = "240 samples/report"]
    #[inline(always)]
    pub fn _240smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_240smpl)
    }
    #[doc = "280 samples/report"]
    #[inline(always)]
    pub fn _280smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_280smpl)
    }
    #[doc = "1 sample/report"]
    #[inline(always)]
    pub fn _1smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_1smpl)
    }
}
impl R {
    #[doc = "Bits 0:3 - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
    #[inline(always)]
    pub fn reportper(&self) -> ReportperR {
        ReportperR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated."]
    #[inline(always)]
    pub fn reportper(&mut self) -> ReportperW<'_, ReportperSpec> {
        ReportperW::new(self, 0)
    }
}
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated\n\nYou can [`read`](crate::Reg::read) this register and get [`reportper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reportper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReportperSpec;
impl crate::RegisterSpec for ReportperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reportper::R`](R) reader structure"]
impl crate::Readable for ReportperSpec {}
#[doc = "`write(|w| ..)` method takes [`reportper::W`](W) writer structure"]
impl crate::Writable for ReportperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPORTPER to value 0"]
impl crate::Resettable for ReportperSpec {}
