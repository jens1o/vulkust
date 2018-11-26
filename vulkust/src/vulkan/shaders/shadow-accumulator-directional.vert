#version 450

#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable
#extension GL_GOOGLE_include_directive : require

#include "common.glsl"

layout (constant_id = 0) const int DIRECTIONAL_CASCADES_COUNT = 6;

layout (location = 0) out vec2 out_uv;

layout (set = 0, binding = 0) uniform LightUBO { Light s; } light_ubo;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    ivec2 uv = ivec2(gl_VertexIndex & 2, (gl_VertexIndex << 1) & 2);
	out_uv = vec2(uv);
    uv <<= 1;
    uv -= 1;
	gl_Position = vec4(uv, 0.999, 1.0);
}
