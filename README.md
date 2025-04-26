Add .env file in root of project specifying graphics backend if your computer does not support default backends (i.e. Vulkan on Windows/Linux, Metal on macOS).

Example:

```
WGPU_BACKEND = dx12
```

Choices:
- "vulkan", "vk"
- "metal", "mtl"
- "dx12", "d3d12"
- "gl", "opengl", "gles"
- "webgpu"
