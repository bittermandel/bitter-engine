#version 450

layout(location=0) in vec3 a_position;
layout(location=0) out vec3 v_color;

layout(set=0, binding=0)
uniform Camera {
    vec3 u_view_position; // unused
    mat4 u_view_proj;
};

layout(set=1, binding=0)
uniform Light {
    mat4 light_proj;
    vec4 light_position;
    vec4 light_color;
};

// Let's keep our light smaller than our other objects
float scale = 0.25;

void main() {
    vec3 v_position = a_position * scale + light_position.xyz;
    gl_Position = u_view_proj * vec4(v_position, 1);

    v_color = light_color.xyz;
}