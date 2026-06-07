#[doc = "Register `SCCR` reader"]
pub type R = crate::R<SccrSpec>;
#[doc = "Register `SCCR` writer"]
pub type W = crate::W<SccrSpec>;
#[doc = "Field `SISL` reader - desc SISL"]
pub type SislR = crate::FieldReader;
#[doc = "Field `SISL` writer - desc SISL"]
pub type SislW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPRD` reader - desc SPRD"]
pub type SprdR = crate::FieldReader;
#[doc = "Field `SPRD` writer - desc SPRD"]
pub type SprdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSTB` reader - desc SSTB"]
pub type SstbR = crate::FieldReader;
#[doc = "Field `SSTB` writer - desc SSTB"]
pub type SstbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - desc SISL"]
    #[inline(always)]
    pub fn sisl(&self) -> SislR {
        SislR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - desc SPRD"]
    #[inline(always)]
    pub fn sprd(&self) -> SprdR {
        SprdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - desc SSTB"]
    #[inline(always)]
    pub fn sstb(&self) -> SstbR {
        SstbR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCCR")
            .field("sisl", &self.sisl())
            .field("sprd", &self.sprd())
            .field("sstb", &self.sstb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SISL"]
    #[inline(always)]
    pub fn sisl(&mut self) -> SislW<'_, SccrSpec> {
        SislW::new(self, 0)
    }
    #[doc = "Bits 16:23 - desc SPRD"]
    #[inline(always)]
    pub fn sprd(&mut self) -> SprdW<'_, SccrSpec> {
        SprdW::new(self, 16)
    }
    #[doc = "Bits 24:29 - desc SSTB"]
    #[inline(always)]
    pub fn sstb(&mut self) -> SstbW<'_, SccrSpec> {
        SstbW::new(self, 24)
    }
}
#[doc = "desc SCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`sccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SccrSpec;
impl crate::RegisterSpec for SccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sccr::R`](R) reader structure"]
impl crate::Readable for SccrSpec {}
#[doc = "`write(|w| ..)` method takes [`sccr::W`](W) writer structure"]
impl crate::Writable for SccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCR to value 0"]
impl crate::Resettable for SccrSpec {}
