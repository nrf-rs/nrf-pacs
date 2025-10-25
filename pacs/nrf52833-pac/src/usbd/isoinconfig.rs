#[doc = "Register `ISOINCONFIG` reader"]
pub type R = crate::R<IsoinconfigSpec>;
#[doc = "Register `ISOINCONFIG` writer"]
pub type W = crate::W<IsoinconfigSpec>;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Response {
    #[doc = "0: Endpoint does not respond in that case"]
    NoResp = 0,
    #[doc = "1: Endpoint responds with a zero-length data packet in that case"]
    ZeroData = 1,
}
impl From<Response> for bool {
    #[inline(always)]
    fn from(variant: Response) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESPONSE` reader - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub type ResponseR = crate::BitReader<Response>;
impl ResponseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Response {
        match self.bits {
            false => Response::NoResp,
            true => Response::ZeroData,
        }
    }
    #[doc = "Endpoint does not respond in that case"]
    #[inline(always)]
    pub fn is_no_resp(&self) -> bool {
        *self == Response::NoResp
    }
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == Response::ZeroData
    }
}
#[doc = "Field `RESPONSE` writer - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub type ResponseW<'a, REG> = crate::BitWriter<'a, REG, Response>;
impl<'a, REG> ResponseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Endpoint does not respond in that case"]
    #[inline(always)]
    pub fn no_resp(self) -> &'a mut crate::W<REG> {
        self.variant(Response::NoResp)
    }
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    #[inline(always)]
    pub fn zero_data(self) -> &'a mut crate::W<REG> {
        self.variant(Response::ZeroData)
    }
}
impl R {
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&mut self) -> ResponseW<'_, IsoinconfigSpec> {
        ResponseW::new(self, 0)
    }
}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nYou can [`read`](crate::Reg::read) this register and get [`isoinconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoinconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoinconfigSpec;
impl crate::RegisterSpec for IsoinconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoinconfig::R`](R) reader structure"]
impl crate::Readable for IsoinconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`isoinconfig::W`](W) writer structure"]
impl crate::Writable for IsoinconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISOINCONFIG to value 0"]
impl crate::Resettable for IsoinconfigSpec {}
