use std::io::{BufRead, BufReader};
use std::sync::Arc;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::Triangle;
use crate::raytracer::lights::Light;
use crate::raytracer::loaders::loader::Loader;
use crate::raytracer::materials::{Material, MaterialPrimitive};
use crate::raytracer::movements::movement::Movement;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::Vec3;
use crate::raytracer::{hittables, materials, textures};
use obj::raw::object::Polygon;
use obj::{load_obj, raw, Obj};
use std::fs::File;
use std::io;
use std::path::Path;

pub struct ObjLoader {}

impl Loader for ObjLoader {
    fn load(input: &Path) -> Result<(Vec<Primitive>, Vec<Arc<dyn Light>>), std::io::Error> {
        let mut primitives: Vec<Primitive> = vec![];
        let mut lights: Vec<Arc<dyn Light>> = vec![];
        let a = BufReader::new(File::open(input).unwrap());
        let mut object = raw::parse_obj(a).unwrap();
        let diffuse: Arc<dyn MaterialPrimitive> = Arc::new(materials::Plain {});
        let colors: Vec<Arc<dyn Texture>> = vec![
            Arc::new(textures::Perlin::new(Color::BLUE)),
            Arc::new(textures::Perlin::new(Color::YELLOW)),
            Arc::new(textures::Perlin::new(Color::WHITE)),
            Arc::new(textures::Perlin::new(Color::PURPLE)),
            Arc::new(textures::Perlin::new(Color::RED)),
            Arc::new(textures::Perlin::new(Color::GREEN)),
        ];
        let mut i = 0;

        /*for obj in &object.objects {
            for grp in &obj.groups {
                let polys: Vec<Primitive> = grp
                    .polys
                    .iter()
                    .map(|x| Primitive {
                        hittable: Arc::new(Triangle::new(
                            Vec3::from(object.position[x[0].0]),
                            Vec3::from(object.position[x[1].0]),
                            Vec3::from(object.position[x[2].0]),
                            Movement::NONE,
                        )),
                        material: Arc::new(Material::new(vec![(1.0, diffuse.clone())])),
                        texture: colors[0].clone(),
                    })
                    .collect();
                /*.vertex(|IndexTuple(p, t, n)| {
                    Primitive {
                        hittable: Arc::new(Triangle::new(object.position[p], )),
                        material: Arc::new(()),
                        texture: Arc::new(()),
                    object.position[p],
                        t.map_or([0., 0.], |t| object.texture[t]),
                        n.map_or([1., 0., 0.], |n| object.normal[n]),

                }
                })
                .collect();*/
            }
        }*/

        for poly in object.polygons {
            i += 1;
            match poly {
                Polygon::P(positions) => {
                    if positions.len() == 3 {
                        primitives.push(Primitive {
                            hittable: Arc::new(hittables::Triangle::new(
                                Vec3::from(object.positions[positions[0]]),
                                Vec3::from(object.positions[positions[1]]),
                                Vec3::from(object.positions[positions[2]]),
                                Movement::NONE,
                            )),
                            material: Arc::new(Material::new(vec![(1.0, diffuse.clone())])),
                            texture: colors[i % colors.len()].clone(),
                        });
                    }
                }
                Polygon::PT(positions) => {
                    if positions.len() == 3 {
                        primitives.push(Primitive {
                            hittable: Arc::new(hittables::Triangle::new(
                                Vec3::from(object.positions[positions[0].0]),
                                Vec3::from(object.positions[positions[1].0]),
                                Vec3::from(object.positions[positions[2].0]),
                                Movement::NONE,
                            )),
                            material: Arc::new(Material::new(vec![(1.0, diffuse.clone())])),
                            texture: colors[i % colors.len()].clone(),
                        });
                    }
                }
                Polygon::PN(positions) => {
                    if positions.len() == 3 {
                        primitives.push(Primitive {
                            hittable: Arc::new(hittables::Triangle::new(
                                Vec3::from(object.positions[positions[0].0]),
                                Vec3::from(object.positions[positions[1].0]),
                                Vec3::from(object.positions[positions[2].0]),
                                Movement::NONE,
                            )),
                            material: Arc::new(Material::new(vec![(1.0, diffuse.clone())])),
                            texture: colors[i % colors.len()].clone(),
                        });
                    }
                }
                Polygon::PTN(positions) => {
                    if positions.len() == 3 {
                        primitives.push(Primitive {
                            hittable: Arc::new(hittables::Triangle::new(
                                Vec3::from(object.positions[positions[0].0]),
                                Vec3::from(object.positions[positions[1].0]),
                                Vec3::from(object.positions[positions[2].0]),
                                Movement::NONE,
                            )),
                            material: Arc::new(Material::new(vec![(1.0, diffuse.clone())])),
                            texture: colors[i % colors.len()].clone(),
                        });
                    }
                }
            }
        }

        Ok((primitives, lights))
    }
}
