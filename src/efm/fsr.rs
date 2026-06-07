#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Field `OTPWERR` reader - desc OTPWERR"]
pub type OtpwerrR = crate::BitReader;
#[doc = "Field `PRTWERR` reader - desc PRTWERR"]
pub type PrtwerrR = crate::BitReader;
#[doc = "Field `PGSZERR` reader - desc PGSZERR"]
pub type PgszerrR = crate::BitReader;
#[doc = "Field `MISMTCH` reader - desc MISMTCH"]
pub type MismtchR = crate::BitReader;
#[doc = "Field `OPTEND` reader - desc OPTEND"]
pub type OptendR = crate::BitReader;
#[doc = "Field `COLERR` reader - desc COLERR"]
pub type ColerrR = crate::BitReader;
#[doc = "Field `RDY` reader - desc RDY"]
pub type RdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc OTPWERR"]
    #[inline(always)]
    pub fn otpwerr(&self) -> OtpwerrR {
        OtpwerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PRTWERR"]
    #[inline(always)]
    pub fn prtwerr(&self) -> PrtwerrR {
        PrtwerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PGSZERR"]
    #[inline(always)]
    pub fn pgszerr(&self) -> PgszerrR {
        PgszerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc MISMTCH"]
    #[inline(always)]
    pub fn mismtch(&self) -> MismtchR {
        MismtchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OPTEND"]
    #[inline(always)]
    pub fn optend(&self) -> OptendR {
        OptendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COLERR"]
    #[inline(always)]
    pub fn colerr(&self) -> ColerrR {
        ColerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSR")
            .field("otpwerr", &self.otpwerr())
            .field("prtwerr", &self.prtwerr())
            .field("pgszerr", &self.pgszerr())
            .field("mismtch", &self.mismtch())
            .field("optend", &self.optend())
            .field("colerr", &self.colerr())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[doc = "desc FSR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0x0100"]
impl crate::Resettable for FsrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
