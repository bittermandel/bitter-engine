#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;


layout(set=1, binding=0) 
uniform Camera {
    vec3 u_view_position; // unused
    mat4 u_view_proj;
};

layout(set=2, binding=0) 
uniform DirectionalLight {
    vec3 u_light_position; // unused
    vec3 u_light_direction;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position.x, a_position.y, a_position.z, 1.0);
    v_normal = a_normal;
    v_tex_coords = a_tex_coords;
}