#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 v_normal;

layout(location=0) out vec4 f_color;


layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

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
    vec4 texture_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    
    vec3 lightDir = normalize(-u_light_direction);

    float diffuse = max(dot(v_normal, lightDir), 0.0);

    f_color = diffuse * texture_color;
} 