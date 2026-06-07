#[doc = "Register `PCRH0` reader"]
pub type R = crate::R<Pcrh0Spec>;
#[doc = "Register `PCRH0` writer"]
pub type W = crate::W<Pcrh0Spec>;
#[doc = "Field `POUT` reader - desc POUT"]
pub type PoutR = crate::BitReader;
#[doc = "Field `POUT` writer - desc POUT"]
pub type PoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUTE` reader - desc POUTE"]
pub type PouteR = crate::BitReader;
#[doc = "Field `POUTE` writer - desc POUTE"]
pub type PouteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOD` reader - desc NOD"]
pub type NodR = crate::BitReader;
#[doc = "Field `NOD` writer - desc NOD"]
pub type NodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - desc DRV"]
pub type DrvR = crate::FieldReader;
#[doc = "Field `DRV` writer - desc DRV"]
pub type DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUU` reader - desc PUU"]
pub type PuuR = crate::BitReader;
#[doc = "Field `PUU` writer - desc PUU"]
pub type PuuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUD` reader - desc PUD"]
pub type PudR = crate::BitReader;
#[doc = "Field `PUD` writer - desc PUD"]
pub type PudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN` reader - desc PIN"]
pub type PinR = crate::BitReader;
#[doc = "Field `INVE` reader - desc INVE"]
pub type InveR = crate::BitReader;
#[doc = "Field `INVE` writer - desc INVE"]
pub type InveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINSEL` reader - desc CINSEL"]
pub type CinselR = crate::BitReader;
#[doc = "Field `CINSEL` writer - desc CINSEL"]
pub type CinselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTE` reader - desc INTE"]
pub type InteR = crate::BitReader;
#[doc = "Field `INTE` writer - desc INTE"]
pub type InteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINAE` reader - desc PINAE"]
pub type PinaeR = crate::BitReader;
#[doc = "Field `PINAE` writer - desc PINAE"]
pub type PinaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTE` reader - desc LTE"]
pub type LteR = crate::BitReader;
#[doc = "Field `LTE` writer - desc LTE"]
pub type LteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDIS` reader - desc DDIS"]
pub type DdisR = crate::BitReader;
#[doc = "Field `DDIS` writer - desc DDIS"]
pub type DdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUT"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUTE"]
    #[inline(always)]
    pub fn poute(&self) -> PouteR {
        PouteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc NOD"]
    #[inline(always)]
    pub fn nod(&self) -> NodR {
        NodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc PUU"]
    #[inline(always)]
    pub fn puu(&self) -> PuuR {
        PuuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PUD"]
    #[inline(always)]
    pub fn pud(&self) -> PudR {
        PudR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PIN"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc INVE"]
    #[inline(always)]
    pub fn inve(&self) -> InveR {
        InveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CINSEL"]
    #[inline(always)]
    pub fn cinsel(&self) -> CinselR {
        CinselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc INTE"]
    #[inline(always)]
    pub fn inte(&self) -> InteR {
        InteR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PINAE"]
    #[inline(always)]
    pub fn pinae(&self) -> PinaeR {
        PinaeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LTE"]
    #[inline(always)]
    pub fn lte(&self) -> LteR {
        LteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DDIS"]
    #[inline(always)]
    pub fn ddis(&self) -> DdisR {
        DdisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCRH0")
            .field("pout", &self.pout())
            .field("poute", &self.poute())
            .field("nod", &self.nod())
            .field("drv", &self.drv())
            .field("puu", &self.puu())
            .field("pud", &self.pud())
            .field("pin", &self.pin())
            .field("inve", &self.inve())
            .field("cinsel", &self.cinsel())
            .field("inte", &self.inte())
            .field("pinae", &self.pinae())
            .field("lte", &self.lte())
            .field("ddis", &self.ddis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc POUT"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Pcrh0Spec> {
        PoutW::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUTE"]
    #[inline(always)]
    pub fn poute(&mut self) -> PouteW<'_, Pcrh0Spec> {
        PouteW::new(self, 1)
    }
    #[doc = "Bit 2 - desc NOD"]
    #[inline(always)]
    pub fn nod(&mut self) -> NodW<'_, Pcrh0Spec> {
        NodW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc DRV"]
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, Pcrh0Spec> {
        DrvW::new(self, 4)
    }
    #[doc = "Bit 6 - desc PUU"]
    #[inline(always)]
    pub fn puu(&mut self) -> PuuW<'_, Pcrh0Spec> {
        PuuW::new(self, 6)
    }
    #[doc = "Bit 7 - desc PUD"]
    #[inline(always)]
    pub fn pud(&mut self) -> PudW<'_, Pcrh0Spec> {
        PudW::new(self, 7)
    }
    #[doc = "Bit 9 - desc INVE"]
    #[inline(always)]
    pub fn inve(&mut self) -> InveW<'_, Pcrh0Spec> {
        InveW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CINSEL"]
    #[inline(always)]
    pub fn cinsel(&mut self) -> CinselW<'_, Pcrh0Spec> {
        CinselW::new(self, 10)
    }
    #[doc = "Bit 12 - desc INTE"]
    #[inline(always)]
    pub fn inte(&mut self) -> InteW<'_, Pcrh0Spec> {
        InteW::new(self, 12)
    }
    #[doc = "Bit 13 - desc PINAE"]
    #[inline(always)]
    pub fn pinae(&mut self) -> PinaeW<'_, Pcrh0Spec> {
        PinaeW::new(self, 13)
    }
    #[doc = "Bit 14 - desc LTE"]
    #[inline(always)]
    pub fn lte(&mut self) -> LteW<'_, Pcrh0Spec> {
        LteW::new(self, 14)
    }
    #[doc = "Bit 15 - desc DDIS"]
    #[inline(always)]
    pub fn ddis(&mut self) -> DdisW<'_, Pcrh0Spec> {
        DdisW::new(self, 15)
    }
}
#[doc = "desc PCRH0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcrh0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcrh0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcrh0Spec;
impl crate::RegisterSpec for Pcrh0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcrh0::R`](R) reader structure"]
impl crate::Readable for Pcrh0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcrh0::W`](W) writer structure"]
impl crate::Writable for Pcrh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCRH0 to value 0"]
impl crate::Resettable for Pcrh0Spec {}
