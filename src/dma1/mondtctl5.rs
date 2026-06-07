#[doc = "Register `MONDTCTL5` reader"]
pub type R = crate::R<Mondtctl5Spec>;
#[doc = "Field `BLKSIZE` reader - desc BLKSIZE"]
pub type BlksizeR = crate::FieldReader<u16>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - desc BLKSIZE"]
    #[inline(always)]
    pub fn blksize(&self) -> BlksizeR {
        BlksizeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONDTCTL5")
            .field("blksize", &self.blksize())
            .field("cnt", &self.cnt())
            .finish()
    }
}
#[doc = "desc MONDTCTL5\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondtctl5Spec;
impl crate::RegisterSpec for Mondtctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondtctl5::R`](R) reader structure"]
impl crate::Readable for Mondtctl5Spec {}
#[doc = "`reset()` method sets MONDTCTL5 to value 0x01"]
impl crate::Resettable for Mondtctl5Spec {
    const RESET_VALUE: u32 = 0x01;
}
