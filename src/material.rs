use std::{f64::consts::PI, sync::Arc};

use crate::{
    pdf::PdfEnum,
    pdf::{CosinePdf, UniformSpherePdf},
    Color, HitRecord, Random, Ray, TexturePtr, Vec3,
};

#[derive(Clone)]
pub enum EmitMaterial {
    DiffuseLight { emit: TexturePtr },
}

impl EmitMaterial {
    pub fn emmitted(&self, _: &Ray, rec: &HitRecord) -> Color {
        match self {
            EmitMaterial::DiffuseLight { emit } => {
                if rec.front_face {
                    emit.value(rec.u, rec.v, &rec.p)
                } else {
                    Color::default()
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PdfScatterRecord {
    pub attenuation: Color,
    pub pdf: PdfEnum<'static>,
}

impl PdfScatterRecord {
    pub fn new(attenuation: Color, pdf: PdfEnum<'static>) -> Self {
        Self { attenuation, pdf }
    }
}

#[derive(Clone)]
pub enum ScatterMaterial {
    Lambertian { albedo: TexturePtr },
    Isotropic { albedo: TexturePtr },
}

impl ScatterMaterial {
    pub fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<PdfScatterRecord> {
        match self {
            ScatterMaterial::Lambertian { albedo } => Some(PdfScatterRecord::new(
                albedo.value(rec.u, rec.v, &rec.p),
                PdfEnum::Cosine(CosinePdf::new(&rec.normal)),
            )),
            ScatterMaterial::Isotropic { albedo } => Some(PdfScatterRecord::new(
                albedo.value(rec.u, rec.v, &rec.p),
                PdfEnum::UniformSphere(UniformSpherePdf::new()),
            )),
        }
    }
    pub fn scattering_pdf(&self, _: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            ScatterMaterial::Lambertian { .. } => {
                let cosine = rec.normal.dot(&scattered.dir.unit_vector());
                if cosine < 0.0 {
                    0.0
                } else {
                    cosine / std::f64::consts::PI
                }
            }
            ScatterMaterial::Isotropic { .. } => 1.0 / (4.0 * PI),
        }
    }
}

#[derive(Clone)]
pub struct SpecularScatterRecord {
    pub attenuation: Color,
    pub ray: Ray,
}

impl SpecularScatterRecord {
    pub fn new(attenuation: Color, ray: Ray) -> Self {
        Self { attenuation, ray }
    }
}

#[derive(Clone)]
pub enum SpecularMaterial {
    Dielectric { ir: f64 },
    Metal { albedo: Color, fuzz: f64 },
}

impl SpecularMaterial {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut Random,
    ) -> Option<SpecularScatterRecord> {
        match self {
            SpecularMaterial::Dielectric { ir } => {
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { *ir };

                let unit_direction = r_in.dir.unit_vector();
                let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let direction = if refraction_ratio * sin_theta > 1.0
                    || Self::reflectance(cos_theta, refraction_ratio) > rng.unit_f64()
                {
                    unit_direction.reflect(&rec.normal)
                } else {
                    unit_direction.refract(&rec.normal, refraction_ratio)
                };

                Some(SpecularScatterRecord::new(
                    Color::new(1.0, 1.0, 1.0),
                    Ray::new(rec.p.clone(), direction, r_in.time),
                ))
            }
            SpecularMaterial::Metal { albedo, fuzz } => {
                let reflected = r_in.dir.unit_vector().reflect(&rec.normal);
                let scattered = Ray::new(
                    rec.p.clone(),
                    reflected + *fuzz * Vec3::random_in_unit_sphere(rng),
                    r_in.time,
                );
                if scattered.dir.dot(&rec.normal) > 0.0 {
                    Some(SpecularScatterRecord::new(albedo.clone(), scattered))
                } else {
                    None
                }
            }
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

#[derive(Clone)]
pub enum Material {
    Emit(EmitMaterial),
    Scatter(ScatterMaterial),
    Specular(SpecularMaterial),
}

pub type MaterialPtr = Arc<Material>;

impl Material {
    pub fn diffuse_light(emit: impl Into<TexturePtr>) -> MaterialPtr {
        Arc::new(Material::Emit(EmitMaterial::DiffuseLight {
            emit: emit.into(),
        }))
    }
    pub fn lambertian(albedo: impl Into<TexturePtr>) -> MaterialPtr {
        Arc::new(Material::Scatter(ScatterMaterial::Lambertian {
            albedo: albedo.into(),
        }))
    }
    pub fn isotropic(albedo: impl Into<TexturePtr>) -> MaterialPtr {
        Arc::new(Material::Scatter(ScatterMaterial::Isotropic {
            albedo: albedo.into(),
        }))
    }
    pub fn dielectric(ir: f64) -> MaterialPtr {
        Arc::new(Material::Specular(SpecularMaterial::Dielectric { ir }))
    }
    pub fn metal(albedo: Color, fuzz: f64) -> MaterialPtr {
        Arc::new(Material::Specular(SpecularMaterial::Metal { albedo, fuzz }))
    }
}
