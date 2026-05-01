#![allow(missing_docs)]

use iced::mouse;
use iced::widget::{scrollable, text};
use iced::{Color, Element, Size, Theme};
use iced_wgpu::Renderer;
use iced_wgpu::wgpu;

pub fn main() {
    use iced_futures::futures::executor;
    use iced_wgpu::wgpu;

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .expect("request adapter");

    let (device, queue) = executor::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
        trace: wgpu::Trace::Off,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
    }))
    .expect("request device");

    benchmark(&adapter, &device, &queue);
}

fn benchmark(adapter: &wgpu::Adapter, device: &wgpu::Device, queue: &wgpu::Queue) {
    use iced_wgpu::graphics;
    use iced_wgpu::graphics::{Antialiasing, Shell};
    use iced_wgpu::wgpu;
    use iced_winit::core;
    use iced_winit::core::renderer;
    use iced_winit::runtime;

    let format = wgpu::TextureFormat::Bgra8UnormSrgb;

    let engine = iced_wgpu::Engine::new(
        adapter,
        device.clone(),
        queue.clone(),
        format,
        Some(Antialiasing::MSAAx4),
        Shell::headless(),
    );

    let mut renderer = Renderer::new(engine, renderer::Settings::default());

    let viewport = graphics::Viewport::with_physical_size(Size::new(3840, 2160), 2.0);

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: 3840,
            height: 2160,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut cache = Some(runtime::user_interface::Cache::default());

    for i in 0..100 {
        let mut user_interface = runtime::UserInterface::build(
            advanced_shaping_jp::<()>(i, 100),
            viewport.logical_size(),
            cache.take().unwrap(),
            &mut renderer,
        );

        user_interface.draw(
            &mut renderer,
            &Theme::Dark,
            &core::renderer::Style {
                text_color: Color::WHITE,
            },
            mouse::Cursor::Unavailable,
        );

        cache = Some(user_interface.into_cache());

        let submission = renderer.present(Some(Color::BLACK), format, &texture_view, &viewport);

        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: None,
        });
    }
}

fn advanced_shaping_jp<'a, Message: 'a>(
    n: usize,
    i: usize,
) -> Element<'a, Message, Theme, Renderer> {
    const LOREM_IPSUM: &str = include_str!("ipsum_jp.txt");

    scrollable(
        text!(
            "{}... Iteration {i} 😎",
            std::iter::repeat(LOREM_IPSUM.chars())
                .flatten()
                .take(n)
                .collect::<String>(),
        )
        .shaping(text::Shaping::Advanced)
        .size(10),
    )
    .into()
}
