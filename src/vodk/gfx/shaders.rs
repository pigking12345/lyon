
pub static TEXT_VERTEX_SHADER: &'static str = &"
attribute vec2 a_position;
attribute vec2 a_tex_coords;
varying vec2 v_tex_coords;
void main() {
  gl_Position = vec4(a_position, 0.0, 1.0);
  v_tex_coords = a_tex_coords;
}
";

pub static TEXT_FRAGMENT_SHADER : &'static str = &"
uniform vec4 u_color;
uniform sampler2D u_texture_0;
varying vec2 v_tex_coords;
void main() {
    gl_FragColor = u_color;
    gl_FragColor.a = texture2D(u_texture_0, v_tex_coords).r * 2.0;
}
";

pub static SOLID_COLOR_FRAGMENT_SHADER : &'static str = &"
uniform vec4 u_color;
void main() {
    gl_FragColor = u_color;
}
";

pub static TEXTURED_FRAGMENT_SHADER : &'static str = &"
uniform vec4 u_color;
uniform sampler2D u_texture_0;
varying vec2 v_tex_coords;
void main() {
    gl_FragColor = texture2D(u_texture_0, v_tex_coords);
}
";

pub static BASIC_VERTEX_SHADER_2D : &'static str = &"
attribute vec2 a_position;
attribute vec2 a_tex_coords;
uniform vec2 u_resolution;
varying vec2 v_tex_coords;
void main() {
  gl_Position = vec4((a_position - u_resolution/2.0) / u_resolution, 0.0, 1.0);
  v_tex_coords = a_tex_coords;
}
";

pub static BASIC_VERTEX_SHADER_3D : &'static str = &"
attribute vec3 a_position;
attribute vec3 a_normals;
attribute vec2 a_tex_coords;
uniform mat4 u_model_mat;
uniform mat4 u_view_mat;
uniform mat4 u_proj_mat;
varying vec3 v_normals;
varying vec2 v_tex_coords;
void main() {
    v_tex_coords = a_tex_coords;
    v_normals = a_normals;
    gl_Position = u_proj_mat * u_view_mat * u_model_mat * vec4(a_position, 1.0);
}
";

pub static NORMALS_FRAGMENT_SHADER : &'static str = &"
varying vec3 v_normals;
varying vec2 v_tex_coords;
void main() {
    vec3 normals = v_normals * 0.5 + vec3(0.5, 0.5, 0.5);
    gl_FragColor = vec4(normals, 1.0);
}
";
