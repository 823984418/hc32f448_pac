#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOCAF` reader - desc EOCAF"]
pub type EocafR = crate::BitReader;
#[doc = "Field `EOCBF` reader - desc EOCBF"]
pub type EocbfR = crate::BitReader;
#[doc = "Field `SASTPDF` reader - desc SASTPDF"]
pub type SastpdfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc EOCAF"]
    #[inline(always)]
    pub fn eocaf(&self) -> EocafR {
        EocafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOCBF"]
    #[inline(always)]
    pub fn eocbf(&self) -> EocbfR {
        EocbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SASTPDF"]
    #[inline(always)]
    pub fn sastpdf(&self) -> SastpdfR {
        SastpdfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("eocaf", &self.eocaf())
            .field("eocbf", &self.eocbf())
            .field("sastpdf", &self.sastpdf())
            .finish()
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
