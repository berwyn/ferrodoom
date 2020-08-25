use std::{borrow::Cow, cell::RefCell};

use winit::window::Window;

#[derive(Default)]
pub struct Renderer {
    instance: RefCell<Option<wgpu::Instance>>,
    adapter: RefCell<Option<wgpu::Adapter>>,
    device: RefCell<Option<wgpu::Device>>,
    queue: RefCell<Option<wgpu::Queue>>,
    render_pipeline: RefCell<Option<wgpu::RenderPipeline>>,
    swapchain: RefCell<Option<wgpu::SwapChain>>,
}

impl Renderer {
    pub async fn prepare(&self, window: &Window) -> crate::Result<()> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Couldn't find a valid adapter!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("Failed to create device!");

        let vertex_module =
            device.create_shader_module(wgpu::include_spirv!("shaders/triangle.vert.spv"));
        let fragment_module =
            device.create_shader_module(wgpu::include_spirv!("shaders/triangle.frag.spv"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fragment_module,
                entry_point: "main",
            }),
            rasterization_state: None,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::TextureFormat::Bgra8UnormSrgb.into()],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let swapchain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swapchain = device.create_swap_chain(&surface, &swapchain_descriptor);

        self.instance.replace(Some(instance));
        self.adapter.replace(Some(adapter));
        self.device.replace(Some(device));
        self.queue.replace(Some(queue));
        self.render_pipeline.replace(Some(render_pipeline));
        self.swapchain.replace(Some(swapchain));

        Ok(())
    }

    pub fn render(&self) {
        let mut swapchain = self.swapchain.borrow_mut();
        let swapchain = swapchain.as_mut().unwrap();

        let queue = self.queue.borrow();
        let queue = queue.as_ref().unwrap();

        let render_pipeline = self.render_pipeline.borrow();
        let render_pipeline = render_pipeline.as_ref().unwrap();

        let device = self.device.borrow();
        let device = device.as_ref().unwrap();

        let frame = swapchain
            .get_current_frame()
            .expect("Unable to acquire next swapchain texture!");

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }
}
