use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::hittables::hittable::Hittables;
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::consts::{MAX_ITERATION, ZERO};
use crate::raytracer::utils::vec::Vec2;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::pipeline::ComputePipeline;
use vulkano::sync::GpuFuture;

pub struct VulkanoIntegrator {
    camera: Camera,
    scene: Scene,
}

impl VulkanoIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> VulkanoIntegrator {
        VulkanoIntegrator { camera, scene }
    }
}

impl Integrator for VulkanoIntegrator {
    fn preprocess(&mut self) {}

    fn render(&mut self) {
        // Create vulkano instance
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("failed to create instance");
        // Get first physical device
        let physical = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no device available");
        // Get first queue which support computing
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family");

        // Create device and queues
        let (device, mut queues) = {
            Device::new(
                physical,
                &Features::none(),
                &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned(),
            )
            .expect("failed to create device")
        };

        // Get first queue
        let queue = queues.next().unwrap();
        // Create random generator needed by trace
        let mut rng = rand::XorShiftRng::new_unseeded();

        let mut spheres: Vec<Sphere> = vec![];

        // Create buffer of spheres
        for primitive in self.scene.primitives.iter() {
            if primitive.hittable.get_type() == Hittables::Sphere {
                spheres.push(primitive.hittable.to_sphere().unwrap());
            }
        }

        let sphere_buffer =
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), spheres).unwrap();

        // Create sphere_pipeline
        let sphere_pipeline = Arc::new({
            mod cs {
                vulkano_shaders::shader! {
                    ty: "compute",
                    src: "
#version 450

layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Spheres {
    float c_x[];
    float c_y[];
    float c_z[];
    float r_2[];
} sphere;

layout(set = 1, binding = 0) buffer Rays {
    float o_x[];
    float o_y[];
    float o_z[];
    float d_x[];
    float d_y[];
    float d_z[];
} rays;

layout(set = 2, binding = 0) buffer Distances {
    float distance[];
} distances;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    uint idy = gl_GlobalInvocationID.y;
    distances.distance[idx] *= 12;
}"
                }
            }
            let shader = cs::Shader::load(device.clone()).unwrap();
            ComputePipeline::new(device.clone(), &shader.main_entry_point(), &()).unwrap()
        });

        let set_spheres = Arc::new(
            PersistentDescriptorSet::start(sphere_pipeline.clone(), 0)
                .add_buffer(sphere_buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        while let Some(mut tile) = self.camera.next_tile() {
            // Create ray buffer
            let mut rays: Vec<Ray> = vec![];
            for pixel_index in 0..tile.rays.len() {
                for ray_index in 0..tile.rays[pixel_index].len() {
                    rays.push(tile.rays[pixel_index][ray_index].clone());
                }
            }

            let ray_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), rays).unwrap();

            // Create datasets
            let set_rays = Arc::new(
                PersistentDescriptorSet::start(sphere_pipeline.clone(), 1)
                    .add_buffer(ray_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            // Distance datasets
            let distance_buffer =
                CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), [0f32; 30])
                    .unwrap();

            let set_distance = Arc::new(
                PersistentDescriptorSet::start(sphere_pipeline.clone(), 2)
                    .add_buffer(distance_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
                .unwrap()
                .dispatch(
                    [1, 1, 1],
                    sphere_pipeline.clone(),
                    (set_rays.clone(), set_spheres.clone(), set_distance.clone()),
                    (),
                )
                .unwrap()
                .build()
                .unwrap();

            let finished = command_buffer.execute(queue.clone()).unwrap();
            finished
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            panic!("Fini sans erreur");
            self.camera.merge_tile(&tile);
        }
        self.camera.save();
    }
}
