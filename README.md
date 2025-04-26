Add .env file in root of project specifying graphics backend if your computer does not default backends (i.e. Vulkan on Windows/Linux, Metal on macOS).

Example:

```
WGPU_BACKEND = dx12
```

Choices:
- "vulkan"
- "metal"
- "dx12"
- "gl"
