#[doc = "Register `PWRC0` reader"]
pub type R = crate::R<Pwrc0Spec>;
#[doc = "Register `PWRC0` writer"]
pub type W = crate::W<Pwrc0Spec>;
#[doc = "Field `PDMDS` reader - desc PDMDS"]
pub type PdmdsR = crate::FieldReader;
#[doc = "Field `PDMDS` writer - desc PDMDS"]
pub type PdmdsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IORTN` reader - desc IORTN"]
pub type IortnR = crate::FieldReader;
#[doc = "Field `IORTN` writer - desc IORTN"]
pub type IortnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWDN` reader - desc PWDN"]
pub type PwdnR = crate::BitReader;
#[doc = "Field `PWDN` writer - desc PWDN"]
pub type PwdnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc PDMDS"]
    #[inline(always)]
    pub fn pdmds(&self) -> PdmdsR {
        PdmdsR::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - desc IORTN"]
    #[inline(always)]
    pub fn iortn(&self) -> IortnR {
        IortnR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - desc PWDN"]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRC0")
            .field("pdmds", &self.pdmds())
            .field("iortn", &self.iortn())
            .field("pwdn", &self.pwdn())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PDMDS"]
    #[inline(always)]
    pub fn pdmds(&mut self) -> PdmdsW<'_, Pwrc0Spec> {
        PdmdsW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc IORTN"]
    #[inline(always)]
    pub fn iortn(&mut self) -> IortnW<'_, Pwrc0Spec> {
        IortnW::new(self, 4)
    }
    #[doc = "Bit 7 - desc PWDN"]
    #[inline(always)]
    pub fn pwdn(&mut self) -> PwdnW<'_, Pwrc0Spec> {
        PwdnW::new(self, 7)
    }
}
#[doc = "desc PWRC0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrc0Spec;
impl crate::RegisterSpec for Pwrc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwrc0::R`](R) reader structure"]
impl crate::Readable for Pwrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrc0::W`](W) writer structure"]
impl crate::Writable for Pwrc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRC0 to value 0"]
impl crate::Resettable for Pwrc0Spec {}
