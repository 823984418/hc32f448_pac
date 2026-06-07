#[doc = "Register `FSTR` reader"]
pub type R = crate::R<FstrSpec>;
#[doc = "Register `FSTR` writer"]
pub type W = crate::W<FstrSpec>;
#[doc = "Field `FEN` reader - desc FEN"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - desc FEN"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFFLUSH` reader - desc TFFLUSH"]
pub type TfflushR = crate::BitReader;
#[doc = "Field `TFFLUSH` writer - desc TFFLUSH"]
pub type TfflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFFLUSH` reader - desc RFFLUSH"]
pub type RfflushR = crate::BitReader;
#[doc = "Field `RFFLUSH` writer - desc RFFLUSH"]
pub type RfflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKTFFLUSH` reader - desc NACKTFFLUSH"]
pub type NacktfflushR = crate::BitReader;
#[doc = "Field `NACKTFFLUSH` writer - desc NACKTFFLUSH"]
pub type NacktfflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFST` reader - desc TFST"]
pub type TfstR = crate::FieldReader;
#[doc = "Field `TFST` writer - desc TFST"]
pub type TfstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFST` reader - desc RFST"]
pub type RfstR = crate::FieldReader;
#[doc = "Field `RFST` writer - desc RFST"]
pub type RfstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc FEN"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TFFLUSH"]
    #[inline(always)]
    pub fn tfflush(&self) -> TfflushR {
        TfflushR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RFFLUSH"]
    #[inline(always)]
    pub fn rfflush(&self) -> RfflushR {
        RfflushR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc NACKTFFLUSH"]
    #[inline(always)]
    pub fn nacktfflush(&self) -> NacktfflushR {
        NacktfflushR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc TFST"]
    #[inline(always)]
    pub fn tfst(&self) -> TfstR {
        TfstR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc RFST"]
    #[inline(always)]
    pub fn rfst(&self) -> RfstR {
        RfstR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSTR")
            .field("fen", &self.fen())
            .field("tfflush", &self.tfflush())
            .field("rfflush", &self.rfflush())
            .field("nacktfflush", &self.nacktfflush())
            .field("tfst", &self.tfst())
            .field("rfst", &self.rfst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc FEN"]
    #[inline(always)]
    pub fn fen(&mut self) -> FenW<'_, FstrSpec> {
        FenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TFFLUSH"]
    #[inline(always)]
    pub fn tfflush(&mut self) -> TfflushW<'_, FstrSpec> {
        TfflushW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RFFLUSH"]
    #[inline(always)]
    pub fn rfflush(&mut self) -> RfflushW<'_, FstrSpec> {
        RfflushW::new(self, 2)
    }
    #[doc = "Bit 3 - desc NACKTFFLUSH"]
    #[inline(always)]
    pub fn nacktfflush(&mut self) -> NacktfflushW<'_, FstrSpec> {
        NacktfflushW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc TFST"]
    #[inline(always)]
    pub fn tfst(&mut self) -> TfstW<'_, FstrSpec> {
        TfstW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc RFST"]
    #[inline(always)]
    pub fn rfst(&mut self) -> RfstW<'_, FstrSpec> {
        RfstW::new(self, 6)
    }
}
#[doc = "desc FSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`fstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstrSpec;
impl crate::RegisterSpec for FstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstr::R`](R) reader structure"]
impl crate::Readable for FstrSpec {}
#[doc = "`write(|w| ..)` method takes [`fstr::W`](W) writer structure"]
impl crate::Writable for FstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSTR to value 0"]
impl crate::Resettable for FstrSpec {}
