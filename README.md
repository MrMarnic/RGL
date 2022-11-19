# RustGameLibrary
Custom made game engine in Rust

# Functionality
- Image Rendering
- Text Rendering (On the fly font loading and scaling)
- Line rendering
- Obj model rendering
- Lighting Support

# How does it work
It uses Wgpu (Frontend for Vulkan/DX12/Metal)
Shader language = GLSL

# Example

```
fn main() {
    let mut game_window = GameWindow::new("Test".to_string(),1100,1100,200,200,Color::new(44,93,130),true);
    let game_events = GameEvents::new();
    let mut scene_handler = SceneHandler::new();
    scene_handler.opened_scene = Box::new(LoadingScene {});

    block_on(game_events.run(scene_handler,game_window,Backends::DX12));
}
```
