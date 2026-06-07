#[doc = "Register `MONDTCTL4` reader"]
pub type R = crate::R<Mondtctl4Spec>;
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
        f.debug_struct("MONDTCTL4")
            .field("blksize", &self.blksize())
            .field("cnt", &self.cnt())
            .finish()
    }
}
#[doc = "desc MONDTCTL4\n\nYou can [`read`](crate::Reg::read) this register and get [`mondtctl4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondtctl4Spec;
impl crate::RegisterSpec for Mondtctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondtctl4::R`](R) reader structure"]
impl crate::Readable for Mondtctl4Spec {}
#[doc = "`reset()` method sets MONDTCTL4 to value 0x01"]
impl crate::Resettable for Mondtctl4Spec {
    const RESET_VALUE: u32 = 0x01;
}
