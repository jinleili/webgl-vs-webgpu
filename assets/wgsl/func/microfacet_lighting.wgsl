// Based on the microfacet theory
// cf: https://qiita.com/mebiusbox2/items/e7063c5dfe1424e0d01a

struct Light {
    pos: vec4f,
    color: vec4f,
    ty: vec4u,
}

struct Material {
    albedo: vec4f,
    roughness: f32,
    reflectance: f32,
    ambient_ratio: f32,
    background_ratio: f32,
}

fn light_direction(light: Light, position: vec3f) -> vec3f {
    var res: vec3f;
    if (light.ty[0] == 0u) {
        res = normalize(light.pos.xyz - position);
    } else {
        res = light.pos.xyz;
    }
    return res;
}

fn irradiance(light: Light, position: vec3f, normal: vec3f) -> vec3f {
    let light_dir = light_direction(light, position);
    return light.color.xyz * clamp(dot(light_dir, normal), 0.0, 1.0);
}

fn diffuse_brdf(material: Material) -> vec3f {
    return material.albedo.xyz * (1.0 - material.reflectance);
}

fn microfacet_distribution(middle: vec3f, normal: vec3f, alpha: f32) -> f32 {
    let dotNH = dot(normal, middle);
    let alpha2 = alpha * alpha;
    let sqrt_denom = 1.0 - dotNH * dotNH * (1.0 - alpha2);
    return alpha2 / (sqrt_denom * sqrt_denom);
}

fn schlick_approxy(v: vec3f, normal: vec3f, k: f32) -> f32 {
    let dotNV = dot(normal, v);
    return dotNV / (dotNV * (1.0 - k) + k);
}

fn geometric_decay(light_dir: vec3f, camera_dir: vec3f, normal: vec3f, alpha: f32) -> f32 {
    let k = alpha / 2.0;
    return schlick_approxy(light_dir, normal, k) * schlick_approxy(camera_dir, normal, k);
}

fn fresnel(f0: vec3f, middle: vec3f, camera_dir: vec3f) -> vec3f {
    var c: f32 = 1.0 - dot(middle, camera_dir);
    c = c * c * c * c * c;
    return f0 + (1.0 - f0) * c;
}

fn specular_brdf(material: Material, camera_dir: vec3f, light_dir: vec3f, normal: vec3f) -> vec3f {
    let specular_color = material.albedo.xyz * material.reflectance;
    let middle = normalize(camera_dir + light_dir);
    let alpha = material.roughness * material.roughness;
    let distribution = microfacet_distribution(middle, normal, alpha);
    let decay = geometric_decay(light_dir, camera_dir, normal, alpha);
    let fresnel_color = fresnel(specular_color, middle, camera_dir);
    let dotCN = clamp(dot(camera_dir, normal), 0.0, 1.0);
    let dotLN = clamp(dot(light_dir, normal), 0.0, 1.0);
    let denom = 4.0 * dotCN * dotLN;
    if (denom < 1.0e-6) {
        return vec3f(0.0, 0.0, 0.0);
    }
    return distribution * decay / denom * fresnel_color;
}

fn microfacet_color(position: vec3f, normal: vec3f, light: Light, camera_dir: vec3f, material: Material) -> vec3f {
    let light_dir = light_direction(light, position);
    let irr = irradiance(light, position, normal);
    let diffuse = diffuse_brdf(material);
    let specular = specular_brdf(material, camera_dir, light_dir, normal);
    return (diffuse + specular) * irr;
}

fn ambient_correction(pre_color: vec3f, material: Material) -> vec3f {
    return pre_color * (1.0 - material.ambient_ratio)
        + material.albedo.xyz * material.ambient_ratio;
}

fn background_correction(pre_color: vec3f, bk_color: vec3f, material: Material) -> vec3f {
    return pre_color * (1.0 - material.background_ratio)
        + bk_color * material.background_ratio;
}

fn phong_color(position: vec3f, normal: vec3f, light: Light, camera_dir: vec3f, material: Material) -> vec3f {
    let light_dir = normalize(light.pos.xyz - position);
    let half_dir = normalize(camera_dir + light_dir);
    let diffuse = max(dot(normal, half_dir), 0.0);
    let specular = pow(max(dot(normal, half_dir), 0.0), 16.0) * 0.1;

    return (0.4 + diffuse + specular) * light.color.rgb;
}
