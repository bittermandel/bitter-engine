#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;

layout(location=0) out vec3 v_color;

layout(set=0, binding=0) 
uniform Camera {
    vec3 u_view_position; // unused
    mat4 u_view_proj;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position.x, a_position.y, a_position.z, 1.0);

    v_color = a_color;
}