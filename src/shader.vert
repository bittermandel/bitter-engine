#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

layout(set=1, binding=0) 
uniform Camera {
    vec3 u_view_position; // unused
    mat4 u_view_proj;
};

layout(location=5) in vec4 model_matrix_0;
layout(location=6) in vec4 model_matrix_1;
layout(location=7) in vec4 model_matrix_2;
layout(location=8) in vec4 model_matrix_3;
layout(location=9) in vec3 normal_matrix_0;
layout(location=10) in vec3 normal_matrix_1;
layout(location=11) in vec3 normal_matrix_2;

void main() {
    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );

    mat3 normal_matrix = mat3(
        normal_matrix_0,
        normal_matrix_1,
        normal_matrix_2
    );

    v_tex_coords = a_tex_coords;
    
    v_normal = normal_matrix * a_normal;

    vec4 model_space = model_matrix * vec4(a_position, 1.0);
    v_position = model_space.xyz;

    gl_Position = u_view_proj * model_space;
}