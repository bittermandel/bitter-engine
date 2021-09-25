#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec4 v_position;

layout(location=0) out vec4 f_color;
layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set=1, binding=0) 
uniform Camera {
    vec3 u_view_position;
    mat4 u_view_proj; // unused
};

layout(set=2, binding=0)
uniform Light {
    mat4 light_proj;
    vec4 light_position;
    vec4 light_color;
};

layout(set = 3, binding = 0) uniform texture2D t_shadow;
layout(set = 3, binding = 1) uniform samplerShadow t_shadow_sampler;

float shadow_calc(vec4 homogeneous_coords) {

    vec2 flip_correction = vec2(0.5, -0.5);
    float proj_correction = 1.0 / homogeneous_coords.w;

    vec2 light_local = homogeneous_coords.xy * flip_correction * proj_correction + vec2(0.5, 0.5);

    float shadow = texture(sampler2DShadow(t_shadow, t_shadow_sampler), vec3(light_local, homogeneous_coords.z * proj_correction));

    return shadow;
}

void main() {
    vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);

    vec4 homogeneous_coords = light_proj * v_position;
    float shadow = 1.0;
    if (homogeneous_coords.w <= 0.0) {
        float shadow = 1.0;
    } else {
        float shadow = shadow_calc(homogeneous_coords);  
    }

    float distance = length(light_position - v_position);
    float attenuation = 1.0 / (1.0 + 0.007 * distance + 0.002 * (distance * distance));

    vec3 ambient = vec3(0.05, 0.05, 0.05);

    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(light_position.xyz - v_position.xyz);
    float diffuse = max(0.0, dot(normal, light_dir));

    vec3 view_dir = normalize(u_view_position - v_position.xyz);
    vec3 half_dir = normalize(view_dir + light_dir);
    float specular = pow(max(dot(normal, half_dir), 0.0), 32);

    diffuse *= attenuation;
    ambient *= attenuation;
    specular *= attenuation;
    
    vec3 color = ambient + shadow * (diffuse * specular * light_color.xyz);

    f_color = vec4(color, 1.0) * object_color;
}