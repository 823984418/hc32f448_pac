#[doc = "Register `BACR` reader"]
pub type R = crate::R<BacrSpec>;
#[doc = "Register `BACR` writer"]
pub type W = crate::W<BacrSpec>;
#[doc = "Field `MUXMD` reader - desc MUXMD"]
pub type MuxmdR = crate::BitReader;
#[doc = "Field `MUXMD` writer - desc MUXMD"]
pub type MuxmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CkselR = crate::FieldReader;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 4 - desc MUXMD"]
    #[inline(always)]
    pub fn muxmd(&self) -> MuxmdR {
        MuxmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 14:15 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACR")
            .field("muxmd", &self.muxmd())
            .field("cksel", &self.cksel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - desc MUXMD"]
    #[inline(always)]
    pub fn muxmd(&mut self) -> MuxmdW<'_, BacrSpec> {
        MuxmdW::new(self, 4)
    }
    #[doc = "Bits 14:15 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<'_, BacrSpec> {
        CkselW::new(self, 14)
    }
}
#[doc = "desc BACR\n\nYou can [`read`](crate::Reg::read) this register and get [`bacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BacrSpec;
impl crate::RegisterSpec for BacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bacr::R`](R) reader structure"]
impl crate::Readable for BacrSpec {}
#[doc = "`write(|w| ..)` method takes [`bacr::W`](W) writer structure"]
impl crate::Writable for BacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACR to value 0x0300"]
impl crate::Resettable for BacrSpec {
    const RESET_VALUE: u32 = 0x0300;
}
