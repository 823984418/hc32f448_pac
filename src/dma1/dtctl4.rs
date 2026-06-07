#[doc = "Register `DTCTL4` reader"]
pub type R = crate::R<Dtctl4Spec>;
#[doc = "Register `DTCTL4` writer"]
pub type W = crate::W<Dtctl4Spec>;
#[doc = "Field `BLKSIZE` reader - desc BLKSIZE"]
pub type BlksizeR = crate::FieldReader<u16>;
#[doc = "Field `BLKSIZE` writer - desc BLKSIZE"]
pub type BlksizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
        f.debug_struct("DTCTL4")
            .field("blksize", &self.blksize())
            .field("cnt", &self.cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - desc BLKSIZE"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BlksizeW<'_, Dtctl4Spec> {
        BlksizeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, Dtctl4Spec> {
        CntW::new(self, 16)
    }
}
#[doc = "desc DTCTL4\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dtctl4Spec;
impl crate::RegisterSpec for Dtctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctl4::R`](R) reader structure"]
impl crate::Readable for Dtctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`dtctl4::W`](W) writer structure"]
impl crate::Writable for Dtctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTL4 to value 0x01"]
impl crate::Resettable for Dtctl4Spec {
    const RESET_VALUE: u32 = 0x01;
}
