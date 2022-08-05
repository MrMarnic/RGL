#version 450

layout(location=0) out vec4 f_color;

layout(location=0) in vec2 tex_coords_out;
layout(location=1) in vec3 normal;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 3, binding = 0)
uniform Light {
    vec4 pos;
    vec4 color;
};

void main() {
    vec4 texel = texture(sampler2D(t_diffuse, s_diffuse), tex_coords_out);
    f_color = texel;
}