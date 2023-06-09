// pub const VERTEX_SHADER_TEST: &str = r#"
// #version 410

// uniform mat4 view;
// uniform mat4 proj;
// uniform mat4 transf;

// layout (location = 0) in vec3 pos;
// layout (location = 1) in vec3 uvw;

// out vec4 f_color;

// void main() {
//     gl_Position = proj * view * transf * vec4(pos, 1.);
//     vec4 extra_c = extra[0];
//     vec3 color = mix(uvw, extra_c.xyz, extra_c.w);
//     gl_PointSize = 2.0;
//     f_color = vec4(color, 1.);
// }
// "#;

// pub const FRAGMENT_SHADER_TEST: &str = r#"
// #version 410
// precision mediump float;

// in vec4 f_color;

// out vec4 out_color;

// void main() {
//     out_color = vec4(1., 0., 0., 1.);
// }
// "#;

// Gradient shader-- for avatars?
pub const GRADIENT_VERT: &str = r#"
#version 410

uniform mat4 view;
uniform mat4 proj;
uniform mat4 transf;
uniform mat4 extra;

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 uvw;

out vec4 f_color;

void main() {
    gl_Position = proj * view * transf * vec4(pos, 1.);
    vec4 extra_c = extra[0];
    vec3 color = mix(uvw, extra_c.xyz, extra_c.w);
    gl_PointSize = 2.0;
    f_color = vec4(color, 1.);
}
"#;

pub const GRADIENT_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    // Get the normalized coordinates of the fragment within the object
    vec2 uv = gl_FragCoord.xy / vec2(extra[2][0], extra[2][1]);

    // Calculate the vertical gradient based on the fragment's y-coordinate
    float gradient = uv.y;

    // Set the output color with the gradient effect
    out_color = vec4(gradient, gradient, gradient, 1.0);
}
"#;

// Cyan
pub const LAV_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.651, 0.576, 0.851, 1.0);
}
"#;