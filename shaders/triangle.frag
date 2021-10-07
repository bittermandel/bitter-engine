#version 450

layout(location=0) in vec3 v_color;
layout(location=0) out vec4 f_color;

layout(set=1, binding=0) 
uniform DirectionalLight {
    vec3 u_light_position; // unused
    vec3 u_light_direction;
};

void main()
{
    f_color = vec4(v_color, 1.0);
} 