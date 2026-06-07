#[doc = "Register `GFC` reader"]
pub type R = crate::R<GfcSpec>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GfcSpec>;
#[doc = "Field `RRFE` reader - desc RRFE"]
pub type RrfeR = crate::BitReader;
#[doc = "Field `RRFE` writer - desc RRFE"]
pub type RrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - desc RRFS"]
pub type RrfsR = crate::BitReader;
#[doc = "Field `RRFS` writer - desc RRFS"]
pub type RrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - desc ANFE"]
pub type AnfeR = crate::FieldReader;
#[doc = "Field `ANFE` writer - desc ANFE"]
pub type AnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - desc ANFS"]
pub type AnfsR = crate::FieldReader;
#[doc = "Field `ANFS` writer - desc ANFS"]
pub type AnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RrfeR {
        RrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RrfsR {
        RrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc ANFE"]
    #[inline(always)]
    pub fn anfe(&self) -> AnfeR {
        AnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc ANFS"]
    #[inline(always)]
    pub fn anfs(&self) -> AnfsR {
        AnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc RRFE"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RrfeW<'_, GfcSpec> {
        RrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RRFS"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RrfsW<'_, GfcSpec> {
        RrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc ANFE"]
    #[inline(always)]
    pub fn anfe(&mut self) -> AnfeW<'_, GfcSpec> {
        AnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc ANFS"]
    #[inline(always)]
    pub fn anfs(&mut self) -> AnfsW<'_, GfcSpec> {
        AnfsW::new(self, 4)
    }
}
#[doc = "desc GFC\n\nYou can [`read`](crate::Reg::read) this register and get [`gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfcSpec;
impl crate::RegisterSpec for GfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GfcSpec {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GfcSpec {}
