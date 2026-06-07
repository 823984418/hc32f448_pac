#[doc = "Register `STR` reader"]
pub type R = crate::R<StrSpec>;
#[doc = "Field `TRIMOK` reader - desc TRIMOK"]
pub type TrimokR = crate::BitReader;
#[doc = "Field `TRMOVF` reader - desc TRMOVF"]
pub type TrmovfR = crate::BitReader;
#[doc = "Field `TRMUDF` reader - desc TRMUDF"]
pub type TrmudfR = crate::BitReader;
#[doc = "Field `CTCBSY` reader - desc CTCBSY"]
pub type CtcbsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc TRIMOK"]
    #[inline(always)]
    pub fn trimok(&self) -> TrimokR {
        TrimokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TRMOVF"]
    #[inline(always)]
    pub fn trmovf(&self) -> TrmovfR {
        TrmovfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TRMUDF"]
    #[inline(always)]
    pub fn trmudf(&self) -> TrmudfR {
        TrmudfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CTCBSY"]
    #[inline(always)]
    pub fn ctcbsy(&self) -> CtcbsyR {
        CtcbsyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR")
            .field("trimok", &self.trimok())
            .field("trmovf", &self.trmovf())
            .field("trmudf", &self.trmudf())
            .field("ctcbsy", &self.ctcbsy())
            .finish()
    }
}
#[doc = "desc STR\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrSpec;
impl crate::RegisterSpec for StrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for StrSpec {}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for StrSpec {}
