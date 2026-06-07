#[doc = "Register `OCR` reader"]
pub type R = crate::R<OcrSpec>;
#[doc = "Register `OCR` writer"]
pub type W = crate::W<OcrSpec>;
#[doc = "Field `COEN` reader - desc COEN"]
pub type CoenR = crate::BitReader;
#[doc = "Field `COEN` writer - desc COEN"]
pub type CoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPS` reader - desc COPS"]
pub type CopsR = crate::BitReader;
#[doc = "Field `COPS` writer - desc COPS"]
pub type CopsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOE` reader - desc CPOE"]
pub type CpoeR = crate::BitReader;
#[doc = "Field `CPOE` writer - desc CPOE"]
pub type CpoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWEN` reader - desc BWEN"]
pub type BwenR = crate::BitReader;
#[doc = "Field `BWEN` writer - desc BWEN"]
pub type BwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWMD` reader - desc BWMD"]
pub type BwmdR = crate::BitReader;
#[doc = "Field `BWMD` writer - desc BWMD"]
pub type BwmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWOL` reader - desc BWOL"]
pub type BwolR = crate::FieldReader;
#[doc = "Field `BWOL` writer - desc BWOL"]
pub type BwolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc COEN"]
    #[inline(always)]
    pub fn coen(&self) -> CoenR {
        CoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc COPS"]
    #[inline(always)]
    pub fn cops(&self) -> CopsR {
        CopsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CPOE"]
    #[inline(always)]
    pub fn cpoe(&self) -> CpoeR {
        CpoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BWEN"]
    #[inline(always)]
    pub fn bwen(&self) -> BwenR {
        BwenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BWMD"]
    #[inline(always)]
    pub fn bwmd(&self) -> BwmdR {
        BwmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc BWOL"]
    #[inline(always)]
    pub fn bwol(&self) -> BwolR {
        BwolR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCR")
            .field("coen", &self.coen())
            .field("cops", &self.cops())
            .field("cpoe", &self.cpoe())
            .field("bwen", &self.bwen())
            .field("bwmd", &self.bwmd())
            .field("bwol", &self.bwol())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc COEN"]
    #[inline(always)]
    pub fn coen(&mut self) -> CoenW<'_, OcrSpec> {
        CoenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc COPS"]
    #[inline(always)]
    pub fn cops(&mut self) -> CopsW<'_, OcrSpec> {
        CopsW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CPOE"]
    #[inline(always)]
    pub fn cpoe(&mut self) -> CpoeW<'_, OcrSpec> {
        CpoeW::new(self, 2)
    }
    #[doc = "Bit 4 - desc BWEN"]
    #[inline(always)]
    pub fn bwen(&mut self) -> BwenW<'_, OcrSpec> {
        BwenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc BWMD"]
    #[inline(always)]
    pub fn bwmd(&mut self) -> BwmdW<'_, OcrSpec> {
        BwmdW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc BWOL"]
    #[inline(always)]
    pub fn bwol(&mut self) -> BwolW<'_, OcrSpec> {
        BwolW::new(self, 6)
    }
}
#[doc = "desc OCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcrSpec;
impl crate::RegisterSpec for OcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocr::R`](R) reader structure"]
impl crate::Readable for OcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ocr::W`](W) writer structure"]
impl crate::Writable for OcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCR to value 0"]
impl crate::Resettable for OcrSpec {}
