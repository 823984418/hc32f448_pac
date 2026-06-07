#[doc = "Register `PFSRC4` reader"]
pub type R = crate::R<Pfsrc4Spec>;
#[doc = "Register `PFSRC4` writer"]
pub type W = crate::W<Pfsrc4Spec>;
#[doc = "Field `FSEL` reader - desc FSEL"]
pub type FselR = crate::FieldReader;
#[doc = "Field `FSEL` writer - desc FSEL"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BFE` reader - desc BFE"]
pub type BfeR = crate::BitReader;
#[doc = "Field `BFE` writer - desc BFE"]
pub type BfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&self) -> BfeR {
        BfeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFSRC4")
            .field("fsel", &self.fsel())
            .field("bfe", &self.bfe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<'_, Pfsrc4Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsrc4Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRC4\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsrc4Spec;
impl crate::RegisterSpec for Pfsrc4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrc4::R`](R) reader structure"]
impl crate::Readable for Pfsrc4Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsrc4::W`](W) writer structure"]
impl crate::Writable for Pfsrc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRC4 to value 0"]
impl crate::Resettable for Pfsrc4Spec {}
