#[doc = "Register `FIR` reader"]
pub type R = crate::R<FirSpec>;
#[doc = "Register `FIR` writer"]
pub type W = crate::W<FirSpec>;
#[doc = "Field `FCKS` reader - desc FCKS"]
pub type FcksR = crate::FieldReader;
#[doc = "Field `FCKS` writer - desc FCKS"]
pub type FcksW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CIEN` reader - desc CIEN"]
pub type CienR = crate::BitReader;
#[doc = "Field `CIEN` writer - desc CIEN"]
pub type CienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGS` reader - desc EDGS"]
pub type EdgsR = crate::FieldReader;
#[doc = "Field `EDGS` writer - desc EDGS"]
pub type EdgsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFF` reader - desc CFF"]
pub type CffR = crate::BitReader;
#[doc = "Field `CFF` writer - desc CFF"]
pub type CffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRF` reader - desc CRF"]
pub type CrfR = crate::BitReader;
#[doc = "Field `CRF` writer - desc CRF"]
pub type CrfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc FCKS"]
    #[inline(always)]
    pub fn fcks(&self) -> FcksR {
        FcksR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - desc CIEN"]
    #[inline(always)]
    pub fn cien(&self) -> CienR {
        CienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc EDGS"]
    #[inline(always)]
    pub fn edgs(&self) -> EdgsR {
        EdgsR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - desc CFF"]
    #[inline(always)]
    pub fn cff(&self) -> CffR {
        CffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CRF"]
    #[inline(always)]
    pub fn crf(&self) -> CrfR {
        CrfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIR")
            .field("fcks", &self.fcks())
            .field("cien", &self.cien())
            .field("edgs", &self.edgs())
            .field("cff", &self.cff())
            .field("crf", &self.crf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - desc FCKS"]
    #[inline(always)]
    pub fn fcks(&mut self) -> FcksW<'_, FirSpec> {
        FcksW::new(self, 0)
    }
    #[doc = "Bit 3 - desc CIEN"]
    #[inline(always)]
    pub fn cien(&mut self) -> CienW<'_, FirSpec> {
        CienW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc EDGS"]
    #[inline(always)]
    pub fn edgs(&mut self) -> EdgsW<'_, FirSpec> {
        EdgsW::new(self, 4)
    }
    #[doc = "Bit 6 - desc CFF"]
    #[inline(always)]
    pub fn cff(&mut self) -> CffW<'_, FirSpec> {
        CffW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CRF"]
    #[inline(always)]
    pub fn crf(&mut self) -> CrfW<'_, FirSpec> {
        CrfW::new(self, 7)
    }
}
#[doc = "desc FIR\n\nYou can [`read`](crate::Reg::read) this register and get [`fir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FirSpec;
impl crate::RegisterSpec for FirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fir::R`](R) reader structure"]
impl crate::Readable for FirSpec {}
#[doc = "`write(|w| ..)` method takes [`fir::W`](W) writer structure"]
impl crate::Writable for FirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIR to value 0"]
impl crate::Resettable for FirSpec {}
