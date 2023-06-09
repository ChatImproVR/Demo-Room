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

// Galaga room walls and floor
pub const WALLS_GR_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.58, 0.525, 0.435, 1.0);
}
"#;

// Brown couches
pub const COUCH_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.102, 0.059, 0.024, 1.0);
}
"#;

// Light brown tables
pub const TABLE_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.361, 0.243, 0.153, 1.0);
}
"#;

// Hallway walls and floor
pub const WALLS_HALL_FRAG: &str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.3, 0.3, 0.3, 1.0);
}
"#;

// Main room walls and floor
pub const WALLS_MR_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.1, 0.1, 0.1, 1.0);
}
"#;


// Bowling room walls and floor
pub const WALLS_BR_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.2, 0.2, 0.2, 1.0);
}
"#;

// Light brown tables
pub const ALLEY_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.635, 0.529, 0.427, 1.0);
}
"#;


// lavender
pub const LAV_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.114, 0.078, 0.2, 1.0);
}
"#;

// off-white
pub const WHITE_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.8, 0.8, 0.8, 1.0);
}
"#;

// DARK GRAY
pub const BLACK_FRAG:&str = r#"
#version 410
precision mediump float;

uniform mat4 extra;

in vec4 f_color;

out vec4 out_color;

void main() {
    out_color = vec4(0.05, 0.05, 0.05, 1.0);
}
"#;