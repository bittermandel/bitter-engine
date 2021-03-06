#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

layout(set=2, binding=0)
uniform Light {
    mat4 light_proj;
    vec4 light_position;
    vec4 light_color;
};

layout(location=5) in vec4 model_matrix_0;
layout(location=6) in vec4 model_matrix_1;
layout(location=7) in vec4 model_matrix_2;
layout(location=8) in vec4 model_matrix_3;

void main() {
    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );

    mat3 normal_matrix = mat3(transpose(inverse(model_matrix)));

    v_tex_coords = a_tex_coords;
    
    v_normal = normal_matrix * a_normal;

    vec4 model_space = model_matrix * vec4(a_position, 1.0);
    v_position = model_space.xyz;

    gl_Position = light_proj * model_space;
}