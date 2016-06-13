#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;


uniform mat4 pos_matrix;
uniform mat4 rot_matrix;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = pos_matrix * rot_matrix * vec4(position, 0.0, 1.0);
}