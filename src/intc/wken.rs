#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WkenSpec>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WkenSpec>;
#[doc = "Field `EIRQWKEN` reader - desc EIRQWKEN"]
pub type EirqwkenR = crate::FieldReader<u16>;
#[doc = "Field `EIRQWKEN` writer - desc EIRQWKEN"]
pub type EirqwkenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SWDTWKEN` reader - desc SWDTWKEN"]
pub type SwdtwkenR = crate::BitReader;
#[doc = "Field `SWDTWKEN` writer - desc SWDTWKEN"]
pub type SwdtwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1WKEN` reader - desc CMP1WKEN"]
pub type Cmp1wkenR = crate::BitReader;
#[doc = "Field `CMP1WKEN` writer - desc CMP1WKEN"]
pub type Cmp1wkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKTMWKEN` reader - desc WKTMWKEN"]
pub type WktmwkenR = crate::BitReader;
#[doc = "Field `WKTMWKEN` writer - desc WKTMWKEN"]
pub type WktmwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALMWKEN` reader - desc RTCALMWKEN"]
pub type RtcalmwkenR = crate::BitReader;
#[doc = "Field `RTCALMWKEN` writer - desc RTCALMWKEN"]
pub type RtcalmwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCPRDWKEN` reader - desc RTCPRDWKEN"]
pub type RtcprdwkenR = crate::BitReader;
#[doc = "Field `RTCPRDWKEN` writer - desc RTCPRDWKEN"]
pub type RtcprdwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0CMPWKEN` reader - desc TMR0CMPWKEN"]
pub type Tmr0cmpwkenR = crate::BitReader;
#[doc = "Field `TMR0CMPWKEN` writer - desc TMR0CMPWKEN"]
pub type Tmr0cmpwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWKEN` reader - desc RXWKEN"]
pub type RxwkenR = crate::BitReader;
#[doc = "Field `RXWKEN` writer - desc RXWKEN"]
pub type RxwkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2WKEN` reader - desc CMP2WKEN"]
pub type Cmp2wkenR = crate::BitReader;
#[doc = "Field `CMP2WKEN` writer - desc CMP2WKEN"]
pub type Cmp2wkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3WKEN` reader - desc CMP3WKEN"]
pub type Cmp3wkenR = crate::BitReader;
#[doc = "Field `CMP3WKEN` writer - desc CMP3WKEN"]
pub type Cmp3wkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4WKEN` reader - desc CMP4WKEN"]
pub type Cmp4wkenR = crate::BitReader;
#[doc = "Field `CMP4WKEN` writer - desc CMP4WKEN"]
pub type Cmp4wkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - desc EIRQWKEN"]
    #[inline(always)]
    pub fn eirqwken(&self) -> EirqwkenR {
        EirqwkenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - desc SWDTWKEN"]
    #[inline(always)]
    pub fn swdtwken(&self) -> SwdtwkenR {
        SwdtwkenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CMP1WKEN"]
    #[inline(always)]
    pub fn cmp1wken(&self) -> Cmp1wkenR {
        Cmp1wkenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc WKTMWKEN"]
    #[inline(always)]
    pub fn wktmwken(&self) -> WktmwkenR {
        WktmwkenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc RTCALMWKEN"]
    #[inline(always)]
    pub fn rtcalmwken(&self) -> RtcalmwkenR {
        RtcalmwkenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc RTCPRDWKEN"]
    #[inline(always)]
    pub fn rtcprdwken(&self) -> RtcprdwkenR {
        RtcprdwkenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TMR0CMPWKEN"]
    #[inline(always)]
    pub fn tmr0cmpwken(&self) -> Tmr0cmpwkenR {
        Tmr0cmpwkenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - desc RXWKEN"]
    #[inline(always)]
    pub fn rxwken(&self) -> RxwkenR {
        RxwkenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - desc CMP2WKEN"]
    #[inline(always)]
    pub fn cmp2wken(&self) -> Cmp2wkenR {
        Cmp2wkenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc CMP3WKEN"]
    #[inline(always)]
    pub fn cmp3wken(&self) -> Cmp3wkenR {
        Cmp3wkenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CMP4WKEN"]
    #[inline(always)]
    pub fn cmp4wken(&self) -> Cmp4wkenR {
        Cmp4wkenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKEN")
            .field("eirqwken", &self.eirqwken())
            .field("swdtwken", &self.swdtwken())
            .field("cmp1wken", &self.cmp1wken())
            .field("wktmwken", &self.wktmwken())
            .field("rtcalmwken", &self.rtcalmwken())
            .field("rtcprdwken", &self.rtcprdwken())
            .field("tmr0cmpwken", &self.tmr0cmpwken())
            .field("rxwken", &self.rxwken())
            .field("cmp2wken", &self.cmp2wken())
            .field("cmp3wken", &self.cmp3wken())
            .field("cmp4wken", &self.cmp4wken())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - desc EIRQWKEN"]
    #[inline(always)]
    pub fn eirqwken(&mut self) -> EirqwkenW<'_, WkenSpec> {
        EirqwkenW::new(self, 0)
    }
    #[doc = "Bit 16 - desc SWDTWKEN"]
    #[inline(always)]
    pub fn swdtwken(&mut self) -> SwdtwkenW<'_, WkenSpec> {
        SwdtwkenW::new(self, 16)
    }
    #[doc = "Bit 19 - desc CMP1WKEN"]
    #[inline(always)]
    pub fn cmp1wken(&mut self) -> Cmp1wkenW<'_, WkenSpec> {
        Cmp1wkenW::new(self, 19)
    }
    #[doc = "Bit 20 - desc WKTMWKEN"]
    #[inline(always)]
    pub fn wktmwken(&mut self) -> WktmwkenW<'_, WkenSpec> {
        WktmwkenW::new(self, 20)
    }
    #[doc = "Bit 21 - desc RTCALMWKEN"]
    #[inline(always)]
    pub fn rtcalmwken(&mut self) -> RtcalmwkenW<'_, WkenSpec> {
        RtcalmwkenW::new(self, 21)
    }
    #[doc = "Bit 22 - desc RTCPRDWKEN"]
    #[inline(always)]
    pub fn rtcprdwken(&mut self) -> RtcprdwkenW<'_, WkenSpec> {
        RtcprdwkenW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TMR0CMPWKEN"]
    #[inline(always)]
    pub fn tmr0cmpwken(&mut self) -> Tmr0cmpwkenW<'_, WkenSpec> {
        Tmr0cmpwkenW::new(self, 23)
    }
    #[doc = "Bit 26 - desc RXWKEN"]
    #[inline(always)]
    pub fn rxwken(&mut self) -> RxwkenW<'_, WkenSpec> {
        RxwkenW::new(self, 26)
    }
    #[doc = "Bit 29 - desc CMP2WKEN"]
    #[inline(always)]
    pub fn cmp2wken(&mut self) -> Cmp2wkenW<'_, WkenSpec> {
        Cmp2wkenW::new(self, 29)
    }
    #[doc = "Bit 30 - desc CMP3WKEN"]
    #[inline(always)]
    pub fn cmp3wken(&mut self) -> Cmp3wkenW<'_, WkenSpec> {
        Cmp3wkenW::new(self, 30)
    }
    #[doc = "Bit 31 - desc CMP4WKEN"]
    #[inline(always)]
    pub fn cmp4wken(&mut self) -> Cmp4wkenW<'_, WkenSpec> {
        Cmp4wkenW::new(self, 31)
    }
}
#[doc = "desc WKEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkenSpec;
impl crate::RegisterSpec for WkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WkenSpec {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WkenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WkenSpec {}
