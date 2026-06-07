#[doc = "Register `MDR` reader"]
pub type R = crate::R<MdrSpec>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MdrSpec>;
#[doc = "Field `CENA` reader - desc CENA"]
pub type CenaR = crate::BitReader;
#[doc = "Field `CENA` writer - desc CENA"]
pub type CenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMD` reader - desc CSMD"]
pub type CsmdR = crate::FieldReader;
#[doc = "Field `CSMD` writer - desc CSMD"]
pub type CsmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSST` reader - desc CSST"]
pub type CsstR = crate::BitReader;
#[doc = "Field `CSST` writer - desc CSST"]
pub type CsstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMON` reader - desc CMON"]
pub type CmonR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CENA"]
    #[inline(always)]
    pub fn cena(&self) -> CenaR {
        CenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc CSMD"]
    #[inline(always)]
    pub fn csmd(&self) -> CsmdR {
        CsmdR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - desc CSST"]
    #[inline(always)]
    pub fn csst(&self) -> CsstR {
        CsstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CMON"]
    #[inline(always)]
    pub fn cmon(&self) -> CmonR {
        CmonR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDR")
            .field("cena", &self.cena())
            .field("csmd", &self.csmd())
            .field("csst", &self.csst())
            .field("cmon", &self.cmon())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc CENA"]
    #[inline(always)]
    pub fn cena(&mut self) -> CenaW<'_, MdrSpec> {
        CenaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc CSMD"]
    #[inline(always)]
    pub fn csmd(&mut self) -> CsmdW<'_, MdrSpec> {
        CsmdW::new(self, 2)
    }
    #[doc = "Bit 4 - desc CSST"]
    #[inline(always)]
    pub fn csst(&mut self) -> CsstW<'_, MdrSpec> {
        CsstW::new(self, 4)
    }
}
#[doc = "desc MDR\n\nYou can [`read`](crate::Reg::read) this register and get [`mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdrSpec;
impl crate::RegisterSpec for MdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MdrSpec {}
